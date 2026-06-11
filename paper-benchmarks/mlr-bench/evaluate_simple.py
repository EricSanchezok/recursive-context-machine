#!/usr/bin/env python3
"""
MLR-Bench Simple Evaluator — Self-contained overall review.
=============================================================

Faithfully replicates the MLR-Bench OVERALL_RUBRIC prompt to evaluate
a paper on Clarity, Novelty, Soundness, and Significance (1-10).
No ground truth required.

Usage:
  python evaluate_simple.py --pdf paper.pdf --output results.json --report
  python evaluate_simple.py --markdown paper.md --output results.json
  python evaluate_simple.py --latex paper.tex --output results.json
"""

import argparse
import json
import os
import re
import sys
import time
from pathlib import Path


# ═══════════════════════════════════════════════════════════════════════
#  MLR-Bench OVERALL_RUBRIC (faithful replica)
# ═══════════════════════════════════════════════════════════════════════

OVERALL_RUBRIC = """
You are an expert machine learning researcher!
You will be given a research paper which is based on a task description.
You might also be given the code of the paper to check the reproducibility of the paper. 
You task is to review the paper in terms of 4 key aspects - Clarity, Novelty, Soundness and Significance.
Please provide a score from 1 to 10 for each aspect and an overall assessment, where 1 is the lowest and 10 is the highest. Lastly, provide a confidence score from 1 to 5 for the overall assessment, where 1 is the lowest and 10 is the highest.

## Evaluation Rubric

1. Clarity (1-10)
    - Is the paper well-written and easy to understand?
    - Are the ideas and contributions clearly articulated?
    - Is the structure of the paper logical and coherent?

    9-10 - The paper is exceptionally well-written, with clear and concise language. The ideas are presented in a logical and coherent manner, making it easy to follow the author's arguments.
    7-8 - The paper is well-written, but there are some areas that could be improved for clarity. The ideas are mostly clear, but there may be some minor issues with the structure or language.
    5-6 - The paper is somewhat difficult to read, with several areas that are unclear or poorly articulated. The structure may be confusing, making it hard to follow the author's arguments.
    3-4 - The paper is poorly written, with many unclear or confusing sections. The ideas are not well-articulated, and the structure is disorganized.
    1-2 - The paper is extremely difficult to read, with numerous unclear or confusing sections. The ideas are poorly articulated, and the structure is completely disorganized.

2. Novelty (1-10)
    - Does the paper present new and original ideas and findings?
    - Are the experimental results and contributions original and novel?
    - Is the work a significant advance over existing research?

    9-10 - The paper presents groundbreaking ideas and findings that are highly original and significant. The contributions are a major advance over existing research and are likely to have a lasting impact on the field.
    7-8 - The paper presents some new and original ideas, and the contributions are significant. The work is a notable advance over existing research, but it may not be as groundbreaking as top-tier papers.
    5-6 - The paper presents some new ideas and findings, but they are not particularly original or significant. The contributions are somewhat incremental and do not represent a major advance over existing research.
    3-4 - The paper presents few new ideas or findings, and those that are presented are not original or significant. The contributions are minimal and do not advance the field.
    1-2 - The paper presents no new ideas, and the contributions are completely unoriginal. The work does not advance the field in any meaningful way.

3. Soundness (1-10)
    - Are the methods and techniques used in the paper sound and appropriate?
    - Are the results and conclusions supported by the data?
    - Are there any major flaws or weaknesses in the experimental design, results or analysis?
    - Are the experimental results reliable and consistent to the code of the paper? Are the experimental results real or fake?
    - Are the visualization and analysis figures based on real experimental results or based on fake data? 

    9-10 - The methods and techniques used in the paper are sound and appropriate. The results are well-supported by the data, and there are no major flaws or weaknesses in the experimental design, results or analysis. The experimental results are fully reliable and consistent with the code of the paper.
    7-8 - The methods and techniques used in the paper are mostly sound, but there may be some minor issues. The results are generally well-supported by the data, but there may be some areas that could be improved. The experimental design, results or analysis may have some minor flaws. The experimental results are mostly reliable.
    5-6 - The methods and techniques used in the paper are somewhat questionable, with several areas that could be improved. The results are not well-supported by the data, and there may be some significant flaws in the experimental design, results or analysis. Some experimental results are not reliable.
    3-4 - The methods and techniques used in the paper are flawed or inappropriate. The results are not well-supported by the data, and there are major flaws in the experimental design, results or analysis. Most of experimental results are not reliable.
    1-2 - The methods and techniques used in the paper are completely unsound. The results are not supported by the data, and there are numerous major flaws in the experimental design, results or analysis. The conclusions drawn from the paper are completely invalid. All experimental results are not reliable.

4. Significance (1-10)
    - Does the paper address an important problem or question?
    - Are the contributions significant to the field?
    - Are the experimental results reproducible and reliable? Do they have a significant impact?
    - Will the work have a lasting impact on the field?

    9-10 - The paper addresses a highly important problem or question, and the results and contributions are significant to the field. The work is likely to have a lasting impact on the field.
    7-8 - The paper addresses an important problem or question, and the results and contributions are significant. The work may have a lasting impact on the field, but it may not be as groundbreaking as top-tier papers.
    5-6 - The paper addresses a somewhat important problem or question, but the results and contributions are not particularly significant. The work may have some impact on the field, but it is unlikely to be lasting.
    3-4 - The paper addresses a minor problem or question, and the results and contributions are minimal. The work is unlikely to have any significant impact on the field.
    1-2 - The paper addresses an unimportant problem or question, and the results and contributions are completely insignificant. The work will have no impact on the field.

5. Overall Assessment (1-10)
    - Based on the above criteria, how would you rate the overall quality of the paper? Note that any single weakness can be critical to lower the overall assessment.
    - Is the paper suitable for publication in a top-tier conference or journal?
    - Would you recommend this paper to your colleagues?

    10 - The paper is of exceptional quality and is highly suitable for publication in a top-tier conference or journal. I would strongly recommend this paper.
    8-9 - The paper is of high quality and is suitable for publication in a top-tier conference or journal. I would recommend this paper.
    6-7 - The paper is of good quality and is suitable for publication in a reputable conference or journal. I would recommend this paper with some reservations.
    4-5 - The paper is of acceptable quality but may not be suitable for publication in a top-tier conference or journal. I would recommend this paper with significant reservations.
    2-3 - The paper is of poor quality and is not suitable for publication in a top-tier conference or journal. I would not recommend this paper.
    1 - The paper is of extremely poor quality and is not suitable for publication in any conference or journal. I would strongly advise against recommending this paper.

6. Confidence Score (1-5)
    - How confident are you in your overall assessment of the paper?

    5 - Extremely confident in the overall assessment.
    4 - Very confident in the overall assessment.
    3 - Moderately confident in the overall assessment.
    2 - Slightly confident in the overall assessment.
    1 - Not confident in the overall assessment.

Please provide a detailed review of the paper, including your scores for each aspect and an overall assessment. Be sure to justify your scores with specific examples from the paper.
Please do not include any personal opinions or biases in your review. Your review should be objective and based solely on the content of the paper. Please provide a confidence score from 1 to 5 for the overall assessment.
Do not hesitate to assign lower scores if the paper does not fully meet the criteria. Avoid giving high scores by default.

## Output Format

Please provide your review in the following format:

```json
{
    "Clarity": {
        "score": <1-10>,
        "justification": "<Your justification here>"
    },
    "Novelty": {
        "score": <1-10>,
        "justification": "<Your justification here>"
    },
    "Soundness": {
        "score": <1-10>,
        "justification": "<Your justification here>"
    },
    "Significance": {
        "score": <1-10>,
        "justification": "<Your justification here>"
    },
    "Overall": {
        "score": <1-10>,
        "strengths": ["<strength 1>", "<strength 2>"],
        "weaknesses": ["<weakness 1>", "<weakness 2>"]
    },
    "Confidence": <1-5>
}
```

Note that any single weakness can be critical to lower the overall assessment.
Please provide detailed justifications for each score, including specific examples from the paper. 
IMPORTANT: Please ensure that your output is a complete and valid JSON object and includes all the fields above. Do not output only a single item or partial content; you must output the entire JSON object.
"""


