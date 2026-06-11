# SciReplicate-Bench

Evaluates LLMs on **algorithmic reproduction from research papers** — can an LLM
correctly re-implement an algorithm based solely on a paper's description?

**Official repo**: [xyzCS/SciReplicate-Bench](https://github.com/xyzCS/SciReplicate-Bench)
**Paper**: [SciReplicate-Bench: Benchmarking LLMs in Agent-driven Algorithmic Reproduction](https://arxiv.org/abs/2504.00255)

---

## ⚠️ Important: SciReplicate-Bench is Different

Unlike [PaperWrite-Bench](../paperwrite-bench/) and [MLR-Bench](../mlr-bench/),
this benchmark **does not evaluate paper writing quality**.

| | PaperWrite-Bench | MLR-Bench | **SciReplicate-Bench** |
|:--|:----------------:|:---------:|:----------------------:|
| **Evaluates** | Writing quality | Research quality | **Code correctness** |
| **Input** | Paper (PDF/LaTeX) | Paper (PDF/MD) | **Generated code** |
| **Output** | Rubric 1-5 | Scores 1-10 | **CodeBLEU + Execution ACC** |
| **Has Simple Mode?** | ✅ Yes | ✅ Yes | ❌ **No** |
| **Runs on Windows?** | ✅ Yes | ✅ Yes | ❌ **Ubuntu + CUDA required** |

SciReplicate-Bench is a **code execution benchmark**. It requires running the
generated code in 36 paper-specific conda environments and checking if outputs
match. This cannot be approximated by an LLM-as-judge.

---

## What You Need to Provide

This benchmark does **not** accept a PDF or paper as input. Instead, it requires:

### The Official Benchmark Dataset (auto-downloaded)

The `setup.ps1` clones the official repo which includes:
- `Data.json` — 100 tasks × (paper abstract, reference code, API dependencies, test scripts)
- `Benchmark/` — 36 paper code repositories (each with its own conda environment)
- Core code: `SciReproducer.py`, `Evaluation.py`, `utils/`

### To Run SciReproducer (code generation)

```
# Nothing extra needed — uses the benchmark dataset
.\run_scireplicate.ps1 -Run -Model gpt-4o-mini
```

This generates code files under `Result/{task_id}/SciReproducer_{model}/`.

### To Evaluate (code execution + metrics)

**Required infrastructure** (cannot run on Windows):

| Requirement | Reason |
|:------------|:-------|
| **Ubuntu 20.04+** | Conda environment scripts use bash |
| **CUDA 12.2** | Many paper environments need GPU |
| **A100 (80GB)** | Official benchmark GPU spec |

```bash
# On Ubuntu only:
bash SciReplicate-Bench/scripts/env.sh /path/to/root
conda activate scireproducer
python Evaluation.py --metric execution_ACC --model gpt-4o-mini --root_path /path/to/root
```

---

## What You Can Run on Windows

On Windows you can only run the **code generation** step and **static metrics**
(CodeBLEU, Reasoning Graph ACC). Execution ACC and Recall require Ubuntu.

```powershell
# ✅ Code generation (runs on Windows)
.\run_scireplicate.ps1 -Run -Model gpt-4o-mini

# ✅ CodeBLEU (syntax-based, no execution needed)
.\run_scireplicate.ps1 -Metric CodeBLEU_Score

# ✅ Reasoning Graph ACC (structure-based, no execution needed)
.\run_scireplicate.ps1 -Metric ReasoningGraph_ACC

# ❌ Execution ACC (requires Ubuntu + CUDA)
.\run_scireplicate.ps1 -Metric execution_ACC
# ❌ Recall (requires Ubuntu)
```

---

## Setup

```powershell
# Clone official repo (auto-downloads benchmark data)
.\setup.ps1

# Check setup
python run_evaluation.py --check
```

After setup, the directory structure will be:

```
scireplicate-bench/
├── SciReplicate-Bench/            # Official repo (cloned by setup.ps1)
│   ├── Data.json                  # 100 tasks with reference code
│   ├── Benchmark/                 # 36 paper code repos
│   ├── SciReproducer.py           # Dual-agent code generator
│   ├── Evaluation.py              # All 4 metrics
│   └── scripts/env.sh             # Conda env setup (Ubuntu only)
├── Result/                        # Generated code output
├── run_evaluation.py              # Python wrapper
├── run_scireplicate.ps1           # One-click runner
├── setup.ps1                      # Setup script
└── README.md                      # This file
```

---

## Usage (Windows)

```powershell
# Run code generation + all metrics
.\run_scireplicate.ps1

# Run code generation only
.\run_scireplicate.ps1 -Run -Model gpt-4o-mini

# Run a specific static metric
.\run_scireplicate.ps1 -Metric CodeBLEU_Score
.\run_scireplicate.ps1 -Metric ReasoningGraph_ACC

# Custom GPU (for Execution ACC on Linux)
.\run_scireplicate.ps1 -Metric execution_ACC -GpuId 1
```

---

## Output

Results are stored in two places:

| Location | Contents |
|:---------|:---------|
| `scireplicate-bench/reports/scireplicate_summary.md` | Summary report |
| `scireplicate-bench/Result/{task_id}/SciReproducer_{model}/` | Generated code per task |

### Metric Descriptions

| Metric | Range | Description | Runs on Windows? |
|:-------|:-----:|:------------|:----------------:|
| **CodeBLEU** | 0-100 | Syntactic + semantic similarity to reference code | ✅ Yes |
| **Execution ACC** | 0-1 | Whether generated code passes test cases | ❌ No (Ubuntu) |
| **Recall** | 0-1 | Coverage of required APIs/functions | ❌ No (Ubuntu) |
| **Reasoning Graph ACC** | 0-1 | Whether algorithm reasoning path matches | ✅ Yes |

---

## Architecture

```
                  SciReplicate-Bench Pipeline
                  ──────────────────────────

  Task (paper abstract + reference code)
         │
         ▼
  ┌─────────────────────────────┐
  │     SciReproducer.py        │
  │  ┌───────────────────────┐  │
  │  │  Paper Agent           │  │  Reads paper, extracts algorithm
  │  └──────────┬────────────┘  │
  │             │               │
  │  ┌──────────▼────────────┐  │
  │  │  Code Agent            │  │  Generates implementation
  │  └───────────────────────┘  │
  └─────────────┬───────────────┘
                │
                ▼
         Generated Code (.pkl)
                │
                ▼
  ┌─────────────────────────────┐
  │     Evaluation.py           │
  │  ├── CodeBLEU               │  Static (Windows OK)
  │  ├── Reasoning Graph ACC    │  Static (Windows OK)
  │  ├── Execution ACC          │  Dynamic (needs Ubuntu)
  │  └── Recall                 │  Dynamic (needs Ubuntu)
  └─────────────────────────────┘
```

---

## References

- [SciReplicate-Bench: Benchmarking LLMs in Agent-driven Algorithmic Reproduction](https://arxiv.org/abs/2504.00255)
- [Official Repo](https://github.com/xyzCS/SciReplicate-Bench)
