# MLR-Bench

Evaluates AI agents on **open-ended machine learning research** — with or without
the official benchmark pipeline.

**Official repo**: [chchenhui/mlrbench](https://github.com/chchenhui/mlrbench)
**Paper**: [MLR-Bench: Evaluating AI Agents on Open-Ended ML Research](https://arxiv.org/abs/2505.19955)

---

## Table of Contents

- [Two Evaluation Modes](#two-evaluation-modes)
- [Quick Start](#quick-start)
- [Simple Mode (No GT Required)](#simple-mode-no-gt-required)
- [Full Mode (Official MLR-Bench Pipeline)](#full-mode-official-mlr-bench-pipeline)
- [Output](#output)
- [Architecture](#architecture)

---

## Two Evaluation Modes

| Feature | Simple Mode | Full Mode |
|:--------|:-----------:|:---------:|
| Input | PDF / Markdown / LaTeX | task.md + agent results |
| Ground Truth needed? | ❌ No | ✅ Yes (MLR-Bench tasks) |
| Clarity (1-10) | ✅ | ✅ |
| Novelty (1-10) | ✅ | ✅ |
| Soundness (1-10) | ✅ | ✅ |
| Significance (1-10) | ✅ | ✅ |
| Overall + Strengths/Weaknesses | ✅ | ✅ |
| Self-consistency check | ✅ | ❌ (n/a) |
| Citation sanity check | ✅ | ❌ (n/a) |
| Figure/Table coverage | ✅ | ❌ (n/a) |
| Human-readable report | ✅ | ❌ |
| Idea / Proposal / Experiment / Writeup | ❌ | ✅ |
| API key needed | ✅ (any LLM) | ✅ (Gemini + Claude) |

---

## Quick Start

### Prerequisites

```powershell
pip install openai pymupdf4llm
```

### Set API Key

```powershell
# DeepSeek
$env:OPENAI_API_KEY = "sk-..."
$env:OPENAI_BASE_URL = "https://api.deepseek.com"

# Or OpenAI
$env:OPENAI_API_KEY = "sk-..."
```

### Run: PDF → Overall Review → Report

```powershell
.\run_mlr.ps1 -Simple -Pdf paper.pdf
```

---

## Simple Mode (No GT Required)

Faithfully replicates the **MLR-Bench OVERALL_RUBRIC** prompt to evaluate a paper
on four dimensions (Clarity, Novelty, Soundness, Significance) plus an Overall
assessment with strengths and weaknesses.

### Input Formats

| Flag | Input | Description |
|:----|:------|:------------|
| `-Pdf file.pdf` | PDF | Auto-converted to Markdown via pymupdf4llm |
| `-Markdown paper.md` | Markdown | From PDF conversion or any source |
| `-Latex paper.tex` | LaTeX | Raw LaTeX source |

### Evaluation Scope

| `-Eval` | Runs |
|:--------|:-----|
| `overall` (default) | Overall review (Clarity/Novelty/Soundness/Significance) |
| `consistency` | Self-consistency check only |
| `citations` | Citation sanity check only |
| `figure-table` | Figure/Table coverage check only |
| `sanity` | All extra checks (no overall review) |
| `all` | Overall review + all extra checks |

### Model

```powershell
-Model deepseek-chat    # Default: gpt-4o
```

### Usage Examples

```powershell
# ── PDF → Overall review (default) ──
.\run_mlr.ps1 -Simple -Pdf paper.pdf

# ── PDF → Full evaluation (review + all checks) ──
.\run_mlr.ps1 -Simple -Pdf paper.pdf -Eval all

# ── Markdown → Citation check only ──
.\run_mlr.ps1 -Simple -Markdown paper.md -Eval citations

# ── With custom model ──
.\run_mlr.ps1 -Simple -Pdf paper.pdf -Eval all -Model gpt-4o
```

### Direct Python Invocation

```powershell
# PDF → overall review
python evaluate_simple.py --pdf paper.pdf --model deepseek-chat `
    --output results.json --report

# LaTeX → all checks
python evaluate_simple.py --latex paper.tex --all --output results.json --report

# Markdown → citation + figure/table only
python evaluate_simple.py --markdown paper.md --citations --figure-table `
    --output results.json --report
```

### Generate Report from Existing JSON

```powershell
python generate_report.py results.json --output report.md
```

---

## Full Mode (Official MLR-Bench Pipeline)

This mode uses the official MLR-Bench codebase and requires the full benchmark
setup, including all task descriptions (`tasks/*.md`) and agent results.

```powershell
# Setup
.\setup.ps1

# End-to-end evaluation
.\run_mlr.ps1 -Mode end-to-end

# Stepwise evaluation (all stages)
.\run_mlr.ps1 -Mode stepwise

# Single review stage
.\run_mlr.ps1 -Review overall
.\run_mlr.ps1 -Review idea
.\run_mlr.ps1 -Review proposal
```

**Requirements**: Docker or Linux recommended, Gemini-2.5-Pro-Preview + Claude-3.7-Sonnet
API keys, coding agent CLI (Claude Code / Codex).

---

## Output

### Directory Structure

```
mlr-bench/
  reports/
    mlr_simple_20260611_193041.json   # Structured JSON
    mlr_simple_20260611_193041.md     # Human-readable report
```

### JSON Structure

```json
{
  "_meta": { "model": "deepseek-chat", "input": "paper.pdf" },
  "overall_review": {
    "Clarity": { "score": 7, "justification": "..." },
    "Novelty": { "score": 6, "justification": "..." },
    "Soundness": { "score": 7, "justification": "..." },
    "Significance": { "score": 6, "justification": "..." },
    "Overall": {
      "score": 7,
      "strengths": ["...", "..."],
      "weaknesses": ["...", "..."]
    },
    "Confidence": 4
  },
  "self_consistency": { "contradictions": [...], "self_consistency_score": 3 },
  "citation_sanity": { "suspicious_references": [...], "citation_quality_score": 4 },
  "figure_table_coverage": { "figure_coverage": 0, "table_coverage": 0 }
}
```

### Report Structure (Markdown)

```
1. 📊 Score Overview          — 4 dimensions + overall + strengths/weaknesses
2. 📝 Detailed Rubric         — Per-dimension score + LLM justification
3. 🔍 Self-Consistency Issues — Contradictions with severity and explanation
4. 📚 Citation Analysis       — Suspicious references with risk levels
5. 📊 Figures & Tables        — Mention vs caption coverage
6. 🎯 Improvement Suggestions — 🔴🟡🟢 prioritized action items
```

---

## Architecture

```
mlr-bench/
├── evaluate_simple.py        # Simple mode — self-contained overall review
├── generate_report.py        # Report generator — JSON → Markdown
├── run_mlr.ps1               # One-click runner — both Simple & Full modes
├── run_evaluation.py         # Full-mode wrapper (official pipeline)
├── setup.ps1                 # Official repo setup
├── mlrbench/                 # Official MLR-Bench repo (Full mode)
│   ├── tasks/*.md
│   └── mlrbench/evals/
│       ├── overall_review.py
│       ├── review_idea.py
│       ├── review_proposal.py
│       ├── review_experiments.py
│       └── review_writeup.py
└── README.md                 # This file
```

### Pipeline Flow

```
Simple Mode:
  PDF ──→ pymupdf4llm ──→ evaluate_simple.py ──→ JSON ──→ generate_report.py ──→ Report (.md)

Full Mode:
  task.md + agent results ──→ official MLR-Bench evals ──→ JSON comparison
```

---

## References

- [MLR-Bench: Evaluating AI Agents on Open-Ended ML Research](https://arxiv.org/abs/2505.19955)
- [Official Repo](https://github.com/chchenhui/mlrbench)
