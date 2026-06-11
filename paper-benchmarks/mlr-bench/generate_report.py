#!/usr/bin/env python3
"""
MLR-Bench Report Generator — JSON → Human-readable Markdown report.
====================================================================

Reads MLR-Bench evaluation JSON and produces a Markdown report with scores,
justifications, and actionable improvement suggestions.

Usage:
  python generate_report.py results.json > report.md
  python generate_report.py results.json --output report.md
"""

import argparse
import json
import sys
from datetime import datetime
from pathlib import Path


def _bar(score: float, max_val: float = 10.0, width: int = 30) -> str:
    filled = round((score / max_val) * width)
    bar = "█" * filled + "░" * (width - filled)
    denom = f"{max_val:.0f}" if max_val == 10 else f"{max_val:.0f}%"
    return f"{bar}  {score:.0f}/{denom}"


def _score_label(s: float) -> str:
    if s >= 9:
        return "🟢 Excellent"
    if s >= 7:
        return "🟡 Good"
    if s >= 5:
        return "🟠 Adequate"
    if s >= 3:
        return "🔴 Weak"
    return "⛔ Poor"


def _score_label_5(s: float) -> str:
    if s >= 5:
        return "🟢 Excellent"
    if s >= 4:
        return "🟡 Good"
    if s >= 3:
        return "🟠 Adequate"
    return "🔴 Weak"


def render_overview(results: dict) -> str:
    rev = results.get("overall_review", {})
    lines = ["## 📊 MLR-Bench Score Overview\n"]
    dimensions = ["Clarity", "Novelty", "Soundness", "Significance"]
    lines.append("| Dimension | Score | Rating |")
    lines.append("|:----------|:-----:|:-------|")
    for d in dimensions:
        entry = rev.get(d, {})
        s = entry.get("score", 0)
        label = _score_label(float(s))
        lines.append(f"| **{d}** | {s}/10 | {label} |")

    overall = rev.get("Overall", {})
    os = overall.get("score", 0)
    conf = rev.get("Confidence", 0)
    lines.append(f"| **Overall** | {os}/10 | {_score_label(float(os))} |")
    lines.append(f"| **Confidence** | {conf}/5 | {_score_label_5(float(conf))} |\n")

    strengths = overall.get("strengths", [])
    weaknesses = overall.get("weaknesses", [])
    if strengths:
        lines.append("### 💪 Strengths\n")
        for s in strengths:
            lines.append(f"- {s}")
        lines.append("")
    if weaknesses:
        lines.append("### ⚠️ Weaknesses\n")
        for w in weaknesses:
            lines.append(f"- {w}")
        lines.append("")
    return "\n".join(lines)


def render_detail(results: dict) -> str:
    rev = results.get("overall_review", {})
    dimensions = ["Clarity", "Novelty", "Soundness", "Significance"]
    lines = ["## 📝 Detailed Evaluation (MLR-Bench Rubric)\n"]
    for d in dimensions:
        entry = rev.get(d, {})
        s = entry.get("score", 0)
        j = entry.get("justification", "")
        lines.append(f"### {d} — {s}/10 {_score_label(float(s))}\n")
        lines.append(f"`{_bar(float(s))}`\n")
        lines.append(f"> {j}\n")
    return "\n".join(lines)


def render_consistency(results: dict) -> str:
    sc = results.get("self_consistency", {})
    contradictions = sc.get("contradictions", [])
    if not contradictions:
        return "*(No consistency issues found)*\n"
    lines = ["## 🔍 Self-Consistency Issues\n"]
    icons = {"high": "🔴", "medium": "🟡", "low": "🟢"}
    for c in contradictions:
        sev = c.get("severity", "medium")
        icon = icons.get(sev, "⚪")
        lines.append(f"### {icon} [{sev.upper()}] {c.get('type', '').replace('_', ' ').title()}")
        lines.append(f"**Claim**: _{c.get('claim', '')}_")
        lines.append(f"**Location**: {c.get('location', '')}")
        lines.append(f"**Explanation**: {c.get('explanation', '')}\n")
    lines.append(f"**Score**: {sc.get('self_consistency_score', 'N/A')}/5\n")
    return "\n".join(lines)


def render_citations(results: dict) -> str:
    cs = results.get("citation_sanity", {})
    suspicious = cs.get("suspicious_references", [])
    total = cs.get("citation_count_total", 0)
    if not suspicious and total == 0:
        return "*(No citation data)*\n"
    lines = ["## 📚 Citation Analysis\n"]
    if suspicious:
        for s in suspicious:
            risk = s.get("risk", "medium")
            icon = {"high": "🔴", "medium": "🟡", "low": "🟢"}.get(risk, "⚪")
            lines.append(f"- {icon} **{s.get('reference', '')}**")
            lines.append(f"  - Suspicion: {s.get('suspicion', '')}")
    else:
        lines.append(f"No suspicious references (out of {total} citations).\n")
    lines.append(f"\n**Score**: {cs.get('citation_quality_score', 'N/A')}/5\n")
    return "\n".join(lines)


