# Proposal: Mixture of Evolving Harness (MoEH)

> **状态**：草案，待评审
> **基于**：当前 main 分支
> **参考**：
> - Shazeer et al. (2017) "Outrageously Large Neural Networks" (MoE)
> - Fedus et al. (2021) "Switch Transformer"
> - Jiang et al. (2024) "Mixtral 8x7B"
> - DeepSeek-MoE (2024)
> - Schulman et al. (2017) "PPO"
> - Shao et al. (2024) "DeepSeekMath" (GRPO)
> - ACE (ICLR 2026), AHE (May 2026), TF-GRPO (Oct 2025)

---

## 0. 一句话总结

**用 Mixture-of-Evolving-Harness（MoEH）小模型作为 advisor，为 LLM Executor 提供 step-level action distribution prior**。8 个 expert slot 通过 GRPO 训练 emergent 分化（不预设语义），每个 step 选 top-2 expert 加权融合作为 soft prompt hint。**LLM 始终保留 12 种 Action 的完整选择自由**。整个 evolve 通过 RCM 原生 Action 空间 + MoEH advisor + 跨 task `SetPrompt`（等价 Replace captain fragment）累积经验实现。

**双模式部署**：训练阶段**全程 gRPC**——通过 RCM gRPC server（[crates/server/DESIGN.md](../../crates/server/DESIGN.md)）暴露 Machine 给 Python trainer，外部驱动 step-by-step，Hybrid Policy（MoEH advisor + Python LLM client）在客户端组合 action；Production 部署用 CLI 模式（Captain Policy + ActionPriorPhase 内置）。

---

## 1. 第一性原理：Capacity-Prompt Decoupling

### 1.1 LLM MoE 的本质

LLM MoE（Mixtral 8x7B 等）解决的是 **"模型知识量"和"推理时计算量"的解耦问题**：

```
Dense Model 的困境:
   capacity = compute
   想要更强（更多 params）→ inference 必然变慢（更多 FLOPs）
   
LLM MoE 解法:
   capacity ↗ compute → 通过 sparse activation 解耦
   
Mixtral 8x7B:
   总参数: 47B (= capacity)
   每次激活: ~13B (top-2 of 8, = compute)
   ratio: 47/13 ≈ 3.6× 容量加成，FLOPs 不变
```

**核心 insight**：把"模型能容纳多少知识"和"推理时算多少 FLOPs"两个原本耦合的指标**解开**。

### 1.2 Agent Harness 的对应痛点

把第一性原理迁移到 agent harness：

| 维度 | LLM 内部 MoE | **MoEH (Harness)** |
|------|--------------|-------------------|
| 想 scale 的资源 | 参数（知识容量） | **agent 累积的经验/策略** |
| 不想增长的资源 | inference FLOPs | **LLM 当前看到的 prompt 长度 + 注意力带宽** |
| 解耦机制 | top-k FFN expert 激活 | **top-k expert hint 注入** |
| Scaling 法则 | 总 params × FLOPs only top-k | **总 evolve 经验 × prompt only top-k** |

**为什么 prompt 长度是 agent 真正的约束**：

1. **物理 context window 有上限**（即使 1M token 也远不能装下"所有人类经验"）
2. **注意力稀释更严重**：
   - "Lost in the middle" 现象（Liu et al. 2023）
   - 长 context 性能不等比例提升甚至下降
   - Token 数翻倍 = LLM API 成本翻倍
3. **ACE paper 的 "context collapse"**：LLM 自己 curate 长 playbook 时，monolithic rewrite 让经验越改越模糊

**single-prompt agent 的困境**：

```
经验越多 → prompt 越长 → 注意力被无关经验稀释 → 性能下降
→ 经验累积进入负收益区
```

### 1.3 MoEH 解决什么

```
MoEH 让"agent 总经验量"和"LLM 当前 prompt 长度"解耦:

   经验 ~ N × s_expert        (8 expert × 各自 distilled 经验)
   prompt ~ k × s_expert      (LLM 看 top-k=2 expert 的内容)
   ratio ~ N / k = 4×

具体数字:
   单 prompt 系统:   50K tokens 总经验 = 50K tokens prompt
   MoEH 系统:        8 × 6K = 48K 总经验, 2 × 6K = 12K prompt
                     experience ↑ 96%, prompt ↓ 76%
   
   未来 scale 时:    16 × 6K = 96K 总经验, 仍然 12K prompt
                     experience ↑ 92%, prompt 不变
```

**MoEH 比 LLM MoE 多一层动态性**：

```
LLM MoE: experts 是 frozen weights（trained once, deployed forever）
MoEH:    experts 是 evolving prior distributions
         (通过 GRPO 持续训练 + 通过 SetPrompt 累积经验内容)
```

Capacity 可以在 deployment 中持续生长——MoEH 是 online learning system。

### 1.4 这个 story 对设计的 implication

- **Experts 必须承载实质 capacity**（不只是"风格 hint"）
- **Experts 必须持续 evolve**（通过 GRPO + SetPrompt）
- **Router 必须 specialize**（top-k 必须是稀疏激活，否则失去解耦意义）
- **k 必须 << N**（k=2, N=8 是 Mixtral 标准 sweet spot）
- **单 expert 容量有上限**（超过时分裂/compact，避免吃掉 prompt budget）

---

## 2. 核心思想

### 2.1 不预设 expert 分类

**错误做法**（拒绝）：

```
expert_0 = "coding harness"
expert_1 = "writing harness"
expert_2 = "research harness"
...
```

工程师拍脑袋分类的问题：
- 任务级分类完全不需要 mixture——任务开头判断 task type 后直接用对应 harness 即可
- 真实的 agent 演化空间是**连续的、无限的**
- 工程师 taxonomy 必然 lossy

**正确做法**（采用）：

- 8 个 expert slot **不预设语义**
- 每个 expert 是 small MLP，输入 state，输出 12-dim action probability distribution
- 通过 GRPO + load balancing + diversity loss 训练，experts 自然分化
- Specialization 是 **emergent**——某 expert 可能学到"recent_hitch > 0 时偏好 Model"，但没人预设
- **命名是 post-hoc 解读**（如果想命名的话）

这跟 LLM 内部 MoE 完全同构——Mixtral 没人告诉 expert3 学"语法"，experts 通过训练自然 specialize。

### 2.2 Step-level，不是 task-level

Harness 在 trajectory 上**每个 step 都可能不同**。mode 其实只是 harness 在某段时间的事后投影，**不是预设的状态机**。

这跟 RCM 现有 Action 空间完全契合——`Activate / Deactivate / Model / SetPrompt` 本来就允许 step-level 自由修改 harness 任意组件。MoEH 仅提供**每 step 的 advisor hint**，不强制 mode 切换。

### 2.3 Advisor 而非 Controller

```
                State (context + env + resources + inbox + macro_obs)
                                    │
                          ┌─────────┴──────────┐
                          ▼                    ▼
                  ┌──────────────┐    ┌────────────────────┐
                  │   MoEH       │    │  LLM Executor      │
                  │   Advisor    │    │  (DeepSeek-V3.1)   │
                  │   (~2M)      │    │                    │
                  │              │    │                    │
                  │ Output:      │    │ Input: context +   │
                  │ Action       │ ─→ │  prior hint        │
                  │ distribution │    │ Output: Action     │
                  └──────────────┘    │  (12 variants)     │
                                      └────────────────────┘
                                              │
                                              ▼
                                       Apply Action
```

**MoEH 是 advisor，不是 controller**——LLM 看 hint 后**自由决策**：
- 可以采纳（hint 推什么就发什么）
- 可以部分采纳（hint 推 Halt 但 LLM 选 Take）
- 可以完全 override（hint 推 Halt 但 LLM 输出 Done）

LLM 仍然在 12 种 Action 里**完整选择**。

### 2.4 为什么 hint 通过 prompt 影响 LLM

不是让 router 直接发 action。三条核心理由：

**理由 1: Content 生成必须靠 LLM**

4/12 个 Action 需要生成 content（Append / Insert / Replace / SetPrompt）。小模型（2M params）写不出有意义的 captain prompt——这是 LLM 才有的能力。

如果 router 直接发 action，仍要 fallback 调 LLM 生成 content。统一让 LLM 看 hint 完整决策 + 生成 content，单一路径。

**理由 2: 避免吕布骑狗**

如果 router 直接决定 action type（即使不生成 content），**router 的能力上限就是整个 agent 的能力上限**。2M params 跟 671B params 的 DeepSeek-V3.1 差几个数量级。

Hint 形式打破这个限制——LLM 仍可完全 override hint。

**理由 3: 跨 type 比较能力**

LLM 看到 "Halt 35% / Append 30% / Take 20%" 这种 distribution，能综合判断"应该 Halt 还是 Append"。

如果拆成 "router 决定 type + LLM 只生成 content if needed" 两个 module，LLM **失去跨 type 比较能力**——只在 router 决定的 type 内决策，看不到其他 type 的可能性。

---

## 3. 决策空间建模

### 3.1 形式化定义

RCM Context Machine 形式化：$\Phi = \omega \circ \pi$。

MoEH 把 $\pi$ 拆成两层（advisor + executor）：

$$\pi(s) = \pi_{\text{LLM}}(s, h)\quad \text{where}\quad h = \text{format}(\pi_{\text{MoEH}}(s))$$

| 符号 | 含义 |
|------|------|
| $s = (c, e, r, p, m)$ | State |
| $\pi_{\text{MoEH}}: s \to \mathbb{P}^{12}$ | MoEH advisor 输出 action probability distribution |
| $h = \text{format}(\pi_{\text{MoEH}}(s))$ | 把 distribution 格式化成自然语言 hint fragment |
| $\pi_{\text{LLM}}: (s, h) \to a$ | LLM Executor 看 state + hint 输出 action |
| $a \in \mathcal{A}$ | Action，12 个 variant |
| $\omega$ | Reactor（不变） |

**性质保留**：

- **Markov**: $s_{t+1}$ 只依赖 $s_t$
- **确定性**：$\Phi$ 框架确定（LLM 随机性装在 $\pi_{\text{LLM}}$ 内）
- **离散有限 Action**: $|\mathcal{A}| = 12$，可 RL

### 3.2 State Space

**State** $s = (c, e, r, p, m)$ 五元组：

| 符号 | 含义 | 类型 |
|------|------|------|
| $c$ | Context (ordered fragment sequence) | `Vec<Fragment>` |
| $e$ | Environment (cwd, vars, root, platform) | `Environment` |
| $r$ | Resources (tools, models, prompts, active_tools, active_model) | `Resources` |
| $p$ | Pending Inbox (FIFO fragment queue) | `Inbox` |
| $m$ | MacroObservation (派生的自我状态感) | `MacroObservation` |

详细字段见 §4。

### 3.3 Action Space

Action enum 12 个 variant，含新增 `SetPrompt`：

| Action | 参数 | 语义 |
|--------|------|------|
| `Append(Fragment)` | fragment | context 末尾追加 |
| `Insert{after, fragment}` | id, fragment | 插入到 id 之后 |
| `Replace{id, fragment}` | id, fragment | 替换并保留 id |
| `Remove(id)` | id | 删除 |
| `Swap(id1, id2)` | id1, id2 | 交换位置 |
| `Take` | — | inbox 头部弹一个进 context |
| `Halt` | — | 触发 reactor (LLM + tools) |
| `Done` | — | 终止 task |
| `Model(name)` | model_name | 切换 active model |
| `Activate(tool)` | tool_name | 激活工具 |
| `Deactivate(tool)` | tool_name | 停用工具 |
| **`SetPrompt{name, content}`** | name, content | **修改 prompt (新增)** |

### 3.4 Action 索引化

```rust
pub const ACTION_NAMES: [&str; 12] = [
    "Append", "Insert", "Replace", "Remove", "Swap",
    "Take", "Halt", "Done",
    "Model", "Activate", "Deactivate", "SetPrompt",
];
```

MoEH 输出 12-dim distribution over action types。LLM 在 hint 引导下决定具体 action（含 type + content）。

---

## 4. Model Input 设计

### 4.1 MacroObservation（结构化低维）

**关键 insight**：尽管 context 中可以推断这些信息，但显式提供"自我感知"让 router 更便捷判断 task 状态（如是否 stuck）。

```rust
pub struct MacroObservation {
    pub step_count: u32,                  // 总 step 数
    pub fragment_count: u32,              // |c|
    pub recent_hitch_count: u32,          // 最近 W=10 步内 Hitch 数
    pub inbox_size: u32,                  // |p|
    pub tokens_used_total: u64,           // 累计 token 消耗
    pub time_since_last_halt: u32,        // 距上次 Halt 的 step 数
    pub active_tools_count: u32,          // 当前激活工具数
    pub steps_in_current_task: u32,       // 当前 task 跑了多久
}
```

**编码到向量**（8 dim, normalized）：

```python
def encode_macro_obs(obs: MacroObservation) -> torch.Tensor:
    return torch.tensor([
        obs.step_count / 50.0,
        obs.fragment_count / 50.0,
        obs.recent_hitch_count / 10.0,
        obs.inbox_size / 10.0,
        obs.tokens_used_total / 50000.0,
        obs.time_since_last_halt / 10.0,
        obs.active_tools_count / 20.0,
        obs.steps_in_current_task / 100.0,
    ])  # [8]
```

### 4.2 Purpose Embedding（task 级 cache）

```python
purpose_emb = qwen3_embedding(state.purpose)  # [1024]
# task 启动时算一次，整个 task 复用
```

Cache 整个 task——purpose 不变。

### 4.3 Context: Per-fragment Embedding + Position-Aware Pool

**每个 fragment 独立 embed**（保留每条语义）：

```python
def encode_context(ctx: Context) -> torch.Tensor:
    """
    Returns: [N, 1024] tensor of per-fragment embeddings
    """
    embs = []
    for frag in ctx.fragments():
        text = format_fragment_for_embedding(frag)
        emb = qwen3_embedding_with_cache(text)  # 缓存命中率 ~90%
        embs.append(emb)
    return torch.stack(embs)  # [N, 1024]

def format_fragment_for_embedding(frag: Fragment) -> str:
    return f"<role={frag.role}><tag={frag.tag}> {frag.content_text()}"
```

**Position-Aware Attention Pool**（context 顺序有语义，最近的更重要）：

```python
class PositionAwarePool(nn.Module):
    def __init__(self, d=128, max_seq=100):
        super().__init__()
        self.query = nn.Parameter(torch.randn(1, 1, d) * 0.02)
        self.pos_emb = nn.Embedding(max_seq, d)
        self.attn = nn.MultiheadAttention(d, 1, batch_first=True)
    
    def forward(self, seq, mask):
        # seq: [B, N, d], mask: [B, N] (True=valid)
        B, N, _ = seq.size()
        if N == 0:
            return torch.zeros(B, seq.size(-1), device=seq.device)
        # 加 position embedding (最新 fragment 在末尾，对应高 position idx)
        positions = torch.arange(N, device=seq.device).clamp(max=99)
        seq = seq + self.pos_emb(positions).unsqueeze(0)
        # Attention pool with learnable query
        q = self.query.expand(B, -1, -1)
        pooled, _ = self.attn(q, seq, seq, key_padding_mask=~mask)
        return pooled.squeeze(1)  # [B, d]
```

### 4.4 Inbox: Per-msg Embedding + Attention Pool

