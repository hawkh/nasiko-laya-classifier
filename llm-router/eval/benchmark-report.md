# Classifier benchmark — eval/h4-validation.json (154 requests)

## Headline

| metric                        | regex            | local          | laya              |
|-------------------------------|------------------|----------------|-------------------|
| type accuracy                 |  32.5%           |  79.2%         |  34.4%            |
| type macro-F1                 | 0.335            | 0.787          | 0.307             |
| complexity exact              |  30.5%           |  66.2%         |  30.5%            |
| complexity within ±1          |  69.5%           |  98.1%         |  72.7%            |
| complexity MAE                | 1.00             | 0.36           | 0.97              |
| hard (4–5) recall             |   0.0%           |  71.4%         |  35.7%            |
| confidence ECE (lower=better) | 0.157            | 0.100          | 0.276             |
| Brier score (lower=better)    | 0.159            | 0.116          | 0.282             |
| latency p50                   | 2 µs             | 55 µs          | 706 ms            |
| latency p95                   | 31 µs            | 255 µs         | 1,062 ms          |
| latency p99                   | 92 µs            | 387 µs         | 1,306 ms          |
| latency mean / max            | 56 µs / 7,759 µs | 83 µs / 395 µs | 739 ms / 1,309 ms |

## Where regex fails — accuracy by failure mode (`tests` tag)

| tag               | n  | regex  | local  | laya   | local vs regex | laya vs regex |
|-------------------|----|--------|--------|--------|----------------|---------------|
| context_dependent | 22 |  13.6% |  59.1% |  36.4% | +45.5 pts      | +22.7 pts     |
| typo_informal     | 18 |  16.7% |  94.4% |  11.1% | +77.8 pts      |  -5.6 pts     |
| multilingual      | 27 |  25.9% |  70.4% |  37.0% | +44.4 pts      | +11.1 pts     |
| short_simple      | 39 |  30.8% |  79.5% |  20.5% | +48.7 pts      | -10.3 pts     |
| no_keyword        | 19 |  31.6% |  68.4% |  31.6% | +36.8 pts      |  +0.0 pts     |
| keyword_trap      | 18 |  44.4% |  77.8% |  38.9% | +33.3 pts      |  -5.6 pts     |
| long_spec         | 18 |  44.4% |  77.8% |  38.9% | +33.3 pts      |  -5.6 pts     |
| plain             | 43 |  48.8% |  95.3% |  39.5% | +46.5 pts      |  -9.3 pts     |

## Per request type — F1

| type                 | n  | regex | local | laya |
|----------------------|----|-------|-------|------|
| code_generation      | 22 | 0.41  | 0.89  | 0.48 |
| code_understanding   | 22 | 0.24  | 0.83  | 0.43 |
| technical_design     | 22 | 0.24  | 0.79  | 0.42 |
| analytical_reasoning | 22 | 0.41  | 0.65  | 0.17 |
| writing              | 22 | 0.31  | 0.82  | 0.23 |
| factual_lookup       | 22 | 0.43  | 0.77  | 0.22 |
| general              | 22 | 0.30  | 0.77  | 0.20 |

## local vs regex: 77 fixed, 5 broken

Requests regex misroutes that local gets right (first 12):

- `val-001` [plain, short_simple] "Fix the typo 'recieve' in the comment above this function."
  gold **code_generation** · regex said general
- `val-002` [typo_informal, short_simple] 'rename var tmp to retry_count pls'
  gold **code_generation** · regex said general
- `val-003` [keyword_trap, short_simple] "I wrote 'redesign auth' in a TODO; do not redesign anything, just change TODO to NOTE."
  gold **code_generation** · regex said general
- `val-005` [plain] "Add a pytest unit test for parse_price that checks '$1,200.50' returns 1200.5."
  gold **code_generation** · regex said general
- `val-007` [multilingual] "Ek Python function likho jo list of dicts ko 'age' key ke basis pe descending order mei..."
  gold **code_generation** · regex said general
- `val-008` [multilingual] 'Escribe una consulta SQL que devuelva los 5 clientes con más pedidos en 2025.'
  gold **code_generation** · regex said general
