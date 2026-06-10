# Bridge Flux：Channel 间通用数据搬运机制

## 问题

当前 `ContextFlux::Fold` 需要把上游 Context channel 里的 assistant 文本"折叠"进下游的 Purpose channel。因为 Flux 的 input channel = output channel（单一 `channel()`），无法跨 channel 搬运，所以引入了 `State::fold_payload` 作为绕过这个限制的侧通道。

`fold_payload` 的污染范围：
- `state.rs`：新增 `fold_payload: String` 字段 + `Default` 实现
- `flux.rs`：`apply()` 里 Fold 模式的 special-case（提取文本 + 写 fold_payload）
- `accelerator.rs`：`merge_input()` 里的 fold_payload fallback + `fire()` 里的合并到 purpose 逻辑 + context reordering 的 `has_content_to_move` 判断
- `graph.rs`：`state_with_channel()` 和 `set_channel()` 里 fold_payload 的 Context channel 传播
- `tests/graph_names.rs`：4 个 Fold 测试，直接断言 `fold_payload` 字段

## 目标

1. 删除 `State::fold_payload`
2. 删除 `ContextFlux::Fold`
3. 引入通用的 channel 间搬运机制，Fold 行为拆解为 `ContextFlux::Last` + `Bridge` + `PurposeFlux::Append`
4. 清除 `accelerator.rs` 里的 Fold special-case

## 核心设计：`FluxMode::Bridge`

```rust
pub enum FluxMode {
    Purpose(PurposeFlux),
    Context(ContextFlux),
    Environment(EnvFlux),
    Resources(ResFlux),
    /// Cross-channel bridge: reads from `from` channel, transforms data,
    /// writes to `to` channel.
    Bridge {
        from: Channel,
        to: Channel,
        kind: BridgeKind,
    },
}

pub enum BridgeKind {
    /// For each slot: extract the last assistant text from context,
    /// join with "\n\n", and write as a purpose string.
    ContextLastTextToPurpose,
}
```

## 行为对比

### 旧方案（Fold）

```
Accel A --[Context]--> Flux(Fold) --[Context, fold_payload]--> Accel B
                                                    ↑
                                        fold_payload = extracted text
```

Accel B 的 `fire()` 内：`combined = fold_payload + "\n\n" + base_purpose`

### 新方案（Last + Bridge）

```
Accel A --[Context]--> Flux(Last) --[Context]--> Bridge(Context→Purpose, LastText) --[Purpose]--> Accel B
```

Bridge 直接将提取文本写入 `state.purpose`。Accel B 的 `merge_input()` 收到非空 purpose，与 base.purpose 合并。

## 关键问题：拼接下游 base purpose

Bridge 写入 `state.purpose` 后，这个 purpose 会通过 Purpose channel 传播到下游 accelerator。下游 accelerator 的 `merge_input()` 当前逻辑是：

```rust
if state.purpose.is_empty() {
    state.purpose.clone_from(&base.purpose);
}
```

如果 Bridge 已经写入 purpose，`base.purpose` 就被跳过了。解决方案：**当 graph 提供的 purpose 和 base purpose 都非空时，拼接它们**。

### `accelerator.rs` 改动

```rust
fn merge_input(&self, input: State) -> State {
    let mut state = input;
    if let AcceleratorBody::Primitive(primitive) = &self.body {
        let base = &primitive.state;
        // NEW: if both graph-wired purpose and base purpose are present,
        // concatenate: upstream result first, then the actual task.
        let base_purpose = base.purpose.clone();
        if !state.purpose.is_empty() && !base_purpose.is_empty()
            && state.purpose != base_purpose
        {
            state.purpose = format!("{}\n\n{}", state.purpose, base_purpose);
        } else if state.purpose.is_empty() {
            state.purpose.clone_from(&base_purpose);
        }
        // ... ctx, env, res merge unchanged (no more fold_payload references)
    }
    state
}
```

**为什么不引入 `PurposeFlux::Prepend`？** 因为下游 accelerator 的 base purpose 是 internal state，不是 graph port。要让一个 Flux 拼接它，需要先把它暴露为 port。这属于更大的架构调整（方案 B），当前阶段用 `merge_input` 的语义变更就够了。

## 详细改动清单

### 1. `flux.rs` — FluxMode 扩展

- 新增 `FluxMode::Bridge { from, to, kind }` 变体
- 新增 `BridgeKind::ContextLastTextToPurpose`
- `channel()` 改为返回 `(input_channel, output_channel)` pair；现有变体返回 `(X, X)`，Bridge 返回 `(from, to)`
- `apply()` 新增 Bridge 分支：读取 from channel 的 slot 数据 → 执行 BridgeKind 对应的 transform → 写入 to channel
- 删除 `ContextFlux::Fold` 变体
- `apply_ctx()` 删除 Fold 空 context 分支
- `apply()` 删除 Fold 的 fold_payload 特殊处理
- `name()` 新增 `"bridge_context_last_to_purpose"`

