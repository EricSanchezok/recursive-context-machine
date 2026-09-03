# RCM Optimizer Loop Protocol — 外环 harness 优化回路

> 状态: v1 (2026-08-31) · 依托: resource-os-v0.md R1 + run report 接口
> 场景: 一个外部优化 agent(如 synergy)在 RCM 上跑 benchmark,按分数与日志反馈持续优化 harness。

## 0. 回路总览

```
┌─ Optimizer (synergy / 任何 LLM agent / 脚本) ─────────────────┐
│  1. 写/改 .rcm harness 文件(策略、工具集、prompts、graph)      │
│  2. 发起 run: accelerate run harness.rcm --run-dir <iter_dir>  │
│     --label <stable-label> [--stream]                          │
│  3. 读反馈: <iter_dir>/report.json (+ stream 事件流 + WAL 轨迹)│
│  4. 按 report 决定下一轮 harness 修改 → 回到 1                 │
└───────────────────────────────────────────────────────────────┘
```

回路的三条硬边界:

1. **优化器在 run 之外**——它不是 RCM 内部的一个 policy,而是外部驱动者;
2. **反馈面是文件与流**——`report.json` 是每轮的固定锚点,`--stream` 是实时观测,`trajectory/` WAL 是全量取证;
3. **harness 修改发生在轮间**——这是 JIT 式生成时优化;运行期自进化走 `resources` 工具(另一条正交轴,见 resource-os-v0.md)。

## 1. 优化器接口(暴露给外环的东西)

### 1.1 触发一次 run

```bash
accelerate run harness.rcm \
  --run-dir runs/iter-007/task-17 \   # 反馈落盘目录
  --label tb2-iter07-task17 \          # 跨轮稳定标识,进 report.json
  --purpose "<task instruction>" \     # 覆盖 .rcm 内的 purpose
  --stream                             # 可选:实时 JSON lines 到 stdout
```

### 1.2 反馈面(run 结束后必读)

`<run_dir>/report.json` — schema `rcm.run.report/v1`:

| 字段 | 内容 | 优化器用途 |
|---|---|---|
| `label` | 调用方给的稳定标识 | 跨轮关联同一 task/配置 |
| `purpose` | 任务预览(截断 2000 chars) | 确认跑的是想跑的 |
| `answer` | 最终 assistant 输出(截断 8000 chars) | 送 benchmark 评分器 |
| `steps.completions` / `steps.tool_calls` | LLM 轮数 / 工具调用数 | 效率信号 |
| `cost.input_tokens` / `output_tokens` / `total_tokens` / `wall_ms` | 成本 | cost-matched 对比轴 |
| `artifacts.trajectory_dirs[]` | 每机器 WAL 目录 | 深挖失败:逐决策回放 |
| `artifacts.registry` | `resources/registry.json`(agent 自注册资源时) | 自进化取证 |
| `artifacts.ledger` | `ledger.json`(用了 ledger 工具时) | 任务进展信号 |

### 1.3 深挖面(按需)

- **`--stream` 事件流**(冻结契约,portal-gateway 同款):completion_end 带 input/output tokens、outcome、failure_kind、retryable——实时失败模式识别;
- **`trajectory/` WAL**:`TrajectoryEvent { step, obs, ledger_transitions, registry_events, event }`——`(obs, action, effects)` 逐决策回放,失败归因到步级;
- **`resources/registry.json`**:agent 在 run 内自注册的资源清单(manifest + provenance)。

## 2. Benchmark 评分的接线方式

RCM 不评卷——评分归 benchmark 的官方 evaluator(harbor/TB2 模式):

```
optimizer → accelerate run (--run-dir, --label) → report.json
                                                      │ answer
                                                      ▼
optimizer → benchmark evaluator (harbor 等) ────→ 分数
                                                      │ score + report
                                                      ▼
optimizer 修订 .rcm → 下一轮
```

report.json 的 `answer` 字段就是送评载体;分数由优化器自己维护在它的迭代账本里(与 RCM 解耦)。

## 3. 防泄漏纪律(继承 JIT,硬约束)

1. **harness 选择/修订不得见 benchmark 分数之外的测试集信息**;
2. 修订循环里可用: report.json(成本/步数)、stream 失败模式、WAL 轨迹、**当轮**得分;
3. 修订循环里不可用: 测试集 ground truth、其他任务分数、评测器内部状态;
4. 低分不触发"针对该任务的特化修复"重跑(防 benchmark 过拟合)——失败修复仅针对 exception(hitch/panic),与 JIT 的 repair 纪律一致。

## 4. 迭代账本约定(优化器侧建议)

每轮一个 `runs/iter-N/` 目录,跨轮聚合靠 `--label` 命名规范:

```
runs/
  iter-001/ task-00/ (report.json, trajectory/, ...)
           task-01/ ...
  iter-002/ ...        # harness 修订后重跑同一 task 集
  summary.csv          # 优化器自维护: label, score, tokens, wall_ms
```

## 5. 与运行期自进化的关系(两条正交轴)

| 轴 | 时机 | 机制 | 接口 |
|---|---|---|---|
| **本协议(轮间)** | run 之间 | 优化器改 `.rcm` / prompts / 工具集 | report.json + stream + WAL |
| **运行期(轮内)** | run 之中 | agent 经 `resources` 工具 CRUD 自己的 harness | registry.json + registry_events 入轨迹 |

两轴共用同一套取证(trajectory WAL),信用分配时可区分「优化器做的改动」与「agent 自己做的改动」(registry_events 只来自后者)。
