#!/usr/bin/env python3
"""
Standalone paper evaluator — no ground truth required.
=======================================================

Evaluates a paper (LaTeX or Markdown) using:
  1) Rubric (1-5 per section)         — matches PaperWrite-Bench core
  2) Self-consistency hallucination   — internal claim consistency
  3) Citation sanity                  — format checks & hallucination risk
  4) Figure/Table coverage            — mention vs. available captions

Usage:
  python evaluate_simple.py --markdown paper.md --auto-eval-points
  python evaluate_simple.py --markdown paper.md --full-sanity
  python evaluate_simple.py --latex paper.tex --auto-eval-points --self-consistency
  python evaluate_simple.py --markdown paper.md --eval-points points.json --all
"""

import argparse
import json
import os
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

# ═══════════════════════════════════════════════════════════════════════
#  Internal types
# ═══════════════════════════════════════════════════════════════════════

VALID_CATEGORIES = [
    "Abstract", "Introduction", "Related Works",
    "Method", "Experiment", "Conclusion",
]

SECTION_CATEGORY_RULES: list[tuple[re.Pattern, str]] = [
    (re.compile(r"abstract"), "Abstract"),
    (re.compile(r"introduction"), "Introduction"),
    (re.compile(r"background|preliminary"), "Method"),
    (re.compile(r"related (works?|work)"), "Related Works"),
    (re.compile(
        r"method|approach|proposed|model|algorithm|framework|architecture"
    ), "Method"),
    (re.compile(
        r"experiment|result|evaluation|performance|benchmark|dataset|metric"
    ), "Experiment"),
    (re.compile(r"conclusion|discussion|future work|limitation"), "Conclusion"),
]


@dataclass
class _Section:
    name: str
    content: str


@dataclass
class _RubricResult:
    section_name: str
    results: list = field(default_factory=list)
    average_score: float = 0.0
    total_count: int = 0

    def model_dump(self):
        return {
            "section_name": self.section_name,
            "results": self.results,
            "average_score": self.average_score,
            "total_count": self.total_count,
        }


# ═══════════════════════════════════════════════════════════════════════
#  Section extraction
# ═══════════════════════════════════════════════════════════════════════


def extract_sections_from_text(text: str) -> list[_Section]:
    """Extract top-level \\section{...} blocks from LaTeX-style text."""
    pattern = re.compile(r"\\(?:section|Section|SECTION)\{([^}]*)\}", re.DOTALL)
    sections: list[_Section] = []
    pos = 0
    while pos < len(text):
        m = pattern.search(text, pos)
        if not m:
            break
        name = m.group(1).strip()
        sec_start = m.end()
        nm = pattern.search(text, sec_start)
        content_end = nm.start() if nm else len(text)
        sections.append(_Section(name=name, content=text[sec_start:content_end].strip()))
        pos = nm.start() if nm else len(text)
    return sections


def classify_section_name_by_rule(section_name: str) -> str | None:
    for pat, cat in SECTION_CATEGORY_RULES:
        if pat.search(section_name):
            return cat
    return None


def _merge_sections_by_category(sections: list[_Section]) -> list[_Section]:
    merged: dict[str, str] = {}
    for sec in sections:
        cat = classify_section_name_by_rule(sec.name.lower())
        if cat:
            merged[cat] = (merged.get(cat, "") + "\n" + sec.content).strip()
    return [_Section(name=k, content=v) for k, v in merged.items()]


# ═══════════════════════════════════════════════════════════════════════
#  LLM API — OpenAI-compatible
# ═══════════════════════════════════════════════════════════════════════