### 2. `state.rs` — 删除 fold_payload

- 删除 `pub fold_payload: String` 字段
- 删除 `Default` 中的对应初始化
- 删除 doc comment

### 3. `accelerator.rs` — 清除 Fold special-case

- `merge_input()`：删除 `fold_payload` 的 fallback 逻辑（`if state.fold_payload.is_empty()` 块）
- `merge_input()`：新增 purpose concatenation 逻辑
- `fire()`：删除 `let fold_payload = state.fold_payload.clone()` 
- `fire()`：删除 `combined_purpose = format!("{}\n\n{}", fold_payload, base_purpose)`
- `fire()`：简化 `has_content_to_move` / `needs_reorder` 判断（不再区分 fold vs non-fold）

### 4. `graph.rs` — 传播逻辑

- `state_with_channel()`：删除 Context 分支中的 `fold_payload.clone_from()` 
- `set_channel()`：删除 Context 分支中的 `fold_payload =`
- `validate_flux_inputs()`：改用 `flux.mode.output_channel()` 验证 FluxOut wire 的 channel 匹配（如果有直接 wire 到 FluxOut 的验证逻辑）——实际上当前 validate_flux_inputs 只检查 slot 填充，wire channel 匹配在 compile.rs 的 `validate_wire` 里

### 5. `wire.rs` — Endpoint 相关

- `Endpoint::FluxSlot { slot, channel }`：channel 的含义变为 input_channel（不变）
- `Endpoint::FluxOut(channel)`：channel 的含义变为 output_channel（不变）
- 两者通过 `FluxMode::channels()` 获取正确值。wire 构建由 compile.rs 负责。

### 6. `compile.rs` — RCM 语法编译

- `flux_mode_from_def()` 新增 Bridge 变体的解析：
  ```
  flux my_bridge {
      channel = "bridge"
      from = "context"
      to = "purpose"  
      mode = "last_text"
      arity = 1
  }
  ```
- `ComponentTag::Flux { channel, arity }` 扩展为两个 channel：
  ```rust
  ComponentTag::Flux { input_channel: Channel, output_channel: Channel, arity: usize }
  ```
- `validate_wire()`：FluxOut wire 的 channel 检查使用 `output_channel` 而不是 `input_channel`
- RCM parser：`FluxDef` 新增 `from` / `to` 字段（可选，仅 Bridge 需要）
- RCM AST：`FluxDef` 新增字段

### 7. `tests/graph_names.rs` — 测试更新

- 4 个 `context_fold_*` 测试重写为 Bridge 等价测试：
  - 不再断言 `fold_payload` 字段
  - 改为构建 Last+Bridge 链并验证最终 purpose
- 新增 Bridge 专用测试

## 实际效果

用户可以在 RCM 里写：

```
graph composite {
    flux fold {
        channel = "bridge"
        from = "context"
        to = "purpose"
        mode = "last_text"
        arity = 1
    }

    // 等价于旧的 context_fold:
    //   flux fold { channel = "context" mode = "fold" arity = 1 }
}
```

或者组合成两段式（如果用户想复用 Last 的 context 输出）：

```
graph composite {
    flux last_extract { channel = "context" mode = "last" arity = 1 }
    flux bridge { channel = "bridge" from = "context" to = "purpose" mode = "last_text" arity = 1 }
    
    search -> last_extract.context -> bridge.context
    bridge.purpose -> writer.purpose
}
```

## 扩展性

未来如果出现其他跨 channel 搬运需求（如 `Resources→Context`、`Context→Environment`），只需新增 `BridgeKind` 变体，无需触碰 State/Accelerator/Graph 核心。

| BridgeKind | from | to | 行为 |
|---|---|---|---|
| `ContextLastTextToPurpose` | Context | Purpose | 提取最后一段 assistant text，拼成 purpose |
| `ContextDigestToPurpose`（未来） | Context | Purpose | Digest 后写入 purpose |
| `ContextToEnvironment`（未来） | Context | Environment | 提取 context 中的文件路径，设定 cwd |

## 疑问与决策点

1. **`merge_input` 的 purpose 拼接语义**：当前默认是"input 空则 fallback 到 base"。改为拼接后，是否影响其他非 Fold 场景的 Purpose channel 传播行为？需要确认现有 tests 是否依赖旧语义。

2. **Bridge 的 slot 语义**：Bridge 读的是 from channel 的 slot 数据，但 slot 的 channel 在 `ComponentRef::slot(slot, channel)` 里指定。这意味着 graph build 时 wire 的 channel 必须匹配 Bridge 的 input_channel。这个匹配由 compile.rs 保证。

3. **是否需要支持纯 graph 组合**（不用 Bridge）：如果下游 accelerator 的 base purpose 被暴露为 graph port，用户就可以用 `Flux(Last) + Flux(Concat)` 纯组合实现，完全不需要 Bridge。这属于方案 B，当前先不展开。