- `val-009` [context_dependent] 'Make it also reject negative quantities with a 400.'
  gold **code_generation** · regex said general
- `val-011` [long_spec] 'Implement an Express middleware for sliding-window rate limiting backed by Redis. Limit...'
  gold **code_generation** · regex said general
- `val-014` [typo_informal] 'need a regex that grabs all the hashtags from a tweet, js'
  gold **code_generation** · regex said general
- `val-015` [plain] 'Convert this class component to a function component with hooks, keeping the 30 second ...'
  gold **code_generation** · regex said general
- `val-016` [multilingual] "Écris un Dockerfile multi-étapes pour une application Node 20 qui n'installe que les dé..."
  gold **code_generation** · regex said general
- `val-019` [short_simple, context_dependent] 'Add a semicolon at the end of line 3.'
  gold **code_generation** · regex said general

Requests local gets wrong that regex got right:

- `val-077` 'Prove by induction that 1^3 + 2^3 + ... + n^3 = (n(n+1)/2)^2.' gold analytical_reasoning · local said code_generation
- `val-138` 'My code has been compiling for three hours and honestly so has my patience. Any advice ...' gold general · local said writing
- `val-146` "Wie geht's dir heute?" gold general · local said factual_lookup
- `val-150` 'Which of the two would you pick?' gold general · local said analytical_reasoning
- `val-154` 'I have a week to prepare a 20-minute presentation to senior leadership about a project ...' gold general · local said technical_design

## laya vs regex: 31 fixed, 28 broken

Requests regex misroutes that laya gets right (first 12):

- `val-001` [plain, short_simple] "Fix the typo 'recieve' in the comment above this function."
  gold **code_generation** · regex said general
- `val-003` [keyword_trap, short_simple] "I wrote 'redesign auth' in a TODO; do not redesign anything, just change TODO to NOTE."
  gold **code_generation** · regex said general
- `val-007` [multilingual] "Ek Python function likho jo list of dicts ko 'age' key ke basis pe descending order mei..."
  gold **code_generation** · regex said general
- `val-008` [multilingual] 'Escribe una consulta SQL que devuelva los 5 clientes con más pedidos en 2025.'
  gold **code_generation** · regex said general
- `val-013` [no_keyword, context_dependent] 'Turn this nested loop into something that runs in linear time.'
  gold **code_generation** · regex said general
- `val-015` [plain] 'Convert this class component to a function component with hooks, keeping the 30 second ...'
  gold **code_generation** · regex said general
- `val-016` [multilingual] "Écris un Dockerfile multi-étapes pour une application Node 20 qui n'installe que les dé..."
  gold **code_generation** · regex said general
- `val-019` [short_simple, context_dependent] 'Add a semicolon at the end of line 3.'
  gold **code_generation** · regex said general
- `val-020` [no_keyword, short_simple] 'Give me a one-liner to list the ten largest files under the current folder on Linux.'
  gold **code_generation** · regex said general
- `val-022` [typo_informal, context_dependent] 'yo can u make this fn async and add a 5s timeout, using asyncio'
  gold **code_generation** · regex said general
- `val-024` [plain] 'Why does this print [1, 2, 3, 4] for both lists?'
  gold **code_understanding** · regex said general
- `val-025` [plain] 'Can you explain this stack trace? It crashes only when the user has no profile picture.'
  gold **code_understanding** · regex said general

Requests laya gets wrong that regex got right:

