# Request-classifier benchmark

Release build; routing columns are `pick_tier` over 50 cold-start seeds (tier costs 15 / 3 / 0.8).

## Held-out validation (154 requests)

| metric | regex | local |
|---|---|---|
| type accuracy | 32.5% | 86.4% |
| type macro-F1 | 0.335 | 0.864 |
| low confidence → configured model, unpinned | 0.0% | 2.6% |
| type accuracy when confident | 32.5% | 86.7% |
| complexity exact | 30.5% | 66.9% |
| complexity within ±1 | 69.5% | 98.1% |
| complexity MAE | 1.00 | 0.35 |
| hard (4–5) recognised as hard | 0.0% | 78.6% |
| confidence ECE (lower is better) | 0.157 | 0.036 |
| Brier score (lower is better) | 0.159 | 0.093 |
| latency p50 | 2 µs | 43 µs |
| latency p95 | 22 µs | 207 µs |
| latency p99 | 41 µs | 249 µs |
| routing: mean tier cost | 2.18 | 2.86 |
| routing: hard → cheapest tier (lower is better) | 57.3% | 10.0% |
| routing: easy → cheapest tier (higher is better) | 61.4% | 75.8% |
| fell back to regex | 0 | 0 |

### Accuracy by failure mode

| tag | regex | local |
|---|---|---|
| context_dependent (n=22) | 13.6% | 68.2% |
| keyword_trap (n=18) | 44.4% | 77.8% |
| long_spec (n=18) | 44.4% | 83.3% |
| multilingual (n=27) | 25.9% | 77.8% |
| no_keyword (n=19) | 31.6% | 78.9% |
| plain (n=43) | 48.8% | 100.0% |
| short_simple (n=39) | 30.8% | 94.9% |
| typo_informal (n=18) | 16.7% | 100.0% |

### F1 by request type

| type | regex | local |
|---|---|---|
| code_generation | 0.41 | 0.95 |
| code_understanding | 0.24 | 0.86 |
| technical_design | 0.24 | 0.86 |
| analytical_reasoning | 0.41 | 0.80 |
| writing | 0.31 | 0.86 |
| factual_lookup | 0.43 | 0.87 |
| general | 0.30 | 0.85 |

### local vs regex: 89 fixed, 6 broken

Broken: `val-065` technical_design → local said analytical_reasoning; `val-140` general → local said analytical_reasoning; `val-143` general → local said writing; `val-145` general → local said factual_lookup; `val-150` general → local said analytical_reasoning; `val-154` general → local said analytical_reasoning

## Hackathon public sample (10 requests)

| metric | regex | local |
|---|---|---|
| type accuracy | 30.0% | 70.0% |
| type macro-F1 | 0.262 | 0.567 |
| low confidence → configured model, unpinned | 0.0% | 10.0% |
| type accuracy when confident | 30.0% | 77.8% |
| complexity exact | 20.0% | 40.0% |
| complexity within ±1 | 60.0% | 80.0% |
| complexity MAE | 1.20 | 0.80 |
| hard (4–5) recognised as hard | 0.0% | 100.0% |
| confidence ECE (lower is better) | 0.150 | 0.191 |
| Brier score (lower is better) | 0.165 | 0.127 |
| latency p50 | 3 µs | 61 µs |
| latency p95 | 13 µs | 210 µs |
| latency p99 | 13 µs | 210 µs |
| routing: mean tier cost | 2.58 | 4.18 |
| routing: hard → cheapest tier (lower is better) | 26.7% | 0.0% |
| routing: easy → cheapest tier (higher is better) | 46.4% | 51.2% |
| fell back to regex | 0 | 0 |

### Accuracy by failure mode

| tag | regex | local |
|---|---|---|
| architecture (n=1) | 0.0% | 100.0% |
| audience (n=1) | 0.0% | 100.0% |
| code_explanation (n=1) | 100.0% | 100.0% |
| code_generation (n=1) | 0.0% | 100.0% |
| code_keyword_low_effort (n=1) | 0.0% | 100.0% |
| constraint_preservation (n=1) | 100.0% | 100.0% |
| context_grounding (n=1) | 0.0% | 100.0% |
| factual (n=1) | 0.0% | 100.0% |
| minimal_edit (n=1) | 0.0% | 0.0% |
| misleading_cache_keyword (n=1) | 0.0% | 0.0% |
| misleading_keywords (n=1) | 0.0% | 0.0% |
| moderate_context (n=2) | 0.0% | 100.0% |
| multi_constraint (n=1) | 0.0% | 100.0% |
| multiple_constraints (n=1) | 0.0% | 100.0% |
| negation (n=1) | 0.0% | 0.0% |
| negative_constraints (n=1) | 0.0% | 100.0% |
| no_context (n=1) | 0.0% | 100.0% |
| reasoning (n=1) | 100.0% | 0.0% |
| risk (n=1) | 100.0% | 0.0% |
| short_context (n=5) | 60.0% | 60.0% |
| short_request (n=2) | 0.0% | 100.0% |
| temporal_reasoning (n=1) | 0.0% | 0.0% |
| writing (n=1) | 100.0% | 100.0% |

### F1 by request type

| type | regex | local |
|---|---|---|
| code_generation | 0.00 | 0.80 |
| code_understanding | 0.50 | 0.67 |
| technical_design | 0.00 | 0.50 |
| analytical_reasoning | 0.67 | 0.00 |
| writing | 0.67 | 1.00 |
| factual_lookup | 0.00 | 1.00 |
| general | 0.00 | 0.00 |

### local vs regex: 5 fixed, 1 broken

Broken: `pub-10` analytical_reasoning → local said technical_design

## Labelled requests with chat history (100 requests)

| metric | regex | local |
|---|---|---|
| type accuracy | 43.0% | 92.0% |
| type macro-F1 | 0.448 | 0.918 |
| low confidence → configured model, unpinned | 0.0% | 0.0% |
| type accuracy when confident | 43.0% | 92.0% |
| complexity exact | 20.0% | 66.0% |
| complexity within ±1 | 64.0% | 93.0% |
| complexity MAE | 1.16 | 0.42 |
| hard (4–5) recognised as hard | 0.0% | 47.1% |
| confidence ECE (lower is better) | 0.125 | 0.053 |
| Brier score (lower is better) | 0.178 | 0.053 |
| latency p50 | 2 µs | 32 µs |
| latency p95 | 10 µs | 52 µs |
| latency p99 | 14 µs | 62 µs |
| routing: mean tier cost | 2.40 | 2.41 |
| routing: hard → cheapest tier (lower is better) | 54.4% | 24.8% |
| routing: easy → cheapest tier (higher is better) | 57.5% | 71.5% |
| fell back to regex | 0 | 0 |

### Accuracy by failure mode

| tag | regex | local |
|---|---|---|
| context_dependent (n=5) | 0.0% | 100.0% |
| plain (n=95) | 45.3% | 91.6% |

### F1 by request type

| type | regex | local |
|---|---|---|
| code_generation | 0.40 | 0.96 |
| code_understanding | 0.35 | 0.86 |
| technical_design | 0.25 | 0.90 |
| analytical_reasoning | 0.70 | 0.85 |
| writing | 0.60 | 0.93 |
| factual_lookup | 0.48 | 0.97 |
| general | 0.36 | 0.97 |

### local vs regex: 50 fixed, 1 broken

Broken: `ar-05` analytical_reasoning → local said factual_lookup

