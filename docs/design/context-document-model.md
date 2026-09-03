# RCM Context Document Model & Action Space v2.1 — 工具即通用 harness 面

> 状态: Implemented (2026-08-31, v2.1) · 取代 11-Action 原子动词层 · 服务 MoEH v5 (plan_003)
> 实现: C1 (cell store + 目录) → C2 (7 动词动作空间 + gRPC/proto/SDK) → C3 (工具三扩展 +
> drain-edits + context.compact/memory.* 工具族) 已全部落地于 feat/context-document-model。
> 决策链: timeline (物化/投影二分 → 12-move 词汇 → v5 转向 → 文档模型 → 工具化收敛)
> v2.1 关键修订: 按用户洞察, 内容生成与记忆取回从 Edit 内容源中移除——
> **一切非结构操作皆为工具**。动作空间收敛为 7 个动词, 表达力全部住进工具注册表。

## 0. 动机

两次收敛:
1. **v2**: 11 个原子动词 (磁带+槽位模型的产物) → 文档模型 + Edit(ops)。病因:
   Swap 零使用、宏层补丁、批量 O(n) 三症状同源。
2. **v2.1**: Edit 的专用内容源 (`llm()` / `recall()`) 与宏库是重复造轮子——
   工具已是本系统的通用扩展面 (R1 registry / MCP attach / ledger 全走此路)。
   压缩、记忆、一切非结构操作 = 工具调用。

## 1. 模型: Context = 有序 cell 文档 (不变)

```
Document := [ Cell ]*
Cell     := { anchor: "@name" | auto_id, role, tag, content,
              meta: { created_step, last_seen_step, bytes, source_completion } }
```

具名槽幂等语义 (`set("@x", ...)` 原地刷新); `@agent/@env/@purpose` 为保留脚手架。
obs.context_directory 携带每 cell 的 anchor/role/tag/bytes/created/last_seen/preview——
**工具与策略脚本共用的只读视图** (陈旧性判断的信号源)。
磁带只在 Halt 时被读取: 编辑是步骤计划, 无中间态一致性负担。

## 2. 动作空间 v2.1 (终版, 7 个动词)

```
Action :=
    Edit(ops, because?)                        // 结构性文档编辑
  | Tool(name, args, because?)                  // 调用任何注册工具 (策略发起)
  | Model(name) | Activate(name) | Deactivate(name)
  | Halt | Done

op      := set(anchor, content) | insert(after, content, anchor?)
         | delete(selector) | move(anchor, after)
content := literal(text, role, tag?) | inbox[call_id]     // 只剩这两种
selector:= anchor | range | 谓词 (role= · tag= · older_than · bytes_gt · ...)
```

- **Take 溶解**: inbox 消费 = `inbox[call_id]` 内容源 (比 FIFO 自由; call_id 寻址幂等可读)。
- **表达力住在工具注册表**: 动作空间是封闭小语法, 工具空间是开放大语义。
  这就是 Resource-OS 论点的逻辑终点——连上下文操作都是资源 (工具)。
- **because 注解** 随动作/工具调用落 WAL——规则级信用分配的原始材料。
- 安全: ops 与工具载荷逐条类型校验, 非法即 hitch; 保留槽删除保护在施加层执行;
  程序空间自由 × 出口空间封闭。

## 3. 工具侧的三个能力扩展 (皆有先例)

1. **读目录**: 工具执行环境注入只读 context 目录 (cell 元数据+预览) ——
   陈旧性/范围判断需要; ledger 已有同款只读环境注入先例。
2. **completion 助手**: ToolRuntime 向内容生成工具暴露一次模型调用
   (`completion(prompt, source)`), token 计入 WAL (显式计价不变)。
3. **结构化副作用**: 工具结果可携带 `{"edits": [...]}` / `{"mutations": [...]}` 载荷,
   fire loop 在 apply 后 drain 施加——**registry_events / ledger_transitions 已验证
   此模式**, 编辑载荷走同一通道并过 op 校验。

## 4. 工具家族 (harness 手段的落法)

| 家族 | 工具 | 说明 |
|---|---|---|
| 上下文自管理 | `context.compact(range_spec, style?)` · `context.pin(anchor)` · `context.archive(range, dest)` · `context.restore(ref)` | compact 内部一次 completion; archive 外置到 memory 资源; restore 取回 |
| 记忆 | `memory.write(key, content)` · `memory.search(query)` · `memory.format()` | 索引=memory 资源 (R2); search 结果落 inbox, 消费即物化 (或经 overlay 投影, 零租金) |
| 账本 | `ledger.*` (已有) | 迁移事件入轨迹 |
| 资源 | `resources.*` (R1 已有) | 运行时 CRUD / MCP attach |
| 委派 | `spawn` (已有) | 子代理扇出 |
| 通用 | 任何 MCP 工具 / builtin | usage-policy 门禁 |

## 5. 治理: 策展工具带 (proposes/disposes 的工具化)

**策略控制执行者能摸到哪些自管理工具** (Activate/Deactivate 即旋钮):

- 全开 → model-in-the-loop 自管理 (MemGPT 式): 执行者看 manifest 里有 compact,
  自己决定何时调;
- 全关 → 策略驱动 (synergy 式): 脚本经 Action::Tool 或纯结构 Edit 管理;
- 谱系中间任何点 → 混合。