def _llm_call(
    model: str, system_msg: str, user_msg: str, temp: float = 0.0,
) -> str:
    api_key = (
        os.environ.get("OPENAI_API_KEY")
        or os.environ.get("ANTHROPIC_API_KEY")
        or os.environ.get("DASHSCOPE_API_KEY")
        or os.environ.get("DEEPSEEK_API_KEY")
        or ""
    )
    base_url = os.environ.get("OPENAI_BASE_URL", "")
    if not api_key:
        print("ERROR: No API key. Set OPENAI_API_KEY / DEEPSEEK_API_KEY.")
        sys.exit(1)
    try:
        from openai import OpenAI
    except ImportError:
        print("ERROR: pip install openai")
        sys.exit(1)
    kwargs = {"api_key": api_key}
    if base_url:
        kwargs["base_url"] = base_url
    client = OpenAI(**kwargs)
    resp = client.chat.completions.create(
        model=model,
        messages=[
            {"role": "system", "content": system_msg},
            {"role": "user", "content": user_msg},
        ],
        temperature=temp,
    )
    return resp.choices[0].message.content or ""


def _parse_json_llm(text: str) -> dict | list:
    """Strip markdown fences and parse JSON from LLM response."""
    cleaned = re.sub(
        r"^```(?:json)?\s*\n?", "", text.strip(), flags=re.MULTILINE,
    )
    cleaned = re.sub(r"\n?```$", "", cleaned.strip(), flags=re.MULTILINE)
    return json.loads(cleaned)


# ═══════════════════════════════════════════════════════════════════════
#  Rubric — auto eval-points + per-section scoring
# ═══════════════════════════════════════════════════════════════════════

AUTO_EVAL_POINTS_PROMPT = """\
You are an expert academic reviewer. Given a list of sections in a research paper,
generate a rubric (eval_points.json) for evaluating the paper's quality.

For each section in VALID_SECTIONS that matches the paper's sections,
define 3-6 key elements that should be present.

VALID_SECTIONS: {valid_sections}
Paper's actual sections: {paper_sections}

Output ONLY valid JSON with this exact structure (no markdown fences):
{{
    "sections": [
        {{
            "section_name": "Introduction",
            "eval_points": [
                {{"element": "Problem motivation", "importance": "high", "description": "..."}},
                ...
            ]
        }},
        ...
    ]
}}

Include only sections from VALID_SECTIONS that match the paper's actual sections.
"""


def auto_generate_eval_points(raw_sec_names: list[str], model: str) -> dict:
    prompt = AUTO_EVAL_POINTS_PROMPT.format(
        valid_sections=", ".join(VALID_CATEGORIES),
        paper_sections=", ".join(raw_sec_names),
    )
    resp = _llm_call(
        model=model,
        system_msg="You generate structured JSON rubrics for paper evaluation.",
        user_msg=prompt,
        temp=0.3,
    )
    data = _parse_json_llm(resp)
    return data if isinstance(data, dict) else {"sections": data}


# ── Single-section rubric ────────────────────────────────────────────

RUBRIC_PROMPT = """\
You are an expert academic reviewer. Evaluate the following section of a paper
based on the given rubric criteria. Score each criterion on a scale of 1-5:

1 - Very Poor: Missing or completely inadequate
2 - Poor: Present but significantly lacking
3 - Acceptable: Adequate but has room for improvement
4 - Good: Well done with minor issues
5 - Excellent: Outstanding, comprehensive, and insightful

Section: {section_name}

Rubric criteria:
{eval_points_text}

Paper content:
{pred_content}

For each criterion, provide:
- score (1-5)
- reasoning (1-2 sentences)

Output ONLY valid JSON with this exact structure (no markdown fences):
{{
    "section_name": "{section_name}",
    "results": [
        {{"element": "criterion name", "score": 4, "reasoning": "..."}},
        ...
    ]
}}
"""


