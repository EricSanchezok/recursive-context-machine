# RCM 开发指南

这份文档面向**第一次进入 RCM 仓库的开发者**，假设你写过 Rust，并且大致知道现在的 LLM agent harness（Claude Code、Cursor、AutoGPT、LangChain agents、OpenAI Assistants 之类）是怎么回事——但**不**假设你看过 RCM 的形式化定义。

读完之后你应该能：

- 把 workspace 跑起来，看到 context tape 动画;
- 知道每个 crate、每个模块在做什么，跟你熟悉的 LLM harness 概念怎么对应;
- 理解 Context Machine 这个模型为什么这样设计、它解决了传统 agent loop 的哪些痛点;
- 能独立完成"加工具 / 改策略 / 改模型 / 编排多 agent"四类常见改动;
- 知道哪些坑现在还在，避免上来就踩。

**阅读建议**：从头读到尾。每一节都为下一节铺垫，跳着读会卡。

如果你想看更短的背景资料：

- [README.md](README.md) —— 项目定位和模块总览，一页篇幅;
- [AGENTS.md](AGENTS.md) —— 命名 / 注释 / 测试 / 提交的硬规矩，所有提交都要遵守;
- [crates/machine/DESIGN.md](crates/machine/DESIGN.md) —— 形式化数学定义。本文 §4 是它的"通俗讲解 + 工程动机"版本，先读本文再回看 DESIGN.md 会更顺。

---

## 1. 环境准备

### 1.1 Rust 工具链

workspace 用的是 `edition = "2024"` 和 `resolver = "3"`，建议用 stable 最新版（1.85+）。

```sh
rustup update stable
rustc --version   # 至少 1.85
```

没有特殊的 `rust-toolchain.toml`，跟着 stable 走即可。

### 1.2 外部依赖（可选）