```python
def encode_inbox(inbox: Inbox) -> torch.Tensor:
    """
    Returns: [K, 1024] tensor
    """
    embs = []
    for msg in inbox.fragments():
        text = format_fragment_for_embedding(msg)
        emb = qwen3_embedding_with_cache(text)
        embs.append(emb)
    return torch.stack(embs) if embs else torch.empty(0, 1024)
```

Inbox 顺序也有语义但不那么强。用 **AttentionPool with Learnable Query**（无 position embedding）：

```python
class AttentionPool(nn.Module):
    def __init__(self, d=128):
        super().__init__()
        self.query = nn.Parameter(torch.randn(1, 1, d) * 0.02)
        self.attn = nn.MultiheadAttention(d, 1, batch_first=True)
    
    def forward(self, seq, mask):
        B = seq.size(0)
        if seq.size(1) == 0:
            return torch.zeros(B, seq.size(-1), device=seq.device)
        q = self.query.expand(B, -1, -1)
        pooled, _ = self.attn(q, seq, seq, key_padding_mask=~mask)
        return pooled.squeeze(1)
```

### 4.5 Tools: Per-tool Embedding (Active 标注) + Attention Pool

**Tool 用 embedding 比 one-hot 强**：

- 携带语义信息（"read 工具" vs "write 工具"）
- 新增 tool 无需重训（embedding 已学到通用语义）
- 跨项目可迁移（同名 tool 对应同 embedding）

```python
def encode_tools(resources: Resources) -> torch.Tensor:
    """
    Returns: [M, 1024] tensor.
    Active tools 在文本里特殊标注，让 embedding 学到 "active" 语义。
    """
    embs = []
    for tool_name, tool in resources.tools.items():
        is_active = tool_name in resources.active_tools
        text = format_tool_for_embedding(tool, is_active)
        emb = qwen3_embedding_with_cache(text)  # tool desc 几乎不变，命中率 ~100%
        embs.append(emb)
    return torch.stack(embs)

def format_tool_for_embedding(tool: Tool, is_active: bool) -> str:
    status = "ACTIVE" if is_active else "INACTIVE"
    return (
        f"[{status}] {tool.name()}: {tool.description()}\n"
        f"Parameters: {tool.parameters_schema_summary()}"
    )
```

Tool 无序，用 AttentionPool（不需要 position embedding）。

### 4.6 Models: Per-model Embedding (Active 标注) + Attention Pool

类似 tools：

```python
def encode_models(resources: Resources) -> torch.Tensor:
    embs = []
    for model_name, model in resources.models.items():
        is_active = (model_name == resources.active_model)
        text = format_model_for_embedding(model, is_active)
        emb = qwen3_embedding_with_cache(text)
        embs.append(emb)
    return torch.stack(embs)  # [P, 1024]

def format_model_for_embedding(model: Model, is_active: bool) -> str:
    status = "ACTIVE" if is_active else "INACTIVE"
    return (
        f"[{status}] {model.name} ({model.protocol})\n"
        f"Context length: {model.context_length}, Cost tier: {model.cost_tier}"
    )
```

### 4.7 Env Embedding

```python
def encode_env(env: Environment) -> torch.Tensor:
    text = format_env_for_embedding(env)
    return qwen3_embedding_with_cache(text)  # [1024]

def format_env_for_embedding(env: Environment) -> str:
    return (
        f"cwd: {env.cwd}\n"
        f"platform: {env.platform}\n"
        f"root: {env.root}\n"
        f"vars_count: {len(env.vars)}"
    )
```

Env 通常每 task 不变，cache 命中率高。

---

## 5. Fusion Architecture

### 5.1 整体架构图

```
═══════════════════════════════════════════════════════════════════════
                 Hierarchical Variable-Length Fusion
═══════════════════════════════════════════════════════════════════════

Level 0: Per-unit Embedding (qwen3-embedding-0.6b, frozen, with cache)
  
  Context  N fragments → embed each → [N × 1024]
  Inbox    K messages  → embed each → [K × 1024]
  Tools    M tools     → embed each → [M × 1024]  (启动时 cache)
  Models   P models    → embed each → [P × 1024]  (启动时 cache)
  Purpose  1 string    → embed      → [1024]      (task 级 cache)
  Env      structured  → embed      → [1024]      (task 级 cache)
  MacroObs structured  → encode     → [8]         (每步重算)
                            │
                            ▼
Level 1: Per-modality Adapter + Attention Pool
  
  Each modality:
    Adapter: Linear(1024, d=128) → [N × 128]
    Per-modality pool → [128]
  
  Specifically:
    Context  [N, 1024] → adapter → [N, 128] → PositionAwarePool → [128]
    Inbox    [K, 1024] → adapter → [K, 128] → AttentionPool     → [128]
    Tools    [M, 1024] → adapter → [M, 128] → AttentionPool     → [128]
    Models   [P, 1024] → adapter → [P, 128] → AttentionPool     → [128]
    Purpose  [1024]    → adapter                                → [128]
    Env      [1024]    → adapter                                → [128]
    MacroObs [8]       → MLP(8→128)                             → [128]
                            │
                            ▼
  7 modality tokens, each [128]
                            │
                            ▼
Level 2: Cross-modality Self-Attention
  
  Stack 7 tokens → [B, 7, 128]
  + modality_type_embedding (区分 7 种 modality 类型)
  ↓
  1-layer self-attention (2 heads, no FFN)
  + residual + LayerNorm
                            │
                            ▼
  attended tokens [B, 7, 128]
                            │
                            ▼
  Flatten → [B, 7 × 128 = 896]
                            │
                            ▼
              ┌────────────────────────────────┐
              │   Router + 8 Experts            │
              │   (next section)                │
              └────────────────────────────────┘
```

### 5.2 Level 1: Per-modality Pool (变长 → 定长)

每个 modality 内部 pool，把变长 sequence 压成定长 token。

**Choice of pool 策略**：

| Modality | 序列长度 | 有顺序 | 推荐 Pool |
|----------|---------|--------|----------|
| Context | 长 (N=10-100) | ✓ | **Position-Aware Pool** |
| Inbox | 短 (K=0-5) | ✓（弱） | **Attention Pool** |
| Tools | 中 (M=5-20) | ✗ | **Attention Pool** |
| Models | 短 (P=1-5) | ✗ | **Attention Pool** |
| Purpose | 1 | — | **No pool** (直接用) |
| Env | 1 | — | **No pool** |
| MacroObs | structured | — | **MLP encoder** |

### 5.3 Level 2: Cross-modality Self-Attention

让不同 modality 互相 attend，学到 cross-modal interaction（如 purpose × tools 关联）：

```python
class CrossModalityAttention(nn.Module):
    def __init__(self, d=128, n_modalities=7, n_heads=2):
        super().__init__()
        self.modality_emb = nn.Embedding(n_modalities, d)
        self.attn = nn.MultiheadAttention(d, n_heads, batch_first=True)
        self.norm = nn.LayerNorm(d)
        # 注意：没有 FFN（避免参数膨胀；下游 Router/Expert 是 FFN）
    
    def forward(self, modality_tokens):
        # modality_tokens: [B, 7, d]
        B = modality_tokens.size(0)
        type_ids = torch.arange(7, device=modality_tokens.device)
        tokens = modality_tokens + self.modality_emb(type_ids).unsqueeze(0)
        
        attended, attn_weights = self.attn(tokens, tokens, tokens)
        return self.norm(tokens + attended)  # [B, 7, d]
```

**为什么 1 layer no FFN**：
- 7 个 modality 不需要 transformer 的 capacity
- 下游 Router/Expert 已经是 FFN，再加 FFN 是冗余
- FFN 占 transformer block 大头参数，省掉减少 ~70% fusion 参数

### 5.4 完整 Shape Spec

```
═══════════════════════════════════════════════════════════════════════
                       Fusion Shape Spec
═══════════════════════════════════════════════════════════════════════

Per-unit Embedding (frozen):
  qwen3-embedding-0.6b output: 1024-dim

Per-modality Adapter (5 individual + 2 structured):
  frag_adapter:    Linear(1024, 128)    → 131,200 params
  inbox_adapter:   Linear(1024, 128)    → 131,200 params
  tool_adapter:    Linear(1024, 128)    → 131,200 params
  model_adapter:   Linear(1024, 128)    → 131,200 params
  purpose_adapter: Linear(1024, 128)    → 131,200 params
  env_adapter:     Linear(1024, 128)    → 131,200 params
  macro_encoder:   MLP(8 → 64 → 128)    → 9,024 params
  Adapter total:                          796,224 params (~796K)

Per-modality Attention Pools:
  PositionAwarePool (Context):
    query [128] + pos_emb [100×128] + attn (4×128×128+128) = 78,720 params
  AttentionPool (Inbox/Tools/Models, ×3):
    query [128] + attn (4×128×128+128) = 65,920 per pool × 3 = 197,760 params
  Pool total:                             276,480 params (~276K)

Cross-modality Attention (Level 2):
  modality_type_emb [7×128]:              896
  MultiheadAttention (2 heads, d=128):
    Q/K/V/O proj 4×(128×128+128) = 66,048
  LayerNorm [128×2]:                      256
  Cross-attn total:                       67,200 params (~67K)

Fusion total (excluding embedding):    ~1,139,904 params (~1.14M)
```

**重要**：Embedding model (qwen3-embedding-0.6b, ~600M params) 是 **frozen**，不算 trainable params。MoEH 只训 fusion + router + experts。

---

## 6. Router + Experts

### 6.1 Router 设计

```python
class Router(nn.Module):
    """
    Input: fused state [B, 896]
    Output: routing probabilities [B, 8]
    """
    def __init__(self, input_dim=896, hidden=256, n_experts=8):
        super().__init__()
        self.l1 = nn.Linear(input_dim, hidden)
        self.l2 = nn.Linear(hidden, n_experts)
        # Noisy top-k (Switch Transformer style)
        self.noise_scale = nn.Parameter(torch.ones(n_experts) * 0.1)
    
    def forward(self, fused, training=True):
        x = F.gelu(self.l1(fused))               # [B, 256]
        logits = self.l2(x)                       # [B, 8]
        if training:
            noise = torch.randn_like(logits) * F.softplus(self.noise_scale)
            logits = logits + noise
        probs = F.softmax(logits, dim=-1)         # [B, 8]
        return probs, logits

# Params: 896*256 + 256 + 256*8 + 8 + 8 = 231,176 (~231K)
```

### 6.2 Expert 设计

```python
class Expert(nn.Module):
    """
    Input: fused state [B, 896]
    Output: action distribution [B, 12]
    """
    def __init__(self, input_dim=896, hidden=128, n_actions=12):
        super().__init__()
        self.l1 = nn.Linear(input_dim, hidden)
        self.l2 = nn.Linear(hidden, n_actions)
    
    def forward(self, fused):
        x = F.gelu(self.l1(fused))               # [B, 128]
        logits = self.l2(x)                       # [B, 12]
        return F.softmax(logits, dim=-1)

# Each expert params: 896*128 + 128 + 128*12 + 12 = 116,236
# 8 experts: 929,888 (~930K)
```

### 6.3 Top-k 选择

```python
# Inference: hard top-2
top_k_vals, top_k_idx = router_probs.topk(2, dim=-1)
top_k_weights = F.softmax(top_k_vals, dim=-1)

# 只跑 top-2 experts
mixed = sum(
    top_k_weights[:, k:k+1] * experts[top_k_idx[:, k]](fused)
    for k in range(2)
)

# Training: soft routing (跑所有 experts，权重 mask top-k)
# 见 §7 PyTorch 完整实现
```

### 6.4 Mixing

最终输出 mixed action distribution：

```
mixed_dist = w_a × expert_a_dist + w_b × expert_b_dist
shape: [B, 12]
```

### 6.5 完整参数量

```
┌────────────────────────────────────────────────────┐
│  Component                | Params      | %       │
├────────────────────────────────────────────────────┤
│  Per-modality Adapters    |   796K      | 35.6%   │
│  Per-modality Pools       |   276K      | 12.4%   │
│  Cross-modality Attention |    67K      |  3.0%   │
│  Router                   |   231K      | 10.3%   │
│  Experts (×8)             |   930K      | 41.6%   │
├────────────────────────────────────────────────────┤
│  TOTAL                    | 2,300K      | 100%    │
│                           | ≈ 2.3M params           │
└────────────────────────────────────────────────────┘

Inference (FP32): ~9 MB weights
Training (with optimizer state): ~36 MB
CPU inference: <2ms per forward
```

---

## 7. PyTorch 完整实现

