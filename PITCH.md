# P2 — Request classifier

The Level 3 router uses a pluggable classifier to estimate request type, complexity,
and confidence. Regex remains the default and preserves seeded routing behaviour.
The embedded local model adds calibrated type probabilities and a complexity prior
to the existing Thompson sampler. Laya is an optional sidecar backend with a circuit
breaker and regex fallback.

## Latest evidence — October 3, 2026

Release benchmark on macOS arm64, through the Rust serving features and `pick_tier`.
Laya was unavailable for this run; earlier Laya measurements remain in Git history.

| Held-out validation (154) | Regex | Previous local | Retrained local |
|---|---|---|---|
| Type accuracy | 32.5% | 79.2% | 86.4% |
| Macro-F1 | 0.335 | 0.787 | 0.864 |
| Complexity within ±1 | 69.5% | 98.1% | 98.1% |
| Hard requests recognised as hard | 0.0% | 71.4% | 78.6% |
| Confidence ECE | 0.157 | 0.100 | 0.036 |
| Hard → cheapest tier | 57.3% | 13.5% | 10.0% |
| Easy → cheapest tier | 61.4% | 77.1% | 75.8% |

The new local model reaches 92% on 100 requests with chat history (previously 86%).
Its hard-request recall on that set improves from 11.8% to 47.1%, leaving substantial
room for improvement. Validation latency was about 0.04 ms p50 and 0.25 ms p99;
latency comparisons with previous hardware are not controlled.

**Public-sample regression:** accuracy falls from 8/10 to 7/10, complexity within ±1
falls from 100% to 80%, and ECE increases from 0.133 to 0.191. This small independent
sample does not support a claim of uniform improvement. No public examples were
added to training to repair these misses.

## Training and confidence

The model trains on 1,806 originals from train-a through train-e, with 1,762 robustness
copies. Two newly added families overlapped validation; all six rows were removed
before fitting. The unchanged leakage guard passes (maximum Jaccard 0.64).
Regularization and temperature come from five-fold CV grouped by paraphrase family,
with augmented copies kept in their family. Type CV accuracy is 85.8%.

The existing 0.4 confidence floor is retained: validation abstention is 2.6%, with
86.7% type accuracy among confident requests. Chat-history abstention is 0%; public
abstention is 1/10. Across all 264 examples the rate is 5/264 (1.9%). The public set
alone therefore exceeds the 5% target. Lowering the floor merely to meet that target
would accept a currently rejected wrong prediction.

Low-confidence requests use the configured model and are not pinned. Benchmark
routing columns replay `pick_tier` over 50 cold-start seeds; they do not measure
end-to-end configured-model fallback, real model quality, or actual dollar savings.

## Verification and next steps

The embedded asset, report, and stricter regression gates are checked in together.
The gates now require validation accuracy and macro-F1 ≥0.84, hard recall ≥0.75,
ECE ≤0.10, low-confidence rate ≤5%, and chat-history accuracy ≥90%, alongside
existing complexity, latency, per-tag, and regex comparisons.

See [the full report](llm-router/eval/benchmark-report.md) and
[reproduction instructions](llm-router/eval/README.md). Remaining work is to evaluate
independent production traffic, investigate the public-sample regression without
training on the public set, and rerun the sidecar comparison. A local-to-Laya cascade
is optional future work and is not implemented here.