| 用途 | 工具 | 安装 |
|------|------|------|
| `find` 工具的 `ast` 模式 | [ast-grep](https://ast-grep.github.io/) (`sg`) | `brew install ast-grep` 或 `cargo install ast-grep --locked` |

`fs` / `find files` / `find text` / `shell` / `add` / `wait` 都不依赖外部二进制，开箱即用。

> 注：`Cargo.toml` 里目前还声明着 `tree-sitter-*` 系列依赖，但源码并未使用，`ast` 模式是 fork 出去调 `sg` 二进制。这些依赖会被在后续清理中删掉，跟你写代码无关。

### 1.3 LLM 凭据

默认模型走 SII 平台的 GPT-4.1（见 [`crates/accelerator/src/model.rs`](crates/accelerator/src/model.rs)），需要：

```sh
export SII_API_KEY=sk-...
```

如果你要本地跑通流程但不想真打 API，看 [§6 测试](#6-测试) 里 `SeqPolicy` 的 mock 写法。

---

## 2. 一分钟跑通

```sh
git clone <repo> RCM && cd RCM
cargo build --release
export SII_API_KEY=...
cargo run --release --bin accelerate -- run "用 fs 工具列出当前目录"
```

你会在终端里看到一条**彩色磁带**实时滚动——这是 RCM 最有标志性的视觉化：

```
╭─ Context Tape ──────────────────────────────╮
│ ■ ■ ■ ■ □ □                                 │
│       ⚙ insert system/text #5 cwd: /Users/. │
╰─ 5 cells · 1 tools · 0.3s ──────────────────╯
```

每个方格（"cell"）代表 LLM 对话历史里的一条消息，齿轮 ⚙ 是当前操作的位置（"写头"）。颜色表示消息的角色和类型：

| 颜色 | 含义 |
|------|------|
| 深蓝 | System prompt（系统提示） |
| 绿 | User（用户输入） |
| 白 | Assistant 文本（模型回复） |
| 黄 | Tool call（模型发出的工具调用） |
| 青 | Tool result（工具执行结果） |
| 红 | Hitch / 移除中（错误 / 正在删除的格子） |

**为什么要做这个动画**：传统 LLM harness 里 context 是个看不见的字符串，调试时只能 dump 整段 JSON。RCM 把"agent 每一步对 context 做了什么"实时画出来——插入了一条 user、替换了 system prompt、调用了工具、把工具结果塞了回去——所有改动都可见、有序、有 id。这种透明度是 RCM 整个设计的核心，等读到 §4 你会理解为什么。

常用 flag：

```sh
cargo run --bin accelerate -- run "..." --speed 0           # 关掉动画延迟，看完整轨迹
cargo run --bin accelerate -- run "..." --context           # 跑完打印整条 context，而不只是最后一条 assistant 文本
cargo run --bin accelerate -- run "..." --format json       # JSON 输出
RUST_LOG=info cargo run --bin accelerate -- run "..."       # 看 tracing 日志（stderr）
```

---

## 3. 项目结构

```
crates/
├── machine/        # 核心：Context Machine 原语 + reactor + LLM dispatch
│   ├── DESIGN.md     # 形式化定义（Φ, π, ω, Action 空间）
│   ├── src/
│   │   ├── lib.rs        # 公共 re-export 入口
│   │   ├── fragment.rs   # Fragment / Role / Content / Text / ToolCall / ...
│   │   ├── context.rs    # Context —— 可寻址、可改写的"磁带"
│   │   ├── inbox.rs      # Inbox —— reactor 与 policy 之间的 buffer
│   │   ├── policy.rs     # Policy / Phase / Action / PhaseOutcome
│   │   ├── machine.rs    # 驱动循环 Machine::run
│   │   ├── reactor.rs    # ω：跑 LLM + 执行工具 + 推 inbox
│   │   ├── completion.rs # LLM dispatch（OpenAI / Anthropic / Gemini）
│   │   ├── resources.rs  # Resources —— 工具 / 模型 / prompt 池 + 激活态
│   │   ├── env.rs        # Environment —— cwd / vars / root / platform
│   │   ├── purpose.rs    # Purpose —— 只读的"意图"
│   │   ├── tool.rs       # Tool trait
│   │   ├── model.rs      # Model / Protocol / Limit / Cost / Modality
│   │   ├── event.rs      # 给 hook! 用的 preview / role_name / content_kind
│   │   └── hook.rs       # hook! 宏 —— 把应用事件打成 tracing event
│   └── tests/
│       ├── common.rs       # SeqPolicy + test_resources（写测试用）
│       ├── machine.rs      # Action 行为测试
│       ├── context.rs / fragment.rs / inbox.rs / tool.rs
│
├── accelerator/    # 内置工具 + 默认策略 + 多 agent 图
│   ├── src/
│   │   ├── lib.rs
│   │   ├── accelerator.rs  # 单 agent：Accelerator + AcceleratorRef + Port/Channel
│   │   ├── state.rs        # State —— purpose+ctx+env+policy+res 五元组
│   │   ├── graph.rs        # Graph —— 多 agent 拓扑构建器
│   │   ├── assembly.rs     # Assembly —— 编译后可运行的图
│   │   ├── flux.rs         # Flux —— 无状态合并节点（Concat/Append/Overlay/Merge）
│   │   ├── model.rs        # 默认模型 gpt-4.1
│   │   ├── policy/
│   │   │   ├── captain.rs    # 默认单 agent Policy
│   │   │   └── phases/       # BootstrapAgent / InjectPurpose / InjectEnv
│   │   ├── tools/          # fs / find / shell / add / wait
│   │   ├── prompts/        # captain.txt
│   │   └── logging.rs      # （目前未被调用，预留）
│
└── cli/            # accelerate 二进制
    ├── src/
    │   ├── main.rs
    │   ├── args.rs       # clap 解析
    │   ├── cmd/run.rs    # 入口：起 tokio runtime、init tracing、跑 graph
    │   ├── hook/mod.rs   # 把 tracing event(target="hook") 转成 HookEvent 枚举
    │   └── output/
    │       ├── tape.rs   # 终端动画（核心 UI）
    │       ├── text.rs   # 普通文本输出
    │       └── json.rs   # JSON 输出
```

**先扫一眼就行，不用细看**。下一节 §4 会带你把每个文件的职责重新过一遍，到时候这张图会变得更有意义。

**读代码建议顺序**（等你读完本指南，想真正动源码时）：
1. `machine/src/fragment.rs` → `context.rs` → `inbox.rs` —— 数据结构层;
2. `machine/src/policy.rs` → `machine.rs` → `reactor.rs` —— 控制流层;
3. `accelerator/src/policy/captain.rs` + `policy/phases/` —— 看真实 Policy 怎么写;
4. `accelerator/src/tools/fs/` 或 `tools/find/` —— 看比较复杂的工具长什么样;
5. `accelerator/src/graph.rs` + `assembly.rs` + `flux.rs` —— 多 agent 怎么编排;
6. `cli/` —— 用户接口，最后看。

---

## 4. 架构

这一章是文档的核心。它会从你**熟悉的 LLM harness 工程概念**出发，一点点搭出 RCM 的整个模型——每个新概念都建立在前一节的基础上，每个设计决策都有清晰的动机。

读完之后你应该能：

- 用 LLM harness 工程的通用词汇理解 RCM 的每个模块在做什么;
- 知道 Context Machine 这个数学模型解决了传统 agent loop 的什么痛点;
- 在脑子里画出一次 `accelerate run "..."` 的完整执行流程图。

> §4.1–§4.12 是通俗讲解 + 设计动机。§4.13 会把同一套模型用一两个数学符号紧凑表达一次，这是 [DESIGN.md](crates/machine/DESIGN.md) 的中文版本。

### 4.1 从一个普通 agent loop 说起

任何 LLM agent（Claude Code、Cursor、AutoGPT、LangChain agents、OpenAI Assistants...）剥到最里面，都长这个样子：

```
1. 准备一段对话历史（system prompt + user 输入 + 之前的 assistant/tool 消息）
2. 把对话历史 + 可用工具列表发给 LLM
3. LLM 返回：要么是最终回答，要么是一组工具调用
4. 如果是工具调用：执行它们 → 把结果追加到对话历史 → 回到第 2 步
5. 如果是最终回答：返回给用户
```

这个 loop 不复杂，但**真正写起来很容易出问题**：

- 对话历史是一个**只能 append 的数组**，过长了想压缩怎么办？想删掉一条过时的 tool result 怎么办？
- LLM 一次返回了**5 个并行工具调用**，工具之一失败了怎么办？要不要让 LLM 看到这个失败？
- 我想在每次调 LLM 之前**注入一些环境信息**（当前时间、cwd、最近的文件改动）怎么办？写在哪一层？
- 我想**切换模型**（用 Haiku 处理简单任务、Sonnet 处理复杂任务）怎么办？
- 多 agent 协作时，agent A 的对话历史怎么传给 agent B？传**全部**还是**摘要**？

这些问题在主流 harness 里通常用"绕"的方式解决：自己写一段 message 压缩函数、加一个全局的 retry 装饰器、在 system prompt 里拼字符串、用 callback 钩子注入额外消息…… **每个绕法都是一个特例，不构成统一抽象**。

RCM 的回答是：**把对话历史本身当成一台可编程的"磁带"，让"策略"成为唯一能改这条磁带的东西**。这就是 Context Machine 模型。下面的小节会逐步搭出这个模型。

---

### 4.2 三层架构鸟瞰

在进入细节之前，先看一眼整个 workspace 的分层：

```
┌────────────────────────────────────────────────────────────┐
│  cli/         accelerate 二进制 + 终端 tape 动画           │  用户接口层
├────────────────────────────────────────────────────────────┤
│  accelerator/ 内置工具 / 默认策略 / 多 agent 编排          │  工程实现层
├────────────────────────────────────────────────────────────┤
│  machine/     Context Machine 原语（Fragment, Context...）  │  形式化原语层
└────────────────────────────────────────────────────────────┘
       依赖方向只能向下：上层用下层，下层不知道上层
```

为什么要分这三层？因为**它们的稳定性不同**：

- `machine` 是数学原语，它的 API 表面对应 [DESIGN.md](crates/machine/DESIGN.md) 的形式化定义。这个定义一旦稳定，理论上不需要随版本演化。
- `accelerator` 是开箱即用的实现——5 个内置工具、1 个默认策略（Captain）、1 个默认模型（gpt-4.1）、多 agent 编排。这些都是工程决策，可能随产品迭代而变。
- `cli` 是命令行界面 + tape 动画 UI。完全可被替换（Web 服务、IDE 插件等），不动下面两层。

[AGENTS.md](AGENTS.md) 把这条边界写进了硬规矩：`machine` 不许引入 provider preset、不许暴露 reactor trait、不许出现 UI 相关代码。每一次提交都要保住这条线。

---

### 4.3 Fragment：上下文的最小单位

这是 RCM 里最基础的概念。理解它之后，后面的一切都顺理成章。

#### 它是什么

在传统 LLM API 里，对话历史是一个 JSON 数组：

```json
[
  { "role": "system", "content": "你是一个助手" },
  { "role": "user", "content": "1+2 等于几" },
  { "role": "assistant", "tool_calls": [...] },
  { "role": "tool", "tool_call_id": "...", "content": "3" }
]
```

RCM 里的对应概念叫 **Fragment**（[fragment.rs](crates/machine/src/fragment.rs)）。每条"消息"是一个 Fragment，但它比 JSON message **多了三件东西**：

```rust
pub struct Fragment {
    pub(crate) id: u64,       // 1. 由 Context 分配的稳定 ID
    pub role: Role,           // 2. System / User / Assistant / Tool（只读）
    pub tag: String,          // 3. 业务标签（"agent" / "purpose" / "env" / "hitch"...）
    pub content: Content,     // 4. 真实内容（文本 / 图 / 音 / 视频 / 工具调用 / ...）
}
```

#### 跟你熟悉的概念对应

| RCM | 你大概率见过的对应物 |
|------|----------------------|
| `Fragment` | OpenAI 的 `ChatCompletionMessage`、Anthropic 的 `MessageParam`、ChatML 的一条 message |
| `Role` | message 的 `role` 字段 |
| `Content::Text` | 普通 message 的 `content` 字符串 |
| `Content::Image/Audio/Video/Document` | 多模态 message 的 content block |
| `Content::ToolCall` | OpenAI 的 `tool_call` / Anthropic 的 `tool_use` |
| `Content::ToolResult` | OpenAI 的 role=tool message / Anthropic 的 `tool_result` |
| `Content::Hitch` | **没有直接对应物**。这是 RCM 独有的"错误也是数据"的设计（见 §4.4 末尾） |

#### 三个关键设计

**为什么有 `id`**：传统 message 数组只能用 index（0、1、2…）引用某条消息，但**只要你插入或删除一条，所有 index 都漂移**。Policy 要做"把那条过时的 tool result 删掉"这种事，靠 index 是不可靠的。RCM 给每个 fragment 分配一个稳定自增 id，存进 Context 之后**永不变**——`Replace` 保留 id、`Remove` 销毁 id、`Append`/`Insert` 分配新 id。这样 Policy 可以跨多轮安全引用某个具体的 fragment。

**为什么 `role` 不可改**：role 是 LLM wire protocol 语义的一部分（system 走系统提示通道、tool 走工具结果通道）。允许策略改 role，等于允许它在 LLM 面前"伪造身份"——后果不可预测。所以 `Fragment` 的 role 是公开字段但**事实上只读**：唯一改它的方法是 `Replace` 整个 fragment。

**为什么有 `tag`**：这是个看起来不起眼但非常实用的设计。Phase（§4.7）经常需要"找到我上次塞进去的那条 system 提示"——光靠 role 没法区分（system 可能有好几条），靠 id 又没存。tag 是个**业务级标签**，比如 Captain 的 `BootstrapAgent` 注入系统提示时打 tag `"agent"`，下一次再调用时通过 `f.role == System && f.tag == "agent"` 就能精确找到自己之前那条。

#### Content 的几种形态

```rust
pub enum Content {
    Text(Text),                     // 纯文本
    Image(Image),                   // 图（OpenAI vision / Claude vision）
    Audio(Audio), Video(Video),     // 音频 / 视频
    Document(Document),             // PDF / 代码等
    ToolCall(ToolCall),             // 模型发出的工具调用
    ToolResult(ToolResult),         // 工具执行结果
    Hitch { message, retryable, code },  // 故障（见 §4.4 末尾）
}
```

`Text` / `Image` / `ToolCall` / `ToolResult` 跟主流 API 一一对应。`Hitch` 是 RCM 独有的，下一节讲。

---

### 4.4 Context：可改写的对话历史

#### 它是什么

`Context`（[context.rs](crates/machine/src/context.rs)）就是 fragment 的有序集合——也就是你熟悉的"对话历史"。但它比传统 message 数组多了**五个原子修改操作**：

```rust
ctx.append(fragment)          // 末尾追加，返回新 id
ctx.insert(after_id, frag)    // 在 id=after_id 之后插入
ctx.replace(id, fragment)     // 替换内容，保留 id
ctx.remove(id)                // 删除
ctx.swap(id1, id2)            // 交换位置
```

每个操作对应 Policy 可以发出的一个 `Action`（§4.6 会讲）。

#### 跟传统 message history 的关键区别

主流 harness 是**只能 append** 的：

```python
# LangChain / OpenAI Assistants / ...
messages.append({"role": "user", "content": "..."})
messages.append({"role": "assistant", "tool_calls": [...]})
# 想删一条？想改一条？只能重建整个数组
```

RCM 让 context 本身可改写：

```rust
let purpose_id = ctx.append(Fragment::user("调研主题 X"));
ctx.append(Fragment::assistant("..."));
// 几轮之后，想压缩前面的对话：
ctx.replace(purpose_id, Fragment::user("调研主题 X (摘要：...)"));
// 想删掉某条过时的 tool result：
ctx.remove(some_id);
```

**这意味着什么**：

- **上下文压缩**变成第一公民操作，不是补丁;
- **多 agent 协作**时可以把一个 agent 的 context 切片传给另一个 agent;
- **错误处理**可以"擦掉失败的工具调用、重新发一遍"，而不是用一堆 retry 装饰器;
- **可视化**变得有意义——tape 动画里你能看到每一条 fragment 的写入、替换、删除（这就是 §2 看到的那条彩色磁带）。

#### Hitch：当错误也是数据

传统 harness 处理工具失败一般有两种路子：(1) 抛异常 + retry 装饰器，(2) 把错误字符串当 tool result 返回给 LLM。前者把错误藏到控制流里，后者污染了 tool result 的语义。

RCM 引入了第三种：**`Content::Hitch`**。当 LLM 超时、工具失败、工具未找到时，reactor 不 panic、不返回 Err，而是构造一个 `Fragment::hitch(message)` 推进 inbox：

```rust
pub fn hitch(message: impl Into<String>) -> Self {
    Self {
        id: 0,
        role: Role::System,
        tag: "hitch".into(),
        content: Content::Hitch { message: message.into(), retryable: false, code: None },
    }
}
```

**优点**：Policy 可以**观察**到这个 hitch（它就是 inbox 里的一个 fragment），然后决定怎么处理——重试、切模型、压缩 context、放弃——所有"错误处理策略"都是 Policy 代码里的普通分支，没有装饰器、没有特殊异常路径。错误进入了数据流。

> 注：当前的默认 Captain 还没真正利用这个能力，遇到 hitch 就直接 Done 了。这是已知坑之一，见 §9。

---

### 4.5 Inbox：reactor 与 policy 之间的暂存区

#### 解决的问题

继续上面的场景：LLM 返回了一组东西——一段文本说"我去查一下" + 3 个并行 tool call。在传统 harness 里，这些通常**一股脑追加到 message history**：

```python
messages.append(assistant_text_msg)
messages.append(tool_call_1_msg)
messages.append(tool_call_2_msg)
messages.append(tool_call_3_msg)
# 等工具跑完
messages.append(tool_result_1_msg)
messages.append(tool_result_2_msg)
messages.append(tool_result_3_msg)
```

但 Policy 经常想做更精细的事：

- "tool_call_2 失败了，我想**只把 tool_result_2 的 hitch 留在 context**，把 tool_call_1 和 3 的结果合并成一条摘要"
- "在追加这堆消息**之前**，先把前面 3 轮的对话压缩一下"
- "tool_result 的内容太长了，我想**只插入摘要、原文存到 env**"

如果让 reactor 直接写 context，这些机会就消失了。

#### Inbox 是什么

`Inbox`（[inbox.rs](crates/machine/src/inbox.rs)）就是一个 FIFO 队列：

```rust
pub struct Inbox {
    fragments: VecDeque<Fragment>,
}
```

Reactor 跑完之后把所有产出 push 进 inbox，**不直接进 context**。Policy 必须主动发出 `Action::Take` 才能把 inbox 头部的 fragment 拿出来放到 context 里。

**类比**：你可以把 inbox 想象成"邮件收件箱"——LLM 把回复扔进收件箱，但你（Policy）决定每一封怎么处理。读完直接归档？转发给别人？删掉？回复？这些都是你的选择，不是邮件服务器代你做的。

#### 一条硬性约束

**Inbox 必须先清空，才能再次调用 reactor**（也就是发 `Action::Halt`）。这是 DESIGN.md §2.4 里的不变量，违反它会让"LLM 产出的信息悄无声息地堆积或丢失"。

Captain 的写法就严格遵守这点：

```rust
if inbox.peek().is_some() { return Action::Take; }   // 永远先消费
// 只有 inbox 空了才考虑 Halt 或 Done
```

#### 跟主流 harness 的对应

| RCM | 主流 harness |
|------|----------------|
| reactor 把产出推进 inbox | LLM SDK 返回 response 对象 |
| `Action::Take` 把 inbox 头部弹进 context | `messages.append(response.choices[0].message)` |
| Inbox 必须先清空才能 Halt | （没有这条约束——所以经常出现"漏处理 tool call"的 bug） |

Inbox 这个抽象的本质是**显式化"LLM 输出怎么进 context"这一步决策**。多数 harness 把它隐式了，所以遇到复杂场景就要"打补丁"。

---

### 4.6 Policy + Action：唯一能改 context 的"写头"

#### 它是什么

`Policy` 是个 trait（[policy.rs](crates/machine/src/policy.rs)），唯一必须实现的方法叫 `decide`：

```rust
pub trait Policy: Send + Sync {
    fn clone_box(&self) -> Box<dyn Policy>;

    fn decide<'a>(
        &'a self,
        purpose: &'a Purpose,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;

    // ...还有四个可选的 Phase 钩子，§4.7 再讲
}
```

每一步主循环，Machine 都调一次 `decide`，拿到一个 `Action`，然后执行它。

#### Action 是什么

`Action` 是个 enum，**只有 Action 能改 context**。所有 Action 加起来构成 Policy 能影响这个世界的全部手段：

| Action | 做什么 | 类比 |
|--------|---------|------|
| `Append(frag)` | 在 context 末尾追加 | `messages.append(...)` |
| `Insert { after, fragment }` | 在 id=after 之后插入 | （主流 harness 通常做不到） |
| `Replace { id, fragment }` | 替换某条 fragment，保留 id | （主流 harness 通常做不到） |
| `Remove(id)` | 删除某条 fragment | （主流 harness 通常做不到） |
| `Swap(id1, id2)` | 交换两条的位置 | （工程便利动作） |
| `Take` | 从 inbox 头部弹一个进 context | 把 LLM 响应追加到 history |
| `Halt` | 触发 reactor —— 调 LLM + 跑工具 | `client.chat.completions.create(...)` |
| `Model(name)` | 切换激活模型 | （罕见，主流 harness 一般静态绑定） |
| `Activate(name)` / `Deactivate(name)` | 启 / 停某个工具 | （罕见） |
| `Done` | 终止 machine 主循环 | break out of loop |

**关键设计**：所有这些动作都是**数据**，不是函数调用。Policy 不直接调 `ctx.append()`，它只返回 `Action::Append(...)`，由 Machine 来执行。这意味着：

- **可重放**：把 Action 流录下来就能完全重建 context 演化;
- **可 mock**：测试时用 `SeqPolicy::new(vec![Action::Append(...), Action::Done])` 喂一串预设 action，整个流程都能跑通而不打真 LLM（见 §6）;
- **可 RL**：Action 空间是有限离散的，整个 agent 就是一个标准 MDP，可以套 RL 算法学策略。

#### 跟主流 agent loop 的对比

| 概念 | 主流 harness | RCM |
|------|----------------|-----|
| "agent 的核心逻辑" | 一个大 while 循环，里面调 LLM、跑工具、追加消息、判断终止条件 | 一个 trait `Policy::decide`，返回 `Action` |
| 改 message history | 直接调 `messages.append(...)` 等 | 必须通过返回 `Action` |
| 调 LLM | 直接调 `client.chat.completions.create(...)` | 返回 `Action::Halt`，由 Machine 调 reactor |
| 终止 | `break` | 返回 `Action::Done` |

**为什么 Policy 是 trait 而不是函数**：策略需要有内部状态（如 Captain 的 `started` 标志、轮次计数、retry 计数）。trait + `&self` + `Atomic*` 是 Rust 里表达"共享、并发安全、内部可变"最经济的方式。AGENTS.md 硬性规定不许用 `Mutex` 存策略状态——atomic 就够了。

---

### 4.7 Phase：确定性的引导逻辑

#### 解决的问题

写过 agent 都知道：**每次调 LLM 之前都要做一些"打杂"的事**。

- 确保 system prompt 在 context 里;
- 注入当前时间、cwd、平台信息;
- 把用户 purpose 转成 user message;
- 清理某些过时字段。

这些事的特点：**确定性、跟 inbox 无关、跟模型决策无关**。如果把它们塞进 `Policy::decide` 里，会让 decide 变成一堆 if-else 的杂烩，每次都要判断"我是不是已经做过这件事"。

#### Phase 是什么

`Phase` 是 Policy 的"同步确定性子部件"。它在四个钩子时刻被调用：

```rust
pub trait Policy: ... {
    fn pre(&self)        -> Vec<Box<dyn Phase>> { vec![] }  // 主循环开始前
    fn post(&self)       -> Vec<Box<dyn Phase>> { vec![] }  // 主循环结束后
    fn pre_halt(&self)   -> Vec<Box<dyn Phase>> { vec![] }  // 每次调 reactor 前
    fn post_halt(&self)  -> Vec<Box<dyn Phase>> { vec![] }  // 每次调 reactor 后
}
```

Phase 自己的接口非常简单——**同步**，**只产生确定性 Action**：

```rust
pub trait Phase: Send + Sync {
    fn name(&self) -> &str;
    fn clone_box(&self) -> Box<dyn Phase>;
    fn decide(&self, purpose, ctx, env, resources) -> PhaseOutcome;
}

pub enum PhaseOutcome {
    Action(Action),     // 还有事要做
    Done,               // 这个 Phase 完成了
}
```

Machine 会**反复调用同一个 Phase**，直到它返回 `Done`。所以 Phase 可以连续发若干 Action 来达到稳态。

#### 一个具体例子

[`BootstrapAgent`](crates/accelerator/src/policy/phases/bootstrap.rs)：

```rust
fn decide(&self, _purpose, ctx, _env, resources) -> PhaseOutcome {
    let desired = resources.prompts.get(&self.prompt_name).cloned().unwrap_or_default();

    if let Some(existing) = ctx.fragments().iter()
        .find(|f| f.role == Role::System && f.tag == "agent")
    {
        if existing.as_text() == Some(&desired) {
            return PhaseOutcome::Done;        // 已经是对的，啥也不用做
        }
        return PhaseOutcome::Action(Action::Replace {  // 内容过时了，覆盖
            id: existing.id(),
            fragment: Fragment::system(desired).with_tag("agent"),
        });
    }

    PhaseOutcome::Action(Action::Append(           // 不存在，追加
        Fragment::system(desired).with_tag("agent")
    ))
}
```

Captain 把它挂在 `pre()` 里，每次 Machine 启动都会自动保证"system prompt 是最新的"。

#### Phase vs Policy::decide

| | `Policy::decide` | `Phase::decide` |
|------|------------------|-----------------|
| 是否异步 | 是 | 否 |
| 能看 inbox 吗 | 能 | 不能 |
| 何时被调 | 主循环每一步 | pre / post / pre_halt / post_halt 四个钩子 |
| 调用次数 | 每步 1 次 | 反复调直到返回 `Done` |
| 能发 `Halt` 吗 | 能 | 不能（会被忽略并 warn） |

**为什么 Phase 不能发 Halt**：Halt 触发 reactor 调 LLM，这是个**重决策**，应该集中在 Policy::decide 里。如果 Phase 也能 Halt，控制流就分散在多处了，代码变得难以追踪。

#### 跟主流 harness 的对应

| RCM | 主流 harness |
|------|----------------|
| `BootstrapAgent` | 系统初始化时拼 system prompt 的逻辑 |
| `InjectPurpose` | 把 user 输入塞进 messages 列表的第一条 |
| `InjectEnv`（每次 pre_halt 注入 cwd / time） | 大多数 harness 没有，需要自己写 callback |

Phase 把这类"重复出现的注入/确认逻辑"抽成了一等公民，可以复用、可以测试、可以挂到任何 Policy 上。

---

### 4.8 Reactor：跟外部世界对接的唯一通道

#### 它是什么

到这里你已经知道：Policy 通过 `Action::Halt` "请求"调 LLM。**真正去调 LLM、执行工具的代码**叫 `reactor`（[reactor.rs](crates/machine/src/reactor.rs)）。

它做的事流程上是这样：

```text
1. 把 ctx 里的所有 fragment 编码成 LLM 协议要求的 messages 格式
2. 把当前激活的 tools 编码成 LLM 协议要求的 tool definitions
3. 按 protocol（OpenAI / Anthropic / Gemini）调对应的 client
4. 拿到 LLM 响应（一组 fragment：text、tool_call）
5. 对每个 tool_call，去 Resources 里找对应的 Tool，用 timeout 包起来执行
6. 把所有 fragment（text、tool_call、tool_result 或 hitch）push 进 inbox
```

#### 关键设计

**Reactor 不是 trait，是 `pub(crate) async fn`**——这意味着**外部 crate 看不到、替换不了**。AGENTS.md 明文：

> reactor is internal to machine. It is a plain async function, not a trait, not injected, not visible to accelerator.

为什么这条规矩这么严？因为如果让外部能替换 reactor，整个形式化模型就崩了——上层可以塞个"假" reactor，绕过 inbox 直接写 context，或者引入 provider preset，或者在 reactor 里偷偷修改 LLM 输出。Reactor 被锁死成"machine 内部的一个 async function"，整个 Context Machine 的语义才有保证。

**所有调用都过 `tokio::time::timeout`**：LLM 调用过 `model.timeout` 秒，工具调用过 `tool.timeout()` 秒。这是防止"agent 被一个挂死的工具拖到天荒地老"的最后一道防线，也是 AGENTS.md 的硬规矩。

**失败永远包成 Hitch**：

```rust
match timeout(deadline, tool.execute(args, env)).await {
    Ok(Ok(result)) => Fragment::tool_result(...),       // 成功
    Ok(Err(msg))   => Fragment::hitch(format!("tool '{}' error: {}", name, msg)),
    Err(_)         => Fragment::hitch(format!("tool '{}' timed out after {}s", ...)),
}
```

LLM 超时也是 Hitch、工具未找到也是 Hitch、工具执行失败也是 Hitch。**Policy 看到的永远是统一的 Hitch fragment**，不需要为每种错误分别处理控制流。

#### 跟主流 harness 的对应

| RCM 概念 | 主流 harness |
|------|----------------|
| `reactor::react` | "调 LLM + 跑工具"那段循环体代码 |
| `completion::complete` 按 protocol dispatch | OpenAI client / Anthropic client 的分支 |
| Tool execute + timeout 包装 | function calling 的执行逻辑 |
| Hitch fragment | try/except + retry decorator（但语义不一样） |

---

### 4.9 Machine：把这一切粘起来的 dispatcher

#### 它是什么

`Machine`（[machine.rs](crates/machine/src/machine.rs)）的结构异常简单：

```rust
pub struct Machine {
    policy: Box<dyn Policy>,
}
```

就**只有 policy** 这一个字段。所有状态（ctx, env, resources）都是从 `run` 方法的参数传进来的：

```rust
pub async fn run(
    &self,
    purpose: &Purpose,
    ctx: &mut Context,
    env: &mut Environment,
    resources: &mut Resources,
)
```

#### 主循环长什么样

```text
run_phases(pre)                  # 启动前的 Phase

loop {
    action = policy.decide(...)
    match action {
        Halt => {
            run_phases(pre_halt)
            reactor::react(...)            # ω：调 LLM + 跑工具
            run_phases(post_halt)
        }
        Done => break
        其他 => apply_action(action)        # 翻译成对 ctx/inbox/resources 的调用
    }
}

run_phases(post)                 # 结束后的 Phase
```

`apply_action` 是**整个系统里唯一会调 `ctx.append()` / `ctx.insert()` / `ctx.replace()` / `ctx.remove()` / `ctx.swap()` 的地方**。这就是为什么我们说"**Policy 是 context 的唯一写者**"——Policy 通过返回 Action，Machine 把 Action 翻译成对 ctx 的调用。

#### 跟主流 agent loop 的对比

主流 harness 的"agent 主循环"通常是几百行代码，混着 prompt 拼接、LLM 调用、工具执行、错误处理、消息追加、终止判断。RCM 的 `Machine::run` 把这些**全部赶到 Policy / Phase / Reactor 里**，自己只做"翻译 Action → 调对应的方法"这件事。

---

### 4.10 Environment / Resources / Purpose：agent 的"世界"

Context 是 agent 的"记忆"，那 agent 的"世界"是什么？RCM 把它拆成三个正交的概念。

#### Environment：外部世界的快照

[`Environment`](crates/machine/src/env.rs)：

```rust
pub struct Environment {
    pub cwd: PathBuf,
    pub vars: HashMap<String, String>,
    pub root: Option<PathBuf>,
    pub platform: String,
}
```

就 4 个字段——当前工作目录、环境变量、项目根、操作系统。工具读它（`shell` 用 `cwd` 起子进程、`fs` 用 `cwd` 解析相对路径），Phase 也读它（`InjectEnv` 把 cwd/platform/time 写进 system prompt）。

> 对应 Claude Code 启动时打印的那段 "cwd / git_branch / platform / today's date" 信息。

#### Resources：agent 自己掌握的工具和模型

[`Resources`](crates/machine/src/resources.rs)：

```rust
pub struct Resources {
    pub tools: HashMap<String, Arc<dyn Tool>>,        // 注册的工具
    pub models: HashMap<String, Model>,                // 注册的模型
    pub prompts: HashMap<String, String>,              // prompt 模板（"captain" → captain.txt）
    pub active_model: String,                          // 当前用哪个模型
    pub active_tools: HashSet<String>,                 // 当前激活哪些工具
}
```

**关键设计：注册 ≠ 激活**。你可以注册 10 个工具，但当前 agent 只激活其中 3 个。把另外 7 个不相关工具的 description 发给 LLM 不仅浪费 token，还会**干扰决策**（LLM 看到一堆无关工具反而容易选错）。Policy 在运行时通过 `Action::Activate("xxx")` / `Action::Deactivate("xxx")` 调整激活态，类似 IDE 的 plugin enable / disable。

#### Purpose：只读的"意图"

[`Purpose`](crates/machine/src/purpose.rs)：

```rust
pub struct Purpose {
    pub text: String,
}
```

就一个字符串。这是 agent 启动时被赋予的"目标"，**整个生命周期内不可修改**——Policy / Phase 都只能读，不能写。

**为什么独立出来**：你可能觉得这跟 user input 没区别，为什么不直接当 user fragment 塞进 context？因为它们的角色不同：

- **Purpose 是策略的元信息**——决定 Policy 怎么决策、Phase 怎么注入。
- **User fragment 是对话内容**——是要发给 LLM 看的。

Captain 的 `InjectPurpose` Phase 就把 Purpose 翻译成一条 `user` fragment 塞进 context：策略层面有 Purpose，对话层面有 user fragment，两者职责清晰。

#### 三者的合并语义（多 agent 用）

| | 合并方式 | 解释 |
|------|----------|------|
| Purpose | `Concat`（拼接） | 多个 agent 的目标拼成一个长目标 |
| Context | `Append` 或 `Replace` | fragments 顺序拼接，或取最后一个非空 |
| Environment | `Overlay` | 后来的 var 覆盖先来的（类似 docker layer） |
| Resources | `Merge` | 工具 / 模型 / prompt 取并集 |

这些会在 §4.14 多 agent 部分用到。

---

### 4.11 Tool / Model：抽象与多 provider 支持

#### Tool

[`Tool`](crates/machine/src/tool.rs) 是个 trait，5 个方法：

```rust
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;                  // LLM 看到的名字
    fn description(&self) -> &str;           // LLM 看到的说明
    fn parameters(&self) -> Value;           // JSON Schema
    fn timeout(&self) -> Duration { ... }    // 默认 180s
    fn execute<'a>(&'a self, args: Value, env: &'a Environment)
        -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>>;
}
```

跟 OpenAI function calling / Anthropic tool use 的 spec 一一对应——`name` / `description` / `parameters` 直接被序列化发给 LLM，`execute` 是真正的执行逻辑。

#### Model + Protocol

[`Model`](crates/machine/src/model.rs) 是数据：

```rust
pub struct Model {
    pub name: String,
    pub protocol: Protocol,         // OpenAI / Anthropic / Gemini
    pub endpoint: Option<String>,   // 自定义 base URL
    pub credentials: Option<String>,
    pub temperature: Option<f64>,
    pub limit: Option<Limit>,
    pub cost: Option<Cost>,
    pub modalities: Option<Modalities>,
    pub timeout: u64,
    // ...
}
```

**关键设计：只有三种 protocol**：

```rust
pub enum Protocol { OpenAI, Anthropic, Gemini }
```

DeepSeek / Groq / Mistral / xAI / Ollama / OpenRouter / SiliconFlow / 通义千问 OpenAI 兼容端点……所有这些 provider 全部走 `Protocol::OpenAI` + 自定义 `endpoint`。为什么不做"DeepSeek preset"？因为 provider preset 是个无底洞——每来一个新 provider 都要加代码、维护它的小怪癖。而 wire protocol 只有三家，所有 provider 都得选一家兼容。**机制层稳定（3 种 protocol），配置层灵活（无数 endpoint）**。

---

### 4.12 把 Machine 层串一遍：完整执行轨迹

到这里你已经认识了 Machine 层所有概念：**Fragment / Context / Inbox / Policy / Action / Phase / Reactor / Machine / Environment / Resources / Purpose / Tool / Model**。现在我们把它们串起来，看一次 `accelerate run "1+2 等于几"` 完整发生了什么：

```
1. cli/cmd/run.rs::run
   构造 Graph，spawn 1 个 State（purpose="1+2 等于几"）
   build → Assembly → run

2. Assembly::run（节点 0 ready）
   resolve_*（首节点无依赖，直接取 input）
   fire(state) → Accelerator::fire → Machine::run

3. Machine::run（policy = Captain）

   [pre Phase] 启动前注入
   ├─ BootstrapAgent: ctx 空 → Append(system "你是一个 AI 助手…", tag="agent")
   └─ InjectPurpose:  没有 purpose fragment → Append(user "1+2 等于几", tag="purpose")

   [主循环 第 1 轮]
   ├─ decide: inbox 空 + 首次调用 → Halt
   └─ apply(Halt):
      ├─ pre_halt Phase: InjectEnv → Append(system "cwd: ... platform: ...", tag="env")
      └─ reactor::react:
         ├─ completion::complete: ctx 三条 + 5 个工具 → 发给 LLM
         ├─ LLM 返回: text "我用 add 工具" + tool_call(add, {first:1, second:2})
         ├─ 执行 add → ToolResult("3")
         └─ inbox.push(text), inbox.push(tool_call), inbox.push(tool_result)

   [主循环 第 2 轮]  decide: inbox 非空 → Take    →    ctx.append(text),         inbox 剩 2
   [主循环 第 3 轮]  decide: inbox 非空 → Take    →    ctx.append(tool_call),    inbox 剩 1
   [主循环 第 4 轮]  decide: inbox 非空 → Take    →    ctx.append(tool_result),  inbox 空

   [主循环 第 5 轮]
   ├─ decide: inbox 空 + 上次是 Tool → Halt
   └─ apply(Halt):
      ├─ pre_halt Phase: InjectEnv → 时间变了？Replace；否则 Done
      └─ reactor::react: LLM 看到 tool result，返回最终文本 "1+2=3"

   [主循环 第 6 轮]  decide: inbox 非空 → Take    →    ctx.append(final_text)
   [主循环 第 7 轮]  decide: inbox 空 + 上次不是 Tool → Done   → break

   [post Phase] Captain 没声明 post，跳过

4. fire 返回 State（含最终 ctx）
5. Assembly 收集 sink 节点的输出
6. cli 渲染最终 assistant 文本（或者按 --format json / --context 决定输出方式）
```

**注意几件事**：

- **整个流程里 context 的写入只发生在 `apply_action` 里**，每一次都对应 Policy / Phase 显式发出的一个 Action。
- **reactor 只跑了 2 次**（第 1 轮 Halt + 第 5 轮 Halt），分别对应 2 次 LLM 调用。
- **inbox 只有"调 LLM 之后"那一小段时间是非空的**，剩下时间一直为空。
- **Captain 的 decide 逻辑只有 4 行 if-else**，所有复杂性都被 Phase / Action / Inbox 这套机制吸收了。

---

### 4.13 形式化定义（DESIGN.md 的中文版）

现在你已经理解整个模型了，我们用一两个数学符号把它紧凑表达一次。完整推导见 [crates/machine/DESIGN.md](crates/machine/DESIGN.md)。

一台 Context Machine 是一个三元组：

$$\mathcal{M} = (\mathcal{C}, \mathcal{E}, \Phi)$$

- $\mathcal{C}$ —— **上下文空间**（所有可能的 fragment 序列）;
- $\mathcal{E}$ —— **环境空间**（外部世界 + Resources）;
- $\Phi$ —— **状态转移**，加上 pending queue $p$（也就是 Inbox）后变成：

$$\Phi(c, e, p) = (c', e', p'), \quad c' = \pi(c, e, p), \quad (e', p') = \omega(c', e)$$

- $\pi$（policy / context engineering）—— 决定怎么改 context;
- $\omega$（reactor）—— 调 LLM + 跑工具，把产出放进 pending queue。

**约束**：`halt`（触发 $\omega$）只在 $p = [\ ]$ 时合法——也就是 inbox 必须先清空。

$\pi$ 的动作空间是有限离散的：

| 模式 | 动作 | 参数空间 |
|------|------|---------|
| Consumption（$p \neq [\ ]$）| `append` / `insert(i)` / `remove(i)` | 1 / $|c|$ / $|c|$ |
| Free（$p = [\ ]$）| `append(k)` / `insert(i, k)` / `remove(i)` / `replace(i, k)` / `halt` | $|e.resources|$ / $|c| \times |e.resources|$ / ... / 1 |

**两个核心性质**：

- **Markov 性**：$x_{t+1}$ 只依赖 $x_t$，不依赖历史。
- **确定性**：$\Phi$ 是函数。LLM 调用的不确定性被装进 $\omega$ 这个"外部副作用"里，不破坏 $\Phi$ 本身的确定性。

**这意味着**：整个 agent 是一个标准 MDP，可以直接套 RL 算法学策略。"Context Engineering"被定义成了一个**有形式定义、有离散 action space、可被学习**的一等对象——这是 RCM 跟其他 agent 框架最大的差别。

---

### 4.14 Accelerator 层：多 agent 编排

到这里你已经理解了 Machine 层的全部。`accelerator` 在它之上加了三件事：默认 agent、多 agent 编排、模型预设。

#### State：单 agent 的"完整身份"

[`State`](crates/accelerator/src/state.rs) 把一个 agent 的所有状态打包：

```rust
pub struct State {
    pub purpose: String,
    pub ctx: Context,
    pub env: Environment,
    pub policy: Box<dyn Policy>,
    pub res: Resources,
}
```

`State::default()` 给你一个**开箱即用**的 agent：cwd 是当前目录、policy 是 Captain、5 个内置工具全激活、模型是 gpt-4.1。

#### Captain：默认 Policy

[`Captain`](crates/accelerator/src/policy/captain.rs) 决策逻辑就 4 行：

```text
inbox 非空            → Take
否则 + 第一次调       → Halt（启动 LLM）
否则 + 上次是 Tool    → Halt（让 LLM 看工具结果）
否则                  → Done
```

挂了三个 Phase：

- `pre`: `BootstrapAgent("captain")` —— 注入系统 prompt（captain.txt）
- `pre`: `InjectPurpose` —— 把 purpose 转成 user fragment
- `pre_halt`: `InjectEnv` —— 每次调 LLM 前刷新 cwd/platform/time

这就是 §4.12 跑通的那个流程。

#### Accelerator：单 agent 的"图节点"封装

[`Accelerator`](crates/accelerator/src/accelerator.rs) = 1 台 Machine + 1 份 State。它同时是**图里的一个节点**：暴露 `purpose_in/out`、`ctx_in/out`、`env_in/out`、`policy_in`、`res_in/out`、`trigger`/`done` 这些端口。

#### Graph + Assembly + Flux：把多个 agent 编排起来

想象一下你要做这件事：

> "agent A 调研，agent B 写总结，agent C 找漏洞。B 看 A 的产出，C 看 A+B 的产出。"

在 LangGraph 之类的框架里你会写一个有状态图，节点之间共享 scratchpad。RCM 用的是**纯函数式 dataflow**：

```rust
let mut graph = Graph::new();
let a = graph.spawn(State { purpose: "调研主题 X".into(), ..State::default() });
let b = graph.spawn(State { purpose: "总结".into(),    ..State::default() });
let c = graph.spawn(State { purpose: "找漏洞".into(),  ..State::default() });

// 数据流：把 a.ctx 和 b.ctx 拼起来给 c
let merge = graph.weave(2, FluxMode::Context(ContextFlux::Append));
graph.wire(a.ctx_out(), merge.slot(0));
graph.wire(b.ctx_out(), merge.slot(1));
graph.wire(merge.out(), c.ctx_in());

// 控制流：a → b → c
graph.wire(a.done(), b.trigger());
graph.wire(b.done(), c.trigger());

let outputs = graph.build()?.run().await;
```

[`Graph`](crates/accelerator/src/graph.rs) 是构建器：三个动词——`spawn` 加 agent、`weave` 加合并节点、`wire` 拉线。`build()` 做拓扑排序 + 编译。

[`Assembly`](crates/accelerator/src/assembly.rs) 是编译后的图，按拓扑顺序跑每个节点，解析输入、跑、唤醒下游。

[`Flux`](crates/accelerator/src/flux.rs) 是**无状态合并节点**——一个纯函数，把 N 个上游的同类型输出合成一个：

| Channel | Mode | 行为 |
|---------|------|------|
| Purpose | `Concat` | 拼接 |
| Context | `Append` | fragments 顺序拼接 |
| Context | `Replace` | 取最后一个非空 |
| Environment | `Overlay` | 后来的 var 覆盖先来的 |
| Resources | `Merge` | 工具 / 模型 / prompt 取并集 |

**Flux 是纯函数**——合并语义完全由 mode 决定，不依赖历史。这意味着图可以被反复求值、可以缓存、原则上可以并行。

#### Channel / Port：类型化的连接

每根线都有 channel 类型：`Purpose` / `Context` / `Environment` / `Resources` / `Pulse`。`wire()` 在 channel 类型不匹配时直接 panic——**"连错线"的错误从运行时挪到了构建期**。

`Pulse` 是控制流（执行顺序）;其他四个是数据流。

#### 跟 LangGraph 等框架的对比

| RCM | LangGraph / CrewAI / 等 |
|------|--------------------------|
| 每个 agent 是 `State → State` 纯函数（fire） | 节点是有状态对象，共享全局 scratchpad |
| Flux 是显式的合并算子 | 合并逻辑藏在节点内部 |
| 类型化 channel | 状态字段用字符串 key |
| 编译期检查无环 | 通常运行时检查 |

代价：当前的 `Assembly::run` 是**顺序执行**——即使两个 agent 没有依赖也不会并行跑。这是 §9 列的已知坑。

---

### 4.15 核心不变量（改 machine 层时不能破坏）

下面这些不变量被多处代码隐式依赖。改 `machine` 层时务必保住：

1. **Policy 是 context 的唯一写者**。Reactor 只能写 inbox。
2. **Inbox 必须先清空才能 Halt**（DESIGN.md §2.4）。
3. **Fragment id 严格单增**，由 Context 在存入时分配，外部不能设。`Replace` 保留 id，`Remove` 销毁 id。
4. **Fragment role 不可改**。role 是 LLM wire protocol 语义的一部分。
5. **所有 LLM / 工具调用过 `tokio::time::timeout`**。无限等待是 bug。
6. **失败一律包成 Hitch fragment**，不 panic、不返回 Err 到 Machine 层。
7. **Reactor 不暴露**（`pub(crate)`，不是 trait，不可注入）。
8. **Machine 不知道 provider preset**。只知道 OpenAI / Anthropic / Gemini 三个 protocol + endpoint。

---

### 4.16 关键设计决策汇总

| 决策 | 取舍 |
|------|------|
| Context 用 id 而不是 index 引用 | id 在插入/删除后稳定，Policy 决策可以跨多轮引用同一 fragment；代价是要维护 `position_of(id)` 这层间接 |
| Inbox 在 reactor 和 policy 之间加 buffer | 让 Policy 显式决策 LLM 产出怎么进 context；代价是要写更多 `Take` action |
| Policy 是 trait，Reactor 是函数 | Policy 是策略，需要被替换、需要内部状态；Reactor 是实现细节，不该被替换 |
| Phase 和 Policy::decide 分开 | 确定性引导逻辑可以同步、可以复用、可以独立测试 |
| Phase 不能产生 Halt | 防止 Phase 绕过主决策循环 |
| 模型抽象只有 3 个 protocol | provider 是配置不是代码 |
| 工具的 description / parameters 是数据 | 工具能否被 LLM 用，只看运行时数据 |
| 失败用 Hitch fragment 表达 | 错误是数据流的一部分，不是控制流异常 |
| Graph 的连接是类型化的 channel | 把"连错线"的错误从运行时挪到构建期 |
| Flux 是无状态纯函数 | 合并语义可分析、可并行、可缓存 |

---

读到这里，整个架构应该都通了。接下来 §5 会教你怎么在这个架构上**做事**——加工具、改策略、加模型、编排多 agent。所有的代码示例都是基于刚才讲过的那套抽象。

---

## 5. 常见开发任务

### 5.1 加一个新工具

工具就是实现 [`machine::Tool`](crates/machine/src/tool.rs) 这一个 trait——也就是 §4.11 讲过的那个。最小例子：

```rust
// crates/accelerator/src/tools/echo.rs
use std::future::Future;
use std::pin::Pin;
use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

pub struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &str { "echo" }

    fn description(&self) -> &str { "Echo back the input string." }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "text": { "type": "string" }
            },
            "required": ["text"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let text = args["text"].as_str().ok_or("missing 'text'")?;
            Ok(ToolResult {
                call_id: String::new(),
                content: text.to_string(),
                title: None,
            })
        })
    }
}
```

然后在 [`accelerator/src/tools/mod.rs`](crates/accelerator/src/tools/mod.rs) 里 `mod echo; pub use echo::EchoTool;` 并把它加进 `builtin_tools()` 列表。Reactor 会自动通过 `Resources::active_tools()` 把它发给 LLM（参考 §4.8）。

**注意点**：

- `call_id` 留空字符串 —— reactor 会自己填，见 [`reactor.rs:61-66`](crates/machine/src/reactor.rs)。
- 默认 timeout 是 180s（`Tool::timeout` 默认实现），需要更短就 override。
- 工具内绝对不能 `panic!` / `unwrap` —— 用 `?` + `String` 错误，reactor 会包成 Hitch（§4.4 末尾解释过 Hitch 的设计）。
- 描述文本最好放在 `mod.txt` / `tool.txt` 里用 `include_str!`，方便 LLM 阅读、方便迭代。参考 [`shell.txt`](crates/accelerator/src/tools/shell.txt)。

### 5.2 写一个新的 Policy

Policy 是 `&self` + 异步决策——也就是 §4.6 讲的那个 trait。看一个最小例子（顺便理解 Captain 在干什么）：

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};
use machine::{Action, Context, Environment, Inbox, Policy, Purpose, Resources, Role};

pub struct LoopUntilDone {
    rounds: AtomicU32,
    max_rounds: u32,
}

impl Clone for LoopUntilDone {
    fn clone(&self) -> Self {
        Self {
            rounds: AtomicU32::new(self.rounds.load(Ordering::Relaxed)),
            max_rounds: self.max_rounds,
        }
    }
}

impl Policy for LoopUntilDone {
    fn clone_box(&self) -> Box<dyn Policy> { Box::new(self.clone()) }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        ctx: &'a Context,
        _env: &'a Environment,
        _res: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            // 1. inbox 非空 → 必须先消费（§4.5 的硬性约束）
            if inbox.peek().is_some() { return Action::Take; }

            // 2. 超过最大轮次 → 退出
            if self.rounds.load(Ordering::Relaxed) >= self.max_rounds {
                return Action::Done;
            }

            // 3. 上一条是 tool 结果 → 继续问 LLM
            if matches!(ctx.fragments().last().map(|f| f.role), Some(Role::Tool)) {
                self.rounds.fetch_add(1, Ordering::Relaxed);
                return Action::Halt;
            }

            // 4. 默认：再问一次 LLM
            self.rounds.fetch_add(1, Ordering::Relaxed);
            Action::Halt
        })
    }
}
```

**几个硬性约束**：

- `&self` + 内部用 `Atomic*` —— [AGENTS.md](AGENTS.md) 明文禁用 `Mutex` 存内部状态。
- inbox 非空时**必须**先 `Take`，不然 reactor 一直 push、inbox 越涨越大。
- `Halt` 前必须保证 inbox 为空，否则违反 DESIGN.md §2.4 的不变量。
- 决策函数应该是无副作用的纯函数（除了内部 atomic）。所有"改 context"都通过返回 Action 表达。

### 5.3 写一个新的 Phase

Phase 是同步、确定性的钩子，详见 §4.7。可以挂在四个时点：

| 钩子 | 触发时机 |
|------|----------|
| `pre()` | machine 主循环开始前 |
| `post()` | machine 主循环结束后 |
| `pre_halt()` | 每次 reactor 调用前 |
| `post_halt()` | 每次 reactor 调用后 |

例子：每次 halt 前注入"当前时间"。参考 [`InjectEnv`](crates/accelerator/src/policy/phases/inject.rs)。

Phase 返回 `PhaseOutcome::Action(...)` 或 `PhaseOutcome::Done`。Machine 会**反复**调用 Phase 直到它返回 `Done`，所以 Phase 可以连续发若干 Action 来达到稳态。常见 idiom（见 `BootstrapAgent`）：

```rust
fn decide(&self, ...) -> PhaseOutcome {
    if 已经是期望状态 { return PhaseOutcome::Done; }
    if 存在但内容不对 { return PhaseOutcome::Action(Action::Replace { ... }); }
    PhaseOutcome::Action(Action::Append(...))
}
```

注意 Phase 不能产生 `Action::Halt`，machine 会把它 `warn!` 掉并忽略（见 [`machine.rs:69-71`](crates/machine/src/machine.rs)）——理由在 §4.7。

### 5.4 加 / 切换 LLM 模型

模型是数据，不是代码（§4.11）。新模型：

```rust
use machine::{Cost, Limit, Model, Modalities, Modality, Protocol};

