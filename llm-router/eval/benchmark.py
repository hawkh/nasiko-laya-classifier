"""Score classifier outputs from `examples/classifier_eval.rs` against gold labels.

    python eval/benchmark.py GOLD.json regex=out-regex.jsonl local=out-local.jsonl [laya=...]

GOLD is an H4 eval file (`{"examples": [{id, query, request_type, complexity, tests?}]}`).
Prints, for every backend: type accuracy and macro-F1, per-type F1, complexity exact / ±1 /
MAE / hard-request recall, confidence calibration (ECE, Brier), and latency p50/p95/p99.
Then the part that matters: accuracy per failure mode (`tests` tag) and per type, with the
delta against regex, and the concrete requests regex gets wrong that the others get right
(and the reverse, so the comparison stays honest). Standard library only.
"""

import json
import math
import sys
from collections import defaultdict

TYPES = [
    "code_generation",
    "code_understanding",
    "technical_design",
    "analytical_reasoning",
    "writing",
    "factual_lookup",
    "general",
]


def pct(x):
    return f"{100 * x:5.1f}%"


def quantile(sorted_vals, q):
    if not sorted_vals:
        return float("nan")
    i = min(len(sorted_vals) - 1, max(0, math.ceil(q * len(sorted_vals)) - 1))
    return sorted_vals[i]


def f1_table(gold, pred):
    out = {}
    for t in TYPES:
        tp = sum(g == t and p == t for g, p in zip(gold, pred))
        fp = sum(g != t and p == t for g, p in zip(gold, pred))
        fn = sum(g == t and p != t for g, p in zip(gold, pred))
        prec = tp / (tp + fp) if tp + fp else 0.0
        rec = tp / (tp + fn) if tp + fn else 0.0
        out[t] = (prec, rec, 2 * prec * rec / (prec + rec) if prec + rec else 0.0)
    return out


def ece(conf, correct, bins=10):
    total, err = len(conf), 0.0
    for b in range(bins):
        lo, hi = b / bins, (b + 1) / bins
        idx = [i for i, c in enumerate(conf) if lo <= c < hi or (b == bins - 1 and c == 1.0)]
        if idx:
            acc = sum(correct[i] for i in idx) / len(idx)
            avg = sum(conf[i] for i in idx) / len(idx)
            err += len(idx) / total * abs(acc - avg)
    return err


def metrics(gold, rows):
    ids = [e["id"] for e in gold]
    by_id = {r["id"]: r for r in rows}
    missing = [i for i in ids if i not in by_id]
    if missing:
        sys.exit(f"output is missing {len(missing)} ids, e.g. {missing[:3]}")
    g_type = [e["request_type"] for e in gold]
    p_type = [by_id[i]["request_type"] for i in ids]
    g_cx = [e["complexity"] for e in gold]
    p_cx = [by_id[i]["complexity"] for i in ids]
    conf = [float(by_id[i]["confidence"]) for i in ids]
    correct = [g == p for g, p in zip(g_type, p_type)]
    lat = sorted(by_id[i]["latency_us"] for i in ids)
    f1 = f1_table(g_type, p_type)
    hard = [i for i, c in enumerate(g_cx) if c >= 4]
    return {
        "n": len(ids),
        "correct": dict(zip(ids, correct)),
        "pred": dict(zip(ids, zip(p_type, p_cx))),
        "acc": sum(correct) / len(ids),
        "macro_f1": sum(v[2] for v in f1.values()) / len(TYPES),
        "f1": f1,
        "cx_exact": sum(g == p for g, p in zip(g_cx, p_cx)) / len(ids),
        "cx_within1": sum(abs(g - p) <= 1 for g, p in zip(g_cx, p_cx)) / len(ids),
        "cx_mae": sum(abs(g - p) for g, p in zip(g_cx, p_cx)) / len(ids),
        "hard_recall": sum(p_cx[i] >= 4 for i in hard) / len(hard) if hard else float("nan"),
        "ece": ece(conf, correct),
        "brier": sum((c - k) ** 2 for c, k in zip(conf, correct)) / len(ids),
        "p50": quantile(lat, 0.50),
        "p95": quantile(lat, 0.95),
        "p99": quantile(lat, 0.99),
        "mean": sum(lat) / len(lat),
        "max": lat[-1],
    }


def fmt_us(us):
    return f"{us:,.0f} µs" if us < 10_000 else f"{us / 1000:,.0f} ms"


