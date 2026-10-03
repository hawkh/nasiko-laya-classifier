# Handoff: P2 request classifier (llm-router)

The branch is `feat/llm-router-optimizer`, in the private repo `hawkh/nasiko-laya-classifier`. PR #1 is open against that repo's `main`, which mirrors Nasiko-Labs/nasiko main.

**Rules:**
- Never push to Nasiko-Labs (`origin`). Push to `private` only.
- No AI attribution lines in commits or PRs.
- Leave `tests/router_e2e.rs` byte-identical to upstream main, and don't run `cargo fmt` over it.
- Don't commit `Cargo.lock`: the copy on upstream main is stale.

## What exists

- **`src/routing/request_classifier.rs`:**
  - The `RequestClassifier` seam, `Classification`, and `ClassifierSource {Regex, Laya, Local, Strands, HybridStrands, Fallback}`.
  - `ClassifierKind::from_label`, selected by `REQUEST_CLASSIFIER=regex|local|laya|strands|strands_raw|hybrid`. Regex is the default.
  - `classify_input`, which builds the classifier's context text from the conversation.
  - `LOW_CONFIDENCE_FLOOR` (0.4) and `Classification::is_low_confidence`, which is true only for model-backed backends, never regex. Strands also retains the probability-equivalent native admission minimum `(1+6*0.4)/7`, so changing confidence semantics does not silently increase acceptance.
- **`src/routing/classifier.rs`, `pick_tier`:**
  1. draws the type from `type_probabilities`;
  2. shifts the Tier1 and Tier3 priors by `complexity_confidence × K × (level − 2)/2`, with K = 1.0;
  3. Thompson-samples the tier.

  With regex, routing is seed-for-seed identical to main.
- **`src/routing/mod.rs`, `route_model` Level 3:** classifies only at fireable boundaries (sticky routing comes from upstream). A low-confidence classification gets `RouteSource::LowConfidence`: the configured model is served and the decision isn't cached.
- **`src/routing/laya.rs`:** the `laya-serve` sidecar backend. It has a circuit breaker (3 failures open it for 30 s, then a probe) and falls back to regex.
- **`src/routing/local_classifier.rs`:** the in-process model.
  - **Features:** FNV-1a hashed into 2^18 buckets: words, word bigrams, character 3–5-grams, shape features, and context words.
  - **Heads:** two softmax heads (type, complexity) with temperature.
  - **Weights:** embedded from `assets/local_classifier.json`.
- **`examples/local_classifier_features.rs`:** dumps the Rust features, so the trainer uses exactly what serving computes.
- **`eval/train_local.py`:** trains on `eval/train-*.jsonl`.
  - **CV:** stratified 5-fold, grouped by paraphrase `family`, with robustness augmentation (typos, casing, padding) kept inside each family.
  - **Tuning:** each head gets its own C grid and a temperature fitted on out-of-fold predictions.
  - **Leakage guard:** fails if any validation query near-duplicates a training query (Jaccard ≥ 0.7).
  - **Requirements:** WSL with `~/laya-venv/bin/python` (scikit-learn installed) and `CARGO_TARGET_DIR=~/nasiko-target`. It takes 30–60 minutes on CPU.