def evaluate_section_by_rubric(
    model: str, section_name: str, pred_content: str, eval_points: list[dict],
) -> _RubricResult:
    if not pred_content or not eval_points:
        return _RubricResult(
            section_name=section_name,
            results=[{"element": p["element"], "score": 1, "reasoning": "No content"}
                     for p in eval_points],
            average_score=1.0 if eval_points else 0.0,
            total_count=len(eval_points),
        )
    pts_text = "\n".join(
        f'- "{p["element"]}" ({p.get("importance", "medium")}): {p.get("description", "")}'
        for p in eval_points
    )
    prompt = RUBRIC_PROMPT.format(
        section_name=section_name,
        eval_points_text=pts_text,
        pred_content=pred_content[:8000],
    )
    resp = _llm_call(
        model=model,
        system_msg="You are an expert academic reviewer. Output ONLY valid JSON.",
        user_msg=prompt,
        temp=0.0,
    )
    try:
        data = _parse_json_llm(resp)
    except (json.JSONDecodeError, KeyError):
        return _RubricResult(
            section_name=section_name,
            results=[{"element": p["element"], "score": 1, "reasoning": "LLM parse failed"}
                     for p in eval_points],
            average_score=1.0,
            total_count=len(eval_points),
        )
    results = data.get("results", [])
    total = sum(r.get("score", 1) for r in results)
    count = len(results) or 1
    return _RubricResult(
        section_name=section_name, results=results,
        average_score=round(total / count, 2), total_count=count,
    )


# ═══════════════════════════════════════════════════════════════════════
#  Self-consistency hallucination check
# ═══════════════════════════════════════════════════════════════════════

SELF_CONSISTENCY_PROMPT = """\
You are an expert academic reviewer performing a self-consistency audit on a
research paper (no ground truth available). Analyse the full paper text below
and identify:

1. INTERNAL CONTRADICTIONS — places where the paper says different things
   about the same claim in different sections (e.g., Introduction claims 40%
   improvement but Abstract says 30%).
2. UNSUPPORTED CLAIMS — strong claims (especially "SOTA", "first", "best",
   "significant") in Abstract / Introduction that are not backed by evidence
   in the Experiment / Results sections.
3. METHOD-EXPERIMENT GAP — method description promises features or analyses
   that the experiment section never evaluates.

Paper sections available: {sections}

Full text (truncated): {text}

Output ONLY valid JSON (no markdown fences):
{{
    "contradictions": [
        {{
            "type": "internal_contradiction|unsupported_claim|method_experiment_gap",
            "severity": "high|medium|low",
            "claim": "what the paper says",
            "location": "which section(s)",
            "explanation": "why this is a problem"
        }}
    ],
    "overall_assessment": "brief summary of consistency issues",
    "self_consistency_score": 1-5
}}

Score 5 = fully self-consistent, 1 = pervasive contradictions.
"""


def check_self_consistency(content: str, model: str) -> dict:
    """Analyse internal consistency without ground truth."""
    sections = extract_sections_from_text(content)
    sec_names = [s.name for s in sections]
    text_sample = content[:15000]  # token limit
    prompt = SELF_CONSISTENCY_PROMPT.format(
        sections=", ".join(sec_names[:20]),
        text=text_sample,
    )
    resp = _llm_call(
        model=model,
        system_msg="You are an expert academic reviewer auditing paper consistency.",
        user_msg=prompt,
        temp=0.0,
    )
    try:
        data = _parse_json_llm(resp)
    except (json.JSONDecodeError, KeyError):
        return {
            "error": "LLM response parse failed",
            "contradictions": [],
            "overall_assessment": "Could not analyse",
            "self_consistency_score": 0,
        }
    return {
        "contradictions": data.get("contradictions", []),
        "overall_assessment": data.get("overall_assessment", ""),
        "self_consistency_score": data.get("self_consistency_score", 0),
    }


# ═══════════════════════════════════════════════════════════════════════
#  Citation sanity check
# ═══════════════════════════════════════════════════════════════════════

CITATION_HALLUCINATION_PROMPT = """\
You are an expert reviewer auditing citation quality. Below is text from a
research paper. Identify suspicious references that may be hallucinated.

Criteria for suspicion:
- References with implausible author names (e.g., "Smith et al." with no
  verifiable detail)
- References that seem too perfectly aligned with the paper's claims
- References cited in a vague way without a specific finding
- References that mix real and fake details (partial hallucination)

Paper text excerpt: {text}

Output ONLY valid JSON (no markdown fences):
{{
    "suspicious_references": [
        {{
            "reference": "text snippet containing the reference",
            "suspicion": "why this looks hallucinated",
            "risk": "high|medium|low"
        }}
    ],
    "citation_density": {{
        "total_citations_found": N,
        "density_assessment": "adequate|sparse|excessive|unknown"
    }},
    "overall_assessment": "summary",
    "citation_quality_score": 1-5
}}
"""


