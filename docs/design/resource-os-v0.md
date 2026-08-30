# RCM Resource-OS Design v0 — 一切皆资源，除 Model 外

> 状态: Proposal (2026-08-30) · 依托: MoEH plan_001 平台弧线 + 「除 Model 外一切皆 harness」调研
> 决策链: `.research/timeline.jsonl` (Resource-OS 三条 insight + 本设计)
> 铁律: 不改 Machine 解释器语义,不膨胀 Action 枚举,MoEH 论文弧线(L3 词汇层)不依赖本设计。

## 0. 定位与证据

**命题**: 除了 Model,agent 系统的一切——工具、记忆、技能、prompt、policy、图、子代理契约、环境——都是同一种对象:**可被自进化 agent 在运行时 CRUD 的资源**。

文献证据(详见调研时间线):
- 边界溶解: AutoMem(记忆=技能) / MemSkill(记忆操作=可演化技能) / SOP 合成(工作流=工具) / MetaSkill-Evolve(技能改进技能) → 收敛为「持久行为工件」;
- 组合先例: AgentSquare MoLAS(四模块统一 IO + 组件级搜索) / MemoHarness(harness 六个可编辑控制维度);
- 类型化优于自由代码: Grammar Search 固定组件搜索 4/5 benchmark 胜 LLM 自由代码;
- 无界控制的反面证据: EvolveTool-Bench 自造工具 96.8% correctness=0(静默腐烂) → 自建工件必须有 conformance 门;
- 资源病实证: AgentRM 40k issues → 调度失败 + 上下文退化 → usage-policy 必须一等公民;
- 工业标准: MCP 三原语 + Agent Skills(SKILL.md + 渐进披露)。

**结论**: 自由控制的正确形态不是自由代码,而是 **manifest 约束的类型化空间 + 按证据爬升的自由度阶梯 + conformance 门**。

## 1. 核心抽象

```
Resource = ( manifest,          state,            interface,          usage-policy )
            描述·来源·约束       私有持久状态        对模型的调用面       预算·权限·可见性
```

### 1.1 Resource kinds(首版九种)

