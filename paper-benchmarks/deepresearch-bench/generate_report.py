"""
Report generator for DeepResearch-Bench evaluation results.
Converts JSON results to human-readable Markdown reports.
"""

import json
from pathlib import Path


def _bar(score: float, width: int = 30) -> str:
    """Render an ASCII progress bar for a 0-1 score."""
    filled = int(score * width)
    bar = "█" * filled + "░" * (width - filled)
    return f"{bar} {score:.4f}"


def _score_label(score: float) -> str:
    """Map 0-1 score to a qualitative label."""
    if score >= 0.90:
        return "Excellent"
    elif score >= 0.80:
        return "Very Good"
    elif score >= 0.70:
        return "Good"
    elif score >= 0.60:
        return "Adequate"
    elif score >= 0.50:
        return "Below Average"
    else:
        return "Poor"


def generate_report(result: dict) -> str:
    """Generate a Markdown report from a DeepResearch-Bench result dict."""
    lines = []

    # ── Header ──
    lines.append("# DeepResearch-Bench Evaluation Report\n")
    lines.append(f"- **Input file**: {result.get('input_file', 'N/A')}")
    lines.append(f"- **Timestamp**: {result.get('timestamp', 'N/A')}")
    lines.append(f"- **Overall Verification Score**: {result.get('overall_verification_score', 0):.2f}/5.0")
    if result.get("notes"):
        lines.append(f"- **Notes**: {result['notes']}")
    lines.append("")

    # ── RACE Results ──
    race = result.get("race")
    if race:
        lines.append("## RACE: Reference-based Adaptive Criteria-driven Evaluation\n")
        lines.append("### Summary Scores (0-1, higher is better)\n")
        lines.append("| Dimension | Score | Rating |")
        lines.append("|-----------|:-----:|:------:|")

        overall = race.get("overall_score", 0)
        lines.append(f"| **Overall** | **{overall:.4f}** | **{_score_label(overall)}** |")

        for dim, label in [
            ("comprehensiveness", "Comprehensiveness"),
            ("insight", "Insight/Depth"),
            ("instruction_following", "Instruction-Following"),
            ("readability", "Readability"),
        ]:
            score = race.get(dim, 0)
            lines.append(f"| {label} | {score:.4f} | {_score_label(score)} |")

        lines.append("")

        # Dimension weights
        if "dimension_weights" in race:
            lines.append("### Dimension Weights\n")
            lines.append("| Dimension | Weight |")
            lines.append("|-----------|:-----:|")
            for dim, weight in race["dimension_weights"].items():
                lines.append(f"| {dim} | {weight:.2f} |")
            lines.append("")

        # Raw scores
        if "raw_target_scores" in race:
            lines.append("### Raw Scores\n")
            lines.append("| Dimension | Target | Reference |")
            lines.append("|-----------|:------:|:---------:|")
            t_raw = race.get("raw_target_scores", {})
            r_raw = race.get("raw_reference_scores", {})
            for dim in ["comprehensiveness", "insight", "instruction_following", "readability"]:
                t = t_raw.get(dim, 0)
                r = r_raw.get(dim, 0)
                lines.append(f"| {dim} | {t:.2f} | {r:.2f} |")
            lines.append("")

        # Progress bars
        lines.append("### Visual Overview\n")
        for dim, label in [
            ("overall_score", "Overall"),
            ("comprehensiveness", "Comprehensiveness"),
            ("insight", "Insight"),
            ("instruction_following", "Instruction-Following"),
            ("readability", "Readability"),
        ]:
            score = race.get(dim, 0)
            lines.append(f"  **{label}**:  {_bar(score)}")
            lines.append("")

    # ── FACT Results ──
    fact = result.get("fact")
    if fact:
        lines.append("## FACT: Factual Abundance & Citation Trustworthiness\n")
        lines.append("### Summary Metrics\n")
        lines.append("| Metric | Value |")
        lines.append("|--------|:-----:|")
        lines.append(f"| Citation Accuracy | {fact.get('citation_accuracy', 0):.2%} |")
        lines.append(f"| Effective Citations | {fact.get('effective_citations', 0):.1f} |")
        lines.append(f"| Total Citations Found | {fact.get('total_citations', 0)} |")
        lines.append(f"| Unique URLs | {fact.get('unique_urls', 0)} |")
        lines.append(f"| Total Validated | {fact.get('total_validated', 0)} |")
        lines.append(f"| Supported | {fact.get('total_supported', 0)} |")
        lines.append(f"| Unsupported | {fact.get('total_unsupported', 0)} |")
        lines.append("")

        # Per-URL breakdown
        details = fact.get("details", [])
        if details:
            lines.append("### Per-URL Citation Validation\n")
            lines.append("| URL | Facts | Supported | Unsupported | Unknown |")
            lines.append("|-----|:-----:|:---------:|:-----------:|:------:|")
            for d in details:
                url = d.get("url", "N/A")
                short_url = url[:60] + "..." if len(url) > 60 else url
                n_facts = len(d.get("validated", [])) or d.get("unknown", 0)
                lines.append(
                    f"| {short_url} | {n_facts} | "
                    f"{d.get('supported', 0)} | {d.get('unsupported', 0)} | "
                    f"{d.get('unknown', 0)} |"
                )
            lines.append("")

    # ── Footer ──
    lines.append("---")
    lines.append(f"*Report generated by DeepResearch-Bench*")

    return "\n".join(lines)


def main():
    import sys
    if len(sys.argv) < 2:
        print("Usage: python generate_report.py <result.json> [output.md]")
        sys.exit(1)

    input_path = Path(sys.argv[1])
    result = json.loads(input_path.read_text(encoding="utf-8"))
    report = generate_report(result)

    if len(sys.argv) >= 3:
        output_path = Path(sys.argv[2])
    else:
        output_path = input_path.with_suffix(".md")

    output_path.write_text(report, encoding="utf-8")
    print(f"[OK] Report saved -> {output_path}")


if __name__ == "__main__":
    main()
