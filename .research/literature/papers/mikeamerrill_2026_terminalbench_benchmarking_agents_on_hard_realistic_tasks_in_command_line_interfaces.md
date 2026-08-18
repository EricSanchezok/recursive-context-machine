# Terminal-Bench: Benchmarking Agents on Hard, Realistic Tasks in Command Line Interfaces

## One-line Thesis

Terminal-Bench 2.0: 89 个精选 CLI 终端环境中的硬任务,每个任务有独特环境、人工写的 solution、完整验证测试;frontier 模型得分 <65%。

## Problem / Gap

现有 benchmark 要么不测真实任务,要么不够难。缺少对 frontier agent 有区分度的 CLI 长 horizon 任务集。

## Method

89 tasks in computer terminal environments, inspired by real workflows。每个任务:独特环境 + human-written solution + comprehensive tests for verification。发布 dataset 和 evaluation harness。

## Key Results

- Frontier models and agents score **less than 65%** (GPT-5.2: 63%)
- 任务数: 89 个人工验证任务
- 附 error analysis 识别模型与 agent 改进方向

## Assumptions

- CLI 终端环境能代表真实 agent 长 horizon 工作负载
- 验证测试充分覆盖任务正确性

## Limitations / Failure Modes

- 任务数偏少(89),统计功效有限
- 单一环境类型(CLI),不覆盖 GUI/web

## Reusable Ingredients

- 官方 evaluation harness(可自动判定 pass/fail)
- 每任务验证测试套件
- 平均解决率(mean pass rate over ≥5 trials)指标约定

## Open Questions

- 是否需要更大规模 CLI 任务集?
- pass@1 vs pass^k vs 平均解决率的报告规范

## Connections

(auto-generated from edges.jsonl — do not edit manually)

## Relevance to This Project

**MoEH 主评测 benchmark**。提案目标: TB2 上 +5% (convincing +10%)。Phase A 数据收集也用 TB2 89 任务跑 RCM 轨迹。所有基线 (No Advisor / Linear / Dense / FoldGRPO / Advisor / MEM1 / Max Context) 与 MoEH 都在同一 TB2 test set 上对比。
