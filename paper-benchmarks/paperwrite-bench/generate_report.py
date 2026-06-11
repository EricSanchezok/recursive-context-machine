#!/usr/bin/env python3
"""
Generate a human-readable improvement report from evaluation JSON output.
=======================================================================

Reads the JSON result from evaluate_simple.py and produces a Markdown report
with scores, LLM reasoning, and actionable improvement suggestions.

Usage:
  python generate_report.py results.json > report.md
  python generate_report.py results.json --output report.md
"""

import argparse
import json
import sys
from datetime import datetime
from pathlib import Path


# ═══════════════════════════════════════════════════════════════════════
#  Helpers
# ═══════════════════════════════════════════════════════════════════════


def _bar(score: float, max_val: float = 5.0, width: int = 30) -> str:
    """Render a horizontal bar like: ████████░░░░░░░░░░░░  2.9/5"""
    filled = round((score / max_val) * width)
    bar = "█" * filled + "░" * (width - filled)
    denom = max_val if max_val == 5 else f"{max_val}%"
    return f"{bar}  {score}/{denom}"


def _severity_icon(sev: str) -> str:
    return {"high": "🔴", "medium": "🟡", "low": "🟢"}.get(sev, "⚪")


def _score_color(s: float) -> str:
    if s >= 4.5:
        return "🟢 Excellent"
    if s >= 3.5:
        return "🟡 Good"
    if s >= 2.5:
        return "🟠 Needs Work"
    return "🔴 Weak"


def _input_path_from_json(results: dict) -> str:
    """Extract input file name from various possible locations."""
    return results.get("input_file", "unknown")


# ═══════════════════════════════════════════════════════════════════════
#  Section renderers
# ═══════════════════════════════════════════════════════════════════════


def render_score_overlay(results: dict) -> str:
    """Render the top-level score card."""
    lines = ["## 📊 Score Overview\n"]

    rubric_avg = results.get("rubric_summary", {}).get("average_score", 0)
    rubric_pts = results.get("rubric_summary", {}).get("total_points", 0)
    sc_score = results.get("self_consistency", {}).get("self_consistency_score", 0)
    cit_score = results.get("citation_sanity", {}).get("citation_quality_score", 0)
    ft = results.get("figure_table_coverage", {})
    fig_cov = ft.get("figure_coverage", 0)
    tab_cov = ft.get("table_coverage", 0)

    lines.append(f"| Dimension | Score | Rating |")
    lines.append(f"|:----------|:-----:|:-------|")

    rubric_icon = _score_color(rubric_avg)
    sc_icon = _score_color(float(sc_score))
    cit_icon = _score_color(float(cit_score))
    fig_text = f"Figures {fig_cov}% / Tables {tab_cov}%"
    fig_bar_f = _bar(float(fig_cov), 100.0, 20) if fig_cov else "░" * 20 + "  0/100%"
    fig_bar_t = _bar(float(tab_cov), 100.0, 20) if tab_cov else "░" * 20 + "  0/100%"

    lines.append(f"| **Rubric** (per-section 1-5) | {rubric_avg:.2f}/5 | {rubric_icon} |")
    lines.append(f"| **Self-Consistency** (1-5) | {sc_score}/5 | {sc_icon} |")
    lines.append(f"| **Citation Quality** (1-5) | {cit_score}/5 | {cit_icon} |")
    lines.append(f"| **Figure Coverage** | {fig_cov}% | `{fig_bar_f}` |")
    lines.append(f"| **Table Coverage** | {tab_cov}% | `{fig_bar_t}` |")

    lines.append(f"\nTotal rubric criteria evaluated: **{rubric_pts}**")
    lines.append("")
    return "\n".join(lines)


def render_rubric_detail(results: dict) -> str:
    """Render per-section rubric breakdown with LLM reasoning."""
    rubric = results.get("rubric", [])
    if not rubric:
        return "*(No rubric data)*\n"

    lines = ["## 📝 Rubric — Per-Section Breakdown\n"]
    lines.append(
        "Each criterion scored 1-5. **Reasoning** shows the LLM judge's explanation, "
        "which directly tells you what to improve.\n"
    )

    for sec in rubric:
        name = sec["section_name"]
        avg = sec["average_score"]
        total = sec["total_count"]
        icon = _score_color(avg)
        lines.append(f"### {name} — {avg:.2f}/5 ({total} criteria) {icon}\n")
        lines.append(f"`{_bar(avg)}`\n")

        for r in sec.get("results", []):
            elem = r["element"]
            score = r["score"]
            reasoning = r.get("reasoning", "")
            colored_score = f"**{score}**"
            lines.append(f"- **{elem}**: **{score}/5**")
            if reasoning:
                lines.append(f"  - *\"{reasoning}\"*")
        lines.append("")

    return "\n".join(lines)