def check_citations(content: str, model: str) -> dict:
    """Analyse citation sanity without bibliography file."""
    # 1. Count citation markers in the raw text
    latex_cites = re.findall(r"\\cite\{[^}]*\}", content)
    markdown_cites = re.findall(r"\[[\d,\s\-]+\]", content)
    parenthetical = re.findall(r"\([^)]*\d{4}[^)]*\)", content)
    total = len(latex_cites) + len(markdown_cites) + len(parenthetical)

    # 2. LLM hallucination check on a sample
    text_sample = content[:12000]
    prompt = CITATION_HALLUCINATION_PROMPT.format(text=text_sample)
    resp = _llm_call(
        model=model,
        system_msg="You audit academic citation quality. Output ONLY valid JSON.",
        user_msg=prompt,
        temp=0.0,
    )
    try:
        data = _parse_json_llm(resp)
    except (json.JSONDecodeError, KeyError):
        data = {
            "suspicious_references": [],
            "overall_assessment": "Could not analyse",
            "citation_quality_score": 0,
        }

    return {
        "citation_count_total": total,
        "latex_citations": len(latex_cites),
        "markdown_or_numeric_citations": len(markdown_cites) + len(parenthetical),
        "suspicious_references": data.get("suspicious_references", []),
        "citation_density": data.get("citation_density", {}),
        "overall_assessment": data.get("overall_assessment", ""),
        "citation_quality_score": data.get("citation_quality_score", 0),
    }


# ═══════════════════════════════════════════════════════════════════════
#  Figure / Table coverage check
# ═══════════════════════════════════════════════════════════════════════


def check_figure_table_coverage(content: str) -> dict:
    """Analyse figure/table mentions vs. available captions."""
    # Find mentions like "Figure 3", "Fig. 3", "Table 2"
    figure_mentions = re.findall(
        r"(?:Figure|Fig\.?|FIGURE)\s+(\d+[a-z]?(?:\.\d+)?)", content,
    )
    table_mentions = re.findall(
        r"(?:Table|TABLE)\s+(\d+[a-z]?(?:\.\d+)?)", content,
    )

    # Find captions
    figure_captions = re.findall(
        r"(?:\\caption\{[^}]*\}|\\includegraphics[^}]*\}|\[Figure\s+\d[^\]]*\])",
        content,
    )
    table_captions = re.findall(
        r"(?:\\caption\{[^}]*table[^}]*\}|\[Table\s+\d[^\]]*\])",
        content,
        re.IGNORECASE,
    )

    # Unique figure/table numbers mentioned
    unique_figs = sorted(set(figure_mentions))
    unique_tabs = sorted(set(table_mentions))

    # "Orphan" mentions: check that most-referenced numbers have a caption
    # For PDF conversions this is heuristic — we treat any text
    # "Figure N:" or "Table N:" at line start as a caption proxy
    lines = content.split("\n")
    caption_figs = set()
    caption_tabs = set()
    for line in lines:
        m = re.match(r"\*\*?Figure\s+(\d+[a-z]?(?:\.\d+)?)\s*[:\*]", line, re.IGNORECASE)
        if m:
            caption_figs.add(m.group(1))
        m = re.match(r"\*\*?Table\s+(\d+[a-z]?(?:\.\d+)?)\s*[:\*]", line, re.IGNORECASE)
        if m:
            caption_tabs.add(m.group(1))

    # Cross-reference: are mentioned figures/tables accounted for?
    fig_gaps = [f for f in unique_figs if f not in caption_figs]
    tab_gaps = [t for t in unique_tabs if t not in caption_tabs]

    return {
        "figure_mentions": len(figure_mentions),
        "table_mentions": len(table_mentions),
        "unique_figures_referenced": unique_figs,
        "unique_tables_referenced": unique_tabs,
        "figure_captions_found": len(caption_figs),
        "table_captions_found": len(caption_tabs),
        "figures_missing_caption": fig_gaps,
        "tables_missing_caption": tab_gaps,
        "figure_coverage": round(len(caption_figs) / max(len(unique_figs), 1) * 100),
        "table_coverage": round(len(caption_tabs) / max(len(unique_tabs), 1) * 100),
        "overall_assessment": (
            f"{len(unique_figs)} figures referenced, {len(caption_figs)} captions found; "
            f"{len(unique_tabs)} tables referenced, {len(caption_tabs)} captions found."
        ),
    }


