//! Request-classifier benchmark, run by `cargo test` — regex vs. the in-process local model
//! (vs. Laya, when `LAYA_URL` points at a reachable `laya-serve`), through the router's own
//! code paths, on three labelled sets:
//!
//! - `eval/h4-validation.json` — 154 held-out requests, 22 per type, each tagged with the
//!   failure mode it exercises (keyword traps, no keywords, multilingual, typos, ...).
//! - `eval/h4-public-sample.json` — the hackathon's 10 public examples
//!   (https://registry.nasiko.dev/r/nasiko/classifier-eval).
//! - `eval/requests.jsonl` — 100 hand-labelled requests with real chat history, classified
//!   from `[Message]`s exactly as a live request is (`classify_input`).
//!
//! For every backend it measures what routing depends on: type accuracy / macro-F1 / per-type
//! F1; complexity exact, ±1, MAE and hard-request recall; confidence calibration (ECE, Brier);
//! latency p50/p95/p99; and the routing outcome — each classification goes through
//! `pick_tier` over fixed seeds at cold start, giving the mean tier cost and how often hard /
//! easy requests land on the cheapest tier. Accuracy is then broken down by failure-mode tag.
//!
//! The quality gates at the bottom fail the build if a retrained model or a feature change
//! makes the local classifier regress. The report prints with `-- --nocapture`, and is
//! written to `BENCH_REPORT` when that is set (CI points it at the job summary):
//!
//! ```sh
//! cargo test --release -p nasiko-llm-router --test classifier_benchmark -- --nocapture
//! LAYA_URL=http://localhost:8000 cargo test --release ... # adds the Laya column
//! ```

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use nasiko_llm_router::ir::Message;
use nasiko_llm_router::routing::classifier::{CellMap, Tier, pick_tier, tier_cost};
use nasiko_llm_router::routing::laya::{LayaClassifier, probe_health};
use nasiko_llm_router::routing::local_classifier::{LocalClassifier, eval_state};
use nasiko_llm_router::routing::request_classifier::classify_input;
use nasiko_llm_router::routing::{
    Classification, ClassifierInput, RegexClassifier, RequestClassifier, RequestType,
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde_json::{Value, json};

const TYPES: [RequestType; 7] = [
    RequestType::CodeGeneration,
    RequestType::CodeUnderstanding,
    RequestType::TechnicalDesign,
    RequestType::AnalyticalReasoning,
    RequestType::Writing,
    RequestType::FactualLookup,
    RequestType::General,
];
/// Cold-start Thompson draws per request for the routing columns.
const SEEDS: u64 = 50;

struct Case {
    id: String,
    query: String,
    /// The exact classifier state the router would build for this request.
    state: String,
    gold_type: RequestType,
    gold_complexity: u8,
    tags: Vec<String>,
}

struct Scored {
    case_idx: usize,
    c: Classification,
    latency: Duration,
}

fn eval_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("eval")
        .join(name)
}

fn gold_type(label: &str) -> RequestType {
    RequestType::from_wire(label).unwrap_or_else(|| panic!("unknown request type {label}"))
}

