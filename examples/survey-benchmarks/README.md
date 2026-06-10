# Survey Benchmarks

学术综述生成评估基准框架，完整实现三篇论文的评估方法。支持任意 OpenAI 兼容 API 的模型。

| Benchmark | 论文 | 评估内容 |
|-----------|------|----------|
| **SurveyBench** | [arXiv:2510.03120v2](https://arxiv.org/abs/2510.03120) | 内容质量 + 测验评估 |
| **DeepSurvey-Bench** | [arXiv:2601.15307v1](https://arxiv.org/abs/2601.15307) | 表层质量(40%) + 学术价值(60%) |
| **DeepScholar-Bench** | [arXiv:2508.20033v2](https://arxiv.org/abs/2508.20033) | 知识综合(40%) + 检索质量(30%) + 可验证性(30%) |

## 目录结构

```
survey-benchmarks/
├── run_full_pipeline.ps1       ← 一站式评估入口（PowerShell）
├── surveybench/prompt.txt      ← SurveyBench 评估 prompt
├── deepsurvey-bench/prompt.txt ← DeepSurvey-Bench 评估 prompt
├── deepscholar-bench/prompt.txt← DeepScholar-Bench 评估 prompt
├── datasets/
│   ├── SciReviewGen/           ← 参考综述数据集（1.09 GB）
│   └── DeepScholar/            ← DeepScholar 参考数据
├── evaluators/
│   ├── llm_client.ps1             ← LLM API 调用器（独立使用）
│   ├── quiz_evaluator.py          ← 测验评估流水线
│   ├── comparative_evaluator.py   ← 参考对比评估
│   ├── verifiability_evaluator.py ← 引用可验证性评估
│   └── nugget_evaluator.py        ← 文献重要性评估
├── loaders/
│   └── scireviewgen_loader.py  ← SciReviewGen 数据集加载器
└── README.md
```

## 快速开始

### 配置 API

**不硬编码 API key**，支持三种配置方式（优先级：CLI 参数 > 环境变量 > .env 文件）：

```powershell
# 方式1：环境变量（推荐日常使用）
$env:EVA_API_KEY = "sk-your-key"
$env:EVA_MODEL = "gpt-4"
$env:EVA_ENDPOINT = "https://api.openai.com/v1/chat/completions"

# 方式2：.env 文件（在项目根目录下创建）
echo "EVA_API_KEY=sk-your-key" > .env
echo "EVA_MODEL=gpt-4" >> .env
echo "EVA_ENDPOINT=https://api.openai.com/v1/chat/completions" >> .env

# 方式3：直接传参
.\run_full_pipeline.ps1 -SurveyPath survey.md -ApiKey "sk-xxx" -Model "gpt-4"
```

环境变量也可使用 `OPENAI_API_KEY`（与 OpenAI 工具链兼容）。

### 运行完整评估

```powershell
cd d:\RCM\examples\survey-benchmarks

# 一键全跑（7项评估）
.\run_full_pipeline.ps1 -SurveyPath "path/to/survey.md" -Topic "Your Topic"

# 跳过不需要的评估
.\run_full_pipeline.ps1 -SurveyPath survey.md -Topic "LLM" `
  -SkipQuiz -SkipComparative

# 使用不同模型和 endpoint
.\run_full_pipeline.ps1 -SurveyPath survey.md -Topic "LLM" `
  -ApiKey "sk-xxx" -Model "gpt-4" -Endpoint "https://api.openai.com/v1/chat/completions"
```

### 评估结果

输出到 `reports/{survey_name}_full_eval/` 目录：

```
reports/survey_full_eval/
├── 00_summary.md              ← 汇总报告（所有分数概览）
├── 01_surveybench_report.md   ← SurveyBench 内容评估
├── 02_deepsurvey_report.md    ← DeepSurvey-Bench 评估
├── 03_deepscholar_report.md   ← DeepScholar-Bench 评估
├── 04_quiz_report.md          ← 测验评估（需指定 Topic）
├── 05_comparative_report.md   ← 参考对比评估（需匹配数据集）
├── 06_verifiability_report.md ← 可验证性评估
└── 07_importance_report.md    ← 文献重要性评估
```

## 7 项评估详解

| # | 评估 | 类型 | 依赖 | 说明 |
|---|------|------|------|------|
| 1 | **SurveyBench** | LLM-as-Judge | 无 | 内容质量，6 维度加权评分（5分制） |
| 2 | **DeepSurvey-Bench** | LLM-as-Judge | 无 | 学术价值，Surface(40%) + Academic(60%) |
| 3 | **DeepScholar-Bench** | LLM-as-Judge | 无 | 研究综合，KS/RQ/V 三方面（百分制） |
| 4 | **Quiz** | 自动化流水线 | --Topic | 测验问答流水线 |
| 5 | **Comparative** | 数据集对比 | --Topic | SciReviewGen 对照 |
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
| **Quiz** | LLM 生成问题 → 提取段落 → 答案验证 | Answerability 分数 (0-100) |
| **Verifiability** | 正则提取声明 → 匹配引用 → LLM 验证 | Claim Coverage, Citation Precision |
| **Comparative** | SciReviewGen 检索 → Nugget 匹配 → ROUGE | Coverage Ratio |
| **Document Importance** | 提取参考文献 → 正则匹配引用数/会议 | Avg Importance (0-100) |

## 数据集

- **SciReviewGen**: 章节级综述数据集，10,000+ 篇计算机科学综述
  - 文件: `datasets/SciReviewGen/split_survey_df.pkl`（需手动下载）
  - 下载: `pip install gdown && gdown https://drive.google.com/uc?id=1S6v-xaCDND4ilK38sEpkfcOoMnffX7Zf`
- **DeepScholar**: 63 条 arXiv 相关工作章节，`datasets/DeepScholar/related_works_combined.csv`

## 与论文一致性

| Benchmark | 评估维度 | 权重体系 | 无参照评估 | 参考对比 | 测验评估 | 可验证性 | 评分制 |
|-----------|---------|---------|:--------:|:--------:|:--------:|:--------:|:-----:|
| SurveyBench | ✅ | ✅ 100% | ✅ | ✅ | ✅ 流水线 | N/A | 1-5 |
| DeepSurvey-Bench | ✅ | ✅ 40/60 | ✅ | ✅ | N/A | N/A | 1-5 |
| DeepScholar-Bench | ✅ | ✅ 40/30/30 | ✅ | ✅ | N/A | ✅ 自动化 | 0-100% |
