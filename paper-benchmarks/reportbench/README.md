<div align="center">
<img src="BandAI.png" alt="Logo" width="400">
</div>

<h1 align="center">ReportBench: Evaluating Deep Research Agents via Academic Survey Tasks</h1>

<div align="center">
<a href="https://www.apache.org/licenses/LICENSE-2.0"><img src="https://img.shields.io/badge/license-Apache%202-blue" alt="license"></a>
<a href="https://arxiv.org/abs/2508.15804" target="_blank"><img src=https://img.shields.io/badge/arXiv-b5212f.svg?logo=arxiv></a>
<a href="https://huggingface.co/datasets/ByteDance-BandAI/ReportBench" target="_blank"><img src=https://img.shields.io/badge/%F0%9F%A4%97%20Hugging%20Face-Dataset-blue></a>
</div>

<h5 align="center"> If you like our project, please give us a star ⭐ on GitHub for the latest update.</h5>


## Overview

**ReportBench** is a comprehensive benchmark for evaluating the factual quality and citation behavior of Deep Research agents. Leveraging expert-authored survey papers as ground truth, ReportBench reverse-engineers domain-specific prompts and provides automated tools to assess both cited and non-cited content.

ReportBench addresses this need by:

- **Leveraging expert surveys**: Uses high-quality, peer-reviewed survey papers from arXiv as gold-standard references.  
- **Reverse-engineering prompts**: Generates task-specific prompts matching each survey’s scope, methods, and temporal constraints.  
- **Automated validation**: Employs a dual-path evaluation to verify citation consistency and factual correctness of non-cited statements.
---

## Benchmark Construction

The dataset construction pipeline consists of four phases:

1. **Survey Paper Identification**  
   - Start from an arXiv metadata snapshot (post-2020).  
   - Filter titles/abstracts for “survey” or “review” and confirm publication status via metadata and LLMs classification.  
   - Retain 600 high-quality, peer-reviewed survey papers.

2. **Fine-Grained Reference Extraction**  
   - Download and parse LaTeX sources to extract all in-text citation commands.  
   - Build a gold-standard set of references mirroring the true citation pattern of each survey.

3. **Prompt Generation**  
   - Reverse-engineer three levels of prompts (sentence, paragraph, detail-rich) via LLMs.  
   - Enforce temporal constraints matching each paper’s publication cutoff.  
   - Add explicit instructions to avoid citing the original survey itself.

4. **Application Domain Distribution**  
   - Classify surveys into ten domains using LLMs.  
   - Perform downsampling while adjusting the distribution to achieve balance, and sample one of the three prompt types to form a 100-task benchmark.

---

## Evaluation Framework

ReportBench’s evaluation workflow consists of two complementary validation procedures:

### 1. Content Quality

- **URL Extraction**: Extract all URL citations from the report, including from the base model and Deep Research outputs.  
- **Normalization and Retrieval**: Normalize and deduplicate URLs, then retrieve the content of each web page.  
- **Document Type Classification**: Use an LLM to determine whether each URL corresponds to a scholarly article and extract its title if applicable.  
- **Title Matching**: Compare the extracted titles against ground-truth references from the expert-authored report and compute an overlap ratio.

### 2.1. Cited Statements

- **Statement Extraction**: Identify all sentences in a generated report containing explicit citations.  
- **Source Retrieval**: Scrape the full text of each cited source.  
- **Semantic Matching**: Use an LLM to locate supporting passages and verify consistency.  
- **Scoring**: Compute a citation alignment score for each report.

### 2.2. Non-Cited Statements

- **Statement Extraction**: Extract factual claims without citations, filtering out common-sense content.  
- **Web-Connected Fact Checking**: Query multiple web-connected LLMs (Gemini Pro and Flash) to independently verify each claim.  
- **Voting Mechanism**: Aggregate judgments via majority vote to compute factual accuracy.

---

## Evaluation Results

We evaluated two Deep Research products alongside their corresponding base LLMs using the ReportBench benchmark. Table 1 summarizes precision, recall, average references per report, citation match rate, cited statement count, non-cited factual accuracy, and non-cited statement count.

