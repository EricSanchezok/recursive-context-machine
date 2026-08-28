# RCM State Layer Design v1 — obs / view / ledger

> 状态: **Implemented** (2026-08-29, branch `feat/state-layer`) · 依托: MoEH idea_001
> 决策链: `.research/timeline.jsonl` (insight×3 + decision×2) · 工程契约:
> Blueprint note `nte_048dc0694001qw7breUlAbspUK`
> P3 (LLM manager kill test) 按用户决策延后; P1/P2/P4/P5 已实现并验证。
>
> 铁律: 本设计只做最小增量,不改 Machine 执行语义,不破坏 Captain 现有行为。

## 0. 背景与目标

MoEH 要训练一个 1M 参数 advisor 替换 harness 的手工上下文启发式。前置条件有三个,
本设计一次补齐:

1. **可学习观察空间** — policy 每步必须能看到预算量规(budget)等工作状态,否则
   「何时压缩」这类决策无从学起;
2. **决策轨迹可回放** — 每步 (obs, action, effects) 必须落 WAL,这就是 BC/GRPO 的
   训练数据单元;
3. **kill test 通道** — 在训练任何小模型之前,先用 LLM context-manager 验证
   「上下文操纵有增益」这个前提,失败则止损 363 GPU-hours。

## 1. 状态模型定案(本轮讨论结论)

### 1.1 三层模型: 三个读者,三个投影

```
Machine State S_t   机器全量真相 (RunState + MachineFrame + telemetry + WAL 内部)
   │
   ├─ obs(s_t)  → controller observation  policy/advisor 的感官 (POMDP 语义)
   ├─ view(s_t) → executor view           Halt 时经投影构建的请求窗口
   └─ WAL       → auditor projection      完整事件流,训练数据与审计
```

### 1.2 六元组(签名稳定,不再动)

```
s_t = (context, inbox, purpose, env, resources, obs)
obs = { budget, ledger_digest, overlay_status, ... }   ← 嵌套聚合,只长内部
```

### 1.3 消歧义钉子(写作与实现的统一词表)

| 通道 | 一句话 | 生命周期 |
|---|---|---|
| inbox | 未读的输出队列(episodic) | FIFO,Take 即消失 |
| context | 已消费的历史(tape,executor 可见) | 持久,付 token 租金直到 Remove |
| obs | 常设传感器(budget 量规/账本摘要/overlay 清单) | 每次读取重新派生,零租金 |
| overlay | 投影声明(policy 写,Halt 时消费成 view) | 声明持久,内容易失 |
| ledger | 会话外的持久结构化状态(模型经工具写,代码守不变量) | 状态迁移,compaction 不碰 |

**物化 vs 投影**: 进上下文有两个机制 — 物化(push,成为 fragment,可编址可回放,
每回合付租金) vs 投影(pull,请求构建时从 durable 通道派生,零历史污染)。
RCM 现状只有物化(`completion.rs:64` 只编码 fragments);synergy 以投影为主。
obs 是感知面,overlay 是投影面,二者共同补上「投影」这一族能力。

**论文用语警示**: agent 文献的 "observation" 默认指环境观测(tool output),在我们的
体系里它以 fragment 形式存在于 inbox/context。写作时必须钉死:
obs = controller observation over harness state。

## 2. 分阶段设计

依赖关系: P1 → P2 → P3(kill test 门) → P4/P5(通过后或并行)

```
P1 obs+budget ──► P2 WAL 轨迹 ──► P3 LLM manager ──► [GATE: kill test]
                                          │通过
                                          ▼
                              P4 overlay 投影   P5 ledger 账本
                                    (advisor 特征增强)  (GRPO 中间信号)
```

### P1 — obs 通道 + budget 量规(最小改动,一切的前置)

**改动点**(全部在 machine/accelerator 两 crate,不触 proto 以外的东西):

