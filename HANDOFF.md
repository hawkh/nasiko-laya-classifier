# Handoff: P2 request classifier (llm-router)

The branch is `feat/llm-router-laya-classifier`, in the private repo `hawkh/nasiko-laya-classifier`. PR #1 is open against that repo's `main`, which mirrors Nasiko-Labs/nasiko main.

**Rules:**
- Never push to Nasiko-Labs (`origin`). Push to `private` only.
- No AI attribution lines in commits or PRs.
- Leave `tests/router_e2e.rs` byte-identical to upstream main, and don't run `cargo fmt` over it.
- Don't commit `Cargo.lock`: the copy on upstream main is stale.

## What exists

- **`src/routing/request_classifier.rs`:**
  - The `RequestClassifier` seam, `Classification`, and `ClassifierSource {Regex, Laya, Local, Fallback}`.
  - `ClassifierKind::from_label`, selected by `REQUEST_CLASSIFIER=regex|local|laya`. Regex is the default.
  - `classify_input`, which builds the classifier's context text from the conversation.
  - `LOW_CONFIDENCE_FLOOR` (0.4) and `Classification::is_low_confidence`, which is true only for model-backed backends, never regex.
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
  - **Laya:** included when `LAYA_URL` is set.
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
PITCH disclose these tradeoffs. No Laya service was available for the new run.

## Completed

- Retrained the embedded model on cleaned train-a through train-e, using the original
  grouped-CV trainer and unchanged validation data.
- Regenerated `eval/benchmark-report.md` for regex and local.
- Raised validation quality gates and added coverage/chat-history regression gates.
- Retained `LOW_CONFIDENCE_FLOOR=0.4`: validation abstention is 2.6%, overall 1.9%.
  Public abstention is 1/10; reducing it would accept a wrong prediction.
- Refreshed PITCH and evaluation reproduction notes.

## Next

1. Assess the public-sample regression on additional independently authored traffic;
   do not copy held-out failures into training or tune repeatedly against them.
2. Run the benchmark with a live Laya sidecar for a current three-backend comparison.
3. Optional: local-first cascade to Laya for low-confidence requests.
4. Evaluate shadow-mode decisions against actual routing outcomes before rollout.