| Test Model                   | Precision | Recall | Avg Refs | Cit. Match Rate | Cit. Stmt Count | Non-Cit Acc | Non-Cit Stmt Count |
|------------------------------|----------:|-------:|---------:|----------------:|----------------:|-------------:|--------------------:|
| **OpenAI Deep Research**     |     0.385 |  0.033 |    9.89 |          78.87% |           88.2 |       95.83% |                38.9 |
| **Gemini Deep Research**     |     0.145 |  0.036 |   32.42 |          72.94% |           96.2 |       92.21% |                49.6 |
| gemini-2.5-flash             |     0.237 |  0.012 |    5.47 |          44.88% |           12.1 |       98.52% |                11.5 |
| gemini-2.5-pro               |     0.269 |  0.010 |    4.27 |          59.24% |            6.58|       96.08% |                 9.35|
| o3                           |     0.299 |  0.031 |   12.26 |          31.43% |           16.16|       82.22% |                11.51|
| claude4-sonnet           |     0.337 |  0.021 |    6.74 |          73.67% |           14.93|       92.64% |                17.07|

*Table 1. Performance metrics of Deep Research products and their base models.*


#### Product-Level  
- **OpenAI Deep Research**: Highest precision (0.385) and citation match rate (78.87%), indicating focused and accurate retrieval with fewer references.  
- **Gemini Deep Research**: Generates many more citations (32.42 vs. 9.89) but yields only marginal recall gain, suggesting over-generation without proportional coverage benefits.  

#### Model-Level  
- **OpenAI vs. o3**: Comparable retrieval metrics, but Deep Research produces far more cited statements (88.2 vs. 16.16) and non-cited statements (38.9 vs. 11.51), and achieves much higher alignment (78.87% vs. 31.43%) and accuracy (95.83% vs. 82.22%).  
- **Gemini Deep Research vs. gemini-2.5-pro**: Trades off precision (0.145 vs. 0.269) for higher recall and citation volume, while maintaining strong alignment (72.94% vs. 59.24%) but slightly lower non-cited statement accuracy.  
- **claude4-sonnet**: Most balanced baseline—moderate precision (0.337), recall (0.021), citation consistency (73.67%), and non-cited statement accuracy (92.64%).  

---

## 🛠️ Installation and Usage

### Prerequisites

**Environment Requirements:**
- Python 3.8+
- Required Python packages (install via pip)

**Install Dependencies:**
```bash
pip install pandas pyyaml langchain-openai tenacity tqdm requests pathlib beautifulsoup4 firecrawl-py python-dotenv
```

**API Keys Setup:**

```bash
# OpenAI / Compatible API Configuration
OPENAI_API_KEY=your_api_key_here
OPENAI_BASE_URL=https://api.openai.com/v1    # or any OpenAI-compatible endpoint
OPENAI_MODEL_NAME=gpt-4o-mini                # or deepseek-chat, etc.

# Web LLM (for full mode with web search)
WEB_LLM_KEY=your_web_llm_key                 # Gemini API key recommended
WEB_LLM_BASE_URL=                            # Web LLM endpoint
```

---

## 🔬 Simple Mode（单篇论文快速评测）

**Simple Mode** 是轻量化评测管线，支持：**输入 PDF/Markdown/LaTeX → 提取陈述 → 输出事实性报告**。

### 两种模式对比

| 特性 | `simple` 模式 | `full` 模式 |
|:----|:-------------|:-----------|
| 引用陈述提取 | ✅ | ✅ |
| 无引用陈述提取 | ✅ | ✅ |
| LLM 知识判断 | ✅（默认） | ✅ |
| 联网搜索验证 | ❌ | ✅ |
| URL 抓取对齐 | ❌ | ✅ |
| 需要 WEB_LLM_KEY | ❌ | ✅ |
| 运行时间 | ~5-8 分钟 | ~15-30 分钟 |

### Quick Start