pub fn claude_4_sonnet() -> Model {
    Model {
        name: "claude-sonnet-4".into(),
        protocol: Protocol::Anthropic,
        endpoint: None,                          // 用 provider 默认
        credentials: std::env::var("ANTHROPIC_API_KEY").ok(),
        limit: Some(Limit { context: 200_000, input: None, output: 8192 }),
        cost: Some(Cost { input: 3.0, output: 15.0, cache_read: Some(0.30), cache_write: None }),
        modalities: Some(Modalities {
            input: vec![Modality::Text, Modality::Image],
            output: vec![Modality::Text],
        }),
        ..Default::default()
    }
}
```

注册：

```rust
let resources = Resources::new()
    .with_model(crate::model::gpt4_1())
    .with_model(claude_4_sonnet());      // 第二个模型不会自动激活
```

运行时切换：在 Policy 里返回 `Action::Model("claude-sonnet-4".into())`。

OpenAI 兼容的第三方 provider（DeepSeek / Groq / Mistral / xAI / Ollama / OpenRouter / ...）全都走 `Protocol::OpenAI` + 自定义 `endpoint`。`machine` 故意不知道任何 provider preset，这是 [AGENTS.md](AGENTS.md) 的硬规矩（§4.11 解释过原因）。

### 5.5 编排多 agent

`Graph` 拼图（详见 §4.14）：

```rust
use accelerator::{Graph, State, FluxMode, ContextFlux};