```python
import torch
import torch.nn as nn
import torch.nn.functional as F

# === Hyperparameters ===
EMB_DIM = 1024              # qwen3-embedding-0.6b output dim
D = 128                     # unified modality dim (Level 1 output)
MAX_CONTEXT_LEN = 100       # max fragments for position embedding
N_EXPERTS = 8
TOP_K = 2
N_ACTIONS = 12
N_MODALITIES = 7
MACRO_INPUT_DIM = 8

# === Attention Pools ===

class AttentionPool(nn.Module):
    """Learnable query attention pool (no position)."""
    def __init__(self, d=D):
        super().__init__()
        self.query = nn.Parameter(torch.randn(1, 1, d) * 0.02)
        self.attn = nn.MultiheadAttention(d, 1, batch_first=True)
    
    def forward(self, seq, mask):
        B = seq.size(0)
        if seq.size(1) == 0:
            return torch.zeros(B, seq.size(-1), device=seq.device)
        q = self.query.expand(B, -1, -1)
        pooled, _ = self.attn(q, seq, seq, key_padding_mask=~mask)
        return pooled.squeeze(1)

class PositionAwarePool(nn.Module):
    """With learned position embedding for ordered sequences."""
    def __init__(self, d=D, max_seq=MAX_CONTEXT_LEN):
        super().__init__()
        self.query = nn.Parameter(torch.randn(1, 1, d) * 0.02)
        self.pos_emb = nn.Embedding(max_seq, d)
        self.attn = nn.MultiheadAttention(d, 1, batch_first=True)
    
    def forward(self, seq, mask):
        B, N, _ = seq.size()
        if N == 0:
            return torch.zeros(B, seq.size(-1), device=seq.device)
        positions = torch.arange(N, device=seq.device).clamp(max=MAX_CONTEXT_LEN-1)
        seq = seq + self.pos_emb(positions).unsqueeze(0)
        q = self.query.expand(B, -1, -1)
        pooled, _ = self.attn(q, seq, seq, key_padding_mask=~mask)
        return pooled.squeeze(1)


# === Hierarchical Fusion ===

class HierarchicalFusion(nn.Module):
    """
    Level 1: per-modality attention pool (内部 pool 变长 sequence)
    Level 2: cross-modality self-attention (跨 modality interaction)
    """
    def __init__(self):
        super().__init__()
        
        # === Per-modality Adapters (1024 → 128) ===
        self.frag_adapter    = nn.Linear(EMB_DIM, D)
        self.inbox_adapter   = nn.Linear(EMB_DIM, D)
        self.tool_adapter    = nn.Linear(EMB_DIM, D)
        self.model_adapter   = nn.Linear(EMB_DIM, D)
        self.purpose_adapter = nn.Linear(EMB_DIM, D)
        self.env_adapter     = nn.Linear(EMB_DIM, D)
        
        # MacroObs: structured features, separate MLP
        self.macro_encoder = nn.Sequential(
            nn.Linear(MACRO_INPUT_DIM, 64),
            nn.GELU(),
            nn.Linear(64, D),
        )
        
        # === Level 1: Per-modality Pools ===
        self.ctx_pool    = PositionAwarePool(D, MAX_CONTEXT_LEN)
        self.inbox_pool  = AttentionPool(D)
        self.tools_pool  = AttentionPool(D)
        self.models_pool = AttentionPool(D)
        
        # === Level 2: Cross-modality ===
        self.modality_type_emb = nn.Embedding(N_MODALITIES, D)
        self.cross_attn = nn.MultiheadAttention(D, num_heads=2, batch_first=True)
        self.cross_norm = nn.LayerNorm(D)
    
    def forward(self, state_dict):
        """
        state_dict:
          "fragments_embs": [B, N, 1024]  (variable N, padded)
          "fragments_mask": [B, N]
          "inbox_embs":     [B, K, 1024]
          "inbox_mask":     [B, K]
          "tools_embs":     [B, M, 1024]
          "tools_mask":     [B, M]
          "models_embs":    [B, P, 1024]
          "models_mask":    [B, P]
          "purpose_emb":    [B, 1024]
          "env_emb":        [B, 1024]
          "macro_obs":      [B, 8]
        """
        # === Level 1: Per-modality ===
        # Variable-length modalities: adapter + pool
        ctx_seq    = self.frag_adapter(state_dict["fragments_embs"])
        ctx_token  = self.ctx_pool(ctx_seq, state_dict["fragments_mask"])
        
        inbox_seq   = self.inbox_adapter(state_dict["inbox_embs"])
        inbox_token = self.inbox_pool(inbox_seq, state_dict["inbox_mask"])
        
        tools_seq   = self.tool_adapter(state_dict["tools_embs"])
        tools_token = self.tools_pool(tools_seq, state_dict["tools_mask"])
        
        models_seq   = self.model_adapter(state_dict["models_embs"])
        models_token = self.models_pool(models_seq, state_dict["models_mask"])
        
        # Fixed-length modalities: adapter only
        purpose_token = self.purpose_adapter(state_dict["purpose_emb"])
        env_token     = self.env_adapter(state_dict["env_emb"])
        macro_token   = self.macro_encoder(state_dict["macro_obs"])
        
        # === Level 2: Cross-modality ===
        # Stack 7 modality tokens
        tokens = torch.stack([
            ctx_token, inbox_token, tools_token, models_token,
            purpose_token, env_token, macro_token,
        ], dim=1)  # [B, 7, D]
        
        # Add modality type embedding
        type_ids = torch.arange(N_MODALITIES, device=tokens.device)
        tokens = tokens + self.modality_type_emb(type_ids).unsqueeze(0)
        
        # Cross-attention + residual + norm
        attended, _ = self.cross_attn(tokens, tokens, tokens)
        tokens = self.cross_norm(tokens + attended)
        
        # Flatten preserving modality identity
        return tokens.flatten(start_dim=1)  # [B, 7*D = 896]


# === Router ===

class Router(nn.Module):
    def __init__(self):
        super().__init__()
        self.l1 = nn.Linear(N_MODALITIES * D, 256)
        self.l2 = nn.Linear(256, N_EXPERTS)
        self.noise_scale = nn.Parameter(torch.ones(N_EXPERTS) * 0.1)
    
    def forward(self, fused, training=True):
        x = F.gelu(self.l1(fused))
        logits = self.l2(x)
        if training:
            noise = torch.randn_like(logits) * F.softplus(self.noise_scale)
            logits = logits + noise
        return F.softmax(logits, dim=-1), logits


# === Expert ===

class Expert(nn.Module):
    def __init__(self):
        super().__init__()
        self.l1 = nn.Linear(N_MODALITIES * D, 128)
        self.l2 = nn.Linear(128, N_ACTIONS)
    
    def forward(self, fused):
        x = F.gelu(self.l1(fused))
        logits = self.l2(x)
        return F.softmax(logits, dim=-1)


# === Full MoEH Model ===

class MoEHActionPrior(nn.Module):
    def __init__(self):
        super().__init__()
        self.fusion = HierarchicalFusion()
        self.router = Router()
        self.experts = nn.ModuleList([Expert() for _ in range(N_EXPERTS)])
    
    def forward(self, state_dict, training=True):
        # 1. Fuse modalities
        fused = self.fusion(state_dict)  # [B, 896]
        
        # 2. Router
        router_probs, router_logits = self.router(fused, training=training)
        
        # 3. Top-k selection
        top_k_vals, top_k_idx = router_probs.topk(TOP_K, dim=-1)
        top_k_weights = F.softmax(top_k_vals, dim=-1)
        
        if not training:
            # === Inference: hard top-k (only run top-2 experts) ===
            B = fused.size(0)
            mixed = torch.zeros(B, N_ACTIONS, device=fused.device)
            for b in range(B):
                for k in range(TOP_K):
                    idx = top_k_idx[b, k].item()
                    weight = top_k_weights[b, k]
                    expert_dist = self.experts[idx](fused[b:b+1]).squeeze(0)
                    mixed[b] += weight * expert_dist
            
            return {
                "mixed_dist": mixed,
                "router_probs": router_probs,
                "top_k_idx": top_k_idx,
                "top_k_weights": top_k_weights,
                "fused": fused,
            }
        else:
            # === Training: soft routing (run all experts) ===
            # Switch Transformer style: non-top-k weights = 0 但 gradient 流过所有 experts
            all_dists = torch.stack(
                [e(fused) for e in self.experts], dim=1
            )  # [B, 8, 12]
            
            # Top-k mask + renormalize
            mask = torch.zeros_like(router_probs)
            mask.scatter_(-1, top_k_idx, 1.0)
            
            weighted = router_probs * mask
            weighted = weighted / (weighted.sum(-1, keepdim=True) + 1e-8)
            
            mixed = (weighted.unsqueeze(-1) * all_dists).sum(dim=1)  # [B, 12]
            
            return {
                "mixed_dist": mixed,
                "router_probs": router_probs,
                "router_logits": router_logits,
                "top_k_idx": top_k_idx,
                "top_k_weights": top_k_weights,
                "expert_dists": all_dists,  # for diversity loss
                "fused": fused,
            }


# === Verify ===
if __name__ == "__main__":
    model = MoEHActionPrior()
    total = sum(p.numel() for p in model.parameters())
    print(f"Total trainable params: {total:,}")
    # Expected: ~2.3M
    
    # Test forward
    B = 2
    state_dict = {
        "fragments_embs": torch.randn(B, 20, EMB_DIM),
        "fragments_mask": torch.ones(B, 20, dtype=torch.bool),
        "inbox_embs":     torch.randn(B, 3, EMB_DIM),
        "inbox_mask":     torch.ones(B, 3, dtype=torch.bool),
        "tools_embs":     torch.randn(B, 10, EMB_DIM),
        "tools_mask":     torch.ones(B, 10, dtype=torch.bool),
        "models_embs":    torch.randn(B, 3, EMB_DIM),
        "models_mask":    torch.ones(B, 3, dtype=torch.bool),
        "purpose_emb":    torch.randn(B, EMB_DIM),
        "env_emb":        torch.randn(B, EMB_DIM),
        "macro_obs":      torch.randn(B, MACRO_INPUT_DIM),
    }
    
    out = model(state_dict, training=False)
    print(f"Inference output:")
    print(f"  mixed_dist:   {out['mixed_dist'].shape}")   # [2, 12]
    print(f"  fused:        {out['fused'].shape}")        # [2, 896]
    print(f"  top_k_idx:    {out['top_k_idx'].shape}")    # [2, 2]
    
    out = model(state_dict, training=True)
    print(f"Training output:")
    print(f"  expert_dists: {out['expert_dists'].shape}")  # [2, 8, 12]
```

---

## 8. Training Pipeline (全程 gRPC 模式)

### 8.1 总览：四阶段，全程通过 gRPC server 驱动 RCM

**关键设计决策**（基于最新 `crates/server/DESIGN.md`）：

> 整个训练过程 RCM 仅作为 gRPC server 暴露 Machine 接口。Python trainer 拥有 policy + reward + task semantics + LLM clients。RCM 只 step。

```
═══════════════════════════════════════════════════════════════════════
                    Training Pipeline Overview (gRPC)
═══════════════════════════════════════════════════════════════════════

Architecture per phase:

  ┌─ Python (trainer) ──────────────────────────┐
  │  - Policy (ColdStart / Hybrid)               │
  │  - DeepSeek/OpenAI client (LLM 调用)         │
  │  - MoEH model (torch, P_B 后激活)            │
  │  - Embedding cache (Qwen3-embedding-0.6b)    │
  │  - GRPO trainer (P_C)                        │
  └──────────────┬──────────────────────────────┘
                 │ gRPC: New / Step / Destroy
  ┌──────────────▼──────────────────────────────┐
  │  RCM gRPC server(s) (Rust, in-memory)        │
  │  - Machine::apply per step                   │
  │  - reactor::react on Halt                    │
  │  - 完全不知道 policy 是什么                  │
  └──────────────────────────────────────────────┘

Phase A: Cold-Start Data Collection
   单 RCM server instance (or N for parallel)
   Python 用 ColdStartPolicy (纯 LLM, 无 MoEH advisor)
   New → loop(Step) → Destroy 跑 500-1000 task
   每 step 缓存 (state_dict, action_cmd, ...) 进 trajectory log
   Cost: ~$50, Time: 1 week
                            │
                            ▼
Phase B: BC Pretrain (Pure Python, no RCM involved)
   Train MoEH to predict LLM action distribution
   Loss: -log(mixed_dist[actual_action_type])
   Cost: ~$5, Time: 1 day
                            │
                            ▼
Phase C: GRPO Iterative Online (gRPC + Hybrid Policy)
   For iteration in 1..20:
     1. N=4 RCM server instances 并行 ready
     2. 选 20 task, each task × k=4 rollouts (4×20=80 并行)
     3. Each rollout uses Hybrid Policy:
        - state ← server
        - state_dict ← Python embedding + encoding
        - prior ← MoEH(state_dict)
        - action_cmd ← LLM(state + hint) or sample from ActionSpace
        - step → new state
     4. Group-relative advantage from k rewards
     5. PPO clipped update on MoEH (4 epochs)
   Cost: ~$1000, Time: 3-4 weeks
                            │
                            ▼
Phase D: Production Deployment (CLI 模式恢复)
   .safetensors → Rust candle 加载
   CLI: accelerate run task.rcm
   Internal Captain Policy 用 ActionPriorPhase 注入 MoEH hint
```

### 8.2 Phase A: Cold-Start Data Collection（gRPC + ColdStartPolicy）

**ColdStartPolicy**：纯 LLM 决策，无 MoEH advisor。用于建立 baseline trajectory 数据集。

```python
# train/cold_start_policy.py

import asyncio
from openai import AsyncOpenAI
from rcm.proto import rcm_pb2
from rcm.client import RCMClient

class ColdStartPolicy:
    """
    Phase A 用 — 完全靠 LLM 决策 (无 MoEH)。
    用 ActionSpace + state 作为 prompt 调 DeepSeek，让它选 verb + content。
    """
    def __init__(self, llm_client: AsyncOpenAI, model: str = "deepseek-chat"):
        self.llm = llm_client
        self.model = model
    
    async def decide(
        self,
        state: rcm_pb2.State,
        action_space: rcm_pb2.ActionSpace,
    ) -> rcm_pb2.ActionCommand:
        # 1. inbox 非空 → 短路 Take (不调 LLM)
        if state.inbox_pending:
            return self._make_take_command(action_space)
        
        # 2. 构造 prompt 给 LLM
        prompt = self._format_state_and_options(state, action_space)
        
        # 3. LLM structured output
        response = await self.llm.chat.completions.create(
            model=self.model,
            messages=[{"role": "user", "content": prompt}],
            response_format={"type": "json_object"},
        )
        choice = self._parse_action_choice(response.choices[0].message.content)
        
        # 4. 构造 ActionCommand
        return self._build_action_command(choice, action_space)
    
    def _format_state_and_options(
        self, state: rcm_pb2.State, action_space: rcm_pb2.ActionSpace
    ) -> str:
        # 复制 RCM 内部 Captain prompt 风格
        context_text = "\n".join(
            f"[{f.role} #{f.id} {f.kind}] {f.text_preview}"
            for f in state.fragments
        )
        options_text = "\n".join(
            f"{i}. {a.label}" for i, a in enumerate(action_space.actions)
        )
        return f"""You are an AI agent. Task: {state.purpose}

# Context
{context_text}

# Available Actions
{options_text}

Pick action index and provide reasoning.
Output JSON: {{"action_index": <int>, "content": <optional string>}}"""

# 主循环
async def collect_phase_a(server_endpoint: str, tasks: list, log_dir: str):
    client = RCMClient(server_endpoint)
    llm = AsyncOpenAI()
    policy = ColdStartPolicy(llm)
    
    for task in tasks:
        # New machine
        mid, state, action_space = await client.new(
            purpose=task.purpose,
            models=["deepseek-chat"],
            tools=task.tools,
            prompts=task.initial_prompts,
        )
        
        trajectory = []
        try:
            while not state.done:
                # 记录 state + action_space (训练 MoEH 用)
                snapshot = {
                    "task_id": task.id,
                    "step_idx": state.step,
                    "state": serialize_state(state),
                    "action_space": serialize_action_space(action_space),
                }
                
                # ColdStartPolicy 决策
                cmd = await policy.decide(state, action_space)
                snapshot["chosen_command"] = serialize_command(cmd)
                snapshot["chosen_action_type"] = verb_to_action_type_idx(cmd.verb)
                
                # Step
                state, action_space = await client.step(mid, cmd)
                trajectory.append(snapshot)
                
                if state.step > MAX_STEPS:
                    break
            
            # 评估并存
            outcome = evaluate_task(task, state)
            save_trajectory(log_dir, task.id, trajectory, outcome)
        finally:
            await client.destroy(mid)

# Usage
asyncio.run(collect_phase_a(
    server_endpoint="localhost:50051",
    tasks=load_benchmark_tasks("terminal-bench-2")[:500],
    log_dir="./trajectory_logs",
))
```

### 8.3 Phase B: BC Pretrain

```python
# train/phase_b_bc.py

import torch
from torch.optim import AdamW
import torch.nn.functional as F
from collections import defaultdict

model = MoEHActionPrior()
optimizer = AdamW(model.parameters(), lr=1e-4)

samples = load_trajectory_data("phase_a_data.json")

for epoch in range(3):
    losses = []
    for batch in batch_iter(samples, batch_size=32):
        # Collate batch with padding for variable-length modalities
        batch_dict = collate_state_batch(batch)
        actions = torch.tensor([s["actual_action_type"] for s in batch])
        
        output = model(batch_dict, training=True)
        mixed_dist = output["mixed_dist"]  # [B, 12]
        
        # Negative log-likelihood
        chosen_prob = mixed_dist.gather(-1, actions.unsqueeze(-1)).squeeze(-1)
        nll_loss = -torch.log(chosen_prob + 1e-8).mean()
        
        # Load balancing (Switch Transformer)
        router_probs = output["router_probs"]
        expert_usage = router_probs.mean(dim=0)
        balance_loss = (expert_usage * router_probs.mean(dim=0)).sum() * N_EXPERTS
        
        loss = nll_loss + 0.01 * balance_loss
        loss.backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        optimizer.step()
        optimizer.zero_grad()
        losses.append(loss.item())
    
    print(f"Epoch {epoch}: avg loss = {sum(losses)/len(losses):.4f}")

torch.save(model.state_dict(), "phase_b_bc.pt")
```

