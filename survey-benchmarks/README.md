# Survey Evaluation Toolkit

学术综述生成评估工具集，实现 SurveyBench / DeepSurvey-Bench / DeepScholar-Bench 三篇论文的评估方法。支持任意 OpenAI 兼容 API 的模型，**无硬编码 API key**。

| Benchmark | 论文 | 评估内容 |
|-----------|------|----------|
| **SurveyBench** | [arXiv:2510.03120v2](https://arxiv.org/abs/2510.03120) | 内容质量 + 测验评估 |
| **DeepSurvey-Bench** | [arXiv:2601.15307v1](https://arxiv.org/abs/2601.15307) | 表层质量(40%) + 学术价值(60%) |
| **DeepScholar-Bench** | [arXiv:2508.20033v2](https://arxiv.org/abs/2508.20033) | 知识综合(40%) + 检索质量(30%) + 可验证性(30%) |

## 特性

- **开箱即用** — 无需下载数据集即可运行全部评估（5 个内置 fallback 主题）
- **两种模式** — Benchmark 模式（预定义 quiz / 可复现）+ Exploratory 模式（动态生成）
- **无硬件码** — API key 通过 `EVA_API_KEY` 环境变量 / `.env` 文件 / CLI 参数传入
- **模块化** — 7 项评估可独立运行，也可一键全跑

## 目录结构

```
survey-benchmarks/
├── run_full_pipeline.ps1       ← 一站式评估入口（PowerShell）
├── download_datasets.ps1       ← 数据集下载（交互式）
├── surveybench/prompt.txt      ← SurveyBench 评估 prompt
├── deepsurvey-bench/prompt.txt ← DeepSurvey-Bench 评估 prompt
├── deepscholar-bench/prompt.txt← DeepScholar-Bench 评估 prompt
├── test_sets/
│   └── surveybench_topics.json ← 内置测试集（5 个主题 + 预定义 quiz + 参考文本）
├── evaluators/
│   ├── llm_client.ps1             ← LLM API 调用器
│   ├── quiz_evaluator.py          ← 测验评估流水线
│   ├── comparative_evaluator.py   ← 参考对比评估
│   ├── verifiability_evaluator.py ← 引用可验证性评估
│   └── nugget_evaluator.py        ← 文献重要性评估
├── loaders/
│   └── scireviewgen_loader.js  ← SciReviewGen 数据集加载器
└── README.md
```

## 快速开始

### 配置 API

```powershell
# 设置 API key（必须）
$env:EVA_API_KEY = "sk-your-key"

# 可选自定义（有合理默认值）
$env:EVA_MODEL = "gpt-4"
$env:EVA_ENDPOINT = "https://api.openai.com/v1/chat/completions"
```

也可在项目根目录创建 `.env` 文件，或在 CLI 直接传参。

### Benchmark 模式（可复现）

```powershell
# 使用内置测试集的预定义 quiz 和参考文本
.\run_full_pipeline.ps1 -SurveyPath "survey.md" -Topic "Graph Neural Networks" `
  -TestSet test_sets/surveybench_topics.json
```

内置 5 个测试主题：

| ID | 主题 | 预定义 quiz | 参考文本 |
|----|------|:----------:|:--------:|
| llm | Large Language Models | ✅ 5 题 | ✅ |
| gnn | Graph Neural Networks | ✅ 5 题 | ✅ |
| cv | Deep Learning for Computer Vision | ✅ 5 题 | ✅ |
| recsys | Recommender Systems | ✅ 5 题 | ✅ |
| rag | Retrieval-Augmented Generation | ✅ 5 题 | ✅ |

提供 `-TestSet` 时：
- Quiz 评估使用**预定义问题**，消除 LLM 自生成导致的不可复现性
- Comparative 评估使用**内置参考文本**进行对比

### Exploratory 模式（方便）

```powershell
# 不带 -TestSet，Quiz 用 LLM 动态生成
.\run_full_pipeline.ps1 -SurveyPath "survey.md" -Topic "Your Topic"
```

### 跳过部分评估

```powershell
.\run_full_pipeline.ps1 -SurveyPath survey.md -Topic "LLM" `
  -SkipQuiz -SkipComparative
```

### 评估结果

输出到 `reports/{survey_name}_full_eval/` 目录：

```
reports/survey_full_eval/
├── 00_summary.md              ← 汇总报告（所有分数概览）
├── 01_surveybench_report.md   ← SurveyBench 内容评估
├── 02_deepsurvey_report.md    ← DeepSurvey-Bench 评估
├── 03_deepscholar_report.md   ← DeepScholar-Bench 评估
├── 04_quiz_report.md          ← 测验评估
├── 05_comparative_report.md   ← 参考对比评估
├── 06_verifiability_report.md ← 可验证性评估
└── 07_importance_report.md    ← 文献重要性评估
```

## 7 项评估详解

| # | 评估 | 类型 | 依赖 | 说明 |
|---|------|------|------|------|
| 1 | **SurveyBench** | LLM-as-Judge | 无 | 内容质量，6 维度加权评分（5分制） |
| 2 | **DeepSurvey-Bench** | LLM-as-Judge | 无 | 学术价值，Surface(40%) + Academic(60%) |
| 3 | **DeepScholar-Bench** | LLM-as-Judge | 无 | 研究综合，KS/RQ/V 三方面（百分制） |
| 4 | **Quiz** | 自动化流水线 | --Topic 或 -TestSet | 测验问答流水线 |
| 5 | **Comparative** | 数据集/LLM对比 | --Topic 或 -TestSet | 参考综述对比 |
| 6 | **Verifiability** | 自动化 | 无 | 声明提取+引用匹配 |
| 7 | **Document Importance** | 自动化 | 无 | 基于引用数/会议等级 |

### LLM-as-Judge 评估（1-3）

三个 benchmark 的 prompt 均按论文定义实现：
- **SurveyBench**: AW(15%) + AC(15%) + SG(25%) + CI(25%) + RR(10%) + NE(10%)，5分制
- **DeepSurvey-Bench**: SQ1+SQ2+SQ3(40%) / AV1+AV2+AV3(60%)，5分制
- **DeepScholar-Bench**: KS(40%) / RQ(30%) / V(30%)，百分制

使用 `temperature=0.0` 保证可复现性。

### 自动化评估（4-7）

| 模块 | 方法 | 输出 |
|------|------|------|
| **Quiz** | 预定义或 LLM 生成问题 → 提取段落 → 答案验证 | Answerability 分数 (0-100) |
| **Verifiability** | 正则提取声明 → 匹配引用 → LLM 验证 | Claim Coverage, Citation Precision |
| **Comparative** | 参考文本提取 Nugget → 对比覆盖度 | Coverage Ratio |
| **Document Importance** | 提取参考文献 → 正则匹配引用数/会议 | Avg Importance (0-100) |

## 数据集

评估工具集**无需下载数据集即可运行**（内置 5 个 fallback 主题）。下载仅用于严格的 Comparative 评估：

```powershell
# 交互式下载
.\download_datasets.ps1

# 或手动下载
pip install gdown
gdown "https://drive.google.com/uc?id=1S6v-xaCDND4ilK38sEpkfcOoMnffX7Zf" `
  -O "datasets/SciReviewGen/split_survey_df.pkl"
```

| 数据集 | 体积 | 用途 | 必须？ |
|--------|:----:|------|:------:|
| SciReviewGen | 1.09 GB | Comparative 参考对比 | ❌ |
| DeepScholar | 50 KB | 内置占位 | ❌ |

## 已知限制

1. **Quiz 动态生成问题不可复现** — 使用 `-TestSet` 加载预定义问题消除此问题
2. **Comparative 无数据集时回退到 LLM 自评** — 不影响其他评估
3. **Citation Precision 需 LLM 验证** — 无 LLM 时显示 N/A
4. **SciReviewGen 数据需手动下载** — 内置 5 个 fallback 主题

## 与论文一致性

| Benchmark | 评估维度 | 权重体系 | 无参照评估 | 参考对比 | 测验评估 | 可验证性 | 评分制 |
|-----------|---------|---------|:--------:|:--------:|:--------:|:--------:|:-----:|
| SurveyBench | ✅ | ✅ 100% | ✅ | ✅ | ✅ 流水线 | N/A | 1-5 |
| DeepSurvey-Bench | ✅ | ✅ 40/60 | ✅ | ✅ | N/A | N/A | 1-5 |
| DeepScholar-Bench | ✅ | ✅ 40/30/30 | ✅ | ✅ | N/A | ✅ 自动化 | 0-100% |
