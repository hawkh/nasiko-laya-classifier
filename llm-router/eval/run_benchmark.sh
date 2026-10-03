#!/usr/bin/env bash
# One command, every benchmark: regex vs. the in-process local model (vs. Laya when a
# laya-serve is reachable) on three gold sets, through the official `classifier_eval`
# harness. Writes eval/benchmark-report.md and prints it.
#
#   bash eval/run_benchmark.sh            # from llm-router/
#   LAYA_URL=http://localhost:8000 bash eval/run_benchmark.sh
set -euo pipefail
cd "$(dirname "$0")/.."
OUT_DIR="${OUT_DIR:-$(mktemp -d)}"
LAYA_URL="${LAYA_URL:-http://localhost:8000}"
PY="${PYTHON:-python3}"

# requests.jsonl (hand-labelled, with chat context) -> the H4 schema the harness reads.
"$PY" - "$OUT_DIR/requests-h4.json" <<'PY'
import json, sys
ex = []
for line in open("eval/requests.jsonl", encoding="utf-8"):
    r = json.loads(line)
    ctx = "\n".join(f"[{m['role']}] {m['text']}" for m in reversed(r["context"]))
    ex.append({"id": r["id"], "query": r["query"], "context": ctx,
               "request_type": r["type"], "complexity": r["complexity"],
               "tests": ["context_dependent"] if r["context"] else ["plain"]})
json.dump({"examples": ex}, open(sys.argv[1], "w", encoding="utf-8"))
PY

backends=(regex local)
if curl -sf -m 3 "$LAYA_URL/health" >/dev/null; then
  backends+=(laya)
else
  echo "laya-serve not reachable at $LAYA_URL — benchmarking regex and local only" >&2
fi

cargo build -q --release -p nasiko-llm-router --example classifier_eval
BIN="${CARGO_TARGET_DIR:-../target}/release/examples/classifier_eval"

report=eval/benchmark-report.md
: > "$report"
for gold in eval/h4-validation.json eval/h4-public-sample.json "$OUT_DIR/requests-h4.json"; do
  [ -f "$gold" ] || continue
  name=$(basename "$gold" .json)
  specs=()
  for b in "${backends[@]}"; do
    REQUEST_CLASSIFIER=$b LAYA_URL=$LAYA_URL EVAL_SET=$gold OUT="$OUT_DIR/$name-$b.jsonl" \
      "$BIN" 2>"$OUT_DIR/$name-$b.log"
    specs+=("$b=$OUT_DIR/$name-$b.jsonl")
  done
  "$PY" eval/benchmark.py "$gold" "${specs[@]}" | sed "s#$OUT_DIR/##" >> "$report"
done
cat "$report"
echo "raw outputs: $OUT_DIR" >&2
