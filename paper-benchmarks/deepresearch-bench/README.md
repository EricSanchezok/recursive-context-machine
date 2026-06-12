# DeepResearch-Bench

DeepResearch-Bench 是一个**研究报告质量评测**子模块，实现官方 DeepResearch Bench 论文的两大评估框架：

- **RACE** (Reference-based Adaptive Criteria-driven Evaluation with Dynamic Weighting) — 研究报告质量评估
- **FACT** (Framework for Factual Abundance and Citation Trustworthiness) — 信息检索与引用可信度评估

## 快速开始

### 前置条件

```powershell
# 1. 安装依赖
pip install requests tqdm pandas numpy PyMuPDF pymupdf4llm

# 2. 配置 API（任意 OpenAI 兼容 API）
$env:API_KEY = 'sk-...'
$env:API_BASE_URL = 'https://api.deepseek.com/v1'
$env:RACE_MODEL = 'deepseek-chat'
```

### 单篇评测（Simple Mode）

```powershell
# 需要一篇研究报告 + 一篇参考报告（RACE 需要对比基准）
.\run_deepresearch.ps1 -Pdf target_paper.pdf -Reference reference_paper.md

# 也支持 Markdown / LaTeX / 纯文本
.\run_deepresearch.ps1 -Markdown paper.md -Reference ref.md

# 如果不想跑 FACT（引用验证），加 -RaceOnly
.\run_deepresearch.ps1 -Pdf paper.pdf -Reference ref.md -RaceOnly
```

**如果未提供 `-Reference`，RACE 会用目标报告自身作为参考，分数将全为 0.5（无意义）。**

### 100 任务基准评测（Full Mode）

```powershell
.\setup.ps1
.\run_deepresearch.ps1 -Full -ModelName my_model
```

## 实际测试示例

以下是用本项目已有的数据运行的测试结果：

```powershell
# 目标: o4-mini AI 生成的 ICLR 2025 Workshop 论文
# 参考: 该 Workshop 的任务描述
python evaluate_simple.py `
    --pdf "..\mlr-bench\mlrbench\ai_scientist_v2_papers\o4-mini\iclr2025_verifai\iclr2025_verifai.pdf" `
    --reference "..\mlr-bench\mlrbench\tasks\iclr2025_verifai.md" `
    --race-only --report
```

**输出摘要：**

| 维度 | 目标分 | 参考分 | 归一化得分 | 说明 |
|------|:------:|:------:|:----------:|------|
| Comprehensiveness | 7.00 | 8.67 | 0.4468 | 参考报告覆盖面更广 |
| Insight/Depth | 7.00 | 7.33 | 0.4884 | 两者深度相近 |
| Instruction-Following | 8.67 | 8.00 | 0.5200 | 论文紧扣主题，略优于参考 |
| Readability | 6.67 | 8.67 | 0.4348 | 参考更简洁 |
| **Overall** | — | — | **0.4784** | |

> RACE 输出的是**归一化相对分数**（0-1），公式为 `target / (target + reference)`。
> 大于 0.5 表示目标优于参考，小于 0.5 表示落后于参考。

## 评测框架说明

### RACE — 研究报告质量评估

RACE 使用 **LLM-as-a-Judge** 方法，为每个研究任务动态生成评估准则并评分。

**流程：**
1. **生成维度权重** → 根据任务描述，LLM 给四个维度分配权重
2. **生成评估准则** → 每个维度动态生成 3 条细粒度准则
3. **对比评分** → LLM 对目标和参考报告逐条评分（1-10），输出归一化分数

**四个评估维度：**

| 维度 | 说明 |
|------|------|
| Comprehensiveness | 覆盖主题关键领域，无重要遗漏 |
| Insight/Depth | 深入分析原因、影响、趋势，提供有价值洞见 |
| Instruction-Following | 紧扣研究主题，直接回答问题 |
| Readability | 结构清晰、语言流畅、易于理解 |

**评分原理：**
- 原始分 1-10（LLM 在每个准则上分别给目标和参考打分）
- 归一化：`dim_score = target_avg / (target_avg + reference_avg)`
- 总分：`overall = sum(dim_score * dim_weight) for all dims`
- 值域 (0, 1)，> 0.5 优于参考，< 0.5 落后参考

### FACT — 引用可信度评估

FACT 自动提取论文中的引用，验证每个引用是否能被源文档支持。

**流程：** 引用提取 → 按 URL 去重 → 抓取源内容（Jina API） → 验证支持性

| 指标 | 定义 |
|------|------|
| Citation Accuracy | 支持的陈述数 / 已验证的总陈述数 |
| Effective Citations | 获得支持的陈述总数 |

## 配置参考

### 环境变量

支持任意 OpenAI 兼容 API。配置优先级：**CLI 参数 > per-judge 环境变量 > 全局环境变量**

| 变量 | 作用对象 | 示例 |
|------|---------|------|
| `API_KEY` | 全局（所有 judge 的 fallback） | `sk-...` |
| `API_BASE_URL` | 全局 API 地址 | `https://api.openai.com/v1` |
| `RACE_API_KEY` | 仅 RACE judge | `sk-...` |
| `RACE_API_BASE` | 仅 RACE API 地址 | `https://api.deepseek.com/v1` |
| `RACE_MODEL` | 仅 RACE 模型名 | `deepseek-chat` / `gpt-4o` |
| `FACT_API_KEY` | 仅 FACT judge | `sk-...` |
| `FACT_API_BASE` | 仅 FACT API 地址 | `https://api.openai.com/v1` |
| `FACT_MODEL` | 仅 FACT 模型名 | `gpt-4o-mini` |
| `JINA_API_KEY` | FACT 网页抓取（可选） | `jina_...` |
| `TEMPERATURE` | 所有 LLM 调用 | `0.0`（默认） |
| `MAX_TOKENS` | LLM 最大 token | `64000`（默认） |