/// An H4-schema file: `{"examples": [{id, query, context, request_type, complexity, tests}]}`.
fn load_h4(name: &str) -> Vec<Case> {
    let raw = std::fs::read_to_string(eval_path(name)).expect("read eval set");
    let data: Value = serde_json::from_str(&raw).expect("valid eval JSON");
    data["examples"]
        .as_array()
        .expect("examples")
        .iter()
        .map(|e| {
            let query = e["query"].as_str().expect("query").to_string();
            Case {
                id: e["id"].as_str().expect("id").into(),
                state: eval_state(&query, e["context"].as_str()),
                query,
                gold_type: gold_type(e["request_type"].as_str().expect("request_type")),
                gold_complexity: e["complexity"].as_u64().expect("complexity") as u8,
                tags: e["tests"]
                    .as_array()
                    .map(|t| {
                        t.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        })
        .collect()
}

/// `requests.jsonl`: chat history as messages, classified through `classify_input`.
fn load_requests() -> Vec<Case> {
    let raw = std::fs::read_to_string(eval_path("requests.jsonl")).expect("read requests");
    raw.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let r: Value = serde_json::from_str(line).expect("valid row");
            let history = r["context"].as_array().expect("context");
            let mut turns: Vec<Value> = history
                .iter()
                .map(|m| json!({ "role": m["role"], "content": m["text"] }))
                .collect();
            turns.push(json!({ "role": "user", "content": r["query"] }));
            let messages: Vec<Message> =
                serde_json::from_value(Value::Array(turns)).expect("messages");
            Case {
                id: r["id"].as_str().expect("id").into(),
                query: r["query"].as_str().expect("query").into(),
                state: classify_input(&messages).expect("a user turn"),
                gold_type: gold_type(r["type"].as_str().expect("type")),
                gold_complexity: r["complexity"].as_u64().expect("complexity") as u8,
                tags: vec![
                    if history.is_empty() {
                        "plain"
                    } else {
                        "context_dependent"
                    }
                    .into(),
                ],
            }
        })
        .collect()
}

async fn run(classifier: &dyn RequestClassifier, cases: &[Case]) -> Vec<Scored> {
    // Warm-up: lazily compiled regexes / model load / connection setup stay out of latency.
    classifier
        .classify(&ClassifierInput {
            query: "hello",
            state: "Latest request:\nhello",
        })
        .await;
    let mut out = Vec::with_capacity(cases.len());
    for (case_idx, case) in cases.iter().enumerate() {
        let started = Instant::now();
        let c = classifier
            .classify(&ClassifierInput {
                query: &case.query,
                state: &case.state,
            })
            .await;
        out.push(Scored {
            case_idx,
            c,
            latency: started.elapsed(),
        });
    }
    out
}

#[derive(Default)]
struct Metrics {
    accuracy: f64,
    macro_f1: f64,
    f1: BTreeMap<&'static str, f64>,
    cx_exact: f64,
    cx_within1: f64,
    cx_mae: f64,
    hard_recall: f64,
    ece: f64,
    brier: f64,
    p50_us: f64,
    p95_us: f64,
    p99_us: f64,
    mean_cost: f64,
    hard_to_cheapest: f64,
    easy_to_cheapest: f64,
    by_tag: BTreeMap<String, (usize, usize)>,
    fallbacks: usize,
    /// Share of requests below `LOW_CONFIDENCE_FLOOR` (served the configured model, unpinned).
    low_confidence: f64,
    /// Type accuracy on the requests the router does trust (at or above the floor).
    confident_accuracy: f64,
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let i = ((q * sorted.len() as f64).ceil() as usize).clamp(1, sorted.len()) - 1;
    sorted[i]
}

fn metrics(cases: &[Case], scored: &[Scored]) -> Metrics {
    let n = scored.len() as f64;
    let mut m = Metrics::default();
    let correct: Vec<bool> = scored
        .iter()
        .map(|s| s.c.request_type == cases[s.case_idx].gold_type)
        .collect();
    m.accuracy = correct.iter().filter(|c| **c).count() as f64 / n;

    for t in TYPES {
        let tp = scored
            .iter()
            .filter(|s| s.c.request_type == t && cases[s.case_idx].gold_type == t)
            .count() as f64;
        let predicted = scored.iter().filter(|s| s.c.request_type == t).count() as f64;
        let actual = scored
            .iter()
            .filter(|s| cases[s.case_idx].gold_type == t)
            .count() as f64;
        let (p, r) = (
            if predicted > 0.0 { tp / predicted } else { 0.0 },
            if actual > 0.0 { tp / actual } else { 0.0 },
        );
        m.f1.insert(
            t.as_str(),
            if p + r > 0.0 {
                2.0 * p * r / (p + r)
            } else {
                0.0
            },
        );
    }
    m.macro_f1 = m.f1.values().sum::<f64>() / TYPES.len() as f64;

    let diffs: Vec<i32> = scored
        .iter()
        .map(|s| i32::from(s.c.complexity) - i32::from(cases[s.case_idx].gold_complexity))
        .collect();
    m.cx_exact = diffs.iter().filter(|d| **d == 0).count() as f64 / n;
    m.cx_within1 = diffs.iter().filter(|d| d.abs() <= 1).count() as f64 / n;
    m.cx_mae = diffs.iter().map(|d| f64::from(d.abs())).sum::<f64>() / n;
    let hard: Vec<&Scored> = scored
        .iter()
        .filter(|s| cases[s.case_idx].gold_complexity >= 4)
        .collect();
    m.hard_recall = if hard.is_empty() {
        f64::NAN
    } else {
        hard.iter().filter(|s| s.c.complexity >= 4).count() as f64 / hard.len() as f64
    };

    // Calibration of the type confidence: 10-bin ECE and Brier score.
    let mut bins = [(0usize, 0.0f64, 0.0f64); 10];
    for (s, ok) in scored.iter().zip(&correct) {
        let b = ((s.c.confidence * 10.0) as usize).min(9);
        bins[b].0 += 1;
        bins[b].1 += s.c.confidence;
        bins[b].2 += f64::from(u8::from(*ok));
    }
    m.ece = bins
        .iter()
        .filter(|b| b.0 > 0)
        .map(|(k, conf, acc)| (*k as f64 / n) * ((acc - conf) / *k as f64).abs())
        .sum();
    m.brier = scored
        .iter()
        .zip(&correct)
        .map(|(s, ok)| (s.c.confidence - f64::from(u8::from(*ok))).powi(2))
        .sum::<f64>()
        / n;

    let mut lat: Vec<f64> = scored
        .iter()
        .map(|s| s.latency.as_secs_f64() * 1e6)
        .collect();
    lat.sort_by(f64::total_cmp);
    (m.p50_us, m.p95_us, m.p99_us) = (
        quantile(&lat, 0.50),
        quantile(&lat, 0.95),
        quantile(&lat, 0.99),
    );

    // Routing outcome: the router's own tier sampler at cold start.
    let cells = CellMap::new();
    let (mut cost, mut hard_cheap, mut hard_n, mut easy_cheap, mut easy_n) = (0.0, 0, 0, 0, 0);
    for s in scored {
        let gold = cases[s.case_idx].gold_complexity;
        for seed in 0..SEEDS {
            let (tier, _) = pick_tier(&s.c, &cells, &mut StdRng::seed_from_u64(seed));
            cost += tier_cost(tier);
            if gold >= 4 {
                hard_n += 1;
                hard_cheap += usize::from(tier == Tier::Tier3);
            } else if gold <= 2 {
                easy_n += 1;
                easy_cheap += usize::from(tier == Tier::Tier3);
            }
        }
    }
    m.mean_cost = cost / (n * SEEDS as f64);
    m.hard_to_cheapest = hard_cheap as f64 / hard_n.max(1) as f64;
    m.easy_to_cheapest = easy_cheap as f64 / easy_n.max(1) as f64;

    for (s, ok) in scored.iter().zip(&correct) {
        for tag in &cases[s.case_idx].tags {
            let e = m.by_tag.entry(tag.clone()).or_default();
            e.0 += 1;
            e.1 += usize::from(*ok);
        }
    }
    m.fallbacks = scored
        .iter()
        .filter(|s| s.c.source.fallback_reason().is_some())
        .count();
    let confident: Vec<bool> = scored
        .iter()
        .zip(&correct)
        .filter(|(s, _)| !s.c.is_low_confidence())
        .map(|(_, ok)| *ok)
        .collect();
    m.low_confidence = 1.0 - confident.len() as f64 / n;
    m.confident_accuracy =
        confident.iter().filter(|ok| **ok).count() as f64 / confident.len().max(1) as f64;
    m
}

fn pct(x: f64) -> String {
    format!("{:.1}%", 100.0 * x)
}

fn us(x: f64) -> String {
    if x < 10_000.0 {
        format!("{x:.0} µs")
    } else {
        format!("{:.0} ms", x / 1000.0)
    }
}

fn table(out: &mut String, header: &[String], rows: &[Vec<String>]) {
    let _ = writeln!(out, "| {} |", header.join(" | "));
    let _ = writeln!(out, "|{}|", vec!["---"; header.len()].join("|"));
    for r in rows {
        let _ = writeln!(out, "| {} |", r.join(" | "));
    }
    out.push('\n');
}

fn report(out: &mut String, title: &str, cases: &[Case], results: &[(&str, Vec<Scored>, Metrics)]) {
    let names: Vec<String> = results.iter().map(|(n, ..)| n.to_string()).collect();
    let header = |first: &str| {
        std::iter::once(first.to_string())
            .chain(names.iter().cloned())
            .collect::<Vec<_>>()
    };
    let row = |label: &str, f: &dyn Fn(&Metrics) -> String| {
        std::iter::once(label.to_string())
            .chain(results.iter().map(|(_, _, m)| f(m)))
            .collect::<Vec<_>>()
    };
    let _ = writeln!(out, "## {title} ({} requests)\n", cases.len());
    table(
        out,
        &header("metric"),
        &[
            row("type accuracy", &|m| pct(m.accuracy)),
            row("type macro-F1", &|m| format!("{:.3}", m.macro_f1)),
            row("low confidence → configured model, unpinned", &|m| {
                pct(m.low_confidence)
            }),
            row("type accuracy when confident", &|m| {
                pct(m.confident_accuracy)
            }),
            row("complexity exact", &|m| pct(m.cx_exact)),
            row("complexity within ±1", &|m| pct(m.cx_within1)),
            row("complexity MAE", &|m| format!("{:.2}", m.cx_mae)),
            row("hard (4–5) recognised as hard", &|m| pct(m.hard_recall)),
            row("confidence ECE (lower is better)", &|m| {
                format!("{:.3}", m.ece)
            }),
            row("Brier score (lower is better)", &|m| {
                format!("{:.3}", m.brier)
            }),
            row("latency p50", &|m| us(m.p50_us)),
            row("latency p95", &|m| us(m.p95_us)),
            row("latency p99", &|m| us(m.p99_us)),
            row("routing: mean tier cost", &|m| {
                format!("{:.2}", m.mean_cost)
            }),
            row("routing: hard → cheapest tier (lower is better)", &|m| {
                pct(m.hard_to_cheapest)
            }),
            row("routing: easy → cheapest tier (higher is better)", &|m| {
                pct(m.easy_to_cheapest)
            }),
            row("fell back to regex", &|m| m.fallbacks.to_string()),
        ],
    );

    let _ = writeln!(out, "### Accuracy by failure mode\n");
    let tags: Vec<&String> = results[0].2.by_tag.keys().collect();
    let mut rows: Vec<Vec<String>> = tags
        .iter()
        .map(|tag| {
            let (n, _) = results[0].2.by_tag[*tag];
            std::iter::once(format!("{tag} (n={n})"))
                .chain(results.iter().map(|(_, _, m)| {
                    let (n, ok) = m.by_tag[*tag];
                    pct(ok as f64 / n as f64)
                }))
                .collect()
        })
        .collect();
    rows.sort();
    table(out, &header("tag"), &rows);

    let _ = writeln!(out, "### F1 by request type\n");
    let rows: Vec<Vec<String>> = TYPES
        .iter()
        .map(|t| row(t.as_str(), &|m| format!("{:.2}", m.f1[t.as_str()])))
        .collect();
    table(out, &header("type"), &rows);

    // The concrete requests each backend fixes and breaks relative to regex.
    let base = &results[0].1;
    for (name, scored, _) in &results[1..] {
        let ok = |s: &Scored| s.c.request_type == cases[s.case_idx].gold_type;
        let fixed: Vec<String> = base
            .iter()
            .zip(scored)
            .filter(|(b, s)| !ok(b) && ok(s))
            .map(|(b, _)| {
                let case = &cases[b.case_idx];
                format!(
                    "`{}` {} → regex said {}",
                    case.id,
                    case.gold_type.as_str(),
                    b.c.request_type.as_str()
                )
            })
            .collect();
        let broken: Vec<String> = base
            .iter()
            .zip(scored)
            .filter(|(b, s)| ok(b) && !ok(s))
            .map(|(_, s)| {
                let case = &cases[s.case_idx];
                format!(
                    "`{}` {} → {name} said {}",
                    case.id,
                    case.gold_type.as_str(),
                    s.c.request_type.as_str()
                )
            })
            .collect();
        let _ = writeln!(
            out,
            "### {name} vs regex: {} fixed, {} broken\n\nBroken: {}\n",
            fixed.len(),
            broken.len(),
            if broken.is_empty() {
                "none".into()
            } else {
                broken.join("; ")
            }
        );
    }
}

async fn laya_backend() -> Option<LayaClassifier> {
    let url = std::env::var("LAYA_URL").ok()?;
    let http = reqwest::Client::new();
    match probe_health(&http, &url).await {
        Ok(_) => Some(LayaClassifier::new(http, &url, "", Duration::from_secs(30))),
        Err(error) => {
            eprintln!("LAYA_URL={url} not reachable ({error}); benchmarking regex and local only");
            None
        }
    }
}

#[tokio::test]
async fn classifier_benchmark() {
    let local = LocalClassifier::embedded().expect("embedded local model");
    let laya = laya_backend().await;
    let mut backends: Vec<(&str, &dyn RequestClassifier)> =
        vec![("regex", &RegexClassifier), ("local", &local)];
    if let Some(laya) = &laya {
        backends.push(("laya", laya));
    }

    let sets = [
        ("Held-out validation", load_h4("h4-validation.json")),
        ("Hackathon public sample", load_h4("h4-public-sample.json")),
        ("Labelled requests with chat history", load_requests()),
    ];
    let mut out = format!(
        "# Request-classifier benchmark\n\n{} build; routing columns are `pick_tier` over {SEEDS} \
         cold-start seeds (tier costs 15 / 3 / 0.8).\n\n",
        if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        }
    );
    let mut all = Vec::new();
    for (title, cases) in &sets {
        let mut results = Vec::new();
        for (name, backend) in &backends {
            let scored = run(*backend, cases).await;
            let m = metrics(cases, &scored);
            results.push((*name, scored, m));
        }
        report(&mut out, title, cases, &results);
        all.push(results);
    }
    println!("{out}");
    if let Ok(path) = std::env::var("BENCH_REPORT") {
        std::fs::write(&path, &out).expect("write BENCH_REPORT");
    }

    // ── Quality gates: the local model must keep beating regex where routing needs it. ──
    let get = |set: usize, name: &str| -> &Metrics {
        &all[set]
            .iter()
            .find(|(n, ..)| *n == name)
            .expect("backend")
            .2
    };
    let (regex, local) = (get(0, "regex"), get(0, "local"));
    assert!(
        local.accuracy >= 0.70,
        "local accuracy {:.3}",
        local.accuracy
    );
    assert!(
        local.accuracy >= regex.accuracy + 0.30,
        "local {:.3} must beat regex {:.3} by 30 pts",
        local.accuracy,
        regex.accuracy
    );
    assert!(
        local.macro_f1 >= 0.70,
        "local macro-F1 {:.3}",
        local.macro_f1
    );
    assert!(
        local.cx_within1 >= 0.90,
        "local complexity ±1 {:.3}",
        local.cx_within1
    );
    assert!(
        local.hard_recall >= 0.50,
        "local hard recall {:.3}",
        local.hard_recall
    );
    assert!(local.ece <= 0.15, "local ECE {:.3}", local.ece);
    assert!(
        local.hard_to_cheapest < regex.hard_to_cheapest,
        "local must send fewer hard requests to the cheapest tier than regex"
    );
    for (tag, (n, ok)) in &local.by_tag {
        let (_, regex_ok) = regex.by_tag[tag];
        assert!(
            *ok >= regex_ok,
            "local is worse than regex on {tag} ({ok} vs {regex_ok} of {n})"
        );
    }
    for (set, (title, _)) in sets.iter().enumerate().skip(1) {
        assert!(
            get(set, "local").accuracy > get(set, "regex").accuracy,
            "local must beat regex on {title}"
        );
    }
    let p99_budget_us = if cfg!(debug_assertions) {
        20_000.0
    } else {
        2_000.0
    };
    assert!(
        local.p99_us <= p99_budget_us,
        "local p99 {:.0} µs over the {p99_budget_us} µs budget",
        local.p99_us
    );
}
