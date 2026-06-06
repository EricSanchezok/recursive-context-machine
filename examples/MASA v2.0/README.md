# MASA — Multi-Agent Survey Automation

MASA 实现了一个多 Agent 迭代式文献综述生成系统。核心思路不是让单一 LLM 直接输出长文，而是将综述写作拆解为可审计、可回滚、可并行的单元级 context machines：

1. **Researcher** 构建带细化准则的大纲与引用地图
2. **SectionPlanner** 将大纲拆解为逐 subsection 的写作计划
3. **Generator** 按计划逐 subsection 独立写作，再组装为完整草稿
4. **Supervisor** 进行全局一致性校验，并标记最需深化的 2-3 个 section
5. **7 个 LLM Judge** 从覆盖度、引用相关性、事实一致性、冗余度、引用均衡、章节均衡、分析深度 7 个维度并行评分
6. **Synthesizer** 加权汇总，决定迭代终止或继续
7. **Polisher** 最终润色定稿

## 设计原则

- **多 Agent 分工**：Researcher、SectionPlanner、Generator、Supervisor、Polisher、7 个 LLM Judge 各司其职，避免单一模型因追求全局协调而牺牲局部深度。
- **记忆库系统**：区分模型记忆库（全局共享：大纲、章节精炼总结、修改建议）与 Agent 记忆库（私有：关键词、工作状态、历史改动），确保上下文一致并模仿人类真实思考过程。
- **先建图，再写作**：复用 autoresearch 的 candidate pool、citation expansion、semantic expansion，所有中间产物落盘。
- **迭代驱动质量**：Judge Panel 从 7 个维度评分，加权汇总后决定 CONTINUE 或 STOP。Analysis Depth 权重最高（25%），以解决综述"广度有余、深度不足"的问题。
- **聚焦式迭代**：Supervisor 每轮识别最薄弱的 2-3 个 section，Generator 在后续迭代中只重写这些 section，而非全文重写。
- **Subsection 级独立生成**：SectionPlanner 生成逐 section 写作计划，Generator 按计划逐个 subsection 独立写作并落盘，最后拼接为完整草稿。每个 subsection 的上下文更聚焦，不会被其他 section 稀释。
- **每个单元都是独立 `.rcm` 文件**：可以单跑、复用，也可以由 end-to-end graph 串联。
- **每次运行写入新的 `runs/<utc timestamp>/`**，不会覆盖旧结果。

## 输入 Topic

入口从下面两个位置读取 topic，优先级从高到低：

1. 环境变量 `AUTORESEARCH_TOPIC`
2. 本地文件 `examples/MASA v2.0/topic.md`

如果两者都不存在，系统会使用一个 smoke-test topic。

示例：

```powershell
$env:DEEPSEEK_API_KEY="sk-..."
cd examples/MASA v2.0
.\run_masa.ps1 -MaxRounds 3
```

也可以单跑某个单元：

```powershell
cargo run --bin accelerate -- run "examples/MASA v2.0/rcm/core/researcher.rcm" --speed 0 --context
cargo run --bin accelerate -- run "examples/MASA v2.0/rcm/core/section_planner.rcm" --speed 0 --context
cargo run --bin accelerate -- run "examples/MASA v2.0/rcm/core/generator.rcm" --speed 0 --context
```

单跑中游单元时，它会优先读取上游 context 中的 `run_dir`；如果没有，就尝试使用最近一次 `runs/*` 目录。

## Pipeline

The context contract is documented in [schema/handoff.md](schema/handoff.md). Graph context only carries handoffs; full intermediate data lives in `run_dir` artifacts.

### Phase 0 — 论文检索与候选池构建（`masa_phase0.rcm`）

1. `anchor.rcm` — 读取 topic，生成 `SurveySpec`，创建 `run_dir`
2. `query_plan.rcm` — 将 `SurveySpec` 转成 typed query program
3. `discovery.rcm` — 并行运行 method、benchmark、survey、frontier scouts，产出初始 candidate pool
4. `expansion.rcm` — 执行 citation graph expansion 和 semantic neighbor expansion

### Core Loop — 迭代写作与精修（`masa_core.rcm`）