# ═══════════════════════════════════════════════════════════════════════
#  LLM Client
# ═══════════════════════════════════════════════════════════════════════


def call_llm(prompt: str, model: str, system: str = "") -> str:
    """Simple LLM call via OpenAI-compatible API."""
    from openai import OpenAI

    api_key = os.environ.get("OPENAI_API_KEY")
    base_url = os.environ.get("OPENAI_BASE_URL")
    if not api_key:
        print("ERROR: Set OPENAI_API_KEY environment variable")
        print("  $env:OPENAI_API_KEY = 'sk-...'")
        print("  $env:OPENAI_BASE_URL = 'https://api.deepseek.com'  # if using DeepSeek")
        sys.exit(1)

    client = OpenAI(api_key=api_key, base_url=base_url)

    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    response = client.chat.completions.create(
        model=model,
        messages=messages,
        temperature=0.3,
        max_tokens=8192,
    )

    return response.choices[0].message.content


def extract_json(text: str) -> dict:
    """Extract a JSON object from LLM response (may be wrapped in ```json ... ```)."""
    # Try finding ```json ... ``` blocks
    match = re.search(r"```(?:json)?\s*\n?({.*?})\s*\n?```", text, re.DOTALL)
    if match:
        return json.loads(match.group(1))
    # Try bare JSON
    match = re.search(r"\{.*\}", text, re.DOTALL)
    if match:
        return json.loads(match.group(0))
    raise ValueError(f"Could not extract JSON from response: {text[:200]}")