# ═══════════════════════════════════════════════════════════════════════
#  Input reading
# ═══════════════════════════════════════════════════════════════════════


def read_input(file_path: Path) -> str:
    """Read LaTeX (.tex), Markdown (.md), or PDF input."""
    if file_path.suffix == ".pdf":
        return _pdf_to_markdown(file_path)
    content = file_path.read_text(encoding="utf-8")
    if file_path.suffix == ".md":
        content = _markdown_to_section_text(content)
    return content


def _pdf_to_markdown(path: Path) -> str:
    """Convert PDF to Markdown via pymupdf4llm."""
    try:
        import pymupdf4llm
        md = pymupdf4llm.to_markdown(str(path))
        return _markdown_to_section_text(md)
    except ImportError:
        print("ERROR: pymupdf4llm not installed. Run: pip install pymupdf4llm")
        print("  Or convert PDF to Markdown manually first and use --markdown.")
        sys.exit(1)


def _markdown_to_section_text(md: str) -> str:
    lines = md.split("\n")
    out = []
    for line in lines:
        m = re.match(r"^(#{1,3})\s+(.+)$", line)
        if m:
            title = m.group(2).strip()
            out.append(f"\\section{{{title}}}")
        else:
            out.append(line)
    return "\n".join(out)


def load_eval_points(path: Path) -> dict:
    if path.exists():
        data = json.loads(path.read_text(encoding="utf-8"))
        if isinstance(data, dict) and "sections" in data:
            return data
        return {"sections": data}
    return {"sections": []}


# ═══════════════════════════════════════════════════════════════════════
#  Main pipeline — runs all requested checks
# ═══════════════════════════════════════════════════════════════════════