### 8.4 Phase C: GRPO Fine-tune

**核心算法**：GRPO（group rollout + relative advantage）+ PPO clipped objective。

```python
# train/phase_c_grpo.py

import torch
import asyncio
import numpy as np

K_ROLLOUTS = 4
N_TASKS_PER_ITER = 20
N_PPO_EPOCHS = 4
PPO_CLIP_EPS = 0.2
GAMMA = 0.95

class GRPOTrainer:
    def __init__(self, model, config):
        self.model = model
        self.config = config
        self.optimizer = AdamW(model.parameters(), lr=5e-5)
    
    async def run_iteration(self, tasks, iter_idx):
        # 1. Export weights for RCM rollout
        weights_path = f"./weights/iter_{iter_idx}.safetensors"
        torch.save(self.model.state_dict(), weights_path)
        
        # 2. Group rollouts (parallel)
        all_samples = []
        for task in tasks:
            rollouts = await self._collect_group_rollouts(task, weights_path)
            samples = self._compute_grpo_advantages(rollouts)
            all_samples.extend(samples)
        
        if not all_samples:
            print(f"Iter {iter_idx}: no signal (all groups had std=0)")
            return
        
        # 3. PPO clipped objective update (4 epochs)
        for ppo_epoch in range(N_PPO_EPOCHS):
            self._ppo_update(all_samples)
        
        # 4. Log
        self._log_metrics(iter_idx)
    
    async def _collect_group_rollouts(self, task, weights_path):
        """
        k rollouts in parallel via gRPC, each connects to one of N=4 RCM
        server instances. Different seeds + temperatures for LLM 多样性。
        """
        # Pool of N RCM server endpoints (k=4 connect to 4 parallel servers)
        rollout_coros = []
        for i in range(K_ROLLOUTS):
            endpoint = self.server_pool[i % len(self.server_pool)]
            client = RCMClient(endpoint)
            policy = HybridPolicy(
                moeh=self.model,         # 当前训练中的 MoEH
                llm=self.llm_client,
                seed=task.id * 100 + i,
                temperature=0.6 + 0.05 * i,
            )
            rollout_coros.append(self._single_rollout(client, task, policy))
        
        return await asyncio.gather(*rollout_coros)
    
    async def _single_rollout(self, client, task, policy):
        """Single rollout via gRPC: New → loop(Step) → Destroy."""
        mid, state, action_space = await client.new(
            purpose=task.purpose,
            models=task.models,
            tools=task.tools,
            prompts=task.initial_prompts,
        )
        
        trajectory = []
        try:
            while not state.done and state.step < MAX_STEPS:
                # Hybrid Policy decision (MoEH prior + LLM if content needed)
                cmd, prior_output = await policy.decide(state, action_space)
                
                trajectory.append({
                    "state_dict": policy.last_state_dict,  # MoEH input cache
                    "actual_action_type": verb_to_action_type_idx(cmd.verb),
                    "chosen_action_logprob": prior_output.chosen_logprob,
                    "step_idx": state.step,
                })
                
                state, action_space = await client.step(mid, cmd)
            
            outcome = await self._evaluate(task, state)
            return Rollout(trajectory=trajectory, outcome=outcome, length=len(trajectory))
        finally:
            await client.destroy(mid)
    
    def _compute_grpo_advantages(self, rollouts):
        """Group-relative advantage computation."""
        rewards = np.array([r.outcome.reward for r in rollouts])
        mean_r = rewards.mean()
        std_r = rewards.std() + 1e-8
        
        # Skip if no signal
        if std_r < 1e-6:
            return []
        
        samples = []
        for rollout_idx, rollout in enumerate(rollouts):
            traj_adv = (rewards[rollout_idx] - mean_r) / std_r
            
            for step in rollout.trajectory:
                # Discounted to per-step
                step_adv = traj_adv * (GAMMA ** (rollout.length - step.idx))
                
                samples.append({
                    "state_dict": step.state_dict,
                    "actual_action_type": step.actual_action_type,
                    "advantage": step_adv,
                    "cached_log_prob": step.chosen_action_logprob,
                })
        
        return samples
    
    def _ppo_update(self, samples):
        """PPO clipped objective + auxiliary losses."""
        np.random.shuffle(samples)
        
        for batch in batch_iter(samples, batch_size=32):
            batch_dict = collate_state_batch(batch)
            batch_action = torch.tensor([s["actual_action_type"] for s in batch])
            batch_adv = torch.tensor([s["advantage"] for s in batch], dtype=torch.float32)
            batch_old_logp = torch.tensor(
                [s["cached_log_prob"] for s in batch], dtype=torch.float32)
            
            # Normalize advantage
            batch_adv = (batch_adv - batch_adv.mean()) / (batch_adv.std() + 1e-8)
            
            # Forward
            output = self.model(batch_dict, training=True)
            mixed_dist = output["mixed_dist"]
            
            # === PPO Policy Loss (Clipped Objective) ===
            chosen_prob = mixed_dist.gather(-1, batch_action.unsqueeze(-1)).squeeze(-1)
            new_log_prob = torch.log(chosen_prob + 1e-8)
            
            ratio = torch.exp(new_log_prob - batch_old_logp)
            clipped = torch.clamp(ratio, 1 - PPO_CLIP_EPS, 1 + PPO_CLIP_EPS)
            policy_loss = -torch.min(ratio * batch_adv, clipped * batch_adv).mean()
            
            # === Auxiliary Losses ===
            router_probs = output["router_probs"]
            
            # Load balancing (Switch Transformer)
            expert_usage = router_probs.mean(dim=0)
            balance_loss = (expert_usage * router_probs.mean(dim=0)).sum() * N_EXPERTS
            
            # Importance (DeepSeek-MoE: 鼓励 experts 平均使用)
            importance = router_probs.sum(dim=0)
            cv = importance.std() / (importance.mean() + 1e-8)
            importance_loss = cv ** 2
            
            # Diversity bonus (pairwise KL on top-k experts)
            expert_dists = output["expert_dists"]
            top_k_idx = output["top_k_idx"]
            div_loss = self._compute_diversity_loss(expert_dists, top_k_idx)
            
            # === Total ===
            loss = (policy_loss
                    + 0.01 * balance_loss
                    + 0.01 * importance_loss
                    + 0.001 * div_loss)
            
            self.optimizer.zero_grad()
            loss.backward()
            torch.nn.utils.clip_grad_norm_(self.model.parameters(), 1.0)
            self.optimizer.step()
    
    def _compute_diversity_loss(self, expert_dists, top_k_idx):
        """Symmetric KL between top-k experts. Negative to maximize divergence."""
        B = expert_dists.size(0)
        div_loss = 0.0
        for b in range(B):
            idx_a, idx_b = top_k_idx[b, 0].item(), top_k_idx[b, 1].item()
            d_a = expert_dists[b, idx_a]
            d_b = expert_dists[b, idx_b]
            kl_ab = F.kl_div(d_a.log(), d_b, reduction='sum')
            kl_ba = F.kl_div(d_b.log(), d_a, reduction='sum')
            div_loss -= (kl_ab + kl_ba) / 2
        return div_loss / B

# Main loop
async def main():
    model = MoEHActionPrior()
    model.load_state_dict(torch.load("phase_b_bc.pt"))
    
    trainer = GRPOTrainer(model, config)
    
    for iteration in range(20):
        tasks = sample_tasks(N_TASKS_PER_ITER)
        await trainer.run_iteration(tasks, iteration)
    
    # Final export
    torch.save(model.state_dict(), "./weights/final.safetensors")

asyncio.run(main())
```

### 8.5 GRPO vs PPO 命名澄清

**GRPO 和 PPO 不是两个独立算法**：

```
GRPO = (k 个 rollouts) + (group-relative advantage) + (PPO clipped update rule)
                                                      ↑
                                          这就是"PPO loss / PPO epochs" 指的东西

PPO 是 surrogate loss formula；GRPO 是 advantage estimation method。
两者组合用，不冲突。
```

我们的整个训练算法叫 GRPO。Phase C 内"4 PPO epochs"指的是用 PPO clipped objective 做 4 次 update epoch。

### 8.6 LLM Executor 随机性的处理

LLM 输出具有 stochasticity（temperature > 0）。Group rollout 通过**多次 sampling 平均**掉这个随机性：

```
Rollout A: same (state, hint) → LLM 输出 Halt → outcome = success
Rollout B: same (state, hint) → LLM 输出 Append → outcome = failure
Rollout C: same (state, hint) → LLM 输出 Halt → outcome = success
Rollout D: same (state, hint) → LLM 输出 Take → outcome = mixed

Group-relative advantage:
  Halt rollouts: advantage ≈ +0.7 (因为成功)
  Append rollouts: advantage ≈ -1.4 (失败)
  Take rollouts: advantage ≈ 0 (中性)
  
→ Model 学到一致信号: "推 Halt 在这种 state 下更好"
```

**LLM 随机性被 group sampling 转成训练信号**——advisor 模式下这是 feature 不是 bug。

### 8.7 Iterative Offline Schedule

```
Iteration  Wall Time  Tasks   API Cost    Cumulative
─────────  ─────────  ──────  ──────────  ──────────
   1       ~2h        20×4=80 $50         $50
   2       ~2h        20×4=80 $50         $100
   ...
   20      ~2h        20×4=80 $50         $1000

Total: 40 wall hours, ~$1000
```

每个 iteration 独立——RCM 跑 rollout 收集数据，Python 训 PPO update，导出新 weights，循环。

### 8.8 训练 vs RCM Runtime 解耦

**RCM 只负责**：
- Inference（加载 weights → predict mixed_dist）
- Trajectory logging（环境变量启用）
- Bench rollout 收集 trajectory

**Python 端只负责**：
- 加载 trajectory data
- BC pretrain
- PPO update with GRPO advantage
- 导出 .safetensors

**两者通过 .safetensors 文件交换 model state，通过 JSON 交换 trajectory data**。Runtime 完全不引入 training framework dependency。

---

## 9. RCM 端工程集成

### 9.1 完整类型定义

```rust
// crates/machine/src/policy.rs (修改)

pub enum Action {
    Append(Fragment),
    Insert { after: u64, fragment: Fragment },
    Replace { id: u64, fragment: Fragment },
    Remove(u64),
    Swap(u64, u64),
    Take,
    Halt,
    Done,
    Model(String),
    Activate(String),
    Deactivate(String),
    SetPrompt { name: String, content: String },  // 新增
}

impl Action {
    pub fn type_idx(&self) -> usize {
        match self {
            Action::Append(_) => 0,
            Action::Insert { .. } => 1,
            Action::Replace { .. } => 2,
            Action::Remove(_) => 3,
            Action::Swap(_, _) => 4,
            Action::Take => 5,
            Action::Halt => 6,
            Action::Done => 7,
            Action::Model(_) => 8,
            Action::Activate(_) => 9,
            Action::Deactivate(_) => 10,
            Action::SetPrompt { .. } => 11,
        }
    }
}
```

```rust
// crates/machine/src/macro_obs.rs (新)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroObservation {
    pub step_count: u32,
    pub fragment_count: u32,
    pub recent_hitch_count: u32,
    pub inbox_size: u32,
    pub tokens_used_total: u64,
    pub time_since_last_halt: u32,
    pub active_tools_count: u32,
    pub steps_in_current_task: u32,
}
```

```rust
// crates/accelerator/src/policy/action_prior.rs (新)

pub trait ActionPrior: Send + Sync {
    fn predict(&self, state_dict: &StateEncodingDict) -> ActionPriorOutput;
}

pub struct StateEncodingDict {
    pub fragments_embs: Vec<Vec<f32>>,    // [N][1024]
    pub inbox_embs: Vec<Vec<f32>>,         // [K][1024]
    pub tools_embs: Vec<Vec<f32>>,         // [M][1024]
    pub models_embs: Vec<Vec<f32>>,        // [P][1024]
    pub purpose_emb: Vec<f32>,              // [1024]
    pub env_emb: Vec<f32>,                  // [1024]
    pub macro_obs: Vec<f32>,                // [8]
}

pub struct ActionPriorOutput {
    pub mixed_distribution: [f32; 12],
    pub top_k_experts: Vec<(usize, f32)>,
}

pub struct NoOpActionPrior;

impl ActionPrior for NoOpActionPrior {
    fn predict(&self, _: &StateEncodingDict) -> ActionPriorOutput {
        ActionPriorOutput {
            mixed_distribution: [1.0 / 12.0; 12],  // uniform
            top_k_experts: vec![],
        }
    }
}

pub struct MoEHActionPrior {
    model: MoEHWrapper,  // candle / tract / onnx loaded
    embedder: Arc<dyn Embedder>,
    cache: Arc<EmbeddingCache>,
}

impl ActionPrior for MoEHActionPrior {
    fn predict(&self, state_dict: &StateEncodingDict) -> ActionPriorOutput {
        self.model.forward(state_dict).unwrap_or_else(|err| {
            warn!("MoEH predict failed: {}, falling back to uniform", err);
            NoOpActionPrior.predict(state_dict)
        })
    }
}
```

### 9.2 Embedding Cache

```rust
// crates/accelerator/src/policy/embedding_cache.rs

use lru::LruCache;
use std::sync::Mutex;

pub struct EmbeddingCache {
    // Fragment cache: LRU 10K entries
    fragment_cache: Mutex<LruCache<u64, Vec<f32>>>,
    
    // Tool / Model / Env / Purpose: 启动时填充或 task 级 cache
    tool_cache: HashMap<String, Vec<f32>>,
    model_cache: HashMap<String, Vec<f32>>,
    env_cache: Mutex<Option<Vec<f32>>>,
    purpose_cache: Mutex<Option<Vec<f32>>>,
    
    embedder: Arc<dyn Embedder>,
}

impl EmbeddingCache {
    pub async fn init(catalog: &Catalog, embedder: Arc<dyn Embedder>) -> Self {
        // Pre-compute tool / model embeddings (这些几乎不变)
        let mut tool_cache = HashMap::new();
        for (name, tool) in &catalog.tools {
            let text = format_tool_for_embedding(tool, /*is_active*/ false);
            tool_cache.insert(name.clone(), embedder.embed(&text).await);
        }
        // ... 类似 model_cache
        
        Self {
            fragment_cache: Mutex::new(LruCache::new(10_000.try_into().unwrap())),
            tool_cache, model_cache,
            env_cache: Mutex::new(None),
            purpose_cache: Mutex::new(None),
            embedder,
        }
    }
    
    pub async fn embed_fragment(&self, frag: &Fragment) -> Vec<f32> {
        let key = self.fragment_key(frag);
        
        {
            let mut cache = self.fragment_cache.lock().unwrap();
            if let Some(emb) = cache.get(&key) {
                return emb.clone();
            }
        }
        
        let text = format_fragment_for_embedding(frag);
        let emb = self.embedder.embed(&text).await;
        
        self.fragment_cache.lock().unwrap().put(key, emb.clone());
        emb
    }
    
    pub async fn embed_fragments_parallel(&self, frags: &[Fragment]) -> Vec<Vec<f32>> {
        use futures::future::join_all;
        join_all(frags.iter().map(|f| self.embed_fragment(f))).await
    }
    
    pub async fn embed_purpose(&self, purpose: &str) -> Vec<f32> {
        let mut cache = self.purpose_cache.lock().unwrap();
        if let Some(emb) = cache.as_ref() {
            return emb.clone();
        }
        let emb = self.embedder.embed(purpose).await;
        *cache = Some(emb.clone());
        emb
    }
    
    fn fragment_key(&self, frag: &Fragment) -> u64 {
        let mut hasher = DefaultHasher::new();
        frag.role.hash(&mut hasher);
        frag.tag.hash(&mut hasher);
        // hash content
        match &frag.content {
            Content::Text(t) => t.text.hash(&mut hasher),
            Content::ToolCall(tc) => { tc.name.hash(&mut hasher); /* ... */ }
            // ... 其他 variants
            _ => {}
        }
        hasher.finish()
    }
}
```