- `val-043` 'Explain what this Kubernetes manifest will actually do when applied, especially the pro...' gold code_understanding · laya said technical_design
- `val-067` 'A diagnostic test has 95% sensitivity and 90% specificity. If 2% of the people tested a...' gold analytical_reasoning · laya said technical_design
- `val-070` 'Estimate how many EV charging stations a city of 800,000 people needs by 2030.' gold analytical_reasoning · laya said technical_design
- `val-079` 'Three independent servers each fail with probability 0.05 during a given hour. What is ...' gold analytical_reasoning · laya said technical_design
- `val-083` 'How many samples do I need per arm to detect a 2 percentage point lift on a 10% baselin...' gold analytical_reasoning · laya said general
- `val-089` 'Write a short email to my landlord asking him to fix the leaking kitchen tap.' gold writing · laya said code_generation
- `val-092` 'Draft a launch announcement for our new offline mode that works for both the blog and a...' gold writing · laya said code_generation
- `val-108` 'Write a short poem about a segmentation fault at 3am.' gold writing · laya said code_generation
- `val-125` 'What are the four ACID properties in database transactions, and what does each mean?' gold factual_lookup · laya said technical_design
- `val-126` 'What is the maximum size of a single item in DynamoDB?' gold factual_lookup · laya said general
- `val-131` 'When was the first iPhone released?' gold factual_lookup · laya said code_generation
- `val-133` 'thanks, that fixed it!' gold general · laya said factual_lookup
- `val-134` 'Hi there' gold general · laya said code_generation
- `val-135` 'What can you help me with?' gold general · laya said code_understanding
- `val-138` 'My code has been compiling for three hours and honestly so has my patience. Any advice ...' gold general · laya said code_generation
- `val-139` 'debugging my sleep schedule, any tips? I keep waking up at 3am' gold general · laya said technical_design
- `val-140` "Is it worth learning Rust in 2026 or is it hype? I'm a Java dev tired of boilerplate." gold general · laya said technical_design
- `val-141` 'आज मन बहुत उदास है, कुछ अच्छा बोलो।' gold general · laya said code_generation
- `val-142` 'Yaar interview se pehle bohot nervous hun, kuch tips do na.' gold general · laya said technical_design
- `val-144` 'Gracias por tu ayuda, eres genial.' gold general · laya said code_understanding
- `val-145` 'Que tipo de música é boa para estudar?' gold general · laya said code_generation
- `val-146` "Wie geht's dir heute?" gold general · laya said code_understanding
- `val-147` 'ok but which laptop shud i buy for college, under 800 bucks' gold general · laya said technical_design
- `val-148` 'I have two job offers. One is a 40% raise at a bank with a long commute and rigid hours...' gold general · laya said technical_design
- `val-151` 'lol nice' gold general · laya said code_understanding
- `val-152` 'Do you ever get tired of questions like these?' gold general · laya said code_understanding
- `val-153` 'How should I split rent fairly with a roommate who has the bigger room?' gold general · laya said technical_design
- `val-154` 'I have a week to prepare a 20-minute presentation to senior leadership about a project ...' gold general · laya said technical_design

# Classifier benchmark — eval/h4-public-sample.json (10 requests)

## Headline

| metric                        | regex            | local           | laya              |
|-------------------------------|------------------|-----------------|-------------------|
| type accuracy                 |  30.0%           |  80.0%          |  40.0%            |
| type macro-F1                 | 0.262            | 0.619           | 0.202             |
| complexity exact              |  20.0%           |  50.0%          |  20.0%            |
| complexity within ±1          |  60.0%           | 100.0%          |  60.0%            |
| complexity MAE                | 1.20             | 0.50            | 1.20              |
| hard (4–5) recall             |   0.0%           | 100.0%          |  33.3%            |
| confidence ECE (lower=better) | 0.150            | 0.133           | 0.311             |
| Brier score (lower=better)    | 0.165            | 0.099           | 0.208             |
| latency p50                   | 12 µs            | 149 µs          | 789 ms            |
| latency p95                   | 13 ms            | 745 µs          | 1,651 ms          |
| latency p99                   | 13 ms            | 745 µs          | 1,651 ms          |
| latency mean / max            | 1,289 µs / 13 ms | 308 µs / 745 µs | 921 ms / 1,651 ms |

## Where regex fails — accuracy by failure mode (`tests` tag)