def table(header, rows):
    widths = [max(len(str(r[i])) for r in [header] + rows) for i in range(len(header))]
    line = lambda r: "| " + " | ".join(str(c).ljust(w) for c, w in zip(r, widths)) + " |"
    print(line(header))
    print("|" + "|".join("-" * (w + 2) for w in widths) + "|")
    for r in rows:
        print(line(r))
    print()


def main():
    gold_path, *specs = sys.argv[1:]
    gold = json.load(open(gold_path, encoding="utf-8"))["examples"]
    backends = {}
    for spec in specs:
        name, path = spec.split("=", 1)
        rows = [json.loads(l) for l in open(path, encoding="utf-8") if l.strip()]
        backends[name] = metrics(gold, rows)
    names = list(backends)
    base = "regex" if "regex" in backends else names[0]
    print(f"# Classifier benchmark — {gold_path} ({len(gold)} requests)\n")

    print("## Headline\n")
    m = backends
    table(
        ["metric"] + names,
        [
            ["type accuracy"] + [pct(m[n]["acc"]) for n in names],
            ["type macro-F1"] + [f"{m[n]['macro_f1']:.3f}" for n in names],
            ["complexity exact"] + [pct(m[n]["cx_exact"]) for n in names],
            ["complexity within ±1"] + [pct(m[n]["cx_within1"]) for n in names],
            ["complexity MAE"] + [f"{m[n]['cx_mae']:.2f}" for n in names],
            ["hard (4–5) recall"] + [pct(m[n]["hard_recall"]) for n in names],
            ["confidence ECE (lower=better)"] + [f"{m[n]['ece']:.3f}" for n in names],
            ["Brier score (lower=better)"] + [f"{m[n]['brier']:.3f}" for n in names],
            ["latency p50"] + [fmt_us(m[n]["p50"]) for n in names],
            ["latency p95"] + [fmt_us(m[n]["p95"]) for n in names],
            ["latency p99"] + [fmt_us(m[n]["p99"]) for n in names],
            ["latency mean / max"] + [f"{fmt_us(m[n]['mean'])} / {fmt_us(m[n]['max'])}" for n in names],
        ],
    )

    print(f"## Where {base} fails — accuracy by failure mode (`tests` tag)\n")
    tags = sorted({t for e in gold for t in e.get("tests", [])})
    rows = []
    for tag in tags:
        ids = [e["id"] for e in gold if tag in e.get("tests", [])]
        accs = {n: sum(m[n]["correct"][i] for i in ids) / len(ids) for n in names}
        rows.append(
            [tag, len(ids)]
            + [pct(accs[n]) for n in names]
            + [f"{100 * (accs[n] - accs[base]):+5.1f} pts" for n in names if n != base]
        )
    rows.sort(key=lambda r: r[2])
    table(["tag", "n"] + names + [f"{n} vs {base}" for n in names if n != base], rows)

    print("## Per request type — F1\n")
    table(
        ["type", "n"] + names,
        [
            [t, sum(e["request_type"] == t for e in gold)]
            + [f"{m[n]['f1'][t][2]:.2f}" for n in names]
            for t in TYPES
        ],
    )

    by_id = {e["id"]: e for e in gold}
    for n in names:
        if n == base:
            continue
        wins = [i for i in by_id if m[n]["correct"][i] and not m[base]["correct"][i]]
        losses = [i for i in by_id if m[base]["correct"][i] and not m[n]["correct"][i]]
        print(f"## {n} vs {base}: {len(wins)} fixed, {len(losses)} broken\n")
        print(f"Requests {base} misroutes that {n} gets right (first 12):\n")
        for i in wins[:12]:
            e = by_id[i]
            q = e["query"].replace("\n", " ")
            q = q if len(q) <= 90 else q[:87] + "..."
            print(
                f"- `{i}` [{', '.join(e.get('tests', []))}] {q!r}\n"
                f"  gold **{e['request_type']}** · {base} said {m[base]['pred'][i][0]}"
            )
        print(f"\nRequests {n} gets wrong that {base} got right:\n")
        for i in losses:
            e = by_id[i]
            q = e["query"].replace("\n", " ")
            q = q if len(q) <= 90 else q[:87] + "..."
            print(f"- `{i}` {q!r} gold {e['request_type']} · {n} said {m[n]['pred'][i][0]}")
        print()


if __name__ == "__main__":
    main()