| kind | manifest 要点 | state | interface | 现有代码锚点 |
|---|---|---|---|---|
| tool | name/desc/schema/**source**(builtin\|mcp\|script\|composite)/provenance | 无(或工具自有) | ToolDefinition + executor | `ToolRuntime.executors`, `Catalog::register_tool`, MCP `registry.rs` |
| model | protocol/endpoint/limit/cost | 无 | 模型选择 | `Model`, `Catalog::register_model` |
| prompt | name/body/token-estimate | 可版本化 | prompts 槽 | `Resources.prompts`, `register_prompt` |
| skill | SKILL.md 风格: 意图+步骤+入口(渐进披露: ~100 tok 广告位) | 无 | activate→注入指令段 | `register_prompt` 泛化 |
| memory | 通道名+schema+保留策略 | **通道存储本体** | read/write/recall 工具族 | `ledger`(首个实例: ledger.json+迁移守卫) |
| subagent | 契约: 输入/result schema/超时 | 运行记录 | spawn 调用 | `SpawnTool`(items+max_parallel) |
| policy | 名+参数 manifest+**装载方式**(builtin\|grpc\|composite) | — | `Policy` trait | `register_policy`, gRPC `ReactPolicy` |
| graph | 节点(=已注册 primitive)+边(flux 通道) | — | graph 执行 | `Graph`, `.rcm` 编译期图 |
| environment | cwd/vars/root/platform | 快照 | env 片段 | `Environment`, `register_environment` |

关键洞见: **policy 和 graph 也是资源**——组合层(L2)本身可被 CRUD。`.rcm` 文件是这些资源的编译期序列化格式;Resource-OS 把同一结构搬到运行时。

### 1.2 与三层的对应

- L1 资源层 = 本设计(注册表 + CRUD + 预算);
- L2 组合层 = policy/graph 资源的组装(AgentSquare 的搜索空间);
- L3 词汇层 = 12 executive moves(MoEH 论文接口,**不变**)。

## 2. 架构: ResourceRegistry + drain-sync 桥

### 2.1 为什么 CRUD 走工具路线而非新 Action 族

- Action 枚举膨胀会破坏 ActionSpace 的可学习性(12 moves 的价值恰恰在封闭词汇);
- synergy 生产模式「模型写结构、代码守不变量」由工具天然满足(ledger 已验证);
- 工具调用已进 WAL effects,审计免费;
- 可见性开关已有一等公民: `Activate`/`Deactivate`/`Model` 三个 Action 保持不变,继续作为 policy 级决策。

### 2.2 数据流

```
resources 工具 (model 在 Halt 中调用)
   │  守卫校验(manifest schema + 迁移合法性 + conformance 门)
   ▼
ResourceRegistry (进程级存储, run_dir/resources/*.json 持久化, 静态表——ledger 同款模式)
   │  每次变更发 registry 事件(带 provenance/版本)
   ▼
fire() 循环 drain-sync (apply 后每步, ledger_digest 富化的同款挂点)
   │  pending 注册 → machine_state.run.resources.tool_definitions/models/prompts
   │                + tool_runtime.insert(executor)
   ▼
模型可见性仍由 policy 决定: Activate("x") / Deactivate("x") / Model("y")
```

工具拿不到 `&mut RunState`——这是特性不是缺陷: 注册表是真相源,RunState 是每步重放的投影,与 obs「派生而非存储」同一哲学。

### 2.3 运行时 MCP attach — L2 工具自由度的现成大门

`McpServerConfig`(Stdio/Http/Sse 三传输)已存在。resources 工具新增 op `attach_mcp`:
agent 用 shell 起一个小 MCP server(自己写的脚本),注册传输配置 → 既有 `tools_for`/`public_tool_name` 代码路径自动流入工具清单。**外部生态(MCP 市场)与自造工具共用同一扇门**,命名空间化(`label__tool`)防冲突。激活前置门: capability probe(空参调用清单)通过才进 tool_definitions。

## 3. CRUD 接口: `resources` 工具规范

ops: `register` / `update` / `retire` / `list` / `describe` / `attach_mcp` / `set_policy`(usage-policy 变更)。

守卫(代码守不变量,模型不可绕过):
1. **manifest schema 校验**——按 kind 严格校验,拒绝畸形;
2. **迁移合法性**——retire 需无 active 引用;update 的 schema 变更需过 conformance;不可覆盖 builtin 资源(只可 Deactivate);
3. **provenance 字段**——`seed`(人写) | `generated`(agent 造) | `evolved`(从 generated 晋升),决定自由度等级与门禁;
4. **命名空间**——agent 注册的资源强制前缀 `gen/`(如 `gen/my_scraper`),与人写资源隔离。

## 4. 自由度阶梯与 conformance 门

```
L0 组合     : 从注册表选择与连接(Activate/graph 组装)          门: 类型校验
L1 参数变异 : manifest 固定,变参数(threshold/budget/策略参数)   门: 参数域校验
L2 结构生成 : 类型化槽位内生成(script 工具/MCP attach/skill 文本) 门: capability probe + conformance suite
L3 自由代码 : ADAS/JIT 式完整程序                              门: validate-repair 循环 + 评审面板 + 沙箱预算
```

- 每个资源记录当前自由度等级;**升降级由证据驱动**: L0 组合被反复选中且成为瓶颈 → 解锁其参数进入 L1;L2 工具连续 N 次任务成功 → 候选晋升 seed(evolved)。
- **conformance suite 是 L2+ 的硬门**(EvolveTool-Bench 教训): 每个自造工具注册时必须附最小验证集(输入→期望输出对),注册与每次 update 时重放;失败即拒绝注册/回滚版本。suite 存 `run_dir/resources/<name>.conf.yaml`。
- L3 仅经 gRPC 外部策略或显式沙箱 manifest,默认关闭。

## 5. usage-policy 与预算(AgentRM 两大资源病的解法)

manifest 内嵌(缺省宽松,可被 set_policy 收紧):

```yaml
usage:
  calls: 50            # 单任务调用上限(BudgetedModel 模式,超限即 hitch)
  tokens: 200000
  timeout_ms: 60000    # 单次调用
  visibility: on_demand # always | on_demand(默认,渐进披露) | manual
  capabilities: [fs_read] # 声明式权限面,shell 类工具默认最严
```

- 执行点: `ToolRuntime.execute` 包一层预算计数器(调用数/超时已有 tokio timeout 先例);模型 token 预算挂 reactor;
- **渐进披露 = usage-policy 的 visibility 字段**: `on_demand` 的 skill/tool 只以 ~100 token 广告位出现在 obs 的资源摘要里,Activate 后才注入全文——Agent Skills 规范的同构实现;
- obs 扩展: `obs.resources_digest`(激活集/预算余量/广告位清单)——advisor 的下一个特征块。

## 6. 审计与轨迹集成

- 每次注册/更新/退役/晋升发 registry 事件,由 fire() 循环并入 TrajectoryEvent(新字段 `registry_events`),与 ledger_transitions 同构;
- conformance 结果(通过率/回滚)随事件落盘——自进化行为的完整取证链;
- 语义不变式: **工具调用 effects + registry 事件 + WAL = 自进化过程的可重放全史**。

## 7. 自进化循环: 三时间尺度 + 种子库(JIT 缺失的那半)

```
步级    : policy/advisor 选 12 moves                     (已有, MoEH)
任务级  : meta-agent 经 resources 工具配置注册表          (本设计 R1-R3)
          JIT 式生成 + 选择(防泄漏纪律: 选择只见 logprob/judge,
          不见 benchmark 分数; 低分不修,仅 exception 修)
跨任务  : 参数积累(advisor 权重) + 种子库生长
          晋升协议: generated →(连续成功+conformance 绿)→ evolved →(人审可选)→ seed
          seed 库 = harness_factory 的运行时孪生(11 手写种子的活版本)
```

这正是 JIT 宣称 "updates the archive" 但代码零实现的机制——**我们的差异化机会**。

## 8. 实施分期(每期独立可验证,直接复用 state-layer 的验证纪律)

| 期 | 内容 | 验收 |
|---|---|---|
| R1 | ResourceRegistry + resources 工具(list/describe/register-tool-via-attach_mcp) + drain-sync + registry 事件入轨迹 | agent 运行时经 MCP attach 扩自己的工具集并被 Activate;全程 WAL 可重放 |
| R2 | skill/memory kinds(prompt 泛化 + ledger 泛化为 memory 通道注册) + 渐进披露 visibility | skill 按需注入;第二记忆通道(如 scratchpad)零代码注册 |
| R3 | usage-policy 预算执行(ToolRuntime 计数器 + obs.resources_digest) | 超限 hitch 可复现;obs 出现资源摘要特征块 |
| R4 | policy/graph 资源化(参数 manifest + composite 组合 + gRPC 装载路径声明) | 运行时组装两 primitive 图并执行 |
| R5 | conformance harness + 晋升协议 + 种子库 | 自造工具带 suite 注册;连续成功晋升 evolved 入库 |

依赖: R1→R2→R3 可并行于 R4;R5 依赖 R1。全部不动 machine crate 公开语义。

## 9. 非目标

- 不替换 Machine 解释器,不新增 Action 变体(可见性永远走 Activate/Deactivate/Model);
- 不做无门禁的运行时代码执行(L3 默认关);
- **MoEH 论文不依赖本设计**(L3 词汇层自洽);Resource-OS 是平台弧线,可独立成文(对标 AIOS/AgentRM 线,差异化: 类型化资源 + 学习型控制器 + conformance 门);
- 权限/沙箱体系(目录级之外的)另立项,usage-policy 的 capabilities 字段仅声明不执行。

## 10. 开放问题

1. registry 的进程级静态表在多进程部署(多 gRPC server)下需文件锁或单写者约定——v0 单进程语义,文档化;
2. graph 资源的运行时执行复用 `Graph` 还是走「图即数据 + 逐节点 primitive fire」的更薄解释器——R4 定;
3. conformance suite 的生成责任(agent 自附 vs 系统自动从 usage 采样)——R5 定,倾向 agent 自附 + 系统补采。