「暴露哪些工具给执行者」本身是可演化决策 (v5 演化循环的搜索对象之一)。
12-move 词汇的最终归宿 = 工具家族的设计直觉, 不再是接口约束。

## 6. 两条压缩路径 (同一机制)

```
执行者主动: 模型 manifest 见 compact → ToolCall → 工具内 completion →
            edits 载荷 drain → 下次 Halt 见精简文档
策略主动:   Action::Tool("compact", range) → 同上; 或纯结构 Edit 零 LLM
```

谁发起/何时/是否授权 = 策略层决策。成本记账: 工具内 completion 的 token
计入该工具调用的 WAL 事件。

## 7. WAL 事件格式

> Schema 版本注记: WAL envelope = TrajectoryEvent { event: StoredEvent, obs, registry_events,
> drain_effects }; drain_effects 与 context_directory_total 为 v2.1 新增字段 (serde default,
> 旧 WAL 可读)。gRPC 侧: State.context_directory (field 20) 携带完整目录行,
> Fragment.anchor (field 8) 携带具名槽; ActionSpace 增加 document_outline/document_cells。
```
TrajectoryEvent { ..., event: StoredEvent {
    step, action: Edit{ops[...], because?} | Tool{name, args, because?} | ...,
    effects: [ CellSet/Inserted/Deleted/Moved · LastSeenUpdated
             · ToolCompleted{tokens_spent, edits_applied, because?} · ... ],
}}
```

一条 Tool 事件携带该次调用的 token 支出与施加的编辑——「程序自发学会抠门」
有直接账本; because 提供规则级归因。

## 8. 可见性规则

| 视角 | 看到 | 看不到 |
|---|---|---|
| Executor (Halt) | project(document, overlay) + 激活工具清单 | obs (除非 overlay 注入)、未消费 inbox |
| Policy 脚本 | obs + 目录 + inbox 深度 + step | cell 全文 (语义判断→显式调用内容工具, 计价)、挂钟 |
| 工具 | args + 只读目录 + (注册的) completion 助手 | 任意文件/网络 (按 capability 门禁) |
| 教练 (轮间) | report + WAL(含 because/成本) + 谱系 diff + 金丝雀 | test 流 ground truth |

刻意边界: 策略盲于全文 (语义判断付费且计价); obs 无时钟 (挂钟进 report);
脚本轮内冻结 (轮内自适应=代码自身逻辑)。

## 9. 模式库 (主流手段 → 工具/动作落法)

1. 全量摘要+丢弃: `Tool(compact, between(@purpose, recent(2)))` — 先写后删在工具内保证;
2. 滚动摘要: compact 的 style=rolling, 摘要槽幂等刷新;
3. 选择性修剪/硬截断: `Edit(delete(谓词))` — 零 LLM;
4. 外置+取回: `context.archive` + `memory.search` (或 restore), 取回可走 overlay 投影;
5. Reflection: overlay critic 人格 → Halt → `Edit(set(@reflection, inbox[...]))`;
6. Plan-and-execute: `set(@plan, ...)` + overlay planner 人格;
7. 预算自知: overlay 注入油表摘要 (零成本);
8. 失败换道: hitch 可见 → Model/Activate;
9. FSM: 策略脚本本身;
10. 结构化输出: response_format 模型变体资源 + Model();
11. Few-shot 包: exemplar 资源, 策略 set 换包;
12. 子代理/集成: spawn / 编译期图;
13. 记忆自管理 (MemGPT 全家): memory.* 工具带全开给执行者。

## 10. 迁移清单 (相对当前 main)

| 项 | 改动 |
|---|---|
| Context 内部 | cell store (anchor 索引+顺序+meta); last_seen 回写在请求组装 |
| Machine::apply | Edit 分支 (op 校验+施加); Tool 分支 (路由 ToolRuntime, 结果入 inbox); 移除 Take/旧 5 动词 |
| 工具环境 | 注入只读目录; completion 助手 (token 计量) |
| drain 通道 | edits 载荷解析与施加 (沿 registry_events 模式) |
| ActionSpace (gRPC) | 菜单=文档大纲+可用工具/操作枚举; ActionCommand 加 Edit/Tool 变体 |
| Captain | 移植为种子脚本 (E1 表达力等价基准) |
| hook 流 | cell effects 发射, 保持 snake_case 契约形状 |
| R2 顺带 | memory 资源 kind + memory.* 工具族 (紧凑实现) |

## 11. 开放问题

1. 工具内 completion 的预算护栏 (单工具调用 ≤2 次内嵌 completion?);
2. inbox 寻址: call_id (倾向, 幂等可读) vs 序号;
3. 旧 WAL 兼容: checkpoint 版本号 + 只读回放;
4. context.* 工具的 capability 命名 (与 usage-policy 对齐)。

## 12. 与既有工作关系

全部承重: obs/油表、overlay、ledger、registry (R1)、优化器回路、report、防泄漏纪律。
v2.1 后 R4-min 范围 = 文档模型 + 7 动词 + 工具三扩展 + context.*/memory.* 首批工具族;
Rhai ScriptedPolicy 建立在 `edit()/tool()` 两个白名单函数上——策略脚本的 API
面极小, 演化搜索的程序描述长度收益保持。