let mut graph = Graph::new();

// 三个 agent
let research = graph.spawn(State { purpose: "调研主题 X".into(), ..State::default() });
let summarize = graph.spawn(State { purpose: "总结".into(),    ..State::default() });
let critique  = graph.spawn(State { purpose: "找漏洞".into(),  ..State::default() });

// 一个 ctx 合并节点：把 research 和 summarize 的 context 拼接
let merge = graph.weave(2, FluxMode::Context(ContextFlux::Append));

// 数据流：research.ctx → merge.slot(0), summarize.ctx → merge.slot(1)
graph.wire(research.ctx_out(), merge.slot(0));
graph.wire(summarize.ctx_out(), merge.slot(1));

// merge.out → critique.ctx_in
graph.wire(merge.out(), critique.ctx_in());

// 执行顺序：research → summarize → critique
graph.wire(research.done(),  summarize.trigger());
graph.wire(summarize.done(), critique.trigger());

let assembly = graph.build().expect("graph 无环");
let outputs = assembly.run().await;            // outputs 是 Vec<State>，里面是 sink 节点的最终状态
```

两类边：

| 边 | 类型 | 语义 |
|----|------|------|
| Pulse 边 | `done() → trigger()` | 执行顺序：上游跑完才会调度下游 |
| State 边 | `*_out() → *_in()` 或 → `flux.slot(i)` | 数据流：上游某个状态字段流到下游 |

Flux 模式（[flux.rs](crates/accelerator/src/flux.rs)）：

| Mode | 行为 |
|------|------|
| `PurposeFlux::Concat` | 把多个 purpose 文本拼接 |
| `ContextFlux::Append` | 把多个 context 的 fragments 顺序拼接 |
| `ContextFlux::Replace` | 取最后一个非空 context |
| `EnvFlux::Overlay` | 后写的 var 覆盖先写的 |
| `ResFlux::Merge` | 模型 / 工具 / prompt 取并集，首次出现的活跃模型胜出 |

---

## 6. 测试

### 6.1 在哪写、怎么写

AGENTS.md：

> Tests go in `tests/`, not inline modules.

所以测试一律放在 `crates/<crate>/tests/`，例如 [`crates/machine/tests/machine.rs`](crates/machine/tests/machine.rs)。

写测试时**不要打真 LLM**。回忆 §4.6 讲的"Action 是数据"的好处——用 [`SeqPolicy`](crates/machine/tests/common.rs) 喂一串预设 `Action`，machine 会照着跑，整个流程都不需要真 LLM：

```rust
mod common;     // 引入 SeqPolicy 和 test_resources
use machine::{Action, Context, Environment, Fragment, Machine, Purpose};