**Cache hit rate 预估**：

| Source | Hit rate | 说明 |
|--------|---------|------|
| Fragment (system prompts, instructions) | ~99% | 几乎不变 |
| Fragment (tool results 历史) | ~95% | 仅新增 miss |
| Inbox 临时 fragment | ~10% | 短暂存在 |
| Tools | ~100% | 启动 cache |
| Models | ~100% | 启动 cache |
| Purpose / Env | ~100% | task 级 cache |

**实际 embedding API 成本**：~$0.0014/task (DeepSeek/Qwen3-embedding 价格)。

### 9.3 Machine 主循环改造

```rust
// crates/machine/src/machine.rs (修改)

pub struct Machine {
    policy: Box<dyn Policy>,
    step_count: AtomicU32,
    tokens_used_total: AtomicU64,
    last_halt_step: AtomicU32,
    recent_hitches: Mutex<VecDeque<u32>>,
    task_start_time: Instant,
}

impl Machine {
    pub async fn run(
        &self,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
    ) {
        self.run_phases(self.policy.pre(), ...).await;
        
        let mut inbox = Inbox::new();
        
        loop {
            // Compute MacroObservation
            let macro_obs = self.compute_macro_obs(ctx, &inbox, resources);
            
            // pre_halt phases (含 ActionPriorPhase)
            for phase in self.policy.pre_halt() {
                self.run_phase(phase, ctx, env, resources, &macro_obs).await;
            }
            
            // Policy decide
            let action = self.policy.decide(
                purpose, ctx, env, resources, &inbox
            ).await;
            
            // Update counters
            self.step_count.fetch_add(1, Relaxed);
            
            // Cleanup macro_obs fragment (注入的 hint 由 next iteration 的 Phase 处理)
            ctx.remove_by_tag("macro_obs");
            
            match action {
                Action::Done => break,
                Action::Halt => {
                    self.last_halt_step.store(self.step_count.load(Relaxed), Relaxed);
                    reactor::react(ctx, env, resources, &mut inbox).await;
                    self.update_hitch_window(ctx);
                }
                Action::SetPrompt { name, content } => {
                    let prev = resources.prompts.insert(name.clone(), content.clone());
                    hook!(event = "set_prompt", name, content_len = content.len(),
                          had_previous = prev.is_some());
                }
                other => apply_action(other, ctx, resources, &mut inbox),
            }
        }
        
        self.run_phases(self.policy.post(), ...).await;
    }
    
    fn compute_macro_obs(&self, ctx: &Context, inbox: &Inbox, res: &Resources) -> MacroObservation {
        MacroObservation {
            step_count: self.step_count.load(Relaxed),
            fragment_count: ctx.fragments().len() as u32,
            recent_hitch_count: self.recent_hitches.lock().unwrap().len() as u32,
            inbox_size: inbox.len() as u32,
            tokens_used_total: self.tokens_used_total.load(Relaxed),
            time_since_last_halt: self.step_count.load(Relaxed)
                .saturating_sub(self.last_halt_step.load(Relaxed)),
            active_tools_count: res.active_tools.len() as u32,
            steps_in_current_task: self.step_count.load(Relaxed),
        }
    }
}
```

### 9.4 ActionPriorPhase

```rust
// crates/accelerator/src/policy/phases/action_prior.rs

pub struct ActionPriorPhase {
    prior: Arc<dyn ActionPrior>,
    embedder_cache: Arc<EmbeddingCache>,
}

impl Phase for ActionPriorPhase {
    fn decide(
        &self,
        purpose: &Purpose,
        ctx: &Context,
        env: &Environment,
        resources: &Resources,
    ) -> PhaseOutcome {
        // 1. Compute MacroObservation (从 Machine state 读)
        let macro_obs = read_macro_obs_from_context(ctx);
        
        // 2. Embed all modalities (with cache)
        let state_dict = self.embedder_cache.encode_state(
            purpose, ctx, env, resources, &macro_obs
        ).await;
        
        // 3. Run MoEH prediction
        let output = self.prior.predict(&state_dict);
        
        // 4. Format as hint
        let hint = format_prior_hint(&output);
        
        // 5. Inject / replace hint fragment
        if let Some(existing) = ctx.fragments().iter()
            .find(|f| f.role == Role::System && f.tag == "action_prior")
        {
            PhaseOutcome::Action(Action::Replace {
                id: existing.id(),
                fragment: Fragment::system(hint).with_tag("action_prior"),
            })
        } else {
            PhaseOutcome::Action(Action::Append(
                Fragment::system(hint).with_tag("action_prior")
            ))
        }
    }
}

fn format_prior_hint(output: &ActionPriorOutput) -> String {
    let mut sorted: Vec<_> = ACTION_NAMES.iter().enumerate()
        .map(|(i, name)| (name, output.mixed_distribution[i]))
        .collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    let top3 = sorted.iter().take(3)
        .map(|(name, p)| format!("  - {} ({:.0}%)", name, p * 100.0))
        .collect::<Vec<_>>()
        .join("\n");
    
    let experts = if !output.top_k_experts.is_empty() {
        format!(
            "\n\nRouting through:\n{}",
            output.top_k_experts.iter()
                .map(|(idx, w)| format!("  - Expert {} (weight {:.0}%)", idx, w * 100.0))
                .collect::<Vec<_>>()
                .join("\n")
        )
    } else {
        String::new()
    };
    
    format!(
        "[Action Prior Hint]\n\
         Historically in similar states, these actions tended to succeed:\n{}{}\n\n\
         (Advisor suggestion. You retain full freedom to choose any action.)",
        top3, experts
    )
}
```

### 9.5 Captain 改造

```rust
// crates/accelerator/src/policy/captain.rs

pub struct Captain {
    first_call: AtomicBool,
    retry: Retry,
    action_prior: Arc<dyn ActionPrior>,
    embedder_cache: Arc<EmbeddingCache>,
}

impl Captain {
    pub fn new(
        action_prior: Option<Arc<dyn ActionPrior>>,
        embedder_cache: Arc<EmbeddingCache>,
    ) -> Self {
        Self {
            first_call: AtomicBool::new(false),
            retry: Retry::default(),
            action_prior: action_prior.unwrap_or_else(|| Arc::new(NoOpActionPrior)),
            embedder_cache,
        }
    }
}

impl Policy for Captain {
    fn decide<'a>(...) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            // inbox 非空 → 短路 Take（不调 prior + LLM）
            if inbox.peek().is_some() {
                return Action::Take;
            }
            
            // First call → Halt
            if !self.first_call.swap(true, Ordering::Relaxed) {
                return Action::Halt;
            }
            
            // 调 LLM (structured output)
            // 注意：ActionPriorPhase 已经在 pre_halt 注入了 hint 到 ctx
            let llm_result = call_llm_structured_output(
                ctx,
                resources,
                ACTION_JSON_SCHEMA,
            ).await;
            
            match llm_result {
                Ok(action) => action,
                Err(LlmError::Timeout) | Err(LlmError::Network(_)) => {
                    if self.retry.should_retry().await {
                        Action::Halt
                    } else {
                        Action::Done
                    }
                }
                Err(LlmError::Parse(msg)) => {
                    Action::Append(
                        Fragment::hitch(format!("Invalid action: {}", msg))
                    )
                }
            }
        })
    }
    
    fn pre_halt(&self) -> Vec<Box<dyn Phase>> {
        vec![
            Box::new(InjectEnv),
            Box::new(ActionPriorPhase::new(
                self.action_prior.clone(),
                self.embedder_cache.clone(),
            )),
        ]
    }
}
```

### 9.6 .rcm v3 语法

```toml
name = "evolved"

model deepseek-v3.1 {
    protocol = "openai"
    endpoint = "https://api.deepseek.com/v1"
    credentials = "${DEEPSEEK_API_KEY}"
    timeout = 180
}

# 跨 task 演化的 prompts
[prompts]
captain = """
你是 AI 助手...

# 累积经验
- 处理 generic 类型时先检查 null
- 排序前确认列表非空
"""

# 可选: MoEH Action Prior
[action_prior]
type = "moeh"
weights = "./moeh_final.safetensors"
context_embedder = "qwen3-embedding-0.6b"
context_embedder_endpoint = "${QWEN_EMBEDDING_ENDPOINT}"
embedding_cache_size = 10000

accelerator {
    purpose = "..."
    models = ["deepseek-v3.1"]
    policy = "captain"
    tools = ["fs", "shell", "find"]
}
```

不写 `[action_prior]` 时 Captain 使用 `NoOpActionPrior`（cold start fallback）。

### 9.7 gRPC Server 端复用

RCM gRPC server（[crates/server/DESIGN.md](../../crates/server/DESIGN.md)）暴露 Machine::apply 给外部。**MoEH inference 不在 server 端发生**——server 仅做 Machine step。所有 advisor / hint 注入逻辑放 Python 客户端（见 §10）。

服务器侧需要的最小改动（与本 proposal 兼容）：

- **保留**：现有 New / Step / Destroy 三个 RPC
- **保留**：现有 ActionSpace 构建逻辑（Free / Consumption mode）
- **可选**：扩 Fragment proto 加 `full_text` 字段（避免 200-char preview 截断对 embedding 不利）
- **不需要新增 RPC**：embedding / MoEH inference 都在 Python 端

---

## 10. gRPC Server 集成（训练核心）

整个 MoEH 训练全程通过 RCM gRPC server 驱动。本章详细描述 Python 客户端如何跟 server 配合。

### 10.1 总体架构

```
═══════════════════════════════════════════════════════════════════════
          MoEH Training via gRPC: N parallel server instances
═══════════════════════════════════════════════════════════════════════

  ┌─ Python Trainer Process ─────────────────────────────────────┐
  │                                                                │
  │   MoEH model (torch.nn.Module)                                 │
  │   ↑↓ gradient update / inference                               │
  │                                                                │
  │   ┌─ Hybrid Policy ──────────────────────────────────────┐    │
  │   │  1. embed_state(state) → state_dict                   │    │
  │   │  2. moeh.predict(state_dict) → action prior           │    │
  │   │  3. if action needs content:                          │    │
  │   │       content = LLM(state + hint)                     │    │
  │   │     else:                                              │    │
  │   │       (sample from ActionSpace 或 LLM 选 verb)        │    │
  │   │  4. build ActionCommand → return                      │    │
  │   └────────────────────────────┬─────────────────────────┘    │
  │                                 │                              │
  │   ┌─ Embedding Service (cached) ┘                              │
  │   │  Qwen3-embedding-0.6b API                                  │
  │   │  LRU 10K fragment cache + per-tool/model/purpose cache     │
  │   └────────────────────────────────────────────────────────────┘
  └──────────────────────┬─────────────────────────────────────────┘
                         │
              ┌──────────┼──────────┬──────────┐
        gRPC channel    gRPC      gRPC      gRPC
              │          │          │          │
         ┌────▼───┐ ┌───▼────┐ ┌───▼────┐ ┌───▼────┐
         │ RCM    │ │ RCM    │ │ RCM    │ │ RCM    │
         │ Server │ │ Server │ │ Server │ │ Server │
         │ #1     │ │ #2     │ │ #3     │ │ #4     │
         │ :50051 │ │ :50052 │ │ :50053 │ │ :50054 │
         └────────┘ └────────┘ └────────┘ └────────┘
         
   N=4 server processes (1 per CPU core typically)
   每个 task × k=4 rollouts → 16 task 同时跑（4 server × 4 task each）
```

### 10.2 Python 端 RCMClient

封装 gRPC stub：

```python
# rcm_client.py
import grpc
from rcm.proto import rcm_pb2, rcm_pb2_grpc

class RCMClient:
    def __init__(self, endpoint: str = "localhost:50051"):
        self.channel = grpc.aio.insecure_channel(endpoint)
        self.stub = rcm_pb2_grpc.RCMStub(self.channel)
    
    async def new(
        self,
        purpose: str,
        models: list[str],
        tools: list[str] | None = None,
        prompts: dict[str, str] | None = None,
    ) -> tuple[str, rcm_pb2.State, rcm_pb2.ActionSpace]:
        req = rcm_pb2.NewRequest(
            purpose=purpose,
            models=models,
            tools=tools or [],
            prompts=prompts or {},
        )
        resp = await self.stub.New(req)
        return resp.machine_id, resp.state, resp.action_space
    
    async def step(
        self,
        machine_id: str,
        command: rcm_pb2.ActionCommand,
    ) -> tuple[rcm_pb2.State, rcm_pb2.ActionSpace]:
        req = rcm_pb2.StepRequest(machine_id=machine_id, command=command)
        resp = await self.stub.Step(req)
        return resp.state, resp.action_space
    
    async def destroy(self, machine_id: str):
        await self.stub.Destroy(rcm_pb2.DestroyRequest(machine_id=machine_id))
    
    async def close(self):
        await self.channel.close()
```

### 10.3 HybridPolicy（核心）

MoEH advisor + Python LLM client 协同：