# ═══════════════════════════════════════════════════════════════════════
#  Input reading
# ═══════════════════════════════════════════════════════════════════════


def read_input(file_path: Path) -> str:
    """Read a PDF, Markdown, or LaTeX file and return plain text."""
    if file_path.suffix == ".pdf":
        return _pdf_to_markdown(file_path)
    content = file_path.read_text(encoding="utf-8")
    if file_path.suffix == ".md":
        return content
    # LaTeX: strip commands to get plain text
    content = re.sub(r"\\(?:section|subsection|textbf|textit|emph)\{[^}]*\}", "", content)
    content = re.sub(r"\\cite\{[^}]*\}", "", content)
    content = re.sub(r"\\[a-zA-Z]+(\[.*?\])?\{.*?\}", "", content)
    return content


def _pdf_to_markdown(path: Path) -> str:
    """Convert PDF to Markdown via pymupdf4llm."""
    try:
        import pymupdf4llm
        return pymupdf4llm.to_markdown(str(path))
    except ImportError:
        print("ERROR: pymupdf4llm not installed. Run: pip install pymupdf4llm")
        print("  Or convert PDF manually and use --markdown.")
        sys.exit(1)


# ═══════════════════════════════════════════════════════════════════════
#  Self-consistency, citation, figure/table checks
# ═══════════════════════════════════════════════════════════════════════


SELF_CONSISTENCY_PROMPT = """
You are an expert peer reviewer. Analyze the following paper for internal consistency issues.

Look for these specific types of problems:
1. **Internal contradictions**: Two parts of the paper that say conflicting things
2. **Unsupported claims**: Claims of SOTA performance or significant results without supporting evidence
3. **Method-vs-experiment gap**: The method described in the methodology differs from what was actually tested
4. **Overblown claims**: Conclusions that go beyond what the evidence supports

For each issue found, specify:
- The type
- The severity (high/medium/low)
- The exact claim or location
- An explanation

If no issues are found, return an empty "contradictions" list.

Output format:
```json
{
    "contradictions": [
        {
            "type": "internal_contradiction",
            "severity": "high",
            "claim": "The paper states both X and Y",
            "location": "Section 3 vs Section 5",
            "explanation": "These are contradictory because..."
        }
    ],
    "overall_assessment": "Summary of consistency evaluation"
}
```
"""