### CLI 参数

```
评估控制:
  --race-only          仅运行 RACE 评估
  --fact-only          仅运行 FACT 评估
  --skip-race          跳过 RACE
  --skip-fact          跳过 FACT

模型选择（通用）:
  --model              所有 judge 模型名
  --api-base           所有 judge API 地址
  --api-key            所有 judge API 密钥

模型选择（单独指定）:
  --race-model         RACE judge 模型
  --race-api-base      RACE judge API 地址
  --race-api-key       RACE judge API 密钥
  --fact-model         FACT judge 模型
  --fact-api-base      FACT judge API 地址
  --fact-api-key       FACT judge API 密钥

输入/输出:
  --pdf / --markdown / --latex / --text    输入文件
  --reference / -r                          参考报告路径
  --task-prompt / -t                        研究任务描述
  --jina-api-key                            Jina API 密钥
  --output / -o                             输出 JSON 路径
  --report                                  同时生成 Markdown 报告
```

### 支持的服务商

| Provider | `API_BASE_URL` | 示例模型 |
|----------|---------------|---------|
| OpenAI | `https://api.openai.com/v1` | `gpt-4o`, `gpt-4o-mini` |
| DeepSeek | `https://api.deepseek.com/v1` | `deepseek-chat` |
| OpenRouter | `https://openrouter.ai/api/v1` | `anthropic/claude-sonnet-4` |
| Groq | `https://api.groq.com/openai/v1` | `llama-3.1-70b-versatile` |
| Together AI | `https://api.together.xyz/v1` | `mistralai/Mixtral-8x22B` |

## 常见问题

### Q: 为什么所有分数都是 0.5？

RACE 是对比式评估。你没提供 `--reference` 参数，代码用目标报告自身作为参考，`target / (target + target) = 0.5`。请提供一篇参考报告。

### Q: RACE 和 FACT 应该用什么模型？

- **RACE** — 推荐使用**强模型**（如 gpt-4o、deepseek-chat、claude-sonnet-4），它需要理解论文内容并做出细粒度判断
- **FACT** — 可以用**轻量模型**（如 gpt-4o-mini、deepseek-chat），任务相对简单

### Q: 如何让两个模型用不同的 API？

分别配置环境变量即可：

```powershell
# RACE 用 OpenAI
$env:RACE_API_KEY = 'sk-openai-...'
$env:RACE_API_BASE = 'https://api.openai.com/v1'
$env:RACE_MODEL = 'gpt-4o'

# FACT 用 DeepSeek（更便宜）
$env:FACT_API_KEY = 'sk-deepseek-...'
$env:FACT_API_BASE = 'https://api.deepseek.com/v1'
$env:FACT_MODEL = 'deepseek-chat'
```

### Q: RACE 的结果怎么解读？

RACE 输出归一化相对分数（0-1），表示目标相对于参考的表现：
- **> 0.5**：目标优于参考
- **≈ 0.5**：两者相当
- **< 0.5**：目标落后参考

原始分（1-10）会一并输出，供你进一步分析。

### Q: FACT 需要额外配置吗？

FACT 的网页抓取依赖 Jina AI（可选）。如果不配置 `JINA_API_KEY`，会尝试直接 HTTP 获取。如果也不成功，相关引用会被标记为 "unknown"。

## 项目结构

```
deepresearch-bench/
├── evaluate_simple.py       # 核心评估器（RACE + FACT）
├── generate_report.py       # Markdown 报告生成器
├── run_deepresearch.ps1     # 一键运行脚本
├── run_evaluation.py        # 统一 CLI 包装器
├── setup.ps1                # 环境设置（官方数据集下载）
├── config.py                # 配置文件
├── requirements.txt         # Python 依赖
├── README.md
└── reports/                 # 评测报告输出目录
```

## 引用

```bibtex
@article{du2025deepresearch,
  author = {Mingxuan Du and Benfeng Xu and Chiwei Zhu and Xiaorui Wang and Zhendong Mao},
  title = {DeepResearch Bench: A Comprehensive Benchmark for Deep Research Agents},
  journal = {arXiv preprint},
  year = {2025},
}
```