```python
# train/hybrid_policy.py

import torch
import asyncio
from openai import AsyncOpenAI
from rcm.proto import rcm_pb2

class HybridPolicy:
    """
    Phase C 用：MoEH 给 prior hint，LLM 看 state + hint 决定 action。
    """
    def __init__(
        self,
        moeh: MoEHActionPrior,
        llm: AsyncOpenAI,
        embedder: EmbeddingService,
        seed: int = 0,
        temperature: float = 0.7,
    ):
        self.moeh = moeh
        self.llm = llm
        self.embedder = embedder
        self.seed = seed
        self.temperature = temperature
        self.last_state_dict = None  # 缓存供训练用
        self.last_prior = None
    
    async def decide(
        self,
        state: rcm_pb2.State,
        action_space: rcm_pb2.ActionSpace,
    ) -> tuple[rcm_pb2.ActionCommand, PriorOutput]:
        # === 短路: inbox 非空 → Take ===
        if state.inbox_pending:
            cmd = make_take_command(action_space)
            return cmd, PriorOutput.empty()
        
        # === Step 1: encode state for MoEH ===
        state_dict = await self._encode_state(state)
        self.last_state_dict = state_dict
        
        # === Step 2: MoEH advisor ===
        with torch.no_grad():
            output = self.moeh(state_dict, training=False)
        mixed_dist = output["mixed_dist"].squeeze(0)  # [12]
        top_k_idx = output["top_k_idx"].squeeze(0)    # [2]
        top_k_weights = output["top_k_weights"].squeeze(0)  # [2]
        
        # === Step 3: format prior hint ===
        hint = self._format_prior_hint(mixed_dist, top_k_idx, top_k_weights)
        
        # === Step 4: LLM decision (with hint injected) ===
        action_cmd = await self._llm_decide(state, action_space, hint)
        
        # === Step 5: cache log_prob for GRPO training ===
        action_type_idx = verb_to_action_type_idx(action_cmd.verb)
        chosen_prob = mixed_dist[action_type_idx].item()
        chosen_logprob = float(torch.log(torch.tensor(chosen_prob + 1e-8)))
        
        prior_output = PriorOutput(
            mixed_dist=mixed_dist,
            top_k_idx=top_k_idx,
            chosen_logprob=chosen_logprob,
        )
        self.last_prior = prior_output
        
        return action_cmd, prior_output
    
    async def _encode_state(self, state: rcm_pb2.State) -> dict:
        """把 gRPC State 编码成 MoEH 输入 state_dict."""
        # 1. 并行 embed 所有 fragments
        frags_embs = await self.embedder.embed_fragments_parallel(state.fragments)
        
        # 2. Inbox: embed 头部 fragment (如有)
        inbox_embs = []
        if state.inbox_pending and state.HasField("inbox_peek"):
            inbox_emb = await self.embedder.embed_fragment(state.inbox_peek)
            inbox_embs.append(inbox_emb)
        
        # 3. Active tools/models embeddings (启动时 cache)
        tools_embs = [
            self.embedder.tool_cache[t]
            for t in state.active_tools + state.available_tools
        ]
        models_embs = [
            self.embedder.model_cache[m]
            for m in [state.active_model] + state.available_models
        ]
        
        # 4. Purpose / Env (task 级 cache)
        purpose_emb = await self.embedder.embed_purpose(state.purpose)
        env_emb = await self.embedder.embed_env(state.workdir, state.env_vars)
        
        # 5. MacroObs (从 State 推算)
        macro_obs = self._compute_macro_obs(state)
        
        return {
            "fragments_embs": torch.stack([torch.tensor(e) for e in frags_embs]).unsqueeze(0),
            "fragments_mask": torch.ones(1, len(frags_embs), dtype=torch.bool),
            "inbox_embs": torch.stack([torch.tensor(e) for e in inbox_embs]).unsqueeze(0)
                         if inbox_embs else torch.zeros(1, 0, EMB_DIM),
            "inbox_mask": torch.ones(1, len(inbox_embs), dtype=torch.bool),
            "tools_embs": torch.stack([torch.tensor(e) for e in tools_embs]).unsqueeze(0),
            "tools_mask": torch.ones(1, len(tools_embs), dtype=torch.bool),
            "models_embs": torch.stack([torch.tensor(e) for e in models_embs]).unsqueeze(0),
            "models_mask": torch.ones(1, len(models_embs), dtype=torch.bool),
            "purpose_emb": torch.tensor(purpose_emb).unsqueeze(0),
            "env_emb": torch.tensor(env_emb).unsqueeze(0),
            "macro_obs": torch.tensor(macro_obs).unsqueeze(0),
        }
    
    def _compute_macro_obs(self, state: rcm_pb2.State) -> list[float]:
        return [
            state.step / 50.0,
            len(state.fragments) / 50.0,
            self._recent_hitches(state) / 10.0,
            (1.0 if state.inbox_pending else 0.0),
            # ... 其他 macro_obs 字段
        ]
    
    def _format_prior_hint(self, mixed_dist, top_k_idx, top_k_weights) -> str:
        """格式化 prior 成自然语言 hint."""
        sorted_actions = sorted(
            enumerate(mixed_dist.tolist()),
            key=lambda x: -x[1]
        )[:3]
        top3_text = "\n".join(
            f"  - {ACTION_NAMES[i]} ({p*100:.0f}%)"
            for i, p in sorted_actions
        )
        experts_text = "\n".join(
            f"  - Expert {top_k_idx[k].item()} (weight {top_k_weights[k].item()*100:.0f}%)"
            for k in range(2)
        )
        return f"""[Action Prior Hint]
Historically in similar states, these actions tended to succeed:
{top3_text}

Routing through:
{experts_text}

(Advisor suggestion. You retain full freedom.)"""
    
    async def _llm_decide(
        self,
        state: rcm_pb2.State,
        action_space: rcm_pb2.ActionSpace,
        hint: str,
    ) -> rcm_pb2.ActionCommand:
        """LLM 看 state + hint + action space 选 action（含 content if needed）."""
        prompt = self._format_decision_prompt(state, action_space, hint)
        
        response = await self.llm.chat.completions.create(
            model="deepseek-chat",
            messages=[{"role": "user", "content": prompt}],
            temperature=self.temperature,
            seed=self.seed,
            response_format={"type": "json_object"},
        )
        choice = self._parse_decision(response.choices[0].message.content)
        return self._build_action_command(choice, action_space)
    
    def _build_action_command(self, choice: dict, action_space) -> rcm_pb2.ActionCommand:
        """根据 LLM 输出构造 ActionCommand."""
        verb = choice["verb"]
        cmd = rcm_pb2.ActionCommand(verb=verb)
        
        # 根据 verb 填充对应字段
        if verb in ("Append", "Insert", "Replace"):
            cmd.fragment.role = choice.get("role", "user")
            cmd.fragment.text = choice["content"]  # ← Python 端 LLM 生成的 content
        if verb in ("Remove", "Replace", "Insert", "Swap"):
            cmd.fragment_id = choice["fragment_id"]
        if verb == "Swap":
            cmd.fragment_id2 = choice["fragment_id2"]
        if verb in ("Model", "Activate", "Deactivate"):
            cmd.name = choice["name"]
        
        return cmd
```

### 10.4 SetPrompt 等价实现（Replace captain fragment）

gRPC ActionSpace 当前不含 `SetPrompt`。等价实现：

**约定**：captain prompt 永远作为 ctx 的**第 0 条 system fragment**（tag="agent"）。

**SetPrompt 等价于**：

```python
# 找到 captain fragment id
captain_frag_id = next(
    f.id for f in state.fragments
    if f.role == "system" and f.tag == "agent"
)

# 用 Replace 替换 content
new_captain_prompt = "..."  # LLM 生成的新 prompt
cmd = rcm_pb2.ActionCommand(
    verb="Replace",
    fragment_id=captain_frag_id,
    fragment=rcm_pb2.FragmentContent(
        role="system",
        text=new_captain_prompt,
    ),
)
state, action_space = await client.step(mid, cmd)
```

**ActionSpace 怎么暴露这个候选**：

Replace 在 Free mode 下列出 "one per fragment × one per prompt"。如果 catalog.prompts 包含 "captain_v2 / captain_v3 / ..." 等候选 prompt，每个都是一个 Replace candidate。

**对 MoEH 训练的 implication**：

- `actual_action_type` 包含 Replace = 2 (而非 SetPrompt = 11)
- MoEH 输出 distribution 也只有 12 个 action type（不含 SetPrompt）
- 但语义上"修改 captain prompt"通过 Replace + content 实现

**Action type 索引调整**（gRPC 兼容）：

```python
ACTION_NAMES = [
    "Append", "Insert", "Replace", "Remove", "Swap",  # 0-4
    "Take", "Halt", "Done",                            # 5-7
    "Model", "Activate", "Deactivate",                 # 8-10
    # 11 留空 (原 SetPrompt 位置)
    # 实际 SetPrompt 通过 Replace (idx=2) 实现，区分通过 fragment.tag
]
```

或者继续保留 12 dim 但 idx 11 永远 0：

```python
N_ACTIONS = 11  # gRPC 模式
# 或 N_ACTIONS = 12 with idx 11 always 0 (兼容 CLI 模式)
```

**推荐保留 N_ACTIONS=12**——CLI 模式仍支持 `SetPrompt` Action variant，训练时只是不激活这个 idx。

### 10.5 Embedding Service (Python 端)

```python
# train/embedding_service.py

from openai import AsyncOpenAI
from lru import LRU
import hashlib

class EmbeddingService:
    """Python 端 embedding cache，跟 RCM server 解耦."""
    def __init__(self, qwen_client: AsyncOpenAI, model="qwen3-embedding-0.6b"):
        self.client = qwen_client
        self.model = model
        # Fragment cache: LRU 10K
        self.fragment_cache = LRU(10000)
        # Tool / Model cache: per-run, 启动时填充
        self.tool_cache = {}
        self.model_cache = {}
        # Purpose / Env cache: per-task
        self.purpose_cache = {}
        self.env_cache = {}
    
    async def embed(self, text: str) -> list[float]:
        response = await self.client.embeddings.create(
            input=text, model=self.model
        )
        return response.data[0].embedding
    
    async def embed_fragment(self, frag: rcm_pb2.Fragment) -> list[float]:
        key = hashlib.md5(
            f"{frag.role}|{frag.kind}|{frag.text_preview}".encode()
        ).hexdigest()
        if key in self.fragment_cache:
            return self.fragment_cache[key]
        
        text = f"<role={frag.role}><kind={frag.kind}> {frag.text_preview}"
        emb = await self.embed(text)
        self.fragment_cache[key] = emb
        return emb
    
    async def embed_fragments_parallel(self, frags) -> list[list[float]]:
        return await asyncio.gather(*[self.embed_fragment(f) for f in frags])
    
    async def init_tool_cache(self, catalog_tools_descriptions: dict):
        """启动时预填充 tool embedding."""
        for name, desc in catalog_tools_descriptions.items():
            self.tool_cache[name] = await self.embed(
                f"[INACTIVE] {name}: {desc}"
            )
            # 同时 cache active 版本
            self.tool_cache[f"{name}__active"] = await self.embed(
                f"[ACTIVE] {name}: {desc}"
            )
    
    async def embed_purpose(self, purpose: str) -> list[float]:
        if purpose not in self.purpose_cache:
            self.purpose_cache[purpose] = await self.embed(purpose)
        return self.purpose_cache[purpose]
```

### 10.6 N 并行 server 部署

```bash
# 启动 4 个 RCM server，绑定不同端口
RUST_LOG=info cargo run -p server -- --port 50051 &
RUST_LOG=info cargo run -p server -- --port 50052 &
RUST_LOG=info cargo run -p server -- --port 50053 &
RUST_LOG=info cargo run -p server -- --port 50054 &

# Python trainer
python train/phase_c_grpo.py \
    --servers localhost:50051,localhost:50052,localhost:50053,localhost:50054 \
    --weights weights/phase_b_bc.pt \
    --iterations 20
```

GRPOTrainer 维护 server_pool，每 task 的 k=4 rollouts 分散到 4 个 server 上并行跑——**真正的 group rollout 并发**。

### 10.7 ActionSpace 的 sampling

Hybrid Policy 在 §10.3 让 LLM 看 state + hint + action_space 选 action——但实际可以更精细：

**方案 A**：LLM 自由选 action（从 ActionSpace 列表里挑）
- Prompt 列出所有 candidate (with index)
- LLM 输出 {"choice": idx, "content": ...}
- 简单但 LLM 看到长 ActionSpace 可能 overwhelmed

**方案 B**：MoEH prior 引导，LLM 在 top-3 type 内选
- 取 mixed_dist 的 top-3 action type
- 仅列出这 3 类的 candidates
- 缩小 LLM 决策空间

**方案 C**：MoEH 直接采样 type，LLM 仅生成 content
- 从 mixed_dist 采样一个 action type
- 如果 type 需要 content (Append/Replace/Insert)，LLM 生成
- 如果不需要，直接构造 ActionCommand
- 最大化训练信号传导

**推荐 B**（balance）：缩小 ActionSpace 让 LLM 决策聚焦，但仍保留 LLM 对 type 的最终选择权（避免吕布骑狗）。

### 10.8 跟 CLI 模式的关系

| | gRPC 模式（训练） | CLI 模式（部署） |
|---|------------------|-----------------|
| Policy 在哪 | Python 客户端 | RCM 内部 Captain |
| LLM client 在哪 | Python 端 (OpenAI SDK) | RCM Reactor 内部 |
| MoEH inference 在哪 | Python torch | Rust candle |
| Embedding 在哪 | Python qwen3 API | Rust 同 + 启动 cache |
| ActionPriorPhase | 不存在（Python 内联） | 挂在 pre_halt |
| SetPrompt 实现 | Replace captain fragment via ActionCommand | 原生 Action::SetPrompt |
| 适用场景 | RL 训练 / GRPO rollout | Production user invocation |

两种模式**共享 catalog.prompts 演化结果**——Phase C 训完导出的 .rcm 在 CLI 模式直接生效。

---

## 11. Bench Runner

### 10.1 数据结构

```rust
// crates/bench/src/lib.rs (新 crate)

pub trait Benchmark: Send + Sync {
    fn name(&self) -> &str;
    fn tasks(&self) -> Vec<TaskSpec>;
    async fn evaluate(&self, task: &TaskSpec, final_state: &State) -> TaskOutcome;
}

pub struct TaskSpec {
    pub id: TaskId,
    pub purpose: String,
    pub initial_files: Option<HashMap<PathBuf, String>>,
    pub success_criterion: SuccessCriterion,
    pub base_seed: u64,
}

pub struct TaskOutcome {
    pub task_id: TaskId,
    pub success: bool,
    pub score: f32,
    pub total_tokens: u64,
    pub latency_secs: f64,
    pub error_msg: Option<String>,
    pub reward: f32,
}

pub struct Metrics {
    pub success_rate: f32,
    pub avg_tokens: f64,
    pub avg_latency: f64,
    pub hitch_rate: f32,
    pub action_type_distribution: HashMap<String, u32>,
    pub expert_utilization: HashMap<usize, u32>,
}
```

### 10.2 主流程

```rust
pub async fn run_benchmark(
    seed_catalog: Catalog,
    bench: Box<dyn Benchmark>,
) -> (Catalog, Metrics) {
    let mut catalog = seed_catalog;
    let mut outcomes = Vec::new();
    let mut last_outcome: Option<TaskOutcome> = None;
    
    for task in bench.tasks() {
        // 1. 构造 State
        let mut state = State::from_catalog(&catalog, &task.purpose);
        
        // 2. 注入 verdict
        if let Some(verdict) = last_outcome.take() {
            inject_verdict_fragment(&mut state.ctx, &verdict);
        }
        
        // 3. 跑 task
        let final_state = Accelerator::new(state).run().await;
        
        // 4. 评估
        let outcome = bench.evaluate(&task, &final_state).await;
        
        // 5. Sync prompts back（跨 task 演化）
        sync_prompts_back(&final_state.res, &mut catalog);
        
        last_outcome = Some(outcome.clone());
        outcomes.push(outcome);
    }
    
    // 6. 导出 evolved catalog
    catalog.to_rcm_file("evolved.rcm")?;
    
    let metrics = aggregate_metrics(&outcomes);
    (catalog, metrics)
}

fn sync_prompts_back(final_res: &Resources, catalog: &mut Catalog) {
    for (name, content) in &final_res.prompts {
        let existing = catalog.prompts.get(name);
        if existing != Some(content) {
            hook!(event = "prompt_evolved",
                  name, new_len = content.len(),
                  old_len = existing.map(|c| c.len()).unwrap_or(0));
            catalog.prompts.insert(name.clone(), content.clone());
        }
    }
}

fn inject_verdict_fragment(ctx: &mut Context, verdict: &TaskOutcome) {
    let content = format!(
        "[Last Task Result]\n\
         Task: {}\nOutcome: {}\nScore: {:.2}\n\
         Tokens used: {}\n{}",
        verdict.task_id,
        if verdict.success { "SUCCESS" } else { "FAILURE" },
        verdict.score, verdict.total_tokens,
        verdict.error_msg.as_deref().unwrap_or("")
    );
    ctx.append(Fragment::system(content).with_tag("verdict"));
}
```