CITATION_SANITY_PROMPT = """
You are an expert at detecting hallucinated or suspicious citations in academic papers.
Analyze the following paper and identify any citations that may be hallucinated.

For each suspicious reference, provide:
- The reference text
- Why it's suspicious
- Risk level (high/medium/low)

Output format:
```json
{
    "citation_count_total": 0,
    "suspicious_references": [
        {
            "reference": "[1] Smith et al., 2023, ...",
            "suspicion": "This paper does not exist or the author/venue/year is fabricated",
            "risk": "high"
        }
    ],
    "overall_assessment": "Summary of citation quality"
}
```
"""

FIGURE_TABLE_PROMPT = """
Analyze the figure and table usage in the following paper.

Count:
- How many figures are mentioned in the text (e.g., "Figure 1", "Fig. 2")
- How many tables are mentioned in the text
- How many figure captions are present
- How many table captions are present

Output format:
```json
{
    "figure_mentions": 0,
    "table_mentions": 0,
    "unique_figures_referenced": [],
    "unique_tables_referenced": [],
    "figure_captions_found": 0,
    "table_captions_found": 0,
    "figures_missing_caption": [],
    "tables_missing_caption": [],
    "overall_assessment": "Summary of figure/table coverage"
}
```
"""


# ═══════════════════════════════════════════════════════════════════════
#  Core evaluation
# ═══════════════════════════════════════════════════════════════════════


def run_overall_review(content: str, model: str) -> dict:
    """Run the MLR-Bench overall review with exact rubric."""
    prompt = OVERALL_RUBRIC + f"\n\n## Paper to Be Reviewed\n\n```\n{content}\n```\n"
    print("  Calling LLM for overall review (Clarity/Novelty/Soundness/Significance)...")
    response = call_llm(prompt, model)
    return extract_json(response)


def run_self_consistency(content: str, model: str) -> dict:
    """Internal self-consistency check."""
    prompt = SELF_CONSISTENCY_PROMPT + f"\n\n## Paper\n\n```\n{content[:8000]}\n```\n"
    print("  Calling LLM for self-consistency check...")
    response = call_llm(prompt, model)
    data = extract_json(response)
    # Derive a score from severity count
    contradictions = data.get("contradictions", [])
    total = len(contradictions)
    if total == 0:
        data["self_consistency_score"] = 5
    elif total <= 2:
        data["self_consistency_score"] = 4
    elif total <= 4:
        data["self_consistency_score"] = 3
    elif total <= 6:
        data["self_consistency_score"] = 2
    else:
        data["self_consistency_score"] = 1
    return data


def run_citation_check(content: str, model: str) -> dict:
    """Citation sanity check."""
    prompt = CITATION_SANITY_PROMPT + f"\n\n## Paper\n\n```\n{content[:10000]}\n```\n"
    print("  Calling LLM for citation sanity check...")
    response = call_llm(prompt, model)
    data = extract_json(response)
    suspicious = data.get("suspicious_references", [])
    total = data.get("citation_count_total", 0)
    if total == 0:
        data["citation_quality_score"] = 5
    else:
        ratio = len(suspicious) / max(total, 1)
        if ratio == 0:
            data["citation_quality_score"] = 5
        elif ratio <= 0.1:
            data["citation_quality_score"] = 4
        elif ratio <= 0.2:
            data["citation_quality_score"] = 3
        elif ratio <= 0.3:
            data["citation_quality_score"] = 2
        else:
            data["citation_quality_score"] = 1
    return data


