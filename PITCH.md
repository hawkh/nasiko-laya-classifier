# P2 — Request classifier: pitch

## 1. The problem (30 seconds)

The router picks a model tier for every request. At Level 3 it decides with a keyword regex.
On our 154 held-out requests that regex:

- **gets the request type right 32% of the time.** Anything without a keyword becomes "general";
- **has no notion of difficulty.** Every request is "complexity 3", so the router can't tell "hi"
  from "design a lock-free hash map";
- **sends 57% of genuinely hard requests to the cheapest tier.**

It can't read other languages, typos, negations ("don't redesign anything…") or the previous
turn ("make it funnier"). Every miss costs money on easy traffic and answer quality on hard traffic.

## 2. The idea: give the router a belief, not a label

We did not bolt a classifier on with hard rules such as "complexity ≥ 4 → Tier1". The router
already has a learning decision-maker, Thompson sampling over learned quality cells, and we
feed the classifier's *uncertainty* into it:

- **Type.** The classifier returns a probability for each type, and the router *draws* the type
  from it. An unsure classifier therefore routes like the mix it's unsure between, instead of
  committing to a wrong guess.
- **Complexity.** It shifts the tier starting points, weighted by how confident the classifier
  is. A confident "trivial" leans toward the cheap tier; an unsure guess barely moves anything.
- **Feedback still rules.** Thompson sampling keeps learning from real outcomes. The classifier
  only sets where learning starts and which cell gets credited.

**Zero regression risk.** With `REQUEST_CLASSIFIER=regex` (the default) routing is identical to
main for the same random seed. A test over 200 seeds proves it.

## 3. What we built

One seam (`RequestClassifier`) with three interchangeable backends, chosen by one env var:

| Backend | What it is | Latency | Can it fail? |
|---|---|---|---|
| `regex` | Today's behaviour, the default | µs | no |
| `local` | Our in-process model: hashed word and character n-grams with two calibrated linear heads (type, complexity), compiled into the binary | ~0.1 ms p50, <1 ms p99 | no |
| `laya` | Multilingual model sidecar (`laya-serve`) | ~0.7 s p50 | yes, so it has a circuit breaker (3 strikes → 30 s open → probe) and falls back to regex. A request never fails because of the classifier |

The router also reads **conversation context**: the latest request plus the three most recent
earlier turns, bounded and deterministic. Follow-ups like "make it funnier and shorter" are
classified correctly.

## 4. Evidence, inside the pipeline

The benchmark is a `cargo test` (`tests/classifier_benchmark.rs`) that runs in CI on every
llm-router PR and posts its report to the job summary. It scores every backend through the
router's own code paths on three labelled sets, and **its quality gates fail the build if the
model regresses**.

**Held-out validation, 154 requests.** Each is tagged with the regex failure mode it tests.

| | regex | **local** | laya |
|---|---|---|---|
| Type accuracy | 32.5% | **79.2%** | 34.4% |
| Macro-F1 | 0.34 | **0.79** | 0.31 |
| Complexity within ±1 | 70% | **98%** | 73% |
| Hard requests recognised as hard | 0% | **71%** | 36% |
| Confidence calibration (ECE, lower is better) | 0.16 | **0.10** | 0.28 |
| **Routing: hard → cheapest tier** (lower is better) | 57% | **13.5%** | 24% |
| **Routing: easy → cheapest tier** (higher is better) | 61% | **77%** | 37% |
| Latency p99 | 0.2 ms | **0.9 ms** | 1.7 s |

**Where regex fails, and what local recovers** (accuracy by failure mode):

| Failure mode | regex | local |
|---|---|---|
| Typos, informal | 17% | 94% |
| Context-dependent follow-ups | 14% | 59% |
| Other languages (Hinglish, Spanish, French, …) | 26% | 70% |
| No keywords | 32% | 68% |
| Keyword traps ("refactor my resume") | 44% | 78% |

The hackathon's public sample (10): 30% regex vs 80% local. Our 100 requests with real chat
history: 43% vs 86%.

**The money story.** The router stops starving hard requests (57% → 13.5% sent to the cheapest
tier) and sends *more* easy traffic to the cheapest tier (61% → 77%). Average spend per request
rises a little (2.2 → 2.7 tier-cost units) because the spend moves to where quality is at stake.

## 5. Engineering discipline

- **One featurisation.** Rust computes the features and Python only trains on what Rust dumps,
  so training and serving cannot drift.
- **Leakage guard.** The trainer refuses to run if a validation query near-duplicates a training
  query. It caught 2 exact duplicates.
- **Honest scoring.** The report lists the requests local *breaks* (5) next to the ones it
  fixes (77).
- **Learning simulation.** 3000 rounds of online learning with feedback, not just one-shot
  accuracy.
- **Operations.** Every Level 3 decision is logged (backend, fallback reason, type
  distribution, complexity, prior shift, latency), so it can be watched and rolled back with
  one env var.

## 6. The hard question: "a logistic regression vs a language model?"

Answer it head-on. It's the reviewer's most likely objection.

- **Why local wins *here*.** It was trained on *our* seven labels and *our* complexity scale.
  Laya is general-purpose and answers zero-shot through multiple-choice questions, so it
  confuses our label boundaries (it calls casual chat "code_understanding", and maths
  "technical_design"). It's a label-alignment problem, not a language-understanding one.
- **Where local is weaker.** It has no real semantics: it only knows the 552 examples it was
  trained on. It misses things such as "What is 17 × 24?" and German small talk. On one set it
  recognises only 12% of hard requests as hard, against Laya's 47%. Our validation data is
  similar in style to the training data, so 79% is likely optimistic for real traffic.
- **Why that's acceptable in a router.** The classifier only sets *starting points*; Thompson
  sampling corrects them with real feedback. And it costs 0.1 ms, against Laya's 0.7 s.

**The production design (next step): a cascade.**

1. Local answers every request in under a millisecond.
2. Only when local's confidence is low does the router ask the language model.

That's language-model judgement where it matters, at a fraction of the latency. The long-term
version replaces the linear model with a small multilingual encoder (a BERT-class model)
*fine-tuned on these labels*, which combines both strengths. The seam, the benchmark and the
gates already exist, so swapping in a better model is a drop-in.

**To prove it on real traffic:** run in shadow mode, where the router logs local's and Laya's
decisions next to the live one, then compare with real feedback before switching.

## 7. One-liner

> We turned the router's keyword guess into a calibrated belief (type, difficulty, confidence)
> and fed that uncertainty into the router's own learning loop. It's 2.4× more accurate than
> regex, cuts hard requests sent to the cheapest tier from 57% to 13.5%, runs in under a
> millisecond, can't take the router down, and is identical to today when switched off.