### 10.3 Group Rollout（仅 training 时启用）

```rust
pub async fn run_group_rollout(
    catalog: &Catalog,
    task: &TaskSpec,
    k: usize,
) -> Vec<Rollout> {
    use futures::future::join_all;
    
    let futures = (0..k).map(|i| {
        let mut state = State::from_catalog(catalog, &task.purpose);
        state.set_seed(task.base_seed + i as u64);
        state.set_temperature(0.6 + 0.05 * i as f32);
        
        async move {
            let trajectory = Accelerator::new(state).run_with_logging().await;
            Rollout { trajectory, ... }
        }
    });
    
    join_all(futures).await
}
```

仅在训练阶段启用——production 部署用 `run_benchmark` (k=1)。

---

## 12. 完整执行轨迹示例

跑 "fix sort bug" task（已加载训好的 MoEH）：

```
[初始状态]
  current_step = 0
  catalog.prompts['captain'] = 演化过的 prompt（含累积经验）
  
[初始 context]
  [system tag=agent]      演化过的 captain prompt
  [system tag=instruction] AGENTS.md 内容
  [system tag=verdict]    上 task: 'bug-91', SUCCESS, 8000 tokens
  [system tag=env]        cwd=/repo, platform=linux
  [user tag=purpose]      修复 sort.py 排序 bug

[step 1]
  Macro: {step=0, fragments=5, hitches=0, inbox=0, ...}
  
  ActionPriorPhase:
    Embed: per-fragment (cached for system fragments) + purpose (cached) + env (cached)
    Fusion: per-modality pool + cross-modality attn → [B, 896]
    Router → [0.05, 0.10, 0.40, 0.20, ..., 0.03]
            ← Expert 3 (0.40) + Expert 6 (0.20) top-2
    Mixed dist: [Halt=0.42, Take=0.15, Append=0.12, SetPrompt=0.05, ...]
    
    注入 [system tag="action_prior"] hint
  
  Captain.decide:
    inbox 空 + first call → Halt
  
  → reactor.react
    LLM 看 ctx (含 prior hint "推 Halt 42%") 决定调 fs read
    inbox 收到 tool_call + tool_result

[step 2-3] inbox 非空 → Take（短路，不调 prior/LLM）

[step 4]
  Macro: {step=3, fragments=8, hitches=0, ...}
  Prior: [Halt=0.5, Append=0.2, ...]
  LLM 看 hint → Action::Halt (尝试写修复)
  reactor: LLM 写代码 + 跑测试 → 测试失败 → Hitch in inbox

[step 5] Take

[step 6]
  Macro: {step=5, fragments=10, hitches=1, ...}
  Prior:
    Router → top-2 = Expert 1 (0.7) + Expert 5 (0.3)
            ← Expert 1 训练后学到"recent_hitch>0 → 切模型"
    Mixed dist: [Model=0.30, Remove=0.18, Halt=0.20, ...]
  
  LLM 看 hint → Action::Model("deepseek-v3.1")  # 采纳建议
  apply: resources.active_model = "deepseek-v3.1"

[step 7]
  Prior: [Remove=0.4, Halt=0.3, ...]
  LLM → Action::Remove(failed_test_fragment_id)

[step 8]
  Prior: [Halt=0.5, ...]
  LLM → Action::Halt
  reactor: 新模型重试 → 成功

[step 9-11] Take

[step 12]
  Macro: {step=11, fragments=16, hitches=1, ...}
  Prior:
    Router → top-2 = Expert 7 (0.65) + Expert 2 (0.35)
            ← Expert 7 训练后学到"task 接近完成 + 有经验 → SetPrompt"
    Mixed dist: [SetPrompt=0.25, Append=0.20, Done=0.18, ...]
  
  LLM → Action::Append("[reflection] 这次学到：处理 generic 类型...")

[step 13]
  Prior: [SetPrompt=0.45, Done=0.20, ...]
  LLM → Action::SetPrompt("captain", "...加上 generic 处理教训")
  apply: resources.prompts["captain"] 更新

[step 14]
  Prior: [Done=0.55, ...]
  LLM → Done
  break

[bench runner 后处理]
  sync_prompts_back: catalog.prompts["captain"] 更新
  evaluate: SUCCESS, tokens=14200
  下个 task 启动时：
    - 用更新后的 catalog.prompts["captain"]
    - 注入本次 verdict 到初始 context
    - MoEH 持续工作
```

**注意点**：

1. **Prior 每个非短路 step 都跑**（mocheap ~2ms inference）
2. **LLM 始终自由**——Prior 只是 hint
3. **Expert 分化是 emergent**：Expert 1 学到"hitch→Model"，Expert 7 学到"近完成→SetPrompt"，没人预设
4. **SetPrompt 跨 task 演化 captain prompt**——capacity 持续生长

---

## 13. 路线图

