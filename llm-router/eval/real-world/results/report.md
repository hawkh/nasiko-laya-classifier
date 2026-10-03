# Real WildChat P2 evaluation — 1500 ms deadline

**Exploratory evaluation against pre-inference Codex annotations; not independent human gold.**

| Metric | Regex | Laya | Strands 4-bit |
|---|---:|---:|---:|
| Type accuracy | 16.0% (16/100) | 42.0% (42/100) | 16.0% (16/100) |
| Macro-F1 | 0.143 | 0.334 | 0.143 |
| Unambiguous type accuracy (63) | 19.0% | 49.2% | 19.0% |
| Near-duplicate-filtered accuracy (96) | 15.6% | 41.7% | 15.6% |
| Alternative-aware type accuracy | 20.0% | 51.0% | 20.0% |
| Initial-turn accuracy (63) | 22.2% | 41.3% | 22.2% |
| Follow-up accuracy (37) | 5.4% | 43.2% | 5.4% |
| Complexity exact | 42.0% | 50.0% | 42.0% |
| Complexity within ±1 | 90.0% | 86.0% | 90.0% |
| Complexity MAE | 0.68 | 0.64 | 0.68 |
| Hard recall (10) | 0.0% | 50.0% | 0.0% |
| Native-confidence ECE | 0.239 | 0.259 | 0.239 |
| Top-label Brier | 0.121 | 0.283 | 0.121 |
| Latency p50 | 0.003 ms | 128.545 ms | 0.003 ms |
| Latency p95 | 0.022 ms | 311.858 ms | 0.029 ms |
| Latency p99 | 0.078 ms | 416.710 ms | 1501.698 ms |
| Error/timeout fallbacks | 0 | 0 | 100 |
| Confidence <0.4 (regex exempt) | 0 | 17 | 0 |
| Accuracy above confidence floor | 16.0% (100 accepted) | 47.0% (83 accepted) | N/A (0 accepted) |
| Confident wrong predictions | 84 | 44 | 0 |
| Changed classifications on repeat | 0 | 0 | 100 |

## Repeat 2

| Backend | Accuracy | p50 | p95 | Fallbacks | Low confidence |
|---|---:|---:|---:|---:|---:|
| regex | 16.0% | 0.002 ms | 0.022 ms | 0 | 0 |
| laya | 42.0% | 134.171 ms | 290.069 ms | 0 | 17 |
| strands | 68.0% | 547.801 ms | 1025.078 ms | 0 | 26 |

Classification changes compare the complete outputs excluding latency. A change in fallback state is an operational change, not necessarily model nondeterminism.


## Type F1 and support

| Type | n | Regex | Laya | Strands |
|---|---:|---:|---:|---:|
| code_generation | 16 | 0.476 | 0.478 | 0.476 |
| code_understanding | 9 | 0.000 | 0.143 | 0.000 |
| technical_design | 6 | 0.000 | 0.421 | 0.000 |
| analytical_reasoning | 9 | 0.000 | 0.182 | 0.000 |
| writing | 41 | 0.178 | 0.613 | 0.178 |
| factual_lookup | 14 | 0.235 | 0.273 | 0.235 |
| general | 5 | 0.109 | 0.231 | 0.109 |