def render_consistency_detail(results: dict) -> str:
    """Render self-consistency findings."""
    sc = results.get("self_consistency", {})
    contradictions = sc.get("contradictions", [])
    if not contradictions:
        return "*(No consistency issues found — or check not run)*\n"

    lines = ["## 🔍 Self-Consistency Issues\n"]
    lines.append(
        f"LLM found **{len(contradictions)}** issues. "
        "Each identifies a concrete weakness in the paper's internal logic.\n"
    )

    for c in contradictions:
        sev = c.get("severity", "medium")
        icon = _severity_icon(sev)
        ctype = c.get("type", "unknown").replace("_", " ").title()
        lines.append(f"### {icon} [{sev.upper()}] {ctype}")
        lines.append(f"**Claim**: _{c.get('claim', '')}_")
        lines.append(f"**Location**: {c.get('location', '')}")
        lines.append(f"**Explanation**: {c.get('explanation', '')}")
        lines.append("")

    lines.append(f"**Overall Assessment**: {sc.get('overall_assessment', '')}")
    lines.append("")
    return "\n".join(lines)


def render_citation_detail(results: dict) -> str:
    """Render citation analysis."""
    cs = results.get("citation_sanity", {})
    suspicious = cs.get("suspicious_references", [])
    total_cites = cs.get("citation_count_total", 0)

    if not suspicious and total_cites == 0:
        return "*(No citation data — or check not run)*\n"

    lines = ["## 📚 Citation Analysis\n"]

    if suspicious:
        lines.append(
            f"LLM flagged **{len(suspicious)}** potentially hallucinated references "
            f"out of **{total_cites}** total citations.\n"
        )
        for s in suspicious:
            risk = s.get("risk", "medium")
            icon = _severity_icon(risk)
            lines.append(f"| {icon} | **{s.get('reference', '')}** |")
            lines.append(f"| | Suspicion: {s.get('suspicion', '')} |")
    else:
        lines.append(f"No suspicious references found (out of {total_cites} citations).\n")

    lines.append(f"\n**Overall Assessment**: {cs.get('overall_assessment', '')}")
    lines.append("")
    return "\n".join(lines)


def render_figure_table_detail(results: dict) -> str:
    """Render figure/table coverage."""
    ft = results.get("figure_table_coverage", {})
    if not ft:
        return "*(No figure/table data — or check not run)*\n"

    lines = ["## 📊 Figures & Tables Coverage\n"]

    lines.append(f"| Metric | Value |")
    lines.append(f"|:-------|:-----:|")
    lines.append(f"| Figure mentions in text | {ft.get('figure_mentions', 0)} |")
    lines.append(f"| Table mentions in text | {ft.get('table_mentions', 0)} |")
    lines.append(f"| Unique figures referenced | {len(ft.get('unique_figures_referenced', []))} |")
    lines.append(f"| Unique tables referenced | {len(ft.get('unique_tables_referenced', []))} |")
    lines.append(f"| Figure captions found | {ft.get('figure_captions_found', 0)} |")
    lines.append(f"| Table captions found | {ft.get('table_captions_found', 0)} |")

    fig_gaps = ft.get("figures_missing_caption", [])
    tab_gaps = ft.get("tables_missing_caption", [])
    if fig_gaps:
        lines.append(f"\nFigures missing captions: {', '.join(fig_gaps)}")
    if tab_gaps:
        lines.append(f"Tables missing captions: {', '.join(tab_gaps)}")

    lines.append(f"\n{ft.get('overall_assessment', '')}")
    lines.append("")
    return "\n".join(lines)