#[tokio::test]
async fn my_test() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("hi")),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&Purpose::default(), &mut ctx, &mut env, &mut resources).await;

    assert_eq!(ctx.len(), 1);
}
```

### 6.2 测什么、不测什么

AGENTS.md 的核心要求：

- **测行为，不测 getter**。"调了 `Action::Remove` 后 context 真的变短了"是行为；"`ctx.fragments()` 跟我刚 push 的一致"是无意义的 getter 测试。
- **不要测 mock 自己**。`SeqPolicy::new` 是工具，不要为它写测试。
- **边界 case 优先**。"remove 一个不存在的 id 会 panic" 比 "remove 第 0 / 1 / 2 个元素" 更值得写。
- **`#[should_panic]` 必须带 `expected = "..."`**。否则任何 panic 都会通过测试。

### 6.3 跑测试

```sh
cargo test --workspace                 # 跑全部
cargo test -p machine                  # 只跑 machine crate
cargo test -p machine swap             # 只跑名字含 swap 的测试
cargo test --workspace -- --nocapture  # 看 println（一般不需要，因为不允许用 println）
```

---

## 7. 日志和可观测性

### 7.1 三个层级

RCM 把"给程序看的事件"和"给人看的日志"显式分开（参考 §4 提到的 `hook!` 设计）：

