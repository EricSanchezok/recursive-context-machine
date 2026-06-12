# Paper Benchmarks

本目录是 RCM 项目的 **LLM 生成论文与研究报告评测管线**，包含 5 个 benchmark 子模块，覆盖从论文写作、代码复现到深度研究报告的完整评估链。

## 评测 Benchmarks

| # | Benchmark | 评估侧重点 | 评测方法 | 核心指标 |
|---|-----------|-----------|---------|---------|
| 1 | **PaperWrite-Bench** | 论文写作的呈现质量、幻觉程度、引用真实性 | LLM-as-Judge 逐部分评分 + 引用 F1 检查 | Rubric 1-5, 自洽性, 引用 F1 |
| 2 | **MLR-Bench** | 端到端机器学习研究的全流程质量（Idea → 实验 → 写作） | 多维 LLM 评审 + 幻觉检测 | Clarity/Novelty/Soundness/Significance 1-10 |
| 3 | **SciReplicate-Bench** | 从论文描述到算法代码的复现正确性 | 代码执行对比 + CodeBLEU + 推理图对齐 | CodeBLEU, Exec ACC, Recall, Graph ACC |
| 4 | **ReportBench** | 深度研究报告的事实正确性与引用质量 | 引用 URL 提取 → 网页抓取 → LLM 语义对齐 | Precision/Recall, 引用匹配率, 事实准确率 |
| 5 | **DeepResearch-Bench** | PhD 级深度研究报告的整体质量与引用可信度 | RACE（对比式自适准则评估）+ FACT（引用可信度验证） | RACE 0-1 四维归一化分, Citation Accuracy |

## 评估覆盖范围

| 论文生命周期阶段 | 覆盖的 Benchmark |
|-----------------|-----------------|
| 文献检索与综述质量 | ReportBench, DeepResearch-Bench |
| 研究 Idea 与 Proposal | MLR-Bench |
| 实验设计与执行 | MLR-Bench |
| 代码复现正确性 | SciReplicate-Bench |
| 论文写作质量 | PaperWrite-Bench |
| 论文整体评审质量 | MLR-Bench, DeepResearch-Bench |
| 引用可信度与事实性 | ReportBench, PaperWrite-Bench, DeepResearch-Bench |

## 快速使用

每个 benchmark 都提供了独立的 README 和运行方式。请参考对应目录获取详细使用说明：

```powershell
# 查看所有可用 benchmark
python run_evaluation.py --list

# 进入对应目录查看详细文档
#   paperwrite-bench/README.md      — 论文写作质量评测
#   mlr-bench/README.md             — ML 研究全流程评测
#   scireplicate-bench/README.md    — 算法复现评测
#   reportbench/README.md           — 深度报告事实性评测
#   deepresearch-bench/README.md    — 深度报告综合质量评测

# 统一运行入口
python run_evaluation.py --benchmark paperwrite --list
python run_evaluation.py --benchmark mlr --check
python run_evaluation.py --benchmark scireplicate --check
python run_evaluation.py --benchmark deepresearch --list
```

各 benchmark 均提供 **Simple Mode**（无需 ground truth，输入论文即可评测）和 **Full Mode**（需要标准答案，调用官方评测管线）。

## 目录结构

```
paper-benchmarks/
├── README.md                     ← 本文件，全局总览
├── run_evaluation.py             ← 统一 CLI 入口
├── run_full_pipeline.ps1         ← 一键全管线运行脚本
│
├── paperwrite-bench/             ← 论文写作质量
│   ├── README.md                 ← 详细使用说明
│   ├── evaluate_simple.py        ← Simple Mode 评估器
│   └── PaperRecon/               ← 官方 PaperRecon 仓库（setup 后）
│
├── mlr-bench/                    ← ML 研究全流程质量
│   ├── README.md
│   └── mlrbench/                 ← 官方 MLR-Bench 仓库（setup 后）
│
├── scireplicate-bench/           ← 算法复现正确性
│   ├── README.md
│   └── SciReplicate-Bench/       ← 官方 SciReplicate-Bench 仓库（setup 后）
│
├── reportbench/                  ← 深度报告事实性
│   ├── README.md
│   ├── evaluate_simple.py
│   └── statement/                ← 陈述提取、URL 抓取、语义对齐模块
│
├── deepresearch-bench/           ← 深度报告综合质量
│   ├── README.md
│   ├── evaluate_simple.py        ← RACE + FACT 评估器
│   └── reports/                  ← 评测报告输出
│
├── shared/
│   ├── config.py                 ← 全局配置（API key、路径）
│   └── env_check.py              ← 环境检查工具
│
└── reports/                      ← 全管线评测报告汇总输出
```

## 环境要求

- Python 3.10+
- Git
- **任意 OpenAI 兼容 API** 的密钥（通过 `API_KEY` 环境变量配置）

各 benchmark 的特定环境要求请参见对应子目录的 README。

## 引用

各 benchmark 的官方论文和仓库链接请参见对应子目录的 README。