1. 新文件 `crates/machine/src/obs.rs`:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Budget {
    /// active model 的 limit.context;无 active model 时为 0
    pub context_limit: u64,
    /// system+messages+tools 三段启发式估算(chars/4,同 synergy bytes/4 口径)
    pub estimated_input: u64,
    /// limit × 0.85 (SOFT_THRESHOLD_RATIO,synergy 生产值)
    pub soft_threshold: u64,
    /// saturating_sub(limit, estimated_input)
    pub headroom: u64,
    /// 上一回合 API 实测 input tokens(telemetry 最新 CompletionRecord)
    /// 有值时策略应优先信它 — synergy prompt-budgeter 的自校正设计
    pub last_actual_input: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Obs {
    pub budget: Budget,
    // P5 填充: pub ledger_digest: Option<LedgerDigest>,
    // P4 填充: pub overlay_status: OverlayStatus,
}
```

2. `PolicyView` 加一个字段(`policy.rs:77-82`):

```rust
pub struct PolicyView<'a> {
    pub run: &'a RunState,
    pub inbox: &'a Inbox,
    pub step: u64,
    pub status: MachineStatus,
    pub obs: &'a Obs,          // 新增
}
```

构造点全仓仅两处: `accelerator/src/accelerator.rs:193`(主循环) +
`accelerator/tests/captain.rs:87`(测试)。爆破半径已核实,很小。

3. 估算器: 纯函数 `obs::measure(run: &RunState) -> Obs`,在 accelerator 主循环
   `decide()` 之前调用(每次 decide 重新派生 — obs 不落 MachineState,天然新鲜,
   不参与序列化)。成本 O(fragments 文本长度求和),微秒级。
   calibration 源: `usage.rs` 的 `CompletionRecord.tokens`(reactor 每次返回
   usage 已入 telemetry),取最新一条的 input。

4. proto 扩展(`proto/rcm.proto`): `State` 消息加 `Obs obs`,含 `Budget` 子消息。
   外部控制器(gRPC Step 循环)由此看到 budget — P3 的 LLM manager 若走进程外
   路线就靠它。SDK 重新生成(`sdks/python/generate.sh`)。

**兼容性**: Captain 不读 obs,行为零变化。serde 全 default,WAL 旧数据可读。

**验证**: `tests/obs_budget.rs` — 估算器对构造 context 的单调性(追加 fragment
→ estimated_input 不减)、headroom 饱和、calibration 优先级;PolicyView 编译通过
即接口闭合。`cargo nextest run -p machine -p accelerator`。

**对 MoEH**: advisor encoder 的第一个数值特征块 = budget 5 维。

### P2 — WAL 轨迹接线(训练数据落地)

**现状**: storage crate 完整(WAL/checkpoint/restore/replay)但零运行路径引用。

**改动点**:

1. 新模块 `crates/accelerator/src/trajectory.rs`(或 CLI 侧): 定义轨迹信封

```rust
pub struct TrajectoryEvent {
    pub step: u64,
    pub action: StoredEvent,      // 复用现有 (step, action, effects)
    pub obs: Obs,                 // P1 产物,决策时的 obs 快照
}
```

在 accelerator 主循环 apply 之后配对写入 `storage::Store`(machine crate 仍不
感知 obs — Machine::apply 签名不变,信封在调用方组装)。

2. CLI `run` 默认把 WAL 落到 `--run-dir`(rcm_run_dir 已导出);结束时 checkpoint。
   server 侧 `MachineManager` 的 Run 增加 per-machine WAL 句柄(Open 起录,
   Destroy 收尾)— 修复「重启即失」的同时就是数据收集。

3. resume 本阶段不做(replay_effects 已存在,后续按需接)。

**验证**: 集成测试 — 跑一个 SeqPolicy 小循环,断言 WAL 文件可 restore 且逐事件
等于内存轨迹;obs 快照与决策顺序一致。复用 storage crate 现有测试模式。

**对 MoEH**: Phase A 数据收集格式就此冻结: `(obs_t, action_t, effects_t)×T`。

### P3 — LLM context-manager + KILL TEST(决策门)

**Manager policy**(进程内实现,零架构改动 — Policy trait 本来就是可插拔大脑):

```rust
// crates/accelerator/src/policy/manager.rs
// .rcm: policy = "manager" 即启用;catalog register_policy("manager", ...)
pub struct Manager { /* 决策模型配置 + 解析退避 */ }
```

`decide()` 流程: 序列化决策 prompt → 调决策模型(独立于 executor 的 model槽,
经 resources 第二个 model 或专用配置) → 严格 JSON 解析 `{verb, ...}` → 构造
Action。自由文本 Append 天然支持(`Action::Append(Fragment)` 接受任意内容;
gRPC 路径的 `ActionCommand.fragment` 同样自由文本,ActionSpace 菜单只是建议
不是约束)。

**决策 prompt 输入**(v1):
- purpose 全文 + obs(budget 5 维)
- context 目录: 每 fragment 的 {id, role, tag, 长度, 首 80 字符摘要}(全文不进
  — manager 自己也是 token 消费者)
- inbox 深度、step、status
- 动作菜单(ActionSpace 子集)

**护栏**: 解析失败重试 ≤2 → 降级 `Take`(若 inbox 非空)或 `Halt`;
manager 调用每步 ≤1 次;manager 模型与 executor 模型用量分开入 telemetry。

**KILL TEST 矩阵**(TB2 子集 ≥30 tasks × 3 seeds,复用 `benchmarks/` 框架):

| 变体 | 说明 | 检验什么 |
|---|---|---|
| C0 captain | 现状基线 | — |
| H1 机械修剪 | 0.85 阈值触发 Remove 最旧 tool-result(保护最近 2 回合) | synergy 式启发式的 RCM 移植 |
| M1 manager-full | 全部 11 动作 | 自由度上限 |
| M2 manager-restricted | 仅 Remove/Append/Halt | 结构性动作子集是否够 |

**指标**: success rate、avg input tokens @Halt、$/task、turns、(M1−C0) 的
配对差。**Gate**: M1 或 M2 相对 C0 与 H1 ≥ +3pp success,或同 success 下
token 成本 −20% → 进 advisor 阶段;全部打平/更差 → 证伪止损,结论照样写进
idea_001(负结果也有价值)。

### P4 — overlay 投影层(通过门后做;零租金注入)

**机制**: Policy trait 加默认方法(不扩 Action 枚举,不破 WAL 语义):

```rust
fn overlay(&self, view: &PolicyView) -> Overlay { Overlay::default() }