| 层级 | 怎么打 | 怎么看 |
|------|--------|--------|
| 操作日志 | `tracing::{debug!, info!, warn!, trace!}` | `RUST_LOG=info cargo run ...` |
| 应用事件（hook） | `machine::hook!(event = "halt", ...)` | CLI 的 tape 动画 / 写自己的 `Layer` |
| Panic / 错误 | 不打。fragment 包成 `Hitch` 进 inbox | tape 上显示红色 |

`hook!` 宏的目标是 `target: "hook"`，level 是 TRACE（[hook.rs](crates/machine/src/hook.rs)）。CLI 通过专门的 `Layer` 过滤这个 target（[cli/src/hook/mod.rs](crates/cli/src/hook/mod.rs)）。所以 `RUST_LOG=info` **不会**把 hook 事件打出来——它们是给 UI 用的，不是给人看的。

### 7.2 常用环境变量

| 变量 | 作用 |
|------|------|
| `RUST_LOG` | 标准 tracing filter，如 `info`、`machine=debug,hook=off` |
| `SII_API_KEY` | 默认 gpt-4.1 模型的凭据 |
| `ACCELERATOR_LOG_DIR` | 预留：日志目录（目前 logging.rs 未被调用，留作日后） |

### 7.3 加自己的事件

如果你在新写的 Policy / Tool 里想暴露事件给 tape 看，**不要**直接 `println!`，要这样：

