# P2 — Request classifier

The Level 3 router uses a pluggable classifier to estimate request type, complexity,
and confidence. Regex remains the default and preserves seeded routing behaviour.
The embedded local model adds calibrated type probabilities and a complexity prior
to the existing Thompson sampler. Laya and Strands are optional sidecar backends with circuit breaking and regex
fallback. Optimized Strands scores type remotely and complexity locally.

## Latest evidence — October 3, 2026

Release benchmark on macOS arm64, through the Rust serving features and `pick_tier`.
The current offline synthetic benchmark includes regex, local and raw/optimized
Strands at a 30-second quality-evaluation deadline. Laya is compared on the frozen
real-user sample. Latency measurements share a constrained 8 GB Apple M3 host.

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
training on the public set, and rerun the sidecar comparison. The optional local-first `hybrid` backend uses a fixed 0.70 threshold and requires
independent quality measurement before deployment.

## Strands performance and tradeoffs

| Raw → optimized Strands | Validation (154) | Chat history (100) | Public (10) |
|---|---:|---:|---:|
| Type accuracy | 90.9% → 90.9% | 97% → 97% | 100% → 100% |
| p95 decision latency | 688 → 345 ms | 477 → 251 ms | 781 → 381 ms |
| Complexity MAE | 0.71 → 0.35 | 0.74 → 0.42 | 0.70 → 0.80 |
| Hard requests → cheapest tier | 13.3% → 10.9% | 13.3% → 24.4% | 4% → 0% |

One remote question approximately halves latency while preserving request types.
Complexity is from the retrained local head, not native Strands. It improves
average validation/chat error, but **hard chat routing and public complexity
regress**. These routing simulations do not include the configured-model fallback
or measure real model answers. No downstream dollar-savings claim is established.

Selected-label probability replaces the native concentration confidence score.
The 0.4 floor stays in place, with an additional probability-equivalent native
admission guard `(1+6*0.4)/7`; confidence relabelling does not silently admit
uncertain requests. A training-only temperature candidate is rejected: grouped
CV accepted errors rise from 17 to 23 despite better NLL/ECE. Identity temperature
1.0 is shipped, and calibration on unseen domains is not established.

The serial MLX server warms the actual rubric before readiness, pins checkpoint
and base revisions, and rejects expired queued work. It cannot cancel active
Metal inference. Initial mixed-model operational testing still exposed
timeout stalls despite startup warm-up; those failures are retained in
`eval/real-world/optimized-results`. See the real-user report for final operational
results and memory-setting changes. The bundled server is an Apple-silicon
evaluation runner; local/regex remain portable.

## Final real-user evidence

On the frozen 100-request WildChat slice, optimized Strands preserves 68% type
accuracy and cuts p95 from 993 to 525 ms (repeat: 993 to 512 ms), compared with
the same-instance native baseline. Hard recall improves from 0/10 to 7/10,
while within-±1 complexity declines from 95% to 91%. Safe-default admission
remains 26%, with 17 accepted wrong types. The sample has machine annotations,
not independent human gold; its alternative-aware Strands accuracy is 80%.

Both Strands modes complete two 1.5-second passes without inference fallback
using bounded residency plus opt-in five-second idle inference. Earlier
startup-only and residency-only first-pass failures remain in the evidence.
Idle maintenance consumes compute and may queue a caller, and startup takes
29.2 seconds separately. These results support optimized Strands as the faster
type-classification candidate, not a universal quality winner or a production
SLA. Synthetic hard-chat routing and complexity regressions remain material.

See [the final real-user comparison](llm-router/eval/real-world/maintained-results/report.md)
and [methodology, tradeoffs and reproduction](llm-router/eval/real-world/README.md).
