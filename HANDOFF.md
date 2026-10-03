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

## Current numbers (model trained on train-a + train-b, 552 rows)

| Validation (154) | regex | local | laya |
|---|---|---|---|
| Type accuracy | 32.5% | 79.2% | 34.4% |
| Hard requests → cheapest tier | 57% | 13.5% | 24% |
| Public (10) | 30% | 80% | 40% |

The competitor PR Nasiko-Labs/nasiko#260 uses the same model family and reports 89.8% on its own validation set and 9/10 on the public set.

## In progress / next

1. **Training data.**
   - `eval/train-c.jsonl` is written: 420 rows, robustness slices, with families.
   - `train-d.jsonl` (realistic traffic) and `train-e.jsonl` (decision-boundary minimal pairs) are being written by other agents. If they're missing, write them with the same schema: `{id, family, query, context, request_type, complexity, tags}`.
   - Never read the validation or public sets while writing training data.
2. **Retrain.** Run `python eval/train_local.py` from `llm-router/` in WSL. Then run:
   ```
   LAYA_URL=http://localhost:8000 BENCH_REPORT=eval/benchmark-report.md cargo test --release -p nasiko-llm-router --test classifier_benchmark -- --nocapture
   ```
   Commit the new `assets/local_classifier.json` and the regenerated `benchmark-report.md`. Raise the gates in the test if accuracy improves.
3. **Tune `LOW_CONFIDENCE_FLOOR`** after retraining. Use the benchmark rows for the low-confidence rate and accuracy-when-confident; aim for 5% or less low-confidence.
4. **Optional cascade:** local first, Laya only when local is unsure.
5. **Refresh** the PR #1 description and `PITCH.md` with the new numbers.