```rust
use machine::hook;
hook!(event = "my_event", arg1 = %value, arg2 = ?debug_value);
```

然后在 [cli/src/hook/mod.rs](crates/cli/src/hook/mod.rs) 的 `HookEvent::from_fields` 里加一条 match 分支，把它翻译成 UI 事件。如果只是想在日志里看到，不必动 UI 代码。

---

## 8. 代码规范要点

完整规矩在 [AGENTS.md](AGENTS.md)，下面是改代码前最容易踩的几条：

- **命名**：禁单字母变量（除了循环 `i`/`j` 和闭包 `|f|`），禁泛型单字母（用 `impl Trait` 或描述性名字），同一概念跨文件名字必须一致。
- **注释**：解释 _why_ 不解释 _what_；不允许 `// defaults to 5 minutes` 这种会过期的具体值注释；小于 200 行的文件不允许 `// ── Foo ──` 分隔符。
- **架构**：`reactor` 是 `machine` 的内部 async 函数，不是 trait 不暴露（§4.8 解释了原因）；`machine` 不知道任何 provider preset（DeepSeek/Groq/...）；Policy 用 `&self`，状态走 atomic，不许用 `Mutex`；所有 IO/LLM 走 `tokio::time::timeout`，超时常量定义为模块级 const。
- **日志**：用 `tracing` 宏，不用 `println!`/`eprintln!`；所有失败必须 `warn!` 出来，要带结构化字段（`?error` 或命名字段），不要拼字符串。
- **测试**：测试在 `tests/`，不许写 `#[cfg(test)] mod tests`；不许测 mock；`#[should_panic]` 必须带 `expected`。
- **执行**：**未经用户确认不许自己改代码**；每批改动必须一个 commit 跑通后再做下一件事；commit 必须原子；不要把别人改的东西混进自己的 commit。
- **研究材料**：所有调研 / 论文 / benchmark 报告放 `research/`，不许放仓库根目录。这个目录已经在 `.gitignore` 里。

