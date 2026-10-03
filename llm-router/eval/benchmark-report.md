# Request-classifier benchmark

Release build; routing columns are `pick_tier` over 50 cold-start seeds (tier costs 15 / 3 / 0.8).

## Held-out validation (154 requests)

| metric | regex | local | laya |
|---|---|---|---|
| type accuracy | 32.5% | 79.2% | 34.4% |
| type macro-F1 | 0.335 | 0.787 | 0.307 |
| complexity exact | 30.5% | 66.2% | 30.5% |
| complexity within ±1 | 69.5% | 98.1% | 72.7% |
| complexity MAE | 1.00 | 0.36 | 0.97 |
| hard (4–5) recognised as hard | 0.0% | 71.4% | 35.7% |
| confidence ECE (lower is better) | 0.157 | 0.100 | 0.276 |
| Brier score (lower is better) | 0.159 | 0.116 | 0.282 |
| latency p50 | 7 µs | 130 µs | 726 ms |
| latency p95 | 84 µs | 632 µs | 1208 ms |
| latency p99 | 166 µs | 911 µs | 1747 ms |
| routing: mean tier cost | 2.18 | 2.70 | 3.75 |
| routing: hard → cheapest tier (lower is better) | 57.3% | 13.5% | 23.6% |
| routing: easy → cheapest tier (higher is better) | 61.4% | 77.1% | 37.3% |
| fell back to regex | 0 | 0 | 0 |

### Accuracy by failure mode

| tag | regex | local | laya |
|---|---|---|---|
| context_dependent (n=22) | 13.6% | 59.1% | 36.4% |
| keyword_trap (n=18) | 44.4% | 77.8% | 38.9% |
| long_spec (n=18) | 44.4% | 77.8% | 38.9% |
| multilingual (n=27) | 25.9% | 70.4% | 37.0% |
| no_keyword (n=19) | 31.6% | 68.4% | 31.6% |
| plain (n=43) | 48.8% | 95.3% | 39.5% |
| short_simple (n=39) | 30.8% | 79.5% | 20.5% |
| typo_informal (n=18) | 16.7% | 94.4% | 11.1% |

### F1 by request type

| type | regex | local | laya |
|---|---|---|---|
| code_generation | 0.41 | 0.89 | 0.48 |
| code_understanding | 0.24 | 0.83 | 0.43 |
| technical_design | 0.24 | 0.79 | 0.42 |
| analytical_reasoning | 0.41 | 0.65 | 0.17 |
| writing | 0.31 | 0.82 | 0.23 |
| factual_lookup | 0.43 | 0.77 | 0.22 |
| general | 0.30 | 0.77 | 0.20 |

### local vs regex: 77 fixed, 5 broken

Broken: `val-077` analytical_reasoning → local said code_generation; `val-138` general → local said writing; `val-146` general → local said factual_lookup; `val-150` general → local said analytical_reasoning; `val-154` general → local said technical_design

### laya vs regex: 31 fixed, 28 broken

Broken: `val-043` code_understanding → laya said technical_design; `val-067` analytical_reasoning → laya said technical_design; `val-070` analytical_reasoning → laya said technical_design; `val-079` analytical_reasoning → laya said technical_design; `val-083` analytical_reasoning → laya said general; `val-089` writing → laya said code_generation; `val-092` writing → laya said code_generation; `val-108` writing → laya said code_generation; `val-125` factual_lookup → laya said technical_design; `val-126` factual_lookup → laya said general; `val-131` factual_lookup → laya said code_generation; `val-133` general → laya said factual_lookup; `val-134` general → laya said code_generation; `val-135` general → laya said code_understanding; `val-138` general → laya said code_generation; `val-139` general → laya said technical_design; `val-140` general → laya said technical_design; `val-141` general → laya said code_generation; `val-142` general → laya said technical_design; `val-144` general → laya said code_understanding; `val-145` general → laya said code_generation; `val-146` general → laya said code_understanding; `val-147` general → laya said technical_design; `val-148` general → laya said technical_design; `val-151` general → laya said code_understanding; `val-152` general → laya said code_understanding; `val-153` general → laya said technical_design; `val-154` general → laya said technical_design

## Hackathon public sample (10 requests)

| metric | regex | local | laya |
|---|---|---|---|
| type accuracy | 30.0% | 80.0% | 40.0% |
| type macro-F1 | 0.262 | 0.619 | 0.202 |
| complexity exact | 20.0% | 50.0% | 20.0% |
| complexity within ±1 | 60.0% | 100.0% | 60.0% |
| complexity MAE | 1.20 | 0.50 | 1.20 |
| hard (4–5) recognised as hard | 0.0% | 100.0% | 33.3% |
| confidence ECE (lower is better) | 0.150 | 0.133 | 0.311 |
| Brier score (lower is better) | 0.165 | 0.099 | 0.208 |
| latency p50 | 17 µs | 318 µs | 727 ms |
| latency p95 | 61 µs | 1259 µs | 1190 ms |
| latency p99 | 61 µs | 1259 µs | 1190 ms |
| routing: mean tier cost | 2.58 | 3.76 | 3.74 |
| routing: hard → cheapest tier (lower is better) | 26.7% | 0.0% | 28.7% |
| routing: easy → cheapest tier (higher is better) | 46.4% | 59.2% | 46.8% |
| fell back to regex | 0 | 0 | 0 |

