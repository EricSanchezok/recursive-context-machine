# Paper Benchmarks

## Overview

A unified evaluation pipeline for three leading **LLM-generated paper** benchmarks.
Designed for the RCM (Recursive Context Machine) project.

| Benchmark | Focus | Metrics | Official Repo |
|-----------|-------|---------|---------------|
| **PaperWrite-Bench** | Paper writing quality | Rubric, Hallucination, Citation F1 | [PaperRecon](https://github.com/Agent4Science-UTokyo/PaperRecon) |
| **MLR-Bench** | ML research quality | Idea, Proposal, Experiment, Writing | [mlrbench](https://github.com/chchenhui/mlrbench) |
| **SciReplicate-Bench** | Algorithm reproduction | CodeBLEU, Exec ACC, Recall, Graph ACC | [SciReplicate-Bench](https://github.com/xyzCS/SciReplicate-Bench) |

This pipeline follows the **official evaluation methods** from each benchmark.

## Setup

### Prerequisites

- Python 3.10+
- Git
- API keys (set in `.env` file):
  - `OPENAI_API_KEY`
  - `ANTHROPIC_API_KEY`
  - `DEEPSEEK_API_KEY` (for PaperWrite-Bench)
  - `OPENROUTER_API_KEY` (optional, for MLR-Bench)

### Quick Start

```powershell
# 1. Set up all benchmarks
cd paperwrite-bench && .\setup.ps1 && cd ..
cd mlr-bench && .\setup.ps1 && cd ..
cd scireplicate-bench && .\setup.ps1 && cd ..

# 2. Create .env file
echo "OPENAI_API_KEY=sk-..." > .env

# 3. Run full pipeline
.\run_full_pipeline.ps1

# 4. Or check environment first
python run_evaluation.py --check
```

## Usage

### Unified CLI

```powershell
# List all benchmarks
python run_evaluation.py --list

# Run all benchmarks
python run_evaluation.py --all

# Run a single benchmark
python run_evaluation.py --benchmark paperwrite --list
python run_evaluation.py --benchmark mlr --stepwise --model-name gpt-4o
python run_evaluation.py --benchmark scireplicate --check
```

### PaperWrite-Bench

```powershell
# List available papers
python run_evaluation.py --benchmark paperwrite --list

# Evaluate a single paper
python run_evaluation.py --benchmark paperwrite --paper paper_1 --eval-mode all
```

### MLR-Bench

```powershell
# Stepwise evaluation (all 4 stages)
python run_evaluation.py --benchmark mlr --stepwise

# Single stage
python run_evaluation.py --benchmark mlr --review idea
```

### SciReplicate-Bench

```powershell
# Run generation
python run_evaluation.py --benchmark scireplicate --run --model gpt-4o-mini

# Evaluate all metrics
python run_evaluation.py --benchmark scireplicate --all-metrics
```

### Full Pipeline

```powershell
# Run everything (with skip flags)
.\run_full_pipeline.ps1                              # all benchmarks
.\run_full_pipeline.ps1 -CheckOnly                   # check env only
.\run_full_pipeline.ps1 -SkipPaperWrite              # skip PaperWrite
.\run_full_pipeline.ps1 -SkipMLR -SkipSciReplicate   # only PaperWrite
```

## Directory Structure

```
paper-benchmarks/
├── run_evaluation.py            ← Unified CLI
├── run_full_pipeline.ps1        ← One-click orchestrator
├── README.md
│
├── paperwrite-bench/            ← PaperWrite-Bench module
│   ├── run_evaluation.py        ← Official PaperRecon wrapper
│   ├── setup.ps1                ← Clone + install
│   └── PaperRecon/              ← Official repo (after setup)
│
├── mlr-bench/                   ← MLR-Bench module
│   ├── run_evaluation.py        ← Official MLR-Bench wrapper
│   ├── setup.ps1
│   └── mlrbench/                ← Official repo (after setup)
│
├── scireplicate-bench/          ← SciReplicate-Bench module
│   ├── run_evaluation.py        ← Official wrapper
│   ├── setup.ps1
│   └── SciReplicate-Bench/      ← Official repo (after setup)
│
├── shared/
│   ├── config.py                ← API key / path config
│   └── env_check.py             ← Environment checker
│
└── reports/                     ← Evaluation reports (generated)
```

## Notes

- **PaperWrite-Bench**: Requires Docker or Pixi on Linux for full pipeline.
  On Windows, use the Python wrapper with API keys for partial evaluation.
- **MLR-Bench**: Pure Python, best Windows compatibility.
- **SciReplicate-Bench**: Requires Ubuntu + CUDA 12.2 + A100 GPU.
  On Windows, you can inspect the evaluation logic and data structure.