---

## 9. 当前已知坑（避免上来就踩）

这几个在仓库当前状态下确实存在，加新功能时容易绕进去：

1. **`shell` 工具的环境变量是空的**。[`shell.rs:83-84`](crates/accelerator/src/tools/shell.rs) 用 `env_clear().envs(&env.vars)`，但 `env.vars` 默认空 HashMap，所以子进程**没有 PATH**。调试 shell 工具时优先在 `Environment::new` 处手动塞 PATH，或者在你的分支里先 fix 这一处。
2. **Captain 一遇 Hitch 就 Done**。LLM 超时 / 网络抖动 / 工具失败包成 Hitch（`Role::System`），Captain 的 `decide` 走"最后一条不是 Tool → Done"分支直接退出。这跟 §4.4 讲的"Hitch 作为数据流"的理念暂时还没落实——开发自定义 Policy 时如果想要重试，自己处理 Hitch。
3. **Hitch 不会被发给 LLM**。[`completion::encode`](crates/machine/src/completion.rs) 对 `Content::Hitch` 返回 `None`，所以即使 hitch 留在 context 里，LLM 也看不到。
4. **`Action::Halt` 在 `apply_action` 里不可达**。如果你看到 `warn!("apply_action received Halt, ...")`，那是防御代码，不要靠它做逻辑。
5. **`Assembly::run` 是顺序的**。即使两个 agent 没有依赖，也不会并行跑（§4.14 末尾提到过）。重活并行要自己改 [assembly.rs](crates/accelerator/src/assembly.rs)（pop 出全部 ready 节点 → `JoinSet::spawn`）。

---

## 10. 提交流程

1. **开工前先 git pull**，确认在干净 branch 上。
2. **改之前先得到用户确认**。AGENTS.md：未经显式请求不许动代码。
3. **每个逻辑改动一个 commit**：
   - 加一个新工具 = 一个 commit
   - 修 shell PATH bug = 一个 commit
   - 不要把 "加工具" 和 "顺手 refactor reactor" 塞同一个 commit。
4. **不要把别人的改动混进来**。`git status` 看到不是你写的（比如别人在改的 Cargo.lock），先 `git restore --staged <file>` 排除。
5. **commit message 风格**看 `git log --oneline`，跟现有风格保持一致。
6. **提交前自检**：
   ```sh
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo build --release
   ```

---

## 11. 下一步去哪

- 想看原版数学推导（含证明性陈述） → [crates/machine/DESIGN.md](crates/machine/DESIGN.md)（本文 §4.13 是它的紧凑中文版，§4.3–§4.12 是通俗讲解 + 设计动机）
- 想理解默认 agent 怎么 boot → [crates/accelerator/src/policy/captain.rs](crates/accelerator/src/policy/captain.rs) 配合 [phases/bootstrap.rs](crates/accelerator/src/policy/phases/bootstrap.rs) 和 [phases/inject.rs](crates/accelerator/src/policy/phases/inject.rs)
- 想加复杂工具，参考最复杂的两个：[tools/fs/](crates/accelerator/src/tools/fs/)（5 个子动作）和 [tools/find/](crates/accelerator/src/tools/find/)（3 个模式 + tree-sitter / ast-grep / regex 混用）
- 想理解多 agent dataflow → [crates/accelerator/src/assembly.rs](crates/accelerator/src/assembly.rs) + [graph.rs](crates/accelerator/src/graph.rs) + [flux.rs](crates/accelerator/src/flux.rs) 一起读

有问题在 issue 里问，或者直接在代码里加 `// FIXME(name):` 然后开 issue。**不要**在代码里留长篇 TODO 注释（AGENTS.md：用 issue tracker，不要 TODO comment）。
