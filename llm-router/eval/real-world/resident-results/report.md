# Real WildChat P2 evaluation — 1500 ms deadline

**Exploratory evaluation against pre-inference Codex annotations; not independent human gold.**

| Metric | Regex | Local | Laya | Strands 4-bit | Strands raw 4-bit |
|---|---:|---:|---:|---:|---:|
| Type accuracy | 16.0% (16/100) | 52.0% (52/100) | 41.0% (41/100) | 16.0% (16/100) | 67.0% (67/100) |
| Macro-F1 | 0.143 | 0.341 | 0.307 | 0.143 | 0.577 |
| Unambiguous type accuracy (63) | 19.0% | 68.3% | 49.2% | 19.0% | 77.8% |
| Near-duplicate-filtered accuracy (96) | 15.6% | 51.0% | 40.6% | 15.6% | 67.7% |
| Alternative-aware type accuracy | 20.0% | 60.0% | 50.0% | 20.0% | 79.0% |
| Initial-turn accuracy (63) | 22.2% | 52.4% | 39.7% | 22.2% | 63.5% |
| Follow-up accuracy (37) | 5.4% | 51.4% | 43.2% | 5.4% | 73.0% |
| Complexity exact | 42.0% | 48.0% | 50.0% | 42.0% | 49.0% |
| Complexity within ±1 | 90.0% | 91.0% | 86.0% | 90.0% | 95.0% |
| Complexity MAE | 0.68 | 0.61 | 0.64 | 0.68 | 0.56 |
| Hard recall (10) | 0.0% | 70.0% | 50.0% | 0.0% | 0.0% |
| Returned-confidence ECE (semantics differ) | 0.239 | 0.231 | 0.268 | 0.239 | 0.151 |
| Top-label Brier | 0.121 | 0.262 | 0.281 | 0.121 | 0.206 |
| Latency p50 | 0.002 ms | 0.054 ms | 126.244 ms | 0.003 ms | 546.254 ms |
| Latency p95 | 0.020 ms | 0.341 ms | 308.744 ms | 0.026 ms | 1025.404 ms |
| Latency p99 | 0.072 ms | 0.370 ms | 400.988 ms | 1502.148 ms | 1198.883 ms |
| Error/timeout fallbacks | 0 | 0 | 1 | 100 | 1 |
| Confidence <0.4 (regex exempt) | 0 | 8 | 17 | 0 | 26 |
| Safe-default policy requests | 0 | 8 | 17 | 0 | 26 |
| Accuracy among policy-accepted types | 16.0% (100 accepted) | 51.1% (92 accepted) | 46.3% (82 accepted) | N/A (0 accepted) | 76.7% (73 accepted) |
| Accepted wrong predictions | 84 | 45 | 44 | 0 | 17 |
| Changed classifications on repeat | 0 | 0 | 1 | 100 | 1 |

## Repeat 2

| Backend | Accuracy | p50 | p95 | Fallbacks | Low confidence |
|---|---:|---:|---:|---:|---:|
| regex | 16.0% | 0.002 ms | 0.020 ms | 0 | 0 |
| local | 52.0% | 0.054 ms | 0.334 ms | 0 | 8 |
| laya | 42.0% | 117.213 ms | 280.836 ms | 0 | 17 |
| strands | 68.0% | 279.804 ms | 546.726 ms | 0 | 26 |
| strands_raw | 68.0% | 550.291 ms | 1065.750 ms | 0 | 26 |

Safe-default policy uses the returned low_confidence flag when present. Optimized Strands exposes probability confidence and preserves native concentration admission with an additional selected-probability minimum of (1+6*0.4)/7; literal confidence <0.4 alone is not its full routing policy.

Accuracy and confidence metrics include all recorded outputs, including operational fallback classifications where present. Accepted-type metrics exclude failure fallbacks; a failed backend does not provide a native model prediction.

Classification changes compare the complete outputs excluding latency. A change in fallback state is an operational change, not necessarily model nondeterminism.


## Type F1 and support

| Type | n | Regex | Local | Laya | Strands 4-bit | Strands raw 4-bit |
|---|---:|---:|---:|---:|---:|---:|
| code_generation | 16 | 0.476 | 0.471 | 0.478 | 0.476 | 0.811 |
| code_understanding | 9 | 0.000 | 0.000 | 0.143 | 0.000 | 0.308 |
| technical_design | 6 | 0.000 | 0.167 | 0.421 | 0.000 | 0.769 |
| analytical_reasoning | 9 | 0.000 | 0.308 | 0.000 | 0.000 | 0.333 |
| writing | 41 | 0.178 | 0.795 | 0.613 | 0.178 | 0.827 |
| factual_lookup | 14 | 0.235 | 0.381 | 0.273 | 0.235 | 0.696 |
| general | 5 | 0.109 | 0.267 | 0.222 | 0.109 | 0.296 |

## Run provenance

Requested backend order: regex, local, laya, strands, strands_raw.
Binary SHA-256: `b14d2485f5b45f06be17345e1c32b9c22539052ded3465fa4460c0cffc6e83bf`.
Frozen-label SHA-256: `c23ddedb82c675ba125c828c0fb2fa7efe5388866d05f6e1cd1d0eba97f661a7`.
Source snapshots, endpoint mappings, runtime and sidecar residency metadata, calibration decisions, and prediction hashes are recorded in [run-manifest.json](run-manifest.json).