| tag                      | n | regex  | local  | laya   | local vs regex | laya vs regex |
|--------------------------|---|--------|--------|--------|----------------|---------------|
| architecture             | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| audience                 | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| code_generation          | 1 |   0.0% | 100.0% | 100.0% | +100.0 pts     | +100.0 pts    |
| code_keyword_low_effort  | 1 |   0.0% | 100.0% | 100.0% | +100.0 pts     | +100.0 pts    |
| context_grounding        | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| factual                  | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| minimal_edit             | 1 |   0.0% | 100.0% | 100.0% | +100.0 pts     | +100.0 pts    |
| misleading_cache_keyword | 1 |   0.0% |   0.0% |   0.0% |  +0.0 pts      |  +0.0 pts     |
| misleading_keywords      | 1 |   0.0% | 100.0% | 100.0% | +100.0 pts     | +100.0 pts    |
| moderate_context         | 2 |   0.0% | 100.0% |  50.0% | +100.0 pts     | +50.0 pts     |
| multi_constraint         | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| multiple_constraints     | 1 |   0.0% | 100.0% | 100.0% | +100.0 pts     | +100.0 pts    |
| negation                 | 1 |   0.0% | 100.0% | 100.0% | +100.0 pts     | +100.0 pts    |
| negative_constraints     | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| no_context               | 1 |   0.0% | 100.0% |   0.0% | +100.0 pts     |  +0.0 pts     |
| short_request            | 2 |   0.0% | 100.0% |  50.0% | +100.0 pts     | +50.0 pts     |
| temporal_reasoning       | 1 |   0.0% |   0.0% |   0.0% |  +0.0 pts      |  +0.0 pts     |
| short_context            | 5 |  60.0% |  60.0% |  40.0% |  +0.0 pts      | -20.0 pts     |
| code_explanation         | 1 | 100.0% | 100.0% | 100.0% |  +0.0 pts      |  +0.0 pts     |
| constraint_preservation  | 1 | 100.0% | 100.0% |   0.0% |  +0.0 pts      | -100.0 pts    |
| reasoning                | 1 | 100.0% |   0.0% |   0.0% | -100.0 pts     | -100.0 pts    |
| risk                     | 1 | 100.0% |   0.0% |   0.0% | -100.0 pts     | -100.0 pts    |
| writing                  | 1 | 100.0% | 100.0% |   0.0% |  +0.0 pts      | -100.0 pts    |

## Per request type — F1

| type                 | n | regex | local | laya |
|----------------------|---|-------|-------|------|
| code_generation      | 3 | 0.00  | 1.00  | 0.75 |
| code_understanding   | 1 | 0.50  | 0.67  | 0.67 |
| technical_design     | 1 | 0.00  | 0.67  | 0.00 |
| analytical_reasoning | 2 | 0.67  | 0.00  | 0.00 |
| writing              | 2 | 0.67  | 1.00  | 0.00 |
| factual_lookup       | 1 | 0.00  | 1.00  | 0.00 |
| general              | 0 | 0.00  | 0.00  | 0.00 |

## local vs regex: 6 fixed, 1 broken

Requests regex misroutes that local gets right (first 12):

- `pub-01` [short_request, short_context, code_keyword_low_effort] 'Fix typo in this Python comment: `# retrun the cached value`.'
  gold **code_generation** · regex said general
- `pub-02` [short_request, no_context, factual] 'What does `Option::take()` do in Rust? Answer in one sentence.'
  gold **factual_lookup** · regex said general
- `pub-05` [multi_constraint, architecture, moderate_context] 'Design migration from synchronous payment-status callbacks to queued processing without...'
  gold **technical_design** · regex said code_understanding
- `pub-07` [code_generation, multiple_constraints, moderate_context] 'Implement a parser that accepts comma-separated integers, rejects empty fields, and rep...'
  gold **code_generation** · regex said general
- `pub-08` [misleading_keywords, negation, minimal_edit] "I wrote 'redesign authentication' as a TODO comment. Do not redesign anything: just cha..."
  gold **code_generation** · regex said general
- `pub-09` [context_grounding, audience, negative_constraints] 'Summarize the following release notes in three bullets for a nontechnical customer. Do ...'
  gold **writing** · regex said general

Requests local gets wrong that regex got right:

- `pub-10` 'Investigate why daily reconciliations show both duplicate charges and missing refunds. ...' gold analytical_reasoning · local said technical_design

