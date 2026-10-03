# Real WildChat P2 evaluation — 30000 ms deadline

**Exploratory evaluation against pre-inference Codex annotations; not independent human gold.**

| Metric | Regex | Laya | Strands 4-bit |
|---|---:|---:|---:|
| Type accuracy | 16.0% (16/100) | 42.0% (42/100) | 68.0% (68/100) |
| Macro-F1 | 0.143 | 0.334 | 0.597 |
| Unambiguous type accuracy (63) | 19.0% | 49.2% | 77.8% |
| Near-duplicate-filtered accuracy (96) | 15.6% | 41.7% | 68.8% |
| Alternative-aware type accuracy | 20.0% | 51.0% | 80.0% |
| Initial-turn accuracy (63) | 22.2% | 41.3% | 65.1% |
| Follow-up accuracy (37) | 5.4% | 43.2% | 73.0% |
| Complexity exact | 42.0% | 50.0% | 49.0% |
| Complexity within ±1 | 90.0% | 86.0% | 95.0% |
| Complexity MAE | 0.68 | 0.64 | 0.56 |
| Hard recall (10) | 0.0% | 50.0% | 0.0% |
| Native-confidence ECE | 0.239 | 0.259 | 0.150 |
| Top-label Brier | 0.121 | 0.283 | 0.206 |
| Latency p50 | 0.002 ms | 147.672 ms | 552.148 ms |
| Latency p95 | 0.022 ms | 348.206 ms | 1059.702 ms |
| Latency p99 | 0.076 ms | 459.587 ms | 1226.617 ms |
| Error/timeout fallbacks | 0 | 0 | 0 |
| Confidence <0.4 (regex exempt) | 0 | 17 | 26 |
| Accuracy above confidence floor | 16.0% (100 accepted) | 47.0% (83 accepted) | 77.0% (74 accepted) |
| Confident wrong predictions | 84 | 44 | 17 |
| Changed classifications on repeat | 0 | 0 | 0 |

## Repeat 2

| Backend | Accuracy | p50 | p95 | Fallbacks | Low confidence |
|---|---:|---:|---:|---:|---:|
| regex | 16.0% | 0.004 ms | 0.031 ms | 0 | 0 |
| laya | 42.0% | 200.382 ms | 545.859 ms | 0 | 17 |
| strands | 68.0% | 677.315 ms | 1211.211 ms | 0 | 26 |

Classification changes compare the complete outputs excluding latency. A change in fallback state is an operational change, not necessarily model nondeterminism.


## Type F1 and support

| Type | n | Regex | Laya | Strands |
|---|---:|---:|---:|---:|
| code_generation | 16 | 0.476 | 0.478 | 0.811 |
| code_understanding | 9 | 0.000 | 0.143 | 0.308 |
| technical_design | 6 | 0.000 | 0.421 | 0.769 |
| analytical_reasoning | 9 | 0.000 | 0.182 | 0.462 |
| writing | 41 | 0.178 | 0.613 | 0.827 |
| factual_lookup | 14 | 0.235 | 0.273 | 0.696 |
| general | 5 | 0.109 | 0.231 | 0.308 |
