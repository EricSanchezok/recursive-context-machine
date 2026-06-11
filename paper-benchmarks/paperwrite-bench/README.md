# PaperWrite-Bench

Evaluates AI-generated academic paper quality — with or without ground truth.

**Official PaperRecon repo**: [Agent4Science-UTokyo/PaperRecon](https://github.com/Agent4Science-UTokyo/PaperRecon)

---

## Table of Contents

- [Two Evaluation Modes](#two-evaluation-modes)
- [Quick Start](#quick-start)
- [Simple Mode (No GT Required)](#simple-mode-no-gt-required)
- [Full Mode (PaperRecon, Requires GT)](#full-mode-paperrecon-requires-gt)
- [Output](#output)
- [Architecture](#architecture)
- [References](#references)

---

## Two Evaluation Modes

| Feature | Simple Mode | Full Mode |
|:--------|:-----------:|:---------:|
| Input | PDF / Markdown / LaTeX | LaTeX + GT resources |
| Ground Truth needed? | ❌ No | ✅ Yes (eval_points, figures, code) |
| Rubric (1-5 per section) | ✅ | ✅ |
| Self-consistency hallucination | ✅ (internal) | ✅ (vs GT) |
| Citation sanity check | ✅ (LLM-based) | ✅ (Citation F1) |
| Figure/Table coverage | ✅ (mention vs caption) | ✅ (full comparison) |
| Human-readable report | ✅ | — |
| API key needed | ✅ (any LLM) | ✅ (GPT-5.4) |
| Docker / Linux needed | ❌ No | ✅ Yes |

---

## Quick Start

### Prerequisites

```powershell
pip install openai pymupdf4llm
```

### Set API Key

```powershell
# DeepSeek
$env:DEEPSEEK_API_KEY = "REDACTED"
$env:OPENAI_BASE_URL = "https://api.deepseek.com"

# Or OpenAI
$env:OPENAI_API_KEY = "sk-..."
```

### Run: PDF → All Evaluations → Report

```powershell
.\run_paperwrite.ps1 -Simple -Pdf paper.pdf -All
```

This single command:
1. Converts PDF to Markdown
2. Auto-generates rubric eval points
3. Scores each section (1-5) with LLM reasoning
4. Checks self-consistency (contradictions, unsupported claims)
5. Audits citation quality
6. Reports figure/table coverage
7. Outputs a structured JSON + a human-readable Markdown report

---

## Simple Mode (No GT Required)

### Input Formats

| Flag | Input | Description |
|:----|:------|:------------|
| `-Pdf file.pdf` | PDF | Auto-converted to Markdown via pymupdf4llm |
| `-Markdown paper.md` | Markdown | From PDF conversion or any source |
| `-Latex paper.tex` | LaTeX | Raw LaTeX source |

### Rubric Options

| Flag | Description |
|:-----|:------------|
| `-AutoEvalPoints` | LLM auto-generates per-section scoring criteria (recommended) |
| `-EvalPoints points.json` | Use a pre-written eval_points.json |

If neither is specified, `-AutoEvalPoints` is used by default.

### Evaluation Flags

| Flag | Runs |
|:-----|:-----|
| *(none)* | Rubric only |
| `-All` | Rubric + Self-consistency + Citations + Figure/Table |
| `-FullSanity` | Self-consistency + Citations + Figure/Table (no rubric) |
| `-SelfConsistency` | Self-consistency hallucination check only |
| `-Citations` | Citation sanity check only |
| `-FigureTable` | Figure/Table coverage check only |

### Model

```powershell
-Model deepseek-chat    # Default: gpt-4o
```

### Usage Examples

```powershell
# ── End-to-end: PDF → full evaluation → report
.\run_paperwrite.ps1 -Simple -Pdf paper.pdf -All

# ── Quick rubric only (fastest, 1 LLM call per section)
.\run_paperwrite.ps1 -Simple -Pdf paper.pdf

# ── From LaTeX with all sanity checks
.\run_paperwrite.ps1 -Simple -Latex paper.tex -FullSanity

# ── From Markdown with custom model, citation check only
.\run_paperwrite.ps1 -Simple -Markdown paper.md -Citations -Model gpt-4o
```

### Direct Python Invocation

```powershell
# PDF → rubric + sanity checks + report
python evaluate_simple.py --pdf paper.pdf --auto-eval-points --all `
    --output results.json --report

# LaTeX → rubric only
python evaluate_simple.py --latex paper.tex --auto-eval-points `
    --output results.json --report

# Markdown → citations + figure/table
python evaluate_simple.py --markdown paper.md --citations --figure-table `
    --output results.json --report
```

### Generate Report from Existing JSON

```powershell
python generate_report.py results.json --output report.md

# Or pipe to stdout
python generate_report.py results.json
```

---

## Full Mode (PaperRecon, Requires GT)

This mode uses the official PaperRecon pipeline. It requires the full ground-truth
dataset (eval_points.json, figures, tables, code, bib file) and is designed for
the standard PaperWrite-Bench benchmark.

```powershell
# Evaluate single paper
.\run_paperwrite.ps1 -Paper paper_1

# Evaluate all papers
.\run_paperwrite.ps1 -AllPapers

# Specific eval mode: rubric / hallucination / citation / all
.\run_paperwrite.ps1 -Paper paper_3 -EvalMode rubric
```

**Requirements**: Docker or Pixi (Linux recommended), Claude Code CLI, GPT-5.4 API.

---

## Output

### Directory Structure

```
paper-benchmarks/
  reports/
    paperwrite_simple_20260611_191807.json   # Structured JSON
    paperwrite_simple_20260611_191807.md     # Human-readable report
```

### JSON Structure

```json
{
  "rubric": [
    {
      "section_name": "Introduction",
      "results": [
        {"element": "Problem motivation", "score": 5, "reasoning": "..."}
      ],
      "average_score": 4.0,
      "total_count": 4
    }
  ],
  "rubric_summary": {
    "total_points": 27,
    "average_score": 4.07
  },
  "self_consistency": {
    "contradictions": [
      {"type": "internal_contradiction", "severity": "high", "claim": "...",
       "location": "Abstract vs. Introduction", "explanation": "..."}
    ],
    "self_consistency_score": 3
  },
  "citation_sanity": {
    "citation_count_total": 36,
    "suspicious_references": [
      {"reference": "[1] ...", "suspicion": "...", "risk": "high"}
    ],
    "citation_quality_score": 2
  },
  "figure_table_coverage": {
    "figure_mentions": 6,
    "table_mentions": 33,
    "figure_coverage": 0,
    "table_coverage": 0
  },
  "_meta": {
    "model": "deepseek-chat",
    "input": "paper.pdf"
  }
}
```

### Report Structure (Markdown)

```
1. 📊 Score Overview          — 4-dimension score card with bar charts
2. 📝 Rubric Breakdown        — Per-section, per-criterion scores + LLM reasoning
3. 🔍 Self-Consistency Issues  — Each contradiction with severity, location, explanation
4. 📚 Citation Analysis       — Suspicious references with risk levels
5. 📊 Figures & Tables        — Mention vs caption coverage
6. 🎯 Improvement Suggestions — 🔴🟡🟢 prioritized action items
```

---

## Architecture

```
paperwrite-bench/
├── evaluate_simple.py        # Core evaluator — LLM-based rubric + sanity checks
├── generate_report.py        # Report generator — JSON → Markdown
├── run_paperwrite.ps1        # One-click runner — both Simple & Full modes
├── PaperRecon/               # Official PaperRecon repo (Full mode only)
│   ├── PaperWrite-Bench/
│   └── paper_recon/
├── run_evaluation.py         # Full-mode wrapper
├── setup.ps1                 # PaperRecon setup script
└── README.md                 # This file
```

### Pipeline Flow

```
Simple Mode:
  PDF ──→ pymupdf4llm ──→ Markdown ──→ evaluate_simple.py ──→ JSON ──→ generate_report.py ──→ Report (.md)

Full Mode:
  GT LaTeX + resources ──→ PaperRecon pipeline ──→ JSON comparison
```

---

## References

- [PaperRecon: Paper Reconstruction Evaluation](https://arxiv.org/pdf/2604.01128) — arXiv:2604.01128
- [Official PaperRecon Repo](https://github.com/Agent4Science-UTokyo/PaperRecon)