## laya vs regex: 3 fixed, 2 broken

Requests regex misroutes that laya gets right (first 12):

- `pub-01` [short_request, short_context, code_keyword_low_effort] 'Fix typo in this Python comment: `# retrun the cached value`.'
  gold **code_generation** · regex said general
- `pub-07` [code_generation, multiple_constraints, moderate_context] 'Implement a parser that accepts comma-separated integers, rejects empty fields, and rep...'
  gold **code_generation** · regex said general
- `pub-08` [misleading_keywords, negation, minimal_edit] "I wrote 'redesign authentication' as a TODO comment. Do not redesign anything: just cha..."
  gold **code_generation** · regex said general

Requests laya gets wrong that regex got right:

- `pub-04` "Rewrite this notification to be warmer, but keep the date and time unchanged: 'Your int..." gold writing · laya said code_generation
- `pub-10` 'Investigate why daily reconciliations show both duplicate charges and missing refunds. ...' gold analytical_reasoning · laya said code_understanding

# Classifier benchmark — requests-h4.json (100 requests)

## Headline

| metric                        | regex          | local          | laya              |
|-------------------------------|----------------|----------------|-------------------|
| type accuracy                 |  43.0%         |  86.0%         |  56.0%            |
| type macro-F1                 | 0.448          | 0.860          | 0.543             |
| complexity exact              |  20.0%         |  54.0%         |  32.0%            |
| complexity within ±1          |  64.0%         |  90.0%         |  86.0%            |
| complexity MAE                | 1.16           | 0.60           | 0.84              |
| hard (4–5) recall             |   0.0%         |  11.8%         |  47.1%            |
| confidence ECE (lower=better) | 0.125          | 0.069          | 0.248             |
| Brier score (lower=better)    | 0.178          | 0.081          | 0.286             |
| latency p50                   | 6 µs           | 92 µs          | 603 ms            |
| latency p95                   | 71 µs          | 150 µs         | 849 ms            |
| latency p99                   | 97 µs          | 176 µs         | 1,181 ms          |
| latency mean / max            | 169 µs / 15 ms | 90 µs / 229 µs | 649 ms / 2,010 ms |

## Where regex fails — accuracy by failure mode (`tests` tag)

| tag               | n  | regex  | local  | laya   | local vs regex | laya vs regex |
|-------------------|----|--------|--------|--------|----------------|---------------|
| context_dependent | 5  |   0.0% |  80.0% |  80.0% | +80.0 pts      | +80.0 pts     |
| plain             | 95 |  45.3% |  86.3% |  54.7% | +41.1 pts      |  +9.5 pts     |

## Per request type — F1

| type                 | n  | regex | local | laya |
|----------------------|----|-------|-------|------|
| code_generation      | 14 | 0.40  | 0.89  | 0.63 |
| code_understanding   | 14 | 0.35  | 0.93  | 0.56 |
| technical_design     | 14 | 0.25  | 0.85  | 0.62 |
| analytical_reasoning | 14 | 0.70  | 0.74  | 0.45 |
| writing              | 14 | 0.60  | 0.93  | 0.57 |
| factual_lookup       | 14 | 0.48  | 0.83  | 0.35 |
| general              | 16 | 0.36  | 0.86  | 0.62 |

## local vs regex: 46 fixed, 3 broken

Requests regex misroutes that local gets right (first 12):

- `cg-02` [plain] 'Add type hints to this function: def add(a, b): return a + b'
  gold **code_generation** · regex said general
- `cg-03` [plain] 'Implement an LRU cache in Rust with O(1) get and put, generic over key and value types.'
  gold **code_generation** · regex said general
- `cg-04` [context_dependent] 'now add pagination and rate limiting to it'
  gold **code_generation** · regex said general
- `cg-05` [plain] 'Design a sharded, lock-free hash map in C++ with linearizable resize.'
  gold **code_generation** · regex said general
- `cg-07` [plain] 'Write a bash one-liner to count the lines in all .py files under the current directory.'
  gold **code_generation** · regex said general