```
Researcher
  ↓ handoff
SectionPlanner       ← 根据 outline + candidate pool 生成 subsection 级写作计划
  ↓ handoff
Generator            ← 按 section_plan.md 逐 subsection 写作到 memory/subsections/，
  ↓ handoff            然后拼接为 05_draft.md
Supervisor           ← 全局一致性校验 + 识别最薄弱 sections → focus_sections.md
  ↓ handoff (7-way parallel)
JudgeCoverage
JudgeCitationRelevance
JudgeFactualConsistency
JudgeRedundancy
JudgeCitationBalance
JudgeSectionBalance
JudgeDepth           ← 评估分析深度（对比表、机制分析、Pros/Cons）
  ↓ context (merge 7→1)
JudgeSynthesizer     ← 加权汇总，计算总分，裁决 STOP/CONTINUE
```

5. `researcher.rcm` — 基于 candidate pool 生成结构化综述大纲，写入 `memory/outline.md`
6. `section_planner.rcm` — 分析 outline 和 candidate pool，为每个 subsection 生成写作计划（目标字数、关键论文、写作指令、深度级别），写入 `memory/section_plan.md`
7. `generator.rcm` — 按 `section_plan.md` 逐个 subsection 独立写作到 `memory/subsections/[id].md`，最后拼接为 `05_draft.md`
8. `supervisor.rcm` — 全局一致性校验，检查分析深度，标记最薄弱 2-3 个 section 到 `memory/focus_sections.md`。输出 `06_review.md` 与修改建议
9. 7 个 Judge 并行评估各维度：

   | Judge | 评估内容 |
   |-------|---------|
   | Coverage | 各 section 的论文覆盖度，是否有重要方法遗漏 |
   | Citation Relevance | 引用论文与内容的匹配度 |
   | Factual Consistency | 事实性错误、幻觉引用 |
   | Redundancy | 跨 section 内容重复度 |
   | Citation Balance | 引用在 section 间的分布均匀性 |
   | Section Balance | 各 section 篇幅与深度的均衡性 |
   | Analysis Depth | 分析深度——对比表、机制解释、Pros/Cons、定量结果 |

10. `judge_synthesizer.rcm` — 汇总 7 维评分（加权），计算总分，依据阈值与收敛规则裁决 STOP/CONTINUE

    评分权重：

    | 维度 | 权重 |
    |------|------|
    | Analysis Depth | 25% |
    | Coverage | 15% |
    | Citation Relevance | 15% |
    | Factual Consistency | 15% |
    | Redundancy | 10% |
    | Citation Balance | 10% |
    | Section Balance | 10% |

11. `polisher.rcm` — 当裁决为 STOP 时，润色语言风格、统一术语、优化衔接，输出最终定稿 `07_survey.md`

### 迭代控制

迭代由 [`run_masa.ps1`](run_masa.ps1) 负责：

- 若 Synthesizer 裁决 CONTINUE，脚本更新 `memory/iteration_state.md` 并触发下一轮
- Generator 在后续迭代中只重写 `focus_sections.md` 指定的 section，其余 section 保留
- 终止条件：CONVERGED（改进 < 0.15）、CRITICAL_ISSUE（单维度 < 3.0）、MAX_ROUNDS、QUALITY_OK（总分 ≥ 4.3）

### PDF 导出

运行结束后，可用 `export_pdf.js` 将最终稿导出为 PDF：

```powershell
node export_pdf.js runs/<timestamp>/07_survey.md
```

需要安装 Puppeteer：`npm install puppeteer`

## Run Artifacts

Phase 0 产物：
- `phase0/00_survey_spec.md` — 综述规范
- `phase0/01_query_plan.md` — 查询计划
- `phase0/02_candidate_pool.md` — 候选论文池
- `phase0/03_expansion.md` — 扩展结果

Core Loop 产物：
- `memory/outline.md` — 结构化大纲
- `memory/outline_history.md` — 大纲变更历史
- `memory/section_plan.md` — Subsection 级写作计划
- `memory/section_summaries.md` — 章节精炼摘要
- `memory/subsections/*.md` — 各 subsection 独立文件
- `memory/focus_sections.md` — 需深化的 section 列表
- `memory/supervisor_notes.md` — 修改建议
- `memory/judge_report_round[N].md` — 各轮 Judge 报告
- `memory/judge_suggestions.md` — Judge 综合建议
- `memory/judge_*.md` — 各 Judge 维度评估
- `memory/iteration_state.md` — 迭代状态
- `memory/agent_*.md` — Agent 记忆日志
- `05_draft.md` — 当前草稿
- `06_review.md` — Supervisor 评审报告
- `07_survey.md` — 最终定稿

这些文件是 runtime artifacts，默认不进入 git。