def render_figure_table(results: dict) -> str:
    ft = results.get("figure_table_coverage", {})
    if not ft:
        return "*(No figure/table data)*\n"
    lines = ["## 📊 Figures & Tables\n"]
    lines.append(f"| Metric | Value |")
    lines.append(f"|:-------|:-----:|")
    lines.append(f"| Figure mentions | {ft.get('figure_mentions', 0)} |")
    lines.append(f"| Table mentions | {ft.get('table_mentions', 0)} |")
    lines.append(f"| Figure coverage | {ft.get('figure_coverage', 0)}% |")
    lines.append(f"| Table coverage | {ft.get('table_coverage', 0)}% |")
    lines.append(f"\n{ft.get('overall_assessment', '')}\n")
    return "\n".join(lines)


def render_improvements(results: dict) -> str:
    lines = ["## 🎯 Improvement Suggestions\n"]
    suggestions = []

    # From overall review weaknesses
    overall = results.get("overall_review", {}).get("Overall", {})
    for w in overall.get("weaknesses", []):
        suggestions.append(("high", f"**Weakness from overall review**", w))

    # From low scoring dimensions
    for dim in ["Clarity", "Novelty", "Soundness", "Significance"]:
        entry = results.get("overall_review", {}).get(dim, {})
        s = entry.get("score", 10)
        j = entry.get("justification", "")
        if s <= 4:
            suggestions.append(("high", f"**{dim}** ({s}/10)", j))
        elif s <= 6:
            suggestions.append(("medium", f"**{dim}** ({s}/10)", j))

    # From consistency issues
    for c in results.get("self_consistency", {}).get("contradictions", []):
        if c.get("severity") == "high":
            suggestions.append(("high", f"**Consistency**: {c.get('type', '')}", c.get('explanation', '')))
        elif c.get("severity") == "medium":
            suggestions.append(("medium", f"**Consistency**: {c.get('type', '')}", c.get('explanation', '')))

    # From citation issues
    for s in results.get("citation_sanity", {}).get("suspicious_references", []):
        if s.get("risk") == "high":
            suggestions.append(("high", f"**Citation**: {s.get('reference', '')}", s.get('suspicion', '')))

    if not suggestions:
        lines.append("No specific improvement suggestions.\n")
        return "\n".join(lines)

    for label in ("high", "medium", "low"):
        items = [(p, t, d) for p, t, d in suggestions if p == label]
        if not items:
            continue
        icon = {"high": "🔴 HIGH", "medium": "🟡 MEDIUM", "low": "🟢 LOW"}[label]
        lines.append(f"### {icon} Priority\n")
        for _, title, detail in items:
            lines.append(f"- **{title}**")
            if detail:
                if len(detail) > 300:
                    detail = detail[:300] + "..."
                lines.append(f"  - {detail}")
        lines.append("")

    return "\n".join(lines)


def generate_report(results: dict, title: str = "MLR-Bench Evaluation Report") -> str:
    meta = results.get("_meta", {})
    lines = [
        f"# {title}\n",
        f"**Generated**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}  ",
        f"**Model**: {meta.get('model', 'N/A')}  ",
        f"**Input**: {meta.get('input', 'N/A')}\n",
        "---\n",
        render_overview(results),
        "---\n",
        render_detail(results),
        "---\n",
        render_consistency(results),
        "---\n",
        render_citations(results),
        "---\n",
        render_figure_table(results),
        "---\n",
        render_improvements(results),
        "---\n",
        "*Report generated by RCM MLR-Bench Evaluation Pipeline*",
    ]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="Generate MLR-Bench evaluation report")
    parser.add_argument("input", type=str, help="Path to evaluation JSON")
    parser.add_argument("--output", "-o", type=str, help="Output Markdown path")
    parser.add_argument("--title", default="MLR-Bench Evaluation Report", help="Report title")
    args = parser.parse_args()

    path = Path(args.input)
    if not path.exists():
        print(f"ERROR: {path} not found", file=sys.stderr)
        sys.exit(1)

    results = json.loads(path.read_text(encoding="utf-8"))
    report = generate_report(results, title=args.title)

    if args.output:
        out_path = Path(args.output)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(report, encoding="utf-8")
        print(f"Report saved: {out_path}", file=sys.stderr)
    else:
        print(report)


if __name__ == "__main__":
    main()
