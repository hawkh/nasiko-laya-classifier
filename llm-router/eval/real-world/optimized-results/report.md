# Real WildChat P2 evaluation — 1500 ms deadline

**Exploratory evaluation against pre-inference Codex annotations; not independent human gold.**

| Metric | Regex | Laya | Strands 4-bit |
|---|---:|---:|---:|
| Type accuracy | 16.0% (16/100) | 41.0% (41/100) | 16.0% (16/100) |
| Macro-F1 | 0.143 | 0.307 | 0.143 |
| Unambiguous type accuracy (63) | 19.0% | 49.2% | 19.0% |
| Near-duplicate-filtered accuracy (96) | 15.6% | 40.6% | 15.6% |
| Alternative-aware type accuracy | 20.0% | 50.0% | 20.0% |
| Initial-turn accuracy (63) | 22.2% | 39.7% | 22.2% |
| Follow-up accuracy (37) | 5.4% | 43.2% | 5.4% |
| Complexity exact | 42.0% | 50.0% | 42.0% |
| Complexity within ±1 | 90.0% | 86.0% | 90.0% |
| Complexity MAE | 0.68 | 0.64 | 0.68 |
| Hard recall (10) | 0.0% | 50.0% | 0.0% |
| Returned-confidence ECE (semantics differ) | 0.239 | 0.268 | 0.239 |
| Top-label Brier | 0.121 | 0.281 | 0.121 |
| Latency p50 | 0.004 ms | 164.443 ms | 0.003 ms |
| Latency p95 | 0.064 ms | 552.237 ms | 0.027 ms |
| Latency p99 | 0.107 ms | 825.466 ms | 1501.384 ms |
| Error/timeout fallbacks | 0 | 1 | 100 |
| Confidence <0.4 (regex exempt) | 0 | 17 | 0 |
| Safe-default policy requests | 0 | 17 | 0 |
| Accuracy among policy-accepted types | 16.0% (100 accepted) | 46.3% (82 accepted) | N/A (0 accepted) |
| Accepted wrong predictions | 84 | 44 | 0 |
| Changed classifications on repeat | 0 | 1 | 100 |

## Repeat 2

| Backend | Accuracy | p50 | p95 | Fallbacks | Low confidence |
|---|---:|---:|---:|---:|---:|
| regex | 16.0% | 0.003 ms | 0.023 ms | 0 | 0 |
| laya | 42.0% | 144.564 ms | 408.828 ms | 0 | 17 |
| strands | 68.0% | 298.936 ms | 550.170 ms | 0 | 26 |

Safe-default policy uses the returned low_confidence flag when present. Optimized Strands exposes probability confidence and preserves native concentration admission with an additional selected-probability minimum of (1+6*0.4)/7; literal confidence <0.4 alone is not its full routing policy.

Accuracy and confidence metrics include all recorded outputs, including operational fallback classifications where present. Accepted-type metrics exclude failure fallbacks; a failed backend does not provide a native model prediction.

Classification changes compare the complete outputs excluding latency. A change in fallback state is an operational change, not necessarily model nondeterminism.


## Type F1 and support

| Type | n | Regex | Laya | Strands 4-bit |
|---|---:|---:|---:|---:|
| code_generation | 16 | 0.476 | 0.478 | 0.476 |
| code_understanding | 9 | 0.000 | 0.143 | 0.000 |
| technical_design | 6 | 0.000 | 0.421 | 0.000 |
| analytical_reasoning | 9 | 0.000 | 0.000 | 0.000 |
| writing | 41 | 0.178 | 0.613 | 0.178 |
| factual_lookup | 14 | 0.235 | 0.273 | 0.235 |
| general | 5 | 0.109 | 0.222 | 0.109 |

## Run provenance

Requested backend order: regex, laya, strands.
Binary SHA-256: `b14d2485f5b45f06be17345e1c32b9c22539052ded3465fa4460c0cffc6e83bf`.
Frozen-label SHA-256: `c23ddedb82c675ba125c828c0fb2fa7efe5388866d05f6e1cd1d0eba97f661a7`.
Source snapshots, endpoint mappings, runtime and sidecar residency metadata, calibration decisions, and prediction hashes are recorded in [run-manifest.json](run-manifest.json).