- **`tests/classifier_benchmark.rs`:** the benchmark, run by `cargo test`.
  - **Gold sets:** `eval/h4-validation.json` (154 held-out requests, tagged by failure mode), `eval/h4-public-sample.json` (the official 10, https://registry.nasiko.dev/r/nasiko/classifier-eval) and `eval/requests.jsonl` (100 requests with chat history).
  - **Reports:** accuracy, F1, complexity, calibration (ECE/Brier), latency p50/p95/p99, low-confidence rate, accuracy when confident, routing outcome through `pick_tier`, and per-tag results.
  - **Gates:** quality gates fail the build if the local model regresses.
  - **Sidecars:** Laya included with `LAYA_URL`; optimized/raw Strands included with `STRANDS_URL`. Explicit sidecar runs fail on unexpected fallback or backend provenance. `BENCH_PREDICTIONS` exports classifications with hashes for rescoring.
  - **CI:** `.github/workflows/llm-router-classifier.yml`.

## Current numbers (retrained October 3, 2026)

Training uses 1,806 originals and 1,762 robustness copies. Two families (six rows)
were removed because the unchanged leakage check detected validation overlaps.
Both heads selected C=8; type temperature=0.54, complexity temperature=1.1.

| Validation (154) | regex | local |
|---|---|---|
| Type accuracy | 32.5% | 86.4% |
| Hard requests → cheapest tier | 57.3% | 10.0% |
| ECE | 0.157 | 0.036 |
| Low-confidence rate | 0.0% | 2.6% |

Chat-history type accuracy is 92% (was 86%). Public-sample accuracy regressed to
7/10 (was 8/10); complexity within ±1 regressed to 80% (was 100%). The report and
PITCH disclose these tradeoffs. The initial retraining run had no Laya service; later comparisons are below.

## Completed

- Retrained the embedded model on cleaned train-a through train-e, using the original
  grouped-CV trainer and unchanged validation data.
- Regenerated `eval/benchmark-report.md` for regex and local.
- Raised validation quality gates and added coverage/chat-history regression gates.
- Retained `LOW_CONFIDENCE_FLOOR=0.4`: validation abstention is 2.6%, overall 1.9%.
  Public abstention is 1/10; reducing it would accept a wrong prediction.
- Refreshed PITCH and evaluation reproduction notes.

## Strands improvements — October 3, 2026

- `strands` asks one remote type question and uses the embedded retrained complexity
  head. `strands_raw` retains both native questions. The shared factory powers
  runtime and `classifier_eval`; API token and timeout come from typed config.
- Type confidence is the selected-label probability. A separate conservative
  admission guard preserves the native concentration threshold, subject to wire
  rounding at the boundary. Scalar confidence below 0.4 and full safe-default
  rejection are reported separately. Regex remains default and failure fallback.
- Training-only grouped CV rejects temperature sharpening: NLL improves, but
  accepted errors increase from 17 to 23. Shipped temperature is identity 1.0.
- The pinned serial MLX evaluation server warms the actual short/long rubric before
  readiness. Deadlines discard expired queue work; running Metal work cannot be
  cancelled. Load/warm-up took about 29–35 seconds on this shared Mac, recorded separately.
  Bounded MLX residency alone did not prevent initial operational timeouts; the
  opt-in idle-inference setting is measured separately and never caches responses.
- `ROUTING_SEED=<u64>` makes tier sampling reproducible for identical inputs and
  learned state. Unset preserves exploration. Low-confidence and sticky-cache
  behavior are covered by runtime tests.

The final 30-second offline benchmark preserves type accuracy: 90.9% validation,
97% chat history, 100% public sample. Raw → optimized p95 is 688 → 345 ms,
477 → 251 ms and 781 → 381 ms respectively. All 264 requests have zero inference
fallbacks; 50-seed routing replay and existing local quality gates pass.

**Tradeoffs:** hard chat requests routed to the cheapest tier increase from 13.3%
to 24.4%; public complexity within ±1 falls from 90% to 80%. Complexity MAE
improves on validation/chat, but this is not a uniform routing-quality improvement.
The raw two-question backend also timed out in its first 1.5-second warm-up; the
30-second benchmark does not establish operational deadline success.

Runtime unit suite: 418 passed, 2 ignored. Provider translation: 5 passed. Six
external end-to-end tests remain ignored and byte-identical to main. Server/runner
mock suites: 32 + 8 passed. Predictions, model hashes and source provenance are in
`eval/strands/benchmark-predictions` and `eval/real-world`.

See `eval/strands/README.md` and `eval/real-world/README.md` for reproduction. The
bundled server is Apple-silicon-only; the Rust local/regex paths remain portable.

## Next

1. Assess the public-sample regression on additional independently authored traffic;
   do not copy held-out failures into training or tune repeatedly against them.
2. Review independently human-labelled real traffic and the hard chat/public complexity regressions before choosing a production backend.
3. Measure any local-first cascade independently; the optional `hybrid` 0.70 policy is not validated as a production recommendation.
4. Evaluate shadow-mode decisions against actual routing outcomes before rollout.

## Final real-user measurement

The frozen 100-request WildChat slice uses pre-inference machine annotations,
not independently reviewed human gold. Optimized/native Strands type accuracy
is 68%; Laya is 42% without fallbacks, local 52%, regex 16%. With bounded
residency plus opt-in idle inference every five seconds, both Strands modes
complete two 1,500 ms passes with zero fallbacks and identical classifications.
Native → optimized p95: 993 → 525 ms (repeat 993 → 512 ms). Hard recall improves
0/10 → 7/10, but complexity within ±1 falls 95% → 91%. Safe-default rate remains
26%; accepted wrong types remain 17. Startup takes 29.2 s separately. Earlier
startup-only and residency-only failures are retained. Idle inference costs
compute, and neither these results nor synthetic replay prove production SLA or
downstream savings. See `eval/real-world/maintained-results/report.md`.