def run_figure_table_check(content: str, model: str) -> dict:
    """Figure/table coverage check."""
    prompt = FIGURE_TABLE_PROMPT + f"\n\n## Paper\n\n```\n{content[:8000]}\n```\n"
    print("  Calling LLM for figure/table coverage check...")
    response = call_llm(prompt, model)
    data = extract_json(response)
    fig_mentions = data.get("figure_mentions", 0)
    tab_mentions = data.get("table_mentions", 0)
    fig_captions = data.get("figure_captions_found", 0)
    tab_captions = data.get("table_captions_found", 0)
    data["figure_coverage"] = round(fig_captions / max(fig_mentions, 1) * 100)
    data["table_coverage"] = round(tab_captions / max(tab_mentions, 1) * 100)
    return data


# ═══════════════════════════════════════════════════════════════════════
#  Main
# ═══════════════════════════════════════════════════════════════════════


def main():
    parser = argparse.ArgumentParser(
        description="MLR-Bench Simple Evaluator — overall review from PDF/Markdown/LaTeX"
    )
    inp = parser.add_mutually_exclusive_group(required=True)
    inp.add_argument("--pdf", type=str, help="PDF file (auto-converted)")
    inp.add_argument("--latex", type=str, help="LaTeX file")
    inp.add_argument("--markdown", type=str, help="Markdown file")

    parser.add_argument("--model", default="gpt-4o", help="LLM model (default: gpt-4o)")
    parser.add_argument("--output", "-o", type=str, default=None, help="Output JSON path")
    parser.add_argument("--report", action="store_true", help="Generate Markdown report")
    parser.add_argument("--self-consistency", action="store_true",
                        help="Run self-consistency check")
    parser.add_argument("--citations", action="store_true",
                        help="Run citation sanity check")
    parser.add_argument("--figure-table", action="store_true",
                        help="Run figure/table coverage check")
    parser.add_argument("--full-sanity", action="store_true",
                        help="All extra checks (consistency + citations + figure/table)")
    parser.add_argument("--all", action="store_true",
                        help="Overall review + all extra checks")

    args = parser.parse_args()

    # ── Read input ──
    input_path = Path(args.pdf or args.latex or args.markdown)
    if not input_path.exists():
        print(f"ERROR: {input_path} not found")
        sys.exit(1)
    content = read_input(input_path)
    print(f"Read {len(content)} chars from {input_path.name}")

    # ── Determine what to run ──
    do_review = not args.full_sanity  # always do review unless only-sanity
    do_all = args.all or (not args.self_consistency and not args.citations
                          and not args.figure_table and not args.full_sanity)
    do_consistency = args.self_consistency or args.full_sanity or args.all
    do_citations = args.citations or args.full_sanity or args.all
    do_figure_table = args.figure_table or args.full_sanity or args.all

    results: dict = {"_meta": {"model": args.model, "input": input_path.name}}

    # ── Overall review ──
    if do_review:
        review = run_overall_review(content, args.model)
        results["overall_review"] = review

    # ── Extra checks ──
    if do_consistency:
        results["self_consistency"] = run_self_consistency(content, args.model)
    if do_citations:
        results["citation_sanity"] = run_citation_check(content, args.model)
    if do_figure_table:
        results["figure_table_coverage"] = run_figure_table_check(content, args.model)

    # ── Save ──
    output_path = None
    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(json.dumps(results, indent=2, ensure_ascii=False),
                               encoding="utf-8")
        print(f"\n  Results: {output_path}")
    else:
        print(json.dumps(results, indent=2, ensure_ascii=False))

    # ── Report ──
    if args.report and output_path:
        results["_meta"] = {"model": args.model, "input": input_path.name}
        report_path = output_path.with_suffix(".md")
        try:
            from generate_report import generate_report
            report_path.write_text(generate_report(results), encoding="utf-8")
            print(f"  Report:  {report_path}")
        except ImportError:
            print("  (generate_report.py not found; skipping report)")


if __name__ == "__main__":
    main()