| 阶段 | 目标 | 工作量 | 关键交付 |
|------|------|--------|---------|
| **P0** | machine 层 issue 收尾 | 1 周 | Hitch encode (#1/#2) + Context Result (#5) |
| **P1** | Action::SetPrompt + apply_action | 2 天 | Variant 加 + 测试 |
| **P2** | MacroObservation + Machine 主循环 改造 | 1 周 | macro_obs 注入 + 状态追踪 |
| **P3** | NoOpActionPrior + ActionPriorPhase + Embedding Cache | 1 周 | Cold start 模式可跑 |
| **P4** | Bench crate + Terminal-Bench adapter | 2 周 | `accelerate bench run` 命令 |
| **P5** | Verdict 注入 + sync_prompts_back + .rcm v3 扩展 | 1 周 | 跨 task 经验闭环 |
| **P6** | Terminal-Bench cold start 实验 | 2 周 | Baseline 数据 + 收集 trajectory |
| **P7** | Phase A: trajectory log 收集 500 task | 1 周 | `phase_a_data.json` |
| **P8** | Phase B: MoEH BC pretrain (Python) | 1 周 | `phase_b_bc.pt` |
| **P9** | Phase C 工程: iterative offline pipeline | 2 周 | Python orchestration + RCM ↔ Python 通路 |
| **P10** | Phase C 训练: 20 iterations GRPO | 4-6 周 | `moeh_final.safetensors` |
| **P11** | candle 加载 + ActionPriorPhase 集成 MoEH | 2 周 | RL-augmented 模式可跑 |
| **P12** | Terminal-Bench RL-augmented 实验 + 调参 | 2 周 | 跟 cold start 对比数字 |

**Total: ~22 周到 RL-augmented 跑通**。

### 里程碑

| Milestone | 通过条件 |
|-----------|---------|
| M1 (P3) | NoOpPrior + Captain 跑通；hook 事件正确 |
| M2 (P6) | Cold start Terminal-Bench success_rate ≥ AHE baseline |
| M3 (P8) | BC 后 MoEH 输出与 LLM action 一致率 > 70% |
| M4 (P10) | 20 iterations GRPO 后 expert utilization 均匀 (std < 0.15) |
| M5 (P12) | RL-augmented 比 cold start: (a) 不退化 (b) inference 速度 ≥ 2× |

---

## 14. AGENTS.md 新增条款

```
## MoEH 设计哲学

- RCM 是 in-memory 运行时。EHS 不引入运行时持久化层。
  trace、catalog 演化全部在进程内。

- "持久化"只允许两种形态：
  1. 编译期 include_str!（默认 prompts 等静态资源）
  2. 进程退出前一次性导出 .rcm 配置（含 [prompts] / [action_prior] section）
  Router weights 启动时一次性加载到内存。
  禁止运行时写文件、写数据库、写 git。

- Policy 本身就是 evolve agent。不引入：
  - EvolveAgent trait
  - MetaAction enum
  - Workspace struct
  - Memory 数据结构
  - 额外的反思 Agent
  - Mode enum（mode 是 emergent，不预设）
  - HarnessExpert preset (experts 分化是 emergent through GRPO)
  Policy 通过 Action::SetPrompt 累积跨 task 经验。

- Action enum 是编译期固定 12 个 variant。

- MoEH:
  - N=8 experts，top-k=2，编译期固定
  - Experts 不预设语义——通过 GRPO 训练 emergent 分化
  - Experts 是 small MLP（不是 prompt 或 config preset）
  - Router 不下沉到 Action level（避免吕布骑狗）

- MoEH model 只决定 action type 的 prior distribution（粗粒度）。
  LLM 决定具体 Action 的 content（细粒度）。
  Router 决策不限制 LLM 的 Action 选择（hint 是 soft advice）。
  禁止用 router 输出强制 mask LLM 的 action 集合。

- ActionPriorPhase 是 optional。.rcm 不含 [action_prior] section 时
  使用 NoOpActionPrior，等价于纯 LLM-as-Policy 模式。

- MacroObservation 字段编译期固定 8 个。新增字段必须 review。
  禁止在 macro_obs 里塞 LLM-level 信息（避免泄露 context 到 router 层）。

- Embedding 通过 frozen Qwen3-embedding-0.6b（或等效），不嵌入 RCM workspace。
  Embedding cache（LRU + per-tool/model/env/purpose）启动时建立或 task 级。

- Trajectory logging 仅在 ACCELERATOR_TRAJECTORY_LOG 环境变量设置时启用。
  Production 部署默认不开。

- 训练 pipeline 在外部 Python 项目。RCM 仅负责：
  - inference（加载 .safetensors）
  - trajectory logging（可选）
  - bench rollout
  禁止在 Rust workspace 内加训练框架依赖（torch / candle-training 等）。

- GRPO 训练:
  - k=4 rollouts per task (parallel, different seeds + temperatures)
  - PPO clipped objective (ε=0.2)
  - Auxiliary losses: load balancing + importance + diversity
  - Iterative offline (每 iteration 重新 rollout)
  - 4 PPO update epochs per iteration

- gRPC Server 模式:
  - RCM gRPC server (crates/server) 不知道 policy / reward / MoEH 存在
  - 仅暴露 New / Step / Destroy 三个 RPC
  - 训练全程通过 gRPC 驱动，policy + LLM client + MoEH 全部在 Python 端
  - Server 端不引入 Python / Torch / OpenAI SDK 依赖
  - ActionSpace 的 "Append per prompt resource" 候选不变 — content 动态生成
    走 ActionCommand.fragment 字段从 client 端传入
  - SetPrompt 通过 Replace ctx 第 0 条 system fragment (tag="agent") 等价实现
    不需要新增 ActionCommand verb

- Training 跟 Production 部署的 dual-mode:
  - 训练 (Phase A-C): gRPC server + Python trainer
  - 部署 (Phase D, end-user): CLI 模式 + Captain Policy + ActionPriorPhase
  - 两种模式共享 catalog.prompts 演化结果 (.rcm 文件互通)
```

---

## 15. 与 RCM 设计哲学的契合

| RCM 哲学 | MoEH 兑现 |
|---------|----------|
| **Zero Runtime IO** | ✓ Weights 启动一次性加载；trajectory log 仅训练时启用 |
| **Self-Hosting** | ✓ ActionPrior 是 Policy 组件，无独立 Agent |
| **Hard Boundaries via Types** | ✓ Action / MacroObs / N_EXPERTS / TOP_K 编译期固定 |
| **轻量化** | ✓ MoEH 2.3M params；inference < 2ms |
| **形式化兼容** | ✓ Φ = ω ∘ π 保留；π 内部 hierarchical |
| **充分利用 Action 空间** | ✓ Prior 引导 LLM 用 step-level modification |
| **真自进化** | ✓ MoEH 训练 emergent + catalog.prompts 跨 task 演化 |
| **不引入预设分类** | ✓ Experts 是 learned MLPs，命名 post-hoc |
| **不引入吕布骑狗** | ✓ Router 只输出 distribution，LLM 完全自由 |
| **Capacity-prompt decoupling** | ✓ 总经验 N×s vs prompt k×s, ratio N/k |
| **可重放性** | ✓ (state_dict, action, prior_output) 完整可重放 |

---

## 16. 跟 paper 的对比

| 维度 | ACE | AHE | TF-GRPO | MemRL | MemQ | **MoEH** |
|------|-----|-----|---------|-------|------|----------|
| 形式化定义 | ✗ | ✗ | ✗ | ✗ | partial | **✓** |
| 不引入持久化层 | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** |
| 不引入额外 Agent | ✗ | ✗ | partial | partial | partial | **✓** |
| 不引入 Memory 数据结构 | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** |
| Hierarchical decision | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** (advisor + LLM) |
| RL 真训模型 | ✗ | ✗ | ✗ | ✗ | partial | **✓** (GRPO on MoEH) |
| MoE 结构 | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** (8 experts) |
| LLM 自由度 | ✓ | ✓ | ✓ | ✓ | ✓ | **✓** (advisor 不限制) |
| Capacity-prompt decoupling | ✗ | ✗ | partial | partial | partial | **✓** |
| Cold start 友好 | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** (NoOpPrior fallback) |

**MoEH 独特位置**：唯一同时实现 hierarchical RL + MoE-on-priors + LLM 自由 + zero IO + capacity-prompt decoupling 的方案。

---

## 17. 风险与缓解

### 风险 1: Phase A 数据 diversity 不够

**症状**：cold start 收集的 trajectory 行为模式有限，BC pretrain 后 MoEH 缺乏 diversity。

**缓解**：
- Phase A 跑 diverse benchmark（Terminal-Bench + SWE-bench + AppWorld 混合）
- 用多个不同 base captain prompt
- 加 entropy bonus 鼓励 distribution diversity

### 风险 2: Expert collapse

**症状**：训练后某些 experts 几乎不被激活，effectively N < 8。

**缓解**：
- Load balancing loss + importance loss + diversity bonus 三层防御
- 监控 expert_usage_std，< 0.1 时报警
- Expert reset：长期 utilization < 5% 的 expert 重新随机初始化

### 风险 3: LLM 完全忽略 prior

**症状**：LLM action 跟 mixed_dist 推荐 always 不一致。

**缓解**：
- 监控 KL(actual_action_one_hot || mixed_dist) 平均值
- captain.txt 明确"prior advisor 通常有用，应该参考"
- 接受 distillation 视角：如果 LLM > prior，那 prior 学的就是 LLM behavior 也 OK

### 风险 4: Embedding cost 失控

**症状**：单 task embedding 调用累计 > $0.1。

**缓解**：
- 强 cache（LRU 10K entries）
- Per-tool/model 启动 cache + per-task purpose/env cache
- 监控 cache hit rate，< 70% 时报警

### 风险 5: candle 加载 MoEH 不顺

**症状**：candle 对 MultiheadAttention 支持有 bug。

**缓解**：
- Fallback 1: 用 tract（ONNX runtime）
- Fallback 2: 用 burn 框架
- Fallback 3: Python subprocess（接受 10-50ms 延迟）

### 风险 6: Catalog prompts 持续膨胀

**症状**：跑 100+ task 后 captain prompt > 10K tokens，撑爆 context window。

**缓解**：
- captain.txt 明确"经验段 < 2000 token，新经验合并而非追加"
- 加 CompactPhase（监控 prompt 长度超阈值时提示 compact）
- 监控 catalog.prompts['captain'].len()，> 阈值报警

### 风险 7: GRPO 训练成本爆炸

**症状**：单 iteration 实际超 $50 预算。

**缓解**：
- Dry run k=4 × 5 task 算清成本
- 用 DeepSeek-V3.1（比 GPT-4.1 便宜 10×）
- 减小 task subset (80 → 30)
- 降低 k=4 → k=2（牺牲 variance reduction）

---

## 18. 终极原则

> **MoEH 学的是"在某种 state 下经验偏好哪种 action type 的 distribution"——
> 一个 distillation of infinite implicit experts。
> Top-k routing 只决定"用哪几个 expert 的 distribution 加权混合"作为 prior hint。
> LLM 在 hint 引导下保留 12 种 Action 的完整选择自由。
> 跨 task 经验通过 LLM 自己发的 SetPrompt 累积进 catalog。
> 进程退出 = 全部内存归零，evolved 结果一次性导出为 .rcm。**

具体翻译：

| 错的做法 | 对的做法 |
|---------|---------|
| 预设 N 个 "coding/writing/research" harness | 8 个 experts 通过 GRPO emergent 分化 |
| Router 强制 mask LLM action | Router 输出 distribution 作为 soft hint |
| LLM 看 mode 切换 prompt 风格 | LLM 看 prior distribution 自由决策 |
| 引入 Memory / Workspace 数据结构 | catalog.prompts + .rcm 导出 |
| 引入额外反思 Agent | LLM 自己决定何时 SetPrompt |
| 让 router 决定 Action 细节 | Router 决定 type prior，LLM 决定 content |
| 用大模型当 router | 2M params 小模型当 router |
| Mean-pool 长 context 到一个 emb | per-fragment embed + attention pool |
| Tools/Models 用 one-hot | 用 embedding (active 标注 + attention pool) |
| 简单 concat 所有 modality | Hierarchical Fusion (per-modality pool + cross-modality attn) |

---

## 附录 A: 实施时需要拍板的小决策

1. **Context embedder 选 Qwen3-embedding-0.6b 还是 BGE-small**
   - 推荐: Qwen3-embedding-0.6b（capacity 强 + 语言原生）

2. **Embedding 用 API 还是本地 candle 加载**
   - 推荐: API（避免嵌入 LLM 到 RCM Rust 端）

3. **`recent_hitch_count` 的 window size W**
   - 推荐: W=10 step

4. **MAX_CONTEXT_LEN 选多大**
   - 推荐: 100（超过 drop 早期 fragment）

5. **Bench runner 顺序 vs 并行**
   - 推荐: 顺序（production） + group rollout（仅训练）

6. **GRPO k**
   - 推荐: k=4

7. **PPO clipped ε**
   - 推荐: 0.2（标准）

8. **训练框架在 Rust workspace 还是独立 Python repo**
   - 推荐: 独立 Python repo

9. **N_EXPERTS 选 8 还是其他**
   - 推荐: 8（Mixtral 标准）

10. **Modality token dim D**
    - 推荐: 128（轻量 + 表达力平衡）

---

## 附录 B: 跟现有 issue 的关系

EHS 把这些 issue 都吸收为前置工作：

- **issue #1 / #2（Hitch 完整版）**：P0 完成
- **issue #5（Context API Result 化）**：P0 完成
- **issue #10（Shell 环境）**：已在 main 分支落地
- **issue #11（Captain hitch 重试）**：已在 main 完成

P0 收尾后 P1+ 进入新疆域。

---

## 附录 C: Variable-Length Batch Padding

训练时 batch 内 sample 的 N/K/M/P 都不同，需要 padding + mask：

```python
def collate_state_batch(samples):
    """
    Pad variable-length modalities to batch max.
    """
    B = len(samples)
    max_N = max(s["n_frags"] for s in samples)
    max_K = max(s["n_inbox"] for s in samples)
    max_M = max(s["n_tools"] for s in samples)
    max_P = max(s["n_models"] for s in samples)
    
    frags_embs = torch.zeros(B, max_N, EMB_DIM)
    frags_mask = torch.zeros(B, max_N, dtype=torch.bool)
    inbox_embs = torch.zeros(B, max_K, EMB_DIM)
    inbox_mask = torch.zeros(B, max_K, dtype=torch.bool)
    tools_embs = torch.zeros(B, max_M, EMB_DIM)
    tools_mask = torch.zeros(B, max_M, dtype=torch.bool)
    models_embs = torch.zeros(B, max_P, EMB_DIM)
    models_mask = torch.zeros(B, max_P, dtype=torch.bool)
    
    purpose_embs = torch.zeros(B, EMB_DIM)
    env_embs = torch.zeros(B, EMB_DIM)
    macro_obs = torch.zeros(B, MACRO_INPUT_DIM)
    
    for b, s in enumerate(samples):
        n = s["n_frags"]; frags_embs[b, :n] = s["frags"]; frags_mask[b, :n] = True
        k = s["n_inbox"]; inbox_embs[b, :k] = s["inbox"]; inbox_mask[b, :k] = True
        m = s["n_tools"]; tools_embs[b, :m] = s["tools"]; tools_mask[b, :m] = True
        p = s["n_models"]; models_embs[b, :p] = s["models"]; models_mask[b, :p] = True
        purpose_embs[b] = s["purpose"]
        env_embs[b] = s["env"]
        macro_obs[b] = s["macro"]
    
    return {
        "fragments_embs": frags_embs, "fragments_mask": frags_mask,
        "inbox_embs": inbox_embs, "inbox_mask": inbox_mask,
        "tools_embs": tools_embs, "tools_mask": tools_mask,
        "models_embs": models_embs, "models_mask": models_mask,
        "purpose_emb": purpose_embs, "env_emb": env_embs,
        "macro_obs": macro_obs,
    }
```

**Bucketing 优化**：按 N（fragment count）排序后分 batch，减少 padding 浪费。

---

## 附录 D: 训练数据格式规范

### Phase A 输出 (trajectory_log JSON)

```json
{
  "task_id": "term-bench-42",
  "step_idx": 5,
  "macro_obs": {
    "step_count": 5,
    "fragment_count": 10,
    "recent_hitch_count": 1,
    "inbox_size": 0,
    "tokens_used_total": 3200,
    "time_since_last_halt": 3,
    "active_tools_count": 5,
    "steps_in_current_task": 5
  },
  "state_dict": {
    "fragments_embs": [[...], [...], ...],
    "inbox_embs": [],
    "tools_embs": [[...], ...],
    "models_embs": [[...]],
    "purpose_emb": [...],
    "env_emb": [...]
  },
  "actual_action": {"type": "Model", "value": "deepseek-v3.1"},
  "actual_action_type": 8,
  "timestamp": "2026-05-20T12:34:56Z"
}
```

### Phase A 任务结果 (task_outcome JSON)

```json
{
  "task_id": "term-bench-42",
  "success": true,
  "score": 0.85,
  "total_tokens": 15000,
  "latency_secs": 45.2,
  "trajectory_length": 18,
  "error_msg": null,
  "reward": 8.5
}
```

### Phase B/C checkpoints

- BC: PyTorch state_dict 标准 `.pt`
- GRPO 每 iteration: `.safetensors`
- Final: 同时导出 `.safetensors` + `.onnx` (RCM candle/tract 加载备选)

---

## 附录 E: Capacity-Prompt Decoupling 详细 Scaling 法则

```
单 prompt 系统:
  experience_capacity ~ prompt_size
  → 经验越多 prompt 越长 → context collapse
  → effective scaling: 受限于 LLM 注意力上限 (~50K tokens)

MoEH 系统:
  experience_capacity = N × s_expert
  prompt_size_per_step = base + k × s_expert    (k=2, base=captain+env+verdict)
  
  Decoupling: experience scales with N, prompt scales with k
  ratio: N / k

具体数字 (s_expert = 6K tokens):
  | Setup           | N  | k | Total Exp | Prompt | Ratio |
  | --------------- | -- | - | --------- | ------ | ----- |
  | Single-prompt   | 1  | 1 | 50K       | 50K    | 1×    |
  | MoEH baseline   | 8  | 2 | 48K       | ~14K   | 3.4×  |
  | MoEH expand     | 16 | 2 | 96K       | ~14K   | 6.9×  |
  | MoEH long-term  | 32 | 2 | 192K      | ~14K   | 13.7× |

关键性质:
  - 增加 N (add expert) 不影响 prompt budget
  - 增加 s_expert (refine experts) 跟 prompt 线性相关
  - sweet spot: N=8 起步，看 utilization 决定是否扩 N
```

---

## 附录 F: Embedding Cache 命中率详细分析

```
Cache 类型           | 内容                  | Hit Rate | Notes
─────────────────────|──────────────────────|──────────|──────────────
Tool cache           | tool desc            | ~100%    | 启动时固定
Model cache          | model desc           | ~100%    | 启动时固定
Purpose cache        | task purpose         | ~100%    | task 级 cache
Env cache            | cwd/platform/...     | ~100%    | task 级 cache
Fragment cache:
  - System frags     | agent/instruction    | ~99%     | 跨多个 step 不变
  - Verdict frag     | last task result     | ~100%    | task 启动注入后不变
  - User frags       | purpose, etc.        | ~95%     | 偶尔修改
  - Tool calls       | LLM 历史 tool calls  | ~95%     | 仅新增的 miss
  - Tool results     | tool 历史输出        | ~95%     | 同上
Inbox cache:
  - Recent results   | reactor 刚 push      | ~10%     | 短暂存在
  - Hitches          | 错误信息             | ~50%     | 类似 hitch 多次出现

加权平均 hit rate: ~85%

单 task 实际 embed call 数估算 (50 step):
  - Fragments: 平均 30 frags × 5% miss = 1.5 miss per step → ~75 calls
  - Inbox: 平均 1.5 msg × 90% miss = 1.35 miss per step → ~67 calls
  - Tools: 0 (启动 cache)
  - Models: 0 (启动 cache)
  - Purpose: 0 (task cache)
  - Env: 0 (task cache)
  - 总: ~142 calls per task

成本 (Qwen3-embedding-0.6b ~$0.00002/1K tokens, 平均 200 tokens/call):
  142 calls × 200 tokens × $0.00002/1K = $0.00057 per task
  100 tasks bench = $0.057
  跟 Phase C 训练 ~$1000 相比可忽略
```


---

## 附录 G: Python Client SDK 完整结构

### G.1 Repo 布局（独立 Python repo）

```
rcm-train/                     # 独立训练 repo（非 RCM workspace）
├── pyproject.toml
├── rcm_proto/                 # 从 crates/server/proto/rcm.proto 生成
│   ├── rcm_pb2.py
│   └── rcm_pb2_grpc.py
├── rcm_client/
│   ├── __init__.py
│   └── client.py              # RCMClient (async gRPC wrapper)
├── moeh/
│   ├── __init__.py
│   ├── model.py               # MoEHActionPrior (本文 §7 实现)
│   ├── fusion.py              # HierarchicalFusion
│   ├── pools.py               # AttentionPool / PositionAwarePool
│   ├── router.py
│   └── expert.py
├── policy/
│   ├── __init__.py
│   ├── cold_start.py          # ColdStartPolicy (Phase A)
│   └── hybrid.py              # HybridPolicy (Phase C, MoEH + LLM)
├── train/
│   ├── phase_a_collect.py     # Cold-start data collection via gRPC
│   ├── phase_b_bc.py          # BC pretrain (no RCM)
│   ├── phase_c_grpo.py        # GRPO online via gRPC
│   └── embedding_service.py   # Qwen3-embedding API + cache
├── eval/
│   ├── terminal_bench.py
│   └── metrics.py
└── scripts/
    ├── start_servers.sh       # 启动 N 个 RCM gRPC server
    └── run_pipeline.sh
```

### G.2 启动脚本示例

```bash
#!/bin/bash
# scripts/start_servers.sh

set -e

# 启动 4 个 RCM gRPC server
for i in 1 2 3 4; do
    port=$((50050 + i))
    RUST_LOG=info ./target/release/rcm-server --port $port &
    echo "Started RCM server #$i on port $port (pid $!)"
done

# 等待所有 server 启动完毕
sleep 2
echo "All servers ready"

# Trap to cleanup
trap "kill $(jobs -p)" EXIT
wait
```

### G.3 完整 pipeline 调用

```bash
#!/bin/bash
# scripts/run_pipeline.sh

# Phase A: Cold-start data collection
python -m train.phase_a_collect \
    --servers localhost:50051,localhost:50052,localhost:50053,localhost:50054 \
    --tasks-file ./terminal_bench_subset.json \
    --output ./trajectory_logs/

# Phase B: BC pretrain (no RCM needed)
python -m train.phase_b_bc \
    --input ./trajectory_logs/ \
    --output ./weights/phase_b_bc.pt \
    --epochs 3

# Phase C: GRPO iterative online
python -m train.phase_c_grpo \
    --servers localhost:50051,localhost:50052,localhost:50053,localhost:50054 \
    --init-weights ./weights/phase_b_bc.pt \
    --iterations 20 \
    --tasks-per-iter 20 \
    --k-rollouts 4 \
    --output ./weights/

# Final export for RCM CLI deployment
python -m train.export_for_rcm \
    --weights ./weights/iter_20.safetensors \
    --output ./moeh_final.safetensors

# Production deployment (回到 CLI 模式)
cd /path/to/rcm-project
accelerate run --action-prior ./moeh_final.safetensors task.rcm
```

### G.4 跟 RCM workspace 的关系

```
RCM workspace (Rust):              rcm-train (Python):
├── crates/                        ├── moeh/        ← MoEH model 定义
│   ├── machine/                   ├── policy/      ← ColdStart / Hybrid
│   ├── accelerator/               ├── train/       ← BC + GRPO
│   ├── server/      ←──┐          ├── eval/        ← benchmark
│   ├── cli/             │         └── scripts/
│   └── ...              │             ↓
└── docs/proposals/      │         (从 RCM workspace 拿 .proto)
    └── evolve-harness-  │
        system.md  ←─────┘ (本 proposal)

通信:
  rcm-train → gRPC → RCM server (Rust binary)
  
RCM workspace 改动:
  - 仅 server crate 可能加 full_text field（可选）
  - 不引入 Python 依赖
  - production 部署的 accelerate CLI 通过 .rcm 加载训好的 weights
```

### G.5 Production 模式无 gRPC

User 端使用：

```bash
# 不需要启动 gRPC server
# 直接用 CLI
accelerate run --action-prior ./moeh_final.safetensors task.rcm
```

CLI 模式下：
- ActionPriorPhase 在 RCM 内部跑
- MoEH 通过 candle 加载 .safetensors
- 一切在单个 Rust 进程内
- 无 Python 依赖
- 零 IO（除最终 .rcm 导出）

**保留 RCM "纯 in-memory runtime" 的核心承诺**。