```bash
# simple 模式（只需 OPENAI_API_KEY）
python evaluate_simple.py --pdf paper.pdf --mode simple

# full 模式（需要 WEB_LLM_KEY 支持联网搜索）
python evaluate_simple.py --pdf paper.pdf --mode full

# 从已有 Markdown 运行
python evaluate_simple.py --text paper.md --mode simple -o reports/result.json
```

### 使用 run_reportbench.ps1（推荐）

```powershell
# 简单模式：仅 LLM 知识判断，无需联网
.\run_reportbench.ps1 -Mode simple -Pdf paper.pdf

# 完整模式：联网搜索验证
.\run_reportbench.ps1 -Mode full -Pdf paper.pdf

# 跳过无引用验证（仅提取统计）
.\run_reportbench.ps1 -Mode simple -Pdf paper.pdf -SkipNonCited
```

### 输出示例

```json
{
  "benchmark": "ReportBench",
  "input_file": "ROCS.md",
  "summary": {
    "cited_statements": { "total": 178 },
    "non_cited_statements": {
      "total": 286,
      "verified_correct": 113,
      "verified_incorrect": 148,
      "accuracy_rate": 0.395
    }
  },
  "overall_verification_score": 1.98,
  "mode": "simple",
  "notes": "simple 模式: 基于 LLM 知识判断（无需联网）"
}
```

### 命令行参数

| 参数 | 说明 |
|:----|:-----|
| `--pdf` / `--markdown` / `--latex` | 输入文件 |
| `--text` | Markdown 文本（替代文件） |
| `--mode simple\|full` | 评测模式（默认 simple） |
| `--output, -o` | 输出 JSON 路径 |
| `--skip-non-cited` | 跳过无引用验证 |
| `--skip-url-verify` | 跳过 URL 抓取对齐 |
| `--work-dir` | 中间产物目录 |

### 环境变量

| 变量 | 必需 | 说明 |
|:----|:----|:-----|
| `OPENAI_API_KEY` | ✅ | LLM API 密钥 |
| `OPENAI_BASE_URL` | ❌ | API 端点（默认 https://api.openai.com/v1） |
| `OPENAI_MODEL_NAME` | ❌ | 模型名（默认 gpt-4o-mini） |
| `WEB_LLM_KEY` | full 模式 | 联网模型 Key |
| `WEB_LLM_BASE_URL` | full 模式 | 联网模型端点 |

---

## 📦 Project Structure

```
ReportBench/
|
├── 📁 ReportBench Release v1.1
│   ├── ReportBench_v1.1.jsonl      # Main dataset file in JSON Lines format
│   └── ReportBench_v1.1_GT         # Ground truth reference data
|
├── 📁 Core Processing Scripts
│   ├── openai_processor.py          # Process OpenAI Deep Research outputs
│   ├── gemini_processor.py          # Process Gemini Deep Research outputs  
│   ├── statement_evaluator.py       # Extract and evaluate factual statements
│   ├── related_work_evaluator.py    # Evaluate citation accuracy and recall
│   └── metrics_calculator.py        # Calculate final performance metrics
│
├── 📁 Configuration & Utilities
│   ├── config.py                    # API keys and model configurations
│   ├── utils.py                     # Common utilities (LLM clients, CSV ops)
│   ├── cache_utils.py               # URL caching and normalization
│   └── .env                         # Environment variables (create this)
│
├── 📁 Evaluation Modules
│   ├── statement/                   # Statement extraction and verification
│   │   ├── extract_citations.py    # Extract cited statements
│   │   ├── extract_no_citations.py # Extract non-cited statements
│   │   ├── scrape_content.py       # Web scraping for citation sources
│   │   ├── match_text.py           # Semantic matching of statements
│   │   ├── verify_alignment.py     # Verify citation-statement alignment
│   │   └── verify_no_citations_web.py # Web-based fact-checking
│   └── process/                     # Data processing utilities
│       ├── extract_activity_structured.py
│       ├── extract_reference_structured.py
│       └── html2markdown.py
│
├── 📁 Scripts & Templates
│   ├── run_test.sh                  # Main evaluation pipeline
│   ├── process_prompt.py            # Prompt processing utilities
│   └── prompt_template/             # Evaluation prompt templates
│
└── 📄 Configuration Files
    ├── README.md                    # Project documentation
    ├── .gitignore                   # Git ignore rules
    └── .env.example                 # Environment variables template
```

