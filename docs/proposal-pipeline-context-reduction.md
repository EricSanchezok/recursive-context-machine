# Proposal: Pipeline Context Reduction

## Motivation

Graph 中 `a.context -> b.context` 传递上游 accelerator 的**完整对话历史**（system prompt、LLM 推理、tool call、tool result、hitch），下游 LLM 收到 10+ 条无关消息，淹没 purpose 指令，进入询问模式而非执行模式。

核心矛盾：上游输出的是**对话过程**，下游需要的是**数据**。

## P1: Flux Last Mode

**动机**：上游 11 cells 对话中，只有最后一个 fragment（LLM 最终总结文本）是下游需要的数据。其余（system、推理、tool call、tool result）是中间态噪音。

**设计**：

`ContextFlux` 新增 `Last` variant。对每个 slot 只取最后一个 fragment，丢弃上游对话历史。

```rust
pub enum ContextFlux {
    Append,   // 全量拼接
    Replace,  // 最后一个非空 context
    Last,     // 每个 slot 只保留最后一个 fragment
}
```

DSL 用法：

```
flux search_result {
    channel = context
    mode = last
    arity = 1
}

search.context -> search_result.slot(0)
search_result.out -> analyst.context
```

**改动范围**：`flux.rs`（apply_ctx 加一个 arm）、parser（识别 `last` 关键字）。不改 machine crate。

## P2: Pipeline-Aware System Prompt

**动机**：Graph 中的子 accelerator 不知道自己在哪个 pipeline、上游完成了什么、自己的角色是什么。LLM 缺乏全局视野。

**设计**：

Graph 执行子 accelerator 前，往 context 里 prepend 一个 `System(tag="pipeline")` fragment：

```
Pipeline: Paper Digest
Upstream: search (✓ found 9 papers on LLM agent memory)
Your role: analyst — download best paper, write analysis
```

内容来源：graph name + 已完成组件的 name/output preview + 当前组件的 name/purpose。

**改动范围**：`graph.rs` 的 `run_frontier`，在 `accelerator.run_with(input)` 前注入 fragment。Phase 层加 guard（`tag="pipeline"` 已存在则跳过）。

## Priority

先 P1 后 P2。P1 改动小、立刻解决实际问题。P2 锦上添花。