### Accuracy by failure mode

| tag | regex | local | laya |
|---|---|---|---|
| architecture (n=1) | 0.0% | 100.0% | 0.0% |
| audience (n=1) | 0.0% | 100.0% | 0.0% |
| code_explanation (n=1) | 100.0% | 100.0% | 100.0% |
| code_generation (n=1) | 0.0% | 100.0% | 100.0% |
| code_keyword_low_effort (n=1) | 0.0% | 100.0% | 100.0% |
| constraint_preservation (n=1) | 100.0% | 100.0% | 0.0% |
| context_grounding (n=1) | 0.0% | 100.0% | 0.0% |
| factual (n=1) | 0.0% | 100.0% | 0.0% |
| minimal_edit (n=1) | 0.0% | 100.0% | 100.0% |
| misleading_cache_keyword (n=1) | 0.0% | 0.0% | 0.0% |
| misleading_keywords (n=1) | 0.0% | 100.0% | 100.0% |
| moderate_context (n=2) | 0.0% | 100.0% | 50.0% |
| multi_constraint (n=1) | 0.0% | 100.0% | 0.0% |
| multiple_constraints (n=1) | 0.0% | 100.0% | 100.0% |
| negation (n=1) | 0.0% | 100.0% | 100.0% |
| negative_constraints (n=1) | 0.0% | 100.0% | 0.0% |
| no_context (n=1) | 0.0% | 100.0% | 0.0% |
| reasoning (n=1) | 100.0% | 0.0% | 0.0% |
| risk (n=1) | 100.0% | 0.0% | 0.0% |
| short_context (n=5) | 60.0% | 60.0% | 40.0% |
| short_request (n=2) | 0.0% | 100.0% | 50.0% |
| temporal_reasoning (n=1) | 0.0% | 0.0% | 0.0% |
| writing (n=1) | 100.0% | 100.0% | 0.0% |

### F1 by request type

| type | regex | local | laya |
|---|---|---|---|
| code_generation | 0.00 | 1.00 | 0.75 |
| code_understanding | 0.50 | 0.67 | 0.67 |
| technical_design | 0.00 | 0.67 | 0.00 |
| analytical_reasoning | 0.67 | 0.00 | 0.00 |
| writing | 0.67 | 1.00 | 0.00 |
| factual_lookup | 0.00 | 1.00 | 0.00 |
| general | 0.00 | 0.00 | 0.00 |

### local vs regex: 6 fixed, 1 broken

Broken: `pub-10` analytical_reasoning → local said technical_design

### laya vs regex: 3 fixed, 2 broken

Broken: `pub-04` writing → laya said code_generation; `pub-10` analytical_reasoning → laya said code_understanding

## Labelled requests with chat history (100 requests)

| metric | regex | local | laya |
|---|---|---|---|
| type accuracy | 43.0% | 86.0% | 56.0% |
| type macro-F1 | 0.448 | 0.860 | 0.543 |
| complexity exact | 20.0% | 54.0% | 32.0% |
| complexity within ±1 | 64.0% | 90.0% | 86.0% |
| complexity MAE | 1.16 | 0.60 | 0.84 |
| hard (4–5) recognised as hard | 0.0% | 11.8% | 47.1% |
| confidence ECE (lower is better) | 0.125 | 0.069 | 0.253 |
| Brier score (lower is better) | 0.178 | 0.081 | 0.287 |
| latency p50 | 4 µs | 84 µs | 608 ms |
| latency p95 | 30 µs | 154 µs | 744 ms |
| latency p99 | 54 µs | 177 µs | 864 ms |
| routing: mean tier cost | 2.40 | 2.20 | 3.26 |
| routing: hard → cheapest tier (lower is better) | 54.4% | 31.3% | 21.6% |
| routing: easy → cheapest tier (higher is better) | 57.5% | 71.5% | 53.1% |
| fell back to regex | 0 | 0 | 0 |

### Accuracy by failure mode

| tag | regex | local | laya |
|---|---|---|---|
| context_dependent (n=5) | 0.0% | 80.0% | 80.0% |
| plain (n=95) | 45.3% | 86.3% | 54.7% |

### F1 by request type

| type | regex | local | laya |
|---|---|---|---|
| code_generation | 0.40 | 0.89 | 0.63 |
| code_understanding | 0.35 | 0.93 | 0.56 |
| technical_design | 0.25 | 0.85 | 0.62 |
| analytical_reasoning | 0.70 | 0.74 | 0.45 |
| writing | 0.60 | 0.93 | 0.57 |
| factual_lookup | 0.48 | 0.83 | 0.35 |
| general | 0.36 | 0.86 | 0.62 |

### local vs regex: 46 fixed, 3 broken

Broken: `ar-05` analytical_reasoning → local said factual_lookup; `ar-11` analytical_reasoning → local said general; `ge-03` general → local said factual_lookup

### laya vs regex: 22 fixed, 9 broken

Broken: `ar-07` analytical_reasoning → laya said technical_design; `ar-09` analytical_reasoning → laya said code_generation; `ar-12` analytical_reasoning → laya said code_understanding; `wr-08` writing → laya said code_generation; `fl-01` factual_lookup → laya said general; `fl-12` factual_lookup → laya said general; `ge-02` general → laya said code_understanding; `ge-08` general → laya said code_generation; `ge-11` general → laya said code_generation