def render_improvement_plan(results: dict) -> str:
    """Generate actionable improvement suggestions from the evaluation data."""
    lines = ["## 🎯 Actionable Improvement Suggestions\n"]
    lines.append(
        "These suggestions are derived directly from the LLM judge's reasoning. "
        "Address them in priority order to improve your paper generation model.\n"
    )

    suggestions: list[tuple[str, str, str]] = []

    # ── From rubric (low-scoring criteria) ──
    for sec in results.get("rubric", []):
        for r in sec.get("results", []):
            if r.get("score", 5) <= 2:
                suggestions.append((
                    "high",
                    f"**{sec['section_name']}** → *{r['element']}* (score {r['score']}/5)",
                    r.get("reasoning", "No detailed feedback available."),
                ))

    # ── From consistency (high-severity contradictions) ──
    for c in results.get("self_consistency", {}).get("contradictions", []):
        if c.get("severity") == "high":
            suggestions.append((
                "high",
                f"**Consistency**: {c.get('type', '').replace('_', ' ').title()}",
                f"{c.get('claim', '')} — {c.get('explanation', '')}",
            ))

    # ── From citations (high-risk references) ──
    for s in results.get("citation_sanity", {}).get("suspicious_references", []):
        if s.get("risk") == "high":
            suggestions.append((
                "high",
                f"**Citation**: Suspicious reference — {s.get('reference', '')}",
                s.get("suspicion", ""),
            ))

    # ── Medium priority from rubric ──
    for sec in results.get("rubric", []):
        for r in sec.get("results", []):
            if r.get("score", 5) in (3,):
                suggestions.append((
                    "medium",
                    f"**{sec['section_name']}** → *{r['element']}* (score {r['score']}/5)",
                    r.get("reasoning", ""),
                ))

    # ── Medium from consistency ──
    for c in results.get("self_consistency", {}).get("contradictions", []):
        if c.get("severity") == "medium":
            suggestions.append((
                "medium",
                f"**Consistency**: {c.get('type', '').replace('_', ' ').title()}",
                f"{c.get('claim', '')} — {c.get('explanation', '')}",
            ))

    # ── Low from consistency ──
    for c in results.get("self_consistency", {}).get("contradictions", []):
        if c.get("severity") == "low":
            suggestions.append((
                "low",
                f"**Consistency**: {c.get('type', '').replace('_', ' ').title()}",
                f"{c.get('claim', '')} — {c.get('explanation', '')}",
            ))

    if not suggestions:
        lines.append("No specific improvement suggestions found.\n")
        return "\n".join(lines)

    # Group by priority
    for priority_label in ("high", "medium", "low"):
        items = [(p, t, d) for p, t, d in suggestions if p == priority_label]
        if not items:
            continue
        icon = {"high": "🔴 HIGH", "medium": "🟡 MEDIUM", "low": "🟢 LOW"}[priority_label]
        lines.append(f"### {icon} Priority\n")
        for _, title, detail in items:
            lines.append(f"- **{title}**")
            if detail:
                # Truncate very long details
                if len(detail) > 300:
                    detail = detail[:300] + "..."
                lines.append(f"  - {detail}")
        lines.append("")

    return "\n".join(lines)


# ═══════════════════════════════════════════════════════════════════════
#  Main
# ═══════════════════════════════════════════════════════════════════════


def generate_report(results: dict, title: str = "Paper Evaluation Report") -> str:
    """Generate the full Markdown report from evaluation results."""
    lines = [
        f"# {title}",
        "",
        f"**Generated**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}  ",
        f"**Model**: {results.get('_meta', {}).get('model', 'N/A')}  ",
        f"**Input**: {results.get('_meta', {}).get('input', _input_path_from_json(results))}",
        "",
        "---",
        "",
        render_score_overlay(results),
        "---",
        "",
        render_rubric_detail(results),
        "---",
        "",
        render_consistency_detail(results),
        "---",
        "",
        render_citation_detail(results),
        "---",
        "",
        render_figure_table_detail(results),
        "---",
        "",
        render_improvement_plan(results),
        "",
        "---",
        "",
        "*Report generated by RCM Paper Evaluation Pipeline*",
        "",
    ]

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Generate human-readable improvement report from evaluation JSON."
    )
    parser.add_argument("input", type=str, help="Path to evaluation results JSON")
    parser.add_argument("--output", "-o", type=str, help="Output Markdown file path")
    parser.add_argument("--title", type=str, default="Paper Evaluation Report",
                        help="Report title")
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
        print(f"Report saved to: {out_path}", file=sys.stderr)
    else:
        print(report)


if __name__ == "__main__":
    main()