pub struct Overlay {
    pub system_prefix: Vec<String>,  // 稳定内容,吃缓存(synergy L1-L2.5 位)
    pub tail: Vec<String>,           // 易变内容(synergy lateSystem 位)
}
```

消费点: Halt 时 reactor 构建请求,`view = encode_context(fragments)` 前后包
overlay(`completion.rs` 请求组装处);overlay 声明进 P2 轨迹信封(审计完整),
投影内容不进 WAL(每次重派生)。**缓存经济学落位规则照抄 synergy**: 前段稳定
→ 设 cache breakpoint,易变内容靠后。

Captain 的 tagged-fragment 幂等 Replace(env 槽)证明「固定槽位」也能用物化模拟,
overlay 是它的零租金版本 — 两条路并存,policy 自选,这本身又是一个可学习决策
(物化 vs 投影的权衡进入动作空间)。

### P5 — ledger 账本(结构性状态;GRPO 中间信号源)

**机制**(synergy 路线: 工具 + 投影策略,不新增 Action 族):

1. 工具 `ledger`: 读写 run_dir 下 JSON;schema `LedgerEntry { id, title, status,
   deps, result, updated_at }`;status 迁移表代码校验(pending→running→
   completed/failed,非法迁移拒绝)。
2. obs.ledger_digest: 状态计数摘要 + 当前节点(P5 填 Obs 预留字段)。
3. overlay 边界槽: 委派/子图场景投 `result` 级内容(4096/16384 截断预算默认值,
   但截断量本身暴露给 policy 可调)。
4. **迁移事件进轨迹信封** — 节点完成时间戳即 GRPO 的中间奖励锚
   (idea_001 弱点 #6 的对策)。

远期(不在 v1): 运行时可写子图 — agent 作者依赖边、spawn 执行,编译期 .rcm 图
与运行时账本统一为同一引擎。这是 RCM 独有身份,单独立项。

## 3. 非目标(v1 明确不做)

权限/审批系统、shell 沙箱、subagent 身份体系、skills/插件、gRPC 流式 watch —
见前轮差距分析,按需另立项。本设计只解决「状态-观察-投影-轨迹」四件事。

## 4. 风险与开放问题

| 风险 | 缓解 |
|---|---|
| chars/4 估算偏差(中文/CJK 尤甚) | calibration 优先(synergy 同策略);后续可换 tokenizer,接口不变 |
| PolicyView 加字段是破坏性变更 | 全仓仅 2 个构造点,一次改完;外部 SDK 走 proto 不受影响 |
| manager 每步 LLM 调用抬成本/延迟 | 目录式摘要(非全文)输入;仅决策点调用;kill test 本身就量测这笔账 |
| 多 system message 在 Anthropic/Gemini 的编码差异 | rig 层处理;P4 加 golden 测试锁三协议编码 |
| ActionSpace 菜单不含自由 Append,外部控制器困惑 | 菜单是建议非约束;proto 文档注明;P3 文档同步 |
| obs 派生成本在超长 context 下线性增长 | 求和是 O(N) 常数极小;若成瓶颈再加增量缓存 |

**开放问题**:
1. 决策模型用哪个槽位配置(`.rcm` 第二 model?专用 `manager {}` 块?)— P3 动工
   前定,倾向 DSL 加 `manager` 块保持显式;
2. H1 保护窗口(最近 2 回合)的回合定义在 RCM 里如何映射(step?completion?)—
   用 completion 边界,与 telemetry 对齐;
3. 轨迹信封是否需要 fragment 内容全文(BC 需要吗)— P2 先存摘要 + run_dir 里
   已有完整 hook 流,全文回放可由 WAL+hooks 重建,避免双写膨胀。

## 5. 决策记录

- 2026-08-22 状态组判定: obs(旧义: 环境观测副本)冗余 → 缺 ledger/budget(timeline insight #1)
- 2026-08-22 物化/投影二分 + view 投影模型(insight #2)
- 2026-08-22 DAG 本体论: 第三类「会话外持久结构化状态」(insight #3)
- 2026-08-22 obs 定名决策: obs = 聚合感知命名空间(decision #2)
- 2026-08-28 本文档: P1-P5 分阶段落地方案