def run_all_checks(
    content: str,
    model: str,
    eval_points_data: dict | None,
    do_rubric: bool,
    do_consistency: bool,
    do_citations: bool,
    do_figure_table: bool,
    output_path: Path | None,
) -> dict:
    """Run selected evaluation modules and aggregate results."""
    results: dict = {}

    # ── Rubric ─────────────────────────────────────────────────────
    if do_rubric and eval_points_data:
        print("\n[1/4] Rubric evaluation...")
        raw = extract_sections_from_text(content)
        classified: list[tuple[str, str]] = []
        for sec in raw:
            cat = classify_section_name_by_rule(sec.name.lower())
            if cat:
                classified.append((cat, sec.content))
        merged: dict[str, str] = {}
        for cat, cont in classified:
            merged[cat] = (merged.get(cat, "") + "\n" + cont).strip()
        section_map = {k: v for k, v in merged.items()}

        rubric_list: list[dict] = []
        total_score = 0.0
        total_pts = 0
        eval_secs = eval_points_data.get("sections", [])
        print(f"  Evaluating {len(eval_secs)} sections...")
        for es in eval_secs:
            sec_name = es["section_name"]
            points = es.get("eval_points", [])
            pred_content = section_map.get(sec_name, "")
            if not pred_content:
                print(f"    {sec_name}: no content → all=1")
                r = {
                    "section_name": sec_name,
                    "results": [{"element": p["element"], "score": 1,
                                 "reasoning": "Not found"} for p in points],
                    "average_score": 1.0, "total_count": len(points),
                }
            else:
                print(f"    {sec_name}: {len(points)} criteria...")
                er = evaluate_section_by_rubric(model, sec_name, pred_content, points)
                r = er.model_dump()
            rubric_list.append(r)
            total_score += r["average_score"] * r["total_count"]
            total_pts += r["total_count"]

        results["rubric"] = rubric_list
        results["rubric_summary"] = {
            "total_points": total_pts,
            "average_score": round(total_score / total_pts, 2) if total_pts else 0.0,
        }

    # ── Self-consistency ────────────────────────────────────────────
    if do_consistency:
        print("\n[2/4] Self-consistency hallucination check...")
        results["self_consistency"] = check_self_consistency(content, model)
        sc = results["self_consistency"]
        print(f"  Score: {sc.get('self_consistency_score', '?')}/5")
        print(f"  Contradictions found: {len(sc.get('contradictions', []))}")

    # ── Citations ───────────────────────────────────────────────────
    if do_citations:
        print("\n[3/4] Citation sanity check...")
        results["citation_sanity"] = check_citations(content, model)
        cs = results["citation_sanity"]
        print(f"  Total citations: {cs.get('citation_count_total', 0)}")
        print(f"  Suspicious: {len(cs.get('suspicious_references', []))}")
        print(f"  Quality score: {cs.get('citation_quality_score', '?')}/5")

    # ── Figure / Table ──────────────────────────────────────────────
    if do_figure_table:
        print("\n[4/4] Figure/Table coverage check...")
        results["figure_table_coverage"] = check_figure_table_coverage(content)
        ft = results["figure_table_coverage"]
        print(f"  Figure coverage: {ft.get('figure_coverage', 0)}%")
        print(f"  Table coverage: {ft.get('table_coverage', 0)}%")

    # ── Save ────────────────────────────────────────────────────────
    if output_path:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(json.dumps(results, indent=2, ensure_ascii=False), encoding="utf-8")
        print(f"\n  Full results: {output_path}")

    return results


def save_report(results: dict, json_path: Path, model: str, input_name: str) -> Path:
    """Generate a Markdown evaluation report alongside the JSON result."""
    # Attach meta info if not already present
    if "_meta" not in results:
        results["_meta"] = {"model": model, "input": input_name}

    report_path = json_path.with_suffix(".md")
    try:
        from generate_report import generate_report
        report = generate_report(results)
        report_path.write_text(report, encoding="utf-8")
        print(f"  Report:        {report_path}")
    except ImportError:
        # Fallback: write minimal report
        lines = [
            f"# Paper Evaluation Report\n",
            f"**Model**: {model}  \n**Input**: {input_name}\n",
            "---\n",
            "*(Full report requires generate_report.py)*\n",
        ]
        report_path.write_text("\n".join(lines), encoding="utf-8")
    return report_path


# ═══════════════════════════════════════════════════════════════════════
#  CLI
# ═══════════════════════════════════════════════════════════════════════