- `cg-10` [plain] 'mujhe ek Python script likh ke do jo CSV file padhe aur har column ka average nikale'
  gold **code_generation** · regex said general
- `cg-11` [plain] 'Escribe una función en JavaScript que valide direcciones de correo electrónico.'
  gold **code_generation** · regex said general
- `cg-12` [plain] 'Create a React component for a paginated table with sortable columns and server-side fi...'
  gold **code_generation** · regex said general
- `cu-02` [plain] "Why does Rust say 'borrowed value does not live long enough' when I return a reference ..."
  gold **code_understanding** · regex said general
- `cu-03` [plain] 'What exactly does git rebase --onto do to my branch history?'
  gold **code_understanding** · regex said general
- `cu-04` [plain] 'My Kubernetes pod keeps restarting with exit code 137, what is going on?'
  gold **code_understanding** · regex said general
- `cu-06` [plain] 'Why is this Python code so slow? for x in big_list:     if x in other_list:         hit...'
  gold **code_understanding** · regex said general

Requests local gets wrong that regex got right:

- `ar-05` 'What is 17 * 24?' gold analytical_reasoning · local said factual_lookup
- `ar-11` 'Solve for x: 2x + 7 = 19' gold analytical_reasoning · local said general
- `ge-03` 'kya haal hai bhai' gold general · local said factual_lookup

## laya vs regex: 22 fixed, 9 broken

Requests regex misroutes that laya gets right (first 12):

- `cg-02` [plain] 'Add type hints to this function: def add(a, b): return a + b'
  gold **code_generation** · regex said general
- `cg-03` [plain] 'Implement an LRU cache in Rust with O(1) get and put, generic over key and value types.'
  gold **code_generation** · regex said general
- `cg-04` [context_dependent] 'now add pagination and rate limiting to it'
  gold **code_generation** · regex said general
- `cg-05` [plain] 'Design a sharded, lock-free hash map in C++ with linearizable resize.'
  gold **code_generation** · regex said general
- `cg-07` [plain] 'Write a bash one-liner to count the lines in all .py files under the current directory.'
  gold **code_generation** · regex said general
- `cg-08` [plain] 'Fix the off-by-one bug: for i in range(len(a) + 1): print(a[i])'
  gold **code_generation** · regex said general
- `cg-11` [plain] 'Escribe una función en JavaScript que valide direcciones de correo electrónico.'
  gold **code_generation** · regex said general
- `cg-12` [plain] 'Create a React component for a paginated table with sortable columns and server-side fi...'
  gold **code_generation** · regex said general
- `cg-14` [context_dependent] 'make it handle unicode escapes and keep zero-copy slices'
  gold **code_generation** · regex said general
- `cu-03` [plain] 'What exactly does git rebase --onto do to my branch history?'
  gold **code_understanding** · regex said general
- `cu-04` [plain] 'My Kubernetes pod keeps restarting with exit code 137, what is going on?'
  gold **code_understanding** · regex said general
- `cu-06` [plain] 'Why is this Python code so slow? for x in big_list:     if x in other_list:         hit...'
  gold **code_understanding** · regex said general

Requests laya gets wrong that regex got right:

- `ar-07` 'Estimate how many piano tuners work in Chicago.' gold analytical_reasoning · laya said technical_design
- `ar-09` 'Derive a closed form for the sum of the first n squares and prove it by induction.' gold analytical_reasoning · laya said code_generation
- `ar-12` 'Prove that the halting problem is undecidable.' gold analytical_reasoning · laya said code_understanding
- `wr-08` 'Write a launch announcement blog post for our new public API.' gold writing · laya said code_generation
- `fl-01` 'What is the capital of France?' gold factual_lookup · laya said general
- `fl-12` 'What is the time complexity of binary search?' gold factual_lookup · laya said general
- `ge-02` 'thanks, that was really helpful!' gold general · laya said code_understanding
- `ge-08` 'lol ok' gold general · laya said code_generation
- `ge-11` 'hmm, let me think about it' gold general · laya said code_generation

raw outputs: /tmp/bench