**Key Components:**

- **Processing Pipeline**: `openai_processor.py` → `statement_evaluator.py` + `related_work_evaluator.py` → `metrics_calculator.py`
- **Input Format**: Model outputs as JSON files with `response`/`content` field containing the generated survey text
- **Output**: Comprehensive evaluation metrics including citation alignment scores and factual accuracy rates

## Quick Start

### 1. Prepare Your Model Data

**Important Note:** The data preparation process differs based on your model type:

#### For OpenAI Deep Research & Gemini Deep Research (Web-based)
These are web-based products that require special data collection:

1. **Use Chrome Extension**: Since these are web interfaces, you need to use a Chrome extension to capture the conversation records
2. **Process with Dedicated Scripts**: Use the corresponding processor to parse the captured data:
   ```bash
   # For OpenAI Deep Research
   python openai_processor.py --input=captured_data_dir --output=parsed_output_dir --markdown
   
   # For Gemini Deep Research  
   python gemini_processor.py --input=captured_data_dir --output=parsed_output_dir
   ```

#### For Other Models (API/Local Models)
**Input Format:** Your model outputs should be saved as JSON files with the following structure:

```json
{
  "response": "Your model's generated survey text here...",
  "arxiv_id": "2024.12345",  // Optional: will be extracted from filename if not present
  "query": "Original query prompt",  // Optional
  // ... other metadata fields
}
```

**Alternative accepted field names for the main content:**
- `response`, `content`, `text`, `message`, `output`, `result`

**File Organization:**
```bash
# Create your model's output directory
mkdir -p your-model-name

# Save each evaluation result as: {arxiv_id}.json
# Example filenames:
your-model-name/2003.00653.json
your-model-name/2004.05937.json
# ... (100 files total for the full benchmark)
```

### 2. Run Evaluation Pipeline

```bash
# 1. Process raw outputs (ONLY for web-based Deep Research products)
# For OpenAI Deep Research:
python openai_processor.py --input=data/test_data/raw_data/your-model-name --output=your-model-name-parsed --markdown
# For Gemini Deep Research:
python gemini_processor.py --input=data/test_data/raw_data/your-model-name --output=your-model-name-parsed --markdown
# For other models: Skip this step, use your JSON files directly

# 2. Extract and evaluate statements
python statement_evaluator.py your-model-name-parsed --output-dir your-model-name-stat-results

# 3. Evaluate citation accuracy  
python related_work_evaluator.py --survey-dir your-model-name-parsed --ground-truth-dir ReportBench_v1.1_GT --result-dir your-model-name-related-work-results

# 4. Calculate final metrics
python metrics_calculator.py your-model-name-stat-results
```

### 3. View Results

Results will be saved in structured directories:
- **Statement Evaluation**: `your-model-name-stat-results/` - Contains citation alignment and factual accuracy scores
- **Related Work Evaluation**: `your-model-name-related-work-results/` - Contains precision/recall for citation discovery  
- **Final Metrics**: Summary CSV files with aggregated performance metrics

**Key Output Files:**
- Individual paper results in `{arxiv_id}/` subdirectories

### 4. HuggingFace Dataset (Optional)

You can easily access ReportBench with the following code:

```python
from datasets import load_dataset
dataset = load_dataset("ByteDance-BandAI/ReportBench")
```

## Citation

```bibtex
@misc{li2025reportbenchevaluatingdeepresearch,
      title={ReportBench: Evaluating Deep Research Agents via Academic Survey Tasks}, 
      author={Minghao Li and Ying Zeng and Zhihao Cheng and Cong Ma and Kai Jia},
      year={2025},
      eprint={2508.15804},
      archivePrefix={arXiv},
      primaryClass={cs.CL},
      url={https://arxiv.org/abs/2508.15804}, 
}
```