def main():
    parser = argparse.ArgumentParser(
        description="Paper evaluation — rubric + sanity checks (no GT needed)"
    )

    inp = parser.add_mutually_exclusive_group(required=True)
    inp.add_argument("--pdf", type=str, help="PDF file (auto-converted to Markdown)")
    inp.add_argument("--latex", type=str, help="LaTeX file (.tex)")
    inp.add_argument("--markdown", type=str, help="Markdown file (.md)")

    parser.add_argument("--model", type=str, default="gpt-4o",
                        help="LLM model")
    parser.add_argument("--output", type=str, help="Output JSON path")

    # Rubric options
    parser.add_argument("--eval-points", type=str,
                        help="eval_points.json for rubric")
    parser.add_argument("--auto-eval-points", action="store_true",
                        help="Auto-generate eval_points via LLM")

    # Additional checks
    parser.add_argument("--self-consistency", action="store_true",
                        help="Run self-consistency hallucination check")
    parser.add_argument("--citations", action="store_true",
                        help="Run citation sanity check")
    parser.add_argument("--figure-table", action="store_true",
                        help="Run figure/table coverage check")
    parser.add_argument("--full-sanity", action="store_true",
                        help="Run all extra checks (consistency + citations + figure/table)")
    parser.add_argument("--all", action="store_true",
                        help="Run rubric + all extra checks")
    parser.add_argument("--report", action="store_true",
                        help="Generate human-readable Markdown report alongside JSON")

    args = parser.parse_args()

    # ── Read input ──────────────────────────────────────────────────
    input_path = Path(args.pdf or args.latex or args.markdown)
    if not input_path.exists():
        print(f"ERROR: {input_path} not found")
        sys.exit(1)
    content = read_input(input_path)
    print(f"Read {len(content)} chars from {input_path.name}")

    # ── Determine which checks to run ───────────────────────────────
    do_rubric = bool(args.eval_points) or bool(args.auto_eval_points) or args.all
    do_consistency = args.self_consistency or args.full_sanity or args.all
    do_citations = args.citations or args.full_sanity or args.all
    do_figure_table = args.figure_table or args.full_sanity or args.all

    # ── Rubric setup ────────────────────────────────────────────────
    eval_points_data: dict | None = None
    if do_rubric:
        raw_names = [s.name for s in extract_sections_from_text(content)]
        if args.eval_points:
            eval_points_data = load_eval_points(Path(args.eval_points))
            print(f"Loaded eval_points ({len(eval_points_data.get('sections', []))} sections)")
        elif args.auto_eval_points or args.all:
            print("Auto-generating eval_points...")
            eval_points_data = auto_generate_eval_points(raw_names, args.model)
            for sec in eval_points_data.get("sections", []):
                print(f"  {sec['section_name']}: {len(sec.get('eval_points', []))} criteria")

    # ── Run ─────────────────────────────────────────────────────────
    results = run_all_checks(
        content=content,
        model=args.model,
        eval_points_data=eval_points_data,
        do_rubric=do_rubric,
        do_consistency=do_consistency,
        do_citations=do_citations,
        do_figure_table=do_figure_table,
        output_path=Path(args.output) if args.output else None,
    )

    # ── Generate report ─────────────────────────────────────────────
    if args.report and args.output:
        json_path = Path(args.output)
        from generate_report import generate_report as gr
        results["_meta"] = {"model": args.model, "input": input_path.name}
        report_path = json_path.with_suffix(".md")
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(gr(results), encoding="utf-8")
        print(f"  Report:        {report_path}")
    elif args.report and not args.output:
        print("  (--report requires --output; printing inline summary instead)", file=sys.stderr)

    # ── Summary ─────────────────────────────────────────────────────
    print(f"\n{'='*60}")
    print("  EVALUATION SUMMARY")
    print(f"{'='*60}")

    if "rubric" in results:
        print(f"\n  ── Rubric ({results['rubric_summary']['average_score']}/5) ──")
        for sec in results["rubric"]:
            print(f"    {sec['section_name']:25s}  {sec['average_score']:.2f}")

    if "self_consistency" in results:
        sc = results["self_consistency"]
        print(f"\n  ── Self-consistency ({sc.get('self_consistency_score', '?')}/5) ──")
        for c in sc.get("contradictions", [])[:3]:
            print(f"    [{c.get('severity','?')}] {c.get('claim','')[:80]}")

    if "citation_sanity" in results:
        cs = results["citation_sanity"]
        print(f"\n  ── Citations ({cs.get('citation_quality_score', '?')}/5) ──")
        print(f"    Total: {cs.get('citation_count_total', 0)}")
        for s in cs.get("suspicious_references", [])[:3]:
            print(f"    [{s.get('risk','?')}] {s.get('reference','')[:80]}")

    if "figure_table_coverage" in results:
        ft = results["figure_table_coverage"]
        print(f"\n  ── Figures & Tables ──")
        print(f"    Figures: {ft.get('figure_coverage', 0)}% coverage")
        print(f"    Tables:  {ft.get('table_coverage', 0)}% coverage")

    print()


if __name__ == "__main__":
    main()
