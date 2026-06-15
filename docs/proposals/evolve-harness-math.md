# MoEH：H-LDP 框架下的具体实例与收敛性分析

> **配套文档**
>
> - [`learnable-harness-math.md`](./learnable-harness-math.md)：一般 H-LDP 框架（**本文的所有定义、假设、定理均建立在它之上**）。
> - [`evolve-harness-system.md`](./evolve-harness-system.md)：MoEH 系统设计。
>
> **本文目标**：
>
> 1. 把 MoEH 严格表述为 H-LDP 的一个**具体实例**；
> 2. 给出 GRPO 梯度估计量 $\hat g_t$ 满足 oracle 条件 (O1)–(O3) 所需的**额外假设**；
> 3. 应用 H-LDP 主定理给出 MoEH 的渐近收敛结论；
> 4. 设计**实验协议**用以经验验证理论假设。
>
> 本文**避免**了旧稿（v0.1）中的几处技术问题，详见附录 C 的"修正映射表"。

---

## 目录

- [§1 引言：MoEH 作为 H-LDP 的实例化](#1-引言)
- [§2 RCM 在 H-LDP 框架中的特化](#2-rcm-在-h-ldp-框架中的特化)
- [§3 MoEH 策略架构](#3-moeh-策略架构)
- [§4 Capacity-Prompt 解耦（Proposition）](#4-capacity-prompt-解耦proposition)
- [§5 GRPO 训练目标](#5-grpo-训练目标)
- [§6 信用分配](#6-信用分配)
- [§7 收敛性分析](#7-收敛性分析)
- [§8 实验设计](#8-实验设计)
- [§9 实现要点](#9-实现要点)
- [§10 与相关工作对比](#10-与相关工作对比)
- [附录 A：符号速查表](#附录-a符号速查表)
- [附录 B：关键引理证明](#附录-b关键引理证明)
- [附录 C：相对 v0.1 的修正映射表](#附录-c相对-v01-的修正映射表)

---

## §1 引言

H-LDP 框架（一般文档）把 learnable harness 抽象为"控制 $(c, e)$ 的可学习策略 $\pi_\theta$"，并给出抽象 oracle 假设下的收敛性主定理。**本文的工作就是把 MoEH 套进这个框架**——给出 $(\mathcal{X}, \mathcal{C}, \mathcal{E}, \mathcal{Y}, \pi_\theta, T, R)$ 的具体形式，给出 $\hat g_t$ 的具体计算公式，并**逐条**验证（或承认为额外假设）所需的条件。

MoEH 在 H-LDP 框架内的三个**特化设计**：

1. **动作因子化**：把 $(c, e)$ 的选择因子化为 advisor 输出的 "hint" + 冻结 LLM 协同生成的 "realized content"——以此把组合复杂动作空间 $\mathcal{C} \times \mathcal{E}$ 压缩为 advisor 可处理的小空间。
2. **MoE 参数化**：把 advisor $\pi_\theta$ 设计为 top-$k$ MoE，以实现**容量-推理成本解耦**（详见 §4）。
3. **GRPO + GAE 估计器**：以群体采样 + 群体相对 advantage + PPO 剪切作为 off-policy 梯度估计 $\hat g_t$；用 GAE-λ 做段内 credit。

下面逐条展开。

---

## §2 RCM 在 H-LDP 框架中的特化

### 2.1 状态空间分解

RCM 的可观测状态由六个相互正交的成分构成：

$$
\mathcal{X} \;=\; \underbrace{\mathcal{C}_{\mathrm{ctx}}}_{\text{context tape}} \,\times\, \underbrace{\mathcal{Q}}_{\text{inbox}} \,\times\, \underbrace{\mathcal{E}_{\mathrm{ws}}}_{\text{workdir+env}} \,\times\, \underbrace{\mathcal{R}}_{\text{resources}} \,\times\, \underbrace{\mathcal{P}}_{\text{purpose}} \,\times\, \underbrace{\mathcal{O}}_{\text{macro-obs}}
$$

记 $x = (c_{\mathrm{ctx}}, q, e_{\mathrm{ws}}, r, p, o)$。其中 $c_{\mathrm{ctx}}$ 是有限长度的 fragment 序列，$q$ 是 FIFO 队列，等等。

> **注**：上式的 $\mathcal{C}_{\mathrm{ctx}}$ 是 RCM 内部的 "context tape"，**不同于** H-LDP 中"送入 LLM 的执行上下文 $\mathcal{C}$"。后者在 RCM 中由 captain 模板根据当前 $(c_{\mathrm{ctx}}, p, \text{hint})$ 渲染而成。

### 2.2 双层动作：Advisor Hint vs Realized Action

H-LDP 的"动作"是 $(c, e) \in \mathcal{C} \times \mathcal{E}$。在 RCM 中我们做如下分解：

**Advisor 动作空间**（小，可枚举）：

$$
\mathcal{A}_{\mathrm{adv}} \;=\; \mathcal{V} \;=\; \{\mathrm{Halt}, \mathrm{Done}, \mathrm{Append}, \mathrm{Insert}, \mathrm{Replace}, \mathrm{Remove}, \mathrm{Swap}, \mathrm{Take}, \mathrm{Model}, \mathrm{Activate}, \mathrm{Deactivate}, \mathrm{SetPrompt}\}, \quad |\mathcal{V}| = 12
$$

Advisor 输出一个 verb 上的离散分布 $\pi_\theta(v \mid x), v \in \mathcal{V}$。

**Realized 动作空间**（大，含自由文本内容）：

$$
\mathcal{A}_{\mathrm{real}} \;=\; \mathcal{V} \,\times\, \Pi
$$

其中 $\Pi$ 是参数空间（Append 的 fragment 文本、Replace 的目标 id + 新文本等），含**自由文本**，非可枚举。

**LLM 的角色**：在 advisor 输出的 verb 分布上抽样一个 verb $v$，然后**为该 verb 生成 content**——也即 $\pi_{\mathrm{LLM}}: \mathcal{X} \times \mathcal{V} \to \Delta(\mathcal{A}_{\mathrm{real}})$。

**与 H-LDP 的对应**：

| H-LDP 符号 | MoEH 对应 |
|-----------|---------|
| $\mathcal{C}$（执行上下文） | captain 模板 + render($c_{\mathrm{ctx}}, p$, hint) 渲染得到的 prompt |
| $\mathcal{E}$（执行环境） | $r$（active model + tools） |
| $\mathcal{Y}$（LLM 输出） | $\mathcal{A}_{\mathrm{real}}$ |
| $\pi_\theta(c, e \mid x)$ | $\pi_\theta(v \mid x)$ 决定 hint 部分；其余固定 |
| $\pi_{\mathrm{LLM}}(y \mid c, e)$ | $\pi_{\mathrm{LLM}}(a' \mid x, \text{render}(v))$ |

> **关键设计**：advisor 只输出 verb hint，content 由 LLM 协同生成。这使得 advisor 的动作空间被压缩到 $|\mathcal{V}| = 12$，可微策略学习成为可能；同时 H-LDP 的"动作"概念被保留（hint + content 联合视为一个 H-LDP action）。

### 2.3 LLM 算子与环境转移

**LLM 算子**：给定状态 $x$ 与 hint $v$，LLM 抽样 realized action：

$$
a' \;\sim\; \pi_{\mathrm{LLM}}(\cdot \mid x, \text{render}(v))
$$

**环境转移**：RCM 的 `Machine::apply` 是纯函数：

$$
x_{t+1} \;=\; \text{apply}(x_t, a'_t)
$$

但若 $a'_t = \mathrm{Halt}$ 会触发 `reactor::react()`（其内含一次 LLM 调用与 tool 执行）。统一记为随机核

$$
x_{t+1} \;\sim\; T(\cdot \mid x_t, a'_t)
$$

H-LDP 中的总转移核（边缘化 hint 与 LLM）：

$$
P_\theta(x_{t+1} \mid x_t) \;=\; \sum_{v \in \mathcal{V}} \pi_\theta(v \mid x_t) \,\int_{a'} \pi_{\mathrm{LLM}}(a' \mid x_t, \text{render}(v)) \,T(x_{t+1} \mid x_t, a') \,\mathrm{d}a'
$$

### 2.4 验证 H-LDP 正则性假设

- **(R1) Horizon 有界**：episode 上限 $T_{\max} = 256$（实现侧 step cap）。✓
- **(R2) 奖励有界**：terminal evaluator 输出 $\in [-1, 1]$。✓
- **(R3) 可测性**：$T$ 是纯函数复合 + 离散 LLM 抽样；可测。✓
- **(R4) 时间齐次性**：需训练前固化 tool snapshots + 固定 seed + 离线 evaluator。✓（工程任务）
- **(R5) Token 截断**：context tape 上限 32K tokens。✓
- **(R6) 参数有界**：$\theta$ 由 weight decay + gradient clip 保持有界。✓

---

## §3 MoEH 策略架构

### 3.1 多模态融合

定义层次化融合 $\phi: \mathcal{X} \to \mathbb{R}^d, d = 896$。

**Step A**：对 context 每条 fragment、inbox 每条 msg、tool/model 每项，分别送入冻结 qwen3-embedding-0.6b 得到 $\mathbb{R}^{1024}$ 向量；purpose/env 整体编码到 $\mathbb{R}^{1024}$。

**Step B**：每个模态用 $1024 \to 128$ 线性投影 + Attention Pool 聚合到 $\mathbb{R}^{128}$。Context 模态用 Position-Aware Pool。

**Step C**：7 个模态 token concat 为 $[7, 128]$，过 2 层 self-attention（2 头，无 FFN），flatten 得 $\phi(x) \in \mathbb{R}^{896}$。

形式：

$$
\phi(x) \;=\; \mathrm{CrossModalAttn}\!\big(\,\mathrm{Pool}_{c_{\mathrm{ctx}}}, \mathrm{Pool}_q, \mathrm{Pool}_T, \mathrm{Pool}_M, \mathrm{Pool}_p, \mathrm{Pool}_e, \mathrm{Pool}_o\,\big)
$$

参数量 $|\theta_\phi| \approx 0.9\text{M}$。

### 3.2 路由器与 Soft Mixture

**Router**：

$$
g(x) \;=\; \mathrm{softmax}\!\left(\, \frac{W_r \phi(x)}{T_r} \,\right) \;\in\; \Delta^{N-1}, \quad W_r \in \mathbb{R}^{N \times d}
$$

**Dense soft mixture**（理论用）：

$$
\pi_\theta^{\mathrm{dense}}(v \mid x) \;=\; \sum_{e=1}^{N} g_e(x) \cdot \pi_e(v \mid x)
$$

这是 $\theta$ 的 $C^1$ 函数（softmax 与 expert 都是 $C^\infty$）。

### 3.3 Top-$k$ 近似：从 dense 到 sparse

**Top-$k$ 路由**（推断与工程用）：

$$
\mathcal{E}_k(x) \;=\; \arg\!\max_{S \subseteq \{1,\ldots,N\}, |S|=k} \sum_{e \in S} g_e(x)
$$

$$
\tilde g_e(x) \;=\; \begin{cases}
g_e(x) / \!\!\sum\limits_{e' \in \mathcal{E}_k(x)} g_{e'}(x) & e \in \mathcal{E}_k(x) \\
0 & \text{otherwise}
\end{cases}
$$

$$
\pi_\theta^{\mathrm{topk}}(v \mid x) \;=\; \sum_{e \in \mathcal{E}_k(x)} \tilde g_e(x) \cdot \pi_e(v \mid x)
$$

> **关于不可微性（详见 §7 与附录 B.3）**：
>
> $\pi_\theta^{\mathrm{topk}}$ 不是 $\theta$ 的全局 $C^1$ 函数——当两个 expert 的 gate logit 排序交换时，$\mathcal{E}_k(x)$ 跳变。但跳变集 $D = \{\theta : \text{some logits tie}\}$ 是 $\Theta$ 中的**Lebesgue 测度零集**（一族超平面的并）。
>
> **理论策略**：所有正式定理对 **dense soft mixture** $\pi_\theta^{\mathrm{dense}}$ 证明；top-$k$ 作为工程近似，其差距 $\|\pi^{\mathrm{topk}} - \pi^{\mathrm{dense}}\|_{\mathrm{TV}}$ 在 $g$ 分布集中时小。
>
> **替代方案**：若需严格可微，可用 sparsemax、entmax-α、Gumbel-top-$k$ 等。本文以 dense 为默认理论对象。

### 3.4 Expert 头

$$
\pi_e(v \mid x) \;=\; \mathrm{softmax}\!\big(\, W_e^{(2)} \,\sigma(W_e^{(1)} \phi(x)) \,\big), \quad e \in \{1, \ldots, N\}
$$

其中 $W_e^{(1)} \in \mathbb{R}^{128 \times 896}$, $W_e^{(2)} \in \mathbb{R}^{|\mathcal{V}| \times 128}$, $\sigma = \mathrm{GELU}$。Expert 头只输出 **verb 分布**。

### 3.5 Hint 渲染与有效策略

**Hint 渲染**：

$$
h_t \;=\; \mathrm{render}(\pi_\theta(\cdot \mid x_t)) \;=\; \mathrm{template}\!\big(\, \arg\max_v \pi_\theta(v \mid x_t),\; \tilde g(x_t) \,\big)
$$

例：`"Recommended action: Append (confidence 0.72, secondary Insert 0.18)"`。

**有效策略**（H-LDP 定义 2 的实例化）：

$$
\pi_\theta^{\mathrm{eff}}(a' \mid x) \;=\; \sum_{v \in \mathcal{V}} \pi_\theta(v \mid x) \cdot \pi_{\mathrm{LLM}}(a' \mid x, \mathrm{render}(v))
$$

### 3.6 可辨识性 (I) 验证策略

H-LDP 框架的**核心非退化条件**是 (I)：$\pi_\theta^{\mathrm{eff}}$ 必须对 $\pi_\theta$ 的变化敏感。在 MoEH 中等价于：**LLM 必须对 hint 敏感**。

**(P-hint) Hint 敏感性假设**：存在 $\kappa > 0$ 使得对几乎所有 $x$ 与任意两个 hint $v_1 \neq v_2$：

$$
\big\| \pi_{\mathrm{LLM}}(\cdot \mid x, \mathrm{render}(v_1)) - \pi_{\mathrm{LLM}}(\cdot \mid x, \mathrm{render}(v_2)) \big\|_{\mathrm{TV}} \;\geq\; \kappa
$$

**验证方式**（实验，详见 §8.4）：训练前用 probing 数据计算

$$
\hat \kappa \;=\; \min_{v_1, v_2, x \in \text{probing}} \big\| \hat \pi_{\mathrm{LLM}}(\cdot \mid x, \mathrm{render}(v_1)) - \hat \pi_{\mathrm{LLM}}(\cdot \mid x, \mathrm{render}(v_2)) \big\|_1
$$

若 $\hat \kappa < 0.05$（经验阈值），需修改 prompt 模板使 hint 更明确。

> **设计含义**：(P-hint) 是 hint via prompt 这一设计的**核心验证条件**。若 LLM 完全无视 hint，则 MoEH 学到的策略与 advisor 输出无关，整个学习退化。

---

## §4 Capacity-Prompt 解耦（Proposition）

> **修正说明**：v0.1 将本结果列为"定理 1"。但其本质是由定义直接得到的复杂度观察，且忽略了实际工程开销。本版改为 **Proposition**，并补充三条 caveat。

**命题 1（名义参数-FLOPs 解耦）.** 在固定 $k$（top-$k$ 路由）的 MoEH 架构中：

$$
\frac{\partial \Theta_{\mathrm{total}}}{\partial N} \;=\; |\theta_e| \;>\; 0, \qquad
\frac{\partial F_{\mathrm{inf}}^{\mathrm{nominal}}}{\partial N} \;=\; F_r' \;\ll\; F_e
$$

其中

$$
\Theta_{\mathrm{total}} = |\theta_\phi| + N \cdot |\theta_e| + |\theta_r|, \quad
F_{\mathrm{inf}}^{\mathrm{nominal}} = F_\phi + F_r + k \cdot F_e
$$

$F_r' = \partial F_r / \partial N = O(d)$（router logits 计算线性 in $N$）。

**证明**：直接由定义求偏导。$\square$

**Caveat 1（router 开销）**：当 $N \gg 16$ 时 $F_r = O(dN)$ 不再可忽略。

**Caveat 2（系统开销未计入 nominal FLOPs）**：top-$k$ 路由的实际成本还包括 expert dispatch、batching、padding、kernel launch、cache miss 等。

**Caveat 3（容量 ≠ 有效能力）**：若 router collapse（少数 expert 主导），或 expert 学到同质策略，则"容量线性增加"不等于"行为模式多样性线性增加"。这是负载均衡（§5.6）与 expert 访问下界（§6.3）所需解决的问题。

---

## §5 GRPO 训练目标

### 5.1 群体采样

对每个 batch 状态 $x_t$，固定 $\theta_{\mathrm{old}}$，并行执行 $K$ 条独立 rollout 直到 episode 结束：

$$
\mathrm{Rollout}^{(i)} \;=\; \big(\, v_t^{(i)},\; \tau^{(i)},\; R^{(i)} \,\big), \quad i = 1, \ldots, K
$$

其中 $v_t^{(i)} \sim \pi_{\theta_{\mathrm{old}}}(\cdot \mid x_t)$ 是 hint，$\tau^{(i)}$ 是 LLM + env 共同决定的后续轨迹，$R^{(i)} = \sum_{s=t}^T \gamma^{s-t} r_s^{(i)}$。

### 5.2 群体相对优势：有偏但低方差

**朴素 GRPO advantage**：

$$
\mu_t \;=\; \frac{1}{K} \sum_{i=1}^K R^{(i)}, \quad
\sigma_t \;=\; \sqrt{\frac{1}{K} \sum_i (R^{(i)} - \mu_t)^2}
$$

$$
\hat A^{(i)}_t \;=\; \frac{R^{(i)} - \mu_t}{\sigma_t + \epsilon_\sigma}
$$

**重要事实（修正自 v0.1）**：朴素 GRPO advantage **不是无偏 baseline**。原因如下：

> **澄清**：v0.1 曾用 $\frac{1}{K}\sum_i \hat A^{(i)}_t = 0$ 论证"GRPO 不引入偏差"。这是**逻辑错误**：策略梯度的无偏性指的是
> $$\mathbb{E}_{(v, R)}\!\big[\, \hat A(R) \cdot \nabla_\theta \log \pi_\theta(v \mid x) \,\big]$$
> 是否等于真实梯度，而**不是** $\hat A$ 自身均值为零。
>
> 一个 baseline $b(x)$ 不引入偏差，关键在于它**与当前动作 $v$ 独立**（条件于 $x$），从而 $\mathbb{E}_v[b(x) \nabla \log \pi(v|x)] = b(x) \nabla \sum_v \pi(v|x) = 0$。
>
> 而朴素 GRPO 中 $\mu_t = \tfrac1K \sum_j R^{(j)}$ **包含 $R^{(i)}$ 自身**，所以 $\mu_t$ 对第 $i$ 个样本不是独立 baseline，会引入 $O(1/K)$ 的偏差。$\sigma_t$ 归一化也类似。

**正确陈述**：

**引理 1（朴素 GRPO 的偏差）.** 设 $b^*(x) = V^{\pi_{\theta_{\mathrm{old}}}}(x)$ 是无偏 baseline。则朴素 GRPO 估计量的偏差满足：

$$
\Big\| \mathbb{E}_K\!\big[\hat A^{(i)}_t \cdot \nabla \log \pi_\theta(v_t^{(i)} | x_t)\big] - \nabla J(\theta) \Big\| \;\leq\; \underbrace{\frac{C_\mu}{K}}_{\text{centering bias}} + \underbrace{C_\sigma \mathbb{E}\!\big[|\sigma_t^{-1} - \mathbb{E}\sigma_t^{-1}|\big]}_{\text{scaling bias}} + \underbrace{C_V \cdot \|b^* - 0\|_\infty}_{\text{baseline gap}}
$$

详见附录 B.1。$\square$

### 5.3 Leave-one-out 修正

为消除"$\mu$ 包含 $R^{(i)}$ 自身"导致的 centering bias，使用 **leave-one-out baseline**：

$$
\mu^{-i}_t \;=\; \frac{1}{K-1} \sum_{j \neq i} R^{(j)}
$$

$$
\hat A^{(i),\mathrm{LOO}}_t \;=\; \frac{R^{(i)} - \mu^{-i}_t}{\hat \sigma_t + \epsilon_\sigma}
$$

其中 $\hat \sigma_t$ 可以仍用全样本计算（作为 scale，不需要独立性）。

**性质**：

$$
\mathbb{E}\!\big[\hat A^{(i),\mathrm{LOO}}_t\big] = \frac{1}{\hat \sigma_t + \epsilon_\sigma} \big( \mathbb{E}[R^{(i)}] - \mathbb{E}[\mu^{-i}_t] \big) = 0 \quad \text{(across batch)}
$$

且 $\mu^{-i}_t$ 与 $R^{(i)}$（条件于 $\theta_{\mathrm{old}}$）独立，故 LOO advantage 作为 baseline 不引入 centering bias。

**剩余偏差**仅来自 $\hat \sigma_t$ 归一化（scale-bias）：

**引理 2（LOO-GRPO 的偏差）.**

$$
\Big\| \mathbb{E}\!\big[\hat A^{(i),\mathrm{LOO}}_t \cdot \nabla \log \pi_\theta\big] - \nabla J(\theta) \Big\| \;\leq\; \underbrace{C_\sigma \cdot \mathrm{Var}(\hat \sigma_t)^{1/2}}_{\text{scale bias only}}
$$

详见附录 B.2。$\square$

> **推荐实现**：使用 LOO-GRPO。代码上只需把
> ```python
> mu = R.mean(dim=group_axis, keepdim=True)
> A = (R - mu) / (sigma + eps)
> ```
> 改为
> ```python
> sum_R = R.sum(dim=group_axis, keepdim=True)
> mu_loo = (sum_R - R) / (K - 1)
> A = (R - mu_loo) / (sigma + eps)
> ```

### 5.4 PPO 剪切：限制 surrogate 而非 ratio

**重要性比**：

$$
\rho^{(i)}_t(\theta) \;=\; \frac{\pi_\theta(v^{(i)}_t \mid x_t)}{\pi_{\theta_{\mathrm{old}}}(v^{(i)}_t \mid x_t)}
$$

**剪切目标**：

$$
L^{(i)}_t(\theta) \;=\; \min\!\Big(\, \rho^{(i)}_t \hat A^{(i)}_t,\;\; \mathrm{clip}(\rho^{(i)}_t,\, 1-\epsilon,\, 1+\epsilon) \hat A^{(i)}_t \,\Big)
$$

> **修正自 v0.1**：v0.1 曾假设 $|\rho^{(i)}_t| \leq 1 + \epsilon$，理由是"PPO clipping"。这是**误解**——clipping 限制的是 surrogate 中使用的 ratio，**不是**实际比值 $\rho$ 本身。

为得到 $\rho$ 的 a.s. 上界，需要额外假设之一：

**(P-floor)（采用本方案）** 存在 $p_{\min} > 0$ 使

$$
\pi_\theta(v \mid x) \;\geq\; p_{\min} \quad \forall (v, x, \theta)
$$

由 (P-floor)：$\rho^{(i)}_t \leq 1/p_{\min}$。

**(P-floor) 的工程实现**：

1. 在 softmax 输出后做 floor 操作：$\pi_\theta(v) \leftarrow (1 - \alpha_{\mathrm{floor}}) \pi_\theta(v) + \alpha_{\mathrm{floor}} / |\mathcal{V}|$，取 $\alpha_{\mathrm{floor}} = 0.01$，得 $p_{\min} = 0.01 / |\mathcal{V}| \approx 8 \times 10^{-4}$。
2. 或加 entropy regularizer $-\lambda H(\pi_\theta)$，使最优分布远离 boundary。

### 5.5 KL 正则

$$
\mathcal{L}_{\mathrm{KL}}(\theta) \;=\; \mathbb{E}_{x_t}\!\left[\, D_{\mathrm{KL}}\!\big(\pi_{\theta_{\mathrm{old}}}(\cdot \mid x_t) \,\|\, \pi_\theta(\cdot \mid x_t)\big) \,\right]
$$

KL 正则作为 (P-floor) 的**补充**而非替代：它控制 advisor 更新步长，但不直接给出 $\rho$ 的 a.s. 界。

### 5.6 负载均衡与正则化目标

**Switch-style 辅助损失**：

$$
f_e \;=\; \frac{1}{|B|} \sum_{x_t \in B} \mathbb{1}\!\big[e \in \mathcal{E}_k(x_t)\big], \quad
P_e \;=\; \frac{1}{|B|} \sum_{x_t \in B} g_e(x_t)
$$

$$
\mathcal{L}_{\mathrm{aux}}(\theta) \;=\; N \cdot \sum_{e=1}^N f_e \cdot P_e
$$

> **修正自 v0.1**：v0.1 主定理声称算法收敛到 **原始目标 $J$** 的稳定点，但实际上 fixed $\alpha > 0$ 下算法优化的是 $\tilde J = J - \alpha \mathcal{L}_{\mathrm{aux}} - \beta \mathcal{L}_{\mathrm{KL}}$。按 H-LDP §9.4 的方式 A，本文**正确陈述为收敛到 $\tilde J$ 的稳定点**，并给出 $\|\nabla J\| = O(\alpha + \beta)$ 的差距上界（详见 §7.6）。

### 5.7 完整目标

$$
\boxed{\;\mathcal{L}_{\mathrm{MoEH}}(\theta) \;=\; -\mathbb{E}_{x_t}\!\left[\, \tfrac{1}{K} \sum_i L^{(i)}_t(\theta) \,\right] \;+\; \beta \,\mathcal{L}_{\mathrm{KL}}(\theta) \;+\; \alpha \,\mathcal{L}_{\mathrm{aux}}(\theta) \;-\; \lambda_H \,\mathcal{L}_{\mathrm{ent}}(\theta)\;}
$$

其中 $\mathcal{L}_{\mathrm{ent}}(\theta) = \mathbb{E}_{x_t}[H(\pi_\theta(\cdot \mid x_t))]$ 是 entropy bonus（用以维持 (P-floor)）。

**正则化目标**：

$$
\tilde J(\theta) \;=\; J(\theta) - \beta \mathcal{L}_{\mathrm{KL}}(\theta; \theta_{\mathrm{old}}) - \alpha \mathcal{L}_{\mathrm{aux}}(\theta) + \lambda_H \mathbb{E}[H(\pi_\theta)]
$$

参数更新：

$$
\theta_{t+1} \;\leftarrow\; \theta_t - \eta_t \nabla_\theta \mathcal{L}_{\mathrm{MoEH}}(\theta_t) \;=\; \theta_t + \eta_t \hat g_t, \quad \hat g_t \approx \nabla \tilde J(\theta_t)
$$

---

## §6 信用分配

### 6.1 段内 GAE-λ

每条 rollout $i$ 内对每步 $t$ 计算 step-level GAE：

$$
\delta_t^{(i)} \;=\; r_t^{(i)} + \gamma V_\psi(x_{t+1}^{(i)}) - V_\psi(x_t^{(i)})
$$

$$
\hat A^{\mathrm{GAE},(i)}_t \;=\; \sum_{l=0}^{T-t-1} (\gamma \lambda)^l \delta_{t+l}^{(i)}
$$

然后跨 group 做 LOO-GRPO 标准化：

$$
\hat A^{(i)}_t \;=\; \frac{\hat A^{\mathrm{GAE},(i)}_t - \mu^{-i,\mathrm{step}}_t}{\hat \sigma^{\mathrm{step}}_t + \epsilon_\sigma}
$$

### 6.2 Mixture-gradient split

对 dense soft mixture $\pi_\theta(v|x) = \sum_e g_e(x) \pi_e(v|x)$：

$$
\nabla_\theta \log \pi_\theta(v|x) \;=\; \frac{1}{\pi_\theta(v|x)} \sum_e \big[ \pi_e(v|x) \nabla_\theta g_e(x) + g_e(x) \nabla_\theta \pi_e(v|x) \big]
$$

每个 expert $e$ 收到的有效信号正比于：

- $g_e(x)$（路由权重）
- $\pi_e(v|x) / \pi_\theta(v|x)$（贡献占比）

### 6.3 Expert 访问下界

> **修正自 v0.1**：v0.1 说"负载均衡保证所有 expert 都被更新"。但这只是 expectation 下的统计性质，**不**保证每个 expert 有正下界的访问概率。

为后续 §7 收敛性中"所有 expert 同等接收梯度信号"的论证，**显式假设**：

**(P-expert) Expert 访问下界**：

$$
\inf_{e, x} \mathbb{P}\!\big[e \in \mathcal{E}_k(x_t)\big] \;\geq\; p_e \;>\; 0
$$

**工程实现**：

1. **Noisy router**：训练时给 router logits 加 Gumbel noise，确保所有 expert 都有非零概率被选中；
2. **Routing floor**：保留至少 $k_{\mathrm{floor}}$ 个 expert 用统一概率分配；
3. **Cycle scheduling**：每 $T_{\mathrm{cycle}}$ 步强制选中一个低利用 expert。

或者直接使用 **dense soft mixture**（$k = N$，无 top-$k$）作为理论对象。在 dense 设定下 (P-expert) 自动满足，因为所有 expert 都参与每一步。

---

## §7 收敛性分析

本节按 H-LDP §10 的 6 步实例化检查表逐条验证 MoEH，最终套用 H-LDP 主定理（定理 1）。

### 7.1 (R1)–(R6) 验证

已在 §2.4 完成。✓

### 7.2 (I) 可辨识性验证

由 (P-hint)（§3.6）：LLM 对 hint 敏感 ⇒ $\pi_\theta^{\mathrm{eff}}$ 对 $\pi_\theta$ 敏感。具体地：

**引理 3（可辨识性下推）.** 在 (P-hint) 下，对任意 $\theta_1, \theta_2$：

$$
\|\pi_{\theta_1}^{\mathrm{eff}} - \pi_{\theta_2}^{\mathrm{eff}}\|_{\mathrm{TV}} \;\geq\; \kappa \cdot \|\pi_{\theta_1} - \pi_{\theta_2}\|_{\mathrm{TV}}
$$

证明：由有效策略定义 $\pi_\theta^{\mathrm{eff}}(a' | x) = \sum_v \pi_\theta(v|x) \pi_{\mathrm{LLM}}(a' | x, \mathrm{render}(v))$。对差异 $\Delta_\pi = \pi_{\theta_1} - \pi_{\theta_2}$，

$$
\pi_{\theta_1}^{\mathrm{eff}} - \pi_{\theta_2}^{\mathrm{eff}} = \sum_v \Delta_\pi(v|x) \pi_{\mathrm{LLM}}(\cdot | x, \mathrm{render}(v))
$$

取 TV norm，应用 (P-hint) 的两个 hint 之间最小差距下界 $\kappa$，得证。详见附录 B.4。$\square$

(I) 验证 ✓（前提为 (P-hint)）。

### 7.3 (O2) 二阶矩有界

**引理 4（GRPO 梯度二阶矩界）.** 在 (P-floor) 与 $\epsilon_\sigma > 0$ 下：

$$
\|\nabla_\theta L^{(i)}_t\|^2 \;\leq\; \underbrace{\big(\tfrac{1}{p_{\min}}\big)^2}_{\text{ratio bound by (P-floor)}} \cdot \underbrace{\big(\tfrac{2 R_{\max}}{(1-\gamma) \epsilon_\sigma}\big)^2}_{\text{advantage bound}} \cdot \underbrace{L_\pi^2}_{\text{score bound}}
$$

其中 $L_\pi = \sup_{v, x, \theta} \|\nabla_\theta \log \pi_\theta(v|x)\|$ 由 (P-floor) + 神经网络 Lipschitz 性给出有界。

加上 $\nabla \mathcal{L}_{\mathrm{KL}}, \nabla \mathcal{L}_{\mathrm{aux}}, \nabla \mathcal{L}_{\mathrm{ent}}$ 的有界性（由参数有界 (R6) + 神经网络 smoothness），$\hat g_t$ 的二阶矩有界：

$$
G^2 \;:=\; \mathbb{E}[\|\hat g_t\|^2] \;<\; \infty
$$

(O2) 验证 ✓。

> **修正自 v0.1**：v0.1 使用 $\sigma_{\min} > 0$ 假设。本版使用 $\epsilon_\sigma > 0$（构造性下界），得到的 $G$ 较大（依赖 $1/\epsilon_\sigma$）但**始终有效**，不会因低 reward variance 而失效。

### 7.4 (O1) + (O3) 偏差分解与控制

设 $\hat g_t$ 是 §5.7 完整 loss 的随机梯度。其期望与 $\nabla \tilde J(\theta_t)$ 的偏差可分解为：

$$
\Big\| \mathbb{E}[\hat g_t | \mathcal{F}_t] - \nabla \tilde J(\theta_t) \Big\| \;\leq\; \underbrace{\delta_t^{\mathrm{PPO}}}_{\text{off-policy}} \;+\; \underbrace{\delta_t^{\mathrm{GRPO}}}_{\text{normalization}} \;+\; \underbrace{\delta_t^{\mathrm{topk}}}_{\text{routing approx}}
$$

**逐项分析**：

**(a) Off-policy 偏差 $\delta_t^{\mathrm{PPO}}$**：

由 PPO clipping + (KL-TR) 控制 $\|\theta_t - \theta_{\mathrm{old}(t)}\|$。设每 $T_{\mathrm{sync}}$ 步同步 $\theta_{\mathrm{old}}$：

$$
\|\theta_t - \theta_{\mathrm{old}(t)}\| \;\leq\; T_{\mathrm{sync}} \cdot G \cdot \max_s \eta_s
$$

故

$$
\delta_t^{\mathrm{PPO}} \;\leq\; C_{\mathrm{PPO}} \cdot \|\theta_t - \theta_{\mathrm{old}(t)}\| \;\leq\; C_1 \eta_t
$$

求和：

$$
\sum_t \eta_t \delta_t^{\mathrm{PPO}} \;\leq\; C_1 \sum_t \eta_t^2 \;<\; \infty
$$

✓（在 (LR) 下 summable）。

**(b) Normalization 偏差 $\delta_t^{\mathrm{GRPO}}$**：

由引理 2（LOO-GRPO）：$\delta_t^{\mathrm{GRPO}} \leq C_\sigma \cdot \mathrm{Var}(\hat \sigma_t)^{1/2}$。在 (P-floor) + (P-expert) 下，可以证明 $\mathrm{Var}(\hat \sigma_t) = O(1/K)$（rollout 数）。故

$$
\delta_t^{\mathrm{GRPO}} \;\leq\; C_2 / \sqrt{K}
$$

若 $K$ 固定，则 $\delta_t^{\mathrm{GRPO}}$ 不为零但**一致有界**：

$$
\sum_t \eta_t \delta_t^{\mathrm{GRPO}} \;\leq\; \frac{C_2}{\sqrt{K}} \sum_t \eta_t \;=\; \infty
$$

**这一项不 summable**——这是 fixed-$K$ GRPO 的本质限制。

**处理方式**：要么 (a) 使用 $K$ 随训练递增的 schedule（$K_t \to \infty$），(b) 接受收敛到 $\tilde J$ 的 $O(1/\sqrt{K})$ 邻域。我们采用 (b)，故主定理结论为"有偏邻域收敛"。

**(c) Top-$k$ 路由偏差 $\delta_t^{\mathrm{topk}}$**：

由 §3.3 分析，dense $\pi^{\mathrm{dense}}$ 与 top-$k$ $\pi^{\mathrm{topk}}$ 的差距：

$$
\|\pi^{\mathrm{topk}} - \pi^{\mathrm{dense}}\|_{\mathrm{TV}} \;\leq\; 2 \sum_{e \notin \mathcal{E}_k(x)} g_e(x)
$$

若 router 输出集中（top-$k$ 占据多数 mass），此项小。在实践中可通过 router temperature $T_r$ 调节。设此项一致 $\leq \delta^{\mathrm{topk}}_{\max}$，类似 (b) 不 summable，但有界。

### 7.5 主定理实例化（收敛到 $\tilde J$ 邻域）

**定理 3（MoEH 主收敛性）.** 假设：

1. **环境与状态**：(R1)–(R6)（§2.4 验证）；
2. **可辨识性**：(P-hint) 成立（§3.6）；
3. **策略下界**：(P-floor) 由 entropy bonus + softmax floor 保证（§5.4）；
4. **Expert 访问下界**：(P-expert) 由 noisy router 保证（§6.3），或使用 dense soft mixture；
5. **学习率**：$\eta_t$ 满足 (LR)（§8.2 of general doc）；
6. **正则化系数**：$\alpha, \beta, \lambda_H$ 固定 $> 0$。

则 MoEH-GRPO 算法生成的 $\{\theta_t\}$ 满足

$$
\boxed{\;\liminf_{t \to \infty} \big\| \nabla_\theta \tilde J(\theta_t) \big\| \;\leq\; \delta_\infty \quad \text{a.s.}\;}
$$

其中

$$
\delta_\infty \;\leq\; \frac{C_2}{\sqrt{K}} \;+\; \delta^{\mathrm{topk}}_{\max}
$$

是由 fixed-$K$ GRPO 与 top-$k$ 近似引入的不可消除偏差。

**证明**：应用 H-LDP 主定理（定理 1）的"$\delta_t \to \delta_\infty$ 收敛到邻域"扩展。详见附录 B.5。$\square$

### 7.6 收敛到 $\tilde J$ 与到 $J$ 的差距

由 H-LDP §9.4 的方式 A，主定理结论是关于 $\tilde J$ 的 stationary point。换算到 $J$：

**推论 1（$J$ 的 stationary point 邻域）.** 在定理 3 条件下，$\liminf$ 处 $\theta_\infty$ 满足

$$
\|\nabla J(\theta_\infty)\| \;\leq\; \alpha \|\nabla \mathcal{L}_{\mathrm{aux}}(\theta_\infty)\| + \beta \|\nabla \mathcal{L}_{\mathrm{KL}}(\theta_\infty; \theta_\infty)\| + \lambda_H \|\nabla \mathcal{L}_{\mathrm{ent}}(\theta_\infty)\| + \delta_\infty
$$

由 $\mathcal{L}_{\mathrm{KL}}(\theta; \theta) = 0$ 且 $\nabla_\theta D_{\mathrm{KL}}(\pi_{\theta'} \| \pi_\theta)\big|_{\theta = \theta'} = 0$（KL 在对角线上一阶为零），故第二项为零。

$$
\|\nabla J(\theta_\infty)\| \;\leq\; \alpha \cdot C_{\mathrm{aux}} + \lambda_H \cdot C_{\mathrm{ent}} + \delta_\infty
$$

实践推荐：$\alpha = 0.01$, $\lambda_H = 0.01$，导致 $J$ 的偏离 $\approx 0.01 \cdot (C_{\mathrm{aux}} + C_{\mathrm{ent}}) + 0.05 / \sqrt{K}$。当 $K = 4$，总误差预计在 $0.03$ 量级。

### 7.7 工程层面的注意

| 项 | 处理 |
|---|---|
| **优化器** | 主定理基于 SGD；使用 Adam 时需补 (Adam-1)、(Adam-2)（见 H-LDP §9.5） |
| **Top-$k$ 不可微** | 跳变集测度零，SGD 几乎必然不命中；理论以 dense 为对象 |
| **$\theta_{\mathrm{old}}$ 同步** | 每 $T_{\mathrm{sync}}$ 步同步一次；$T_{\mathrm{sync}} \cdot \max_s \eta_s$ 决定 PPO bias 系数 $C_1$ |
| **Reward variance 退化** | 由 $\epsilon_\sigma$ 构造性保护；早期低 variance 阶段 advantage 大但有界 |
| **梯度爆炸/消失** | 通过 gradient clip + Adam preconditioning 处理；不进入理论 |

---

## §8 实验设计

> **目的**：经验验证理论假设（(P-hint)、(P-floor)、(P-expert)）与主定理结论，并对比 MoEH 与若干 baseline。
>
> **注**：本节是**实验协议**而非已运行实验。所有"预期结果"基于理论推断。

### 8.1 任务与 evaluator

**任务来源**：基于 `examples/research-assistant/` 的扩展。

**难度分层**：

| 难度 | 描述 | 典型 LLM 调用数 | 任务数 |
|------|------|----------------|--------|
| Easy | 单轮回答（如 weather） | 1–2 | 30 |
| Medium | 简单 pipeline（如 arxiv search + summarize） | 3–5 | 30 |
| Hard | 复杂 graph（如 multi-paper review） | 6–10 | 20 |

**Evaluator**：以 GPT-4 作为 grader，固定 rubric，输出 $\in \{0, 0.5, 1\}$（fail / partial / pass）。固定 seed + cache 保证 (R4)。

**Reward**：$R = $ grader score（terminal-only，shaping 0）。

### 8.2 Baselines

| Baseline | 说明 |
|----------|------|
| **B1: Fixed Harness** | 手写 captain prompt + 固定 policy（无学习） |
| **B2: Single-Head (N=1)** | 退化 MoEH，无 mixture |
| **B3: Dense MoEH (k=N)** | Soft mixture，所有 expert 都激活（无 top-$k$） |
| **B4: MoEH Top-$k$（本工作）** | $N=8$, $k=2$ |
| **B5: MoEH + LOO-GRPO** | 主推：使用 §5.3 leave-one-out advantage |
| **B6: MoEH w/o Hint** | Hint 不渲染到 prompt（消融可辨识性） |

### 8.3 主要 metric

**收敛性**：

- $J_t$：episode-level return 训练曲线
- $\|\hat g_t\|_2$：梯度范数训练曲线
- $D_{\mathrm{KL}}(\pi_{\theta_{\mathrm{old}}} \| \pi_\theta)$：策略漂移
- $\sigma_t$ 分布：reward variance（验证 $\epsilon_\sigma$ 是否生效）
- Clip fraction：PPO 剪切样本占比（健康范围 5–15%）

**容量-成本**：

- Single-step latency（CPU/GPU）vs $N$
- Final $J$ vs $N$
- Effective expert utilization $\mathrm{Entropy}(f)$

**Expert 涌现**：

- 训练后 $f_e$ 分布：是否均衡？
- $g_e(x)$ 在不同状态类（free/consumption mode、long/short context、early/late step）的条件分布
- t-SNE 可视化每个 expert 被激活的状态聚类

**可辨识性**：

- 训练前 $\hat \kappa$（probing）
- 训练中 $\|\pi_{\theta_t}^{\mathrm{eff}} - \pi_{\theta_{t-1}}^{\mathrm{eff}}\|$ vs $\|\pi_{\theta_t} - \pi_{\theta_{t-1}}\|$ 比值的滚动均值

### 8.4 关键 ablation

| Ablation | 操作 | 验证 |
|----------|------|------|
| **A1: hint via prompt → direct action** | Advisor 直接采样 verb 并执行（跳过 LLM 协同） | (P-hint) 必要性 |
| **A2: w/o entropy bonus** | $\lambda_H = 0$ | (P-floor) 必要性 |
| **A3: w/o load balancing** | $\alpha = 0$ | Expert collapse 风险 |
| **A4: w/o noisy router** | 关闭 routing noise | (P-expert) 必要性 |
| **A5: K=1 vs K=4 vs K=16** | 群体大小 | GRPO bias-variance trade-off |
| **A6: vanilla mean vs LOO** | $\mu$ vs $\mu^{-i}$ | LOO 修正的实际效果 |
| **A7: PPO ε ∈ {0.1, 0.2, 0.3}** | 剪切阈值 | Off-policy bias 控制 |

### 8.5 预期结果与失败模式

**预期成功**：

- B4/B5 在 Medium/Hard 上显著优于 B1/B2（capacity helps）
- B5 > B4：LOO 修正减少 bias，训练曲线更平稳
- A1 失败：advisor 学不到东西（验证 hint via prompt 的必要性）
- A3 失败：少数 expert dominate，最终 $J$ 与 B2 相当

**预期失败模式**（理论上的潜在风险）：

| 失败 | 触发条件 | 缓解 |
|------|---------|------|
| (P-hint) 不成立 | 模板写得太模糊 | Probing + 重写 prompt |
| Expert collapse | $\alpha$ 过小或 router temperature 过低 | 增大 $\alpha$、加 noisy routing |
| Reward variance 长期为 0 | Evaluator 输出过早饱和 | 改用 dense rubric / 增加 partial credit |
| Off-policy bias 累积 | $T_{\mathrm{sync}}$ 过长 | 减小 $T_{\mathrm{sync}}$ |

### 8.6 计算预算

预估单次完整实验（30 + 30 + 20 任务，4 baselines × 7 ablations × 3 seeds = 84 runs，每 run ≈ 2 GPU-hour）：约 **170 GPU-hour**。

可分阶段执行：先 B1–B5 主对比（~30 GPU-hour），结果显著后再做 ablations。

---

## §9 实现要点

### 9.1 参数规模

| 模块 | 参数量 |
|------|--------|
| Per-modality projection ($7 \times 1024 \to 128$) | ~0.92 M |
| Per-modality attention pool (7 个) | ~0.05 M |
| Cross-modality self-attention (2 层 × 2 头) | ~0.13 M |
| Router ($W_r \in \mathbb{R}^{8 \times 896}$) | ~7 K |
| Expert 头 ($N=8$) | ~0.93 M |
| Value head | ~0.12 M |
| **合计** | **~2.3 M** |

CPU 推断 $< 2$ ms/step。

### 9.2 训练超参（参考）

| 超参 | 默认 | 说明 |
|------|------|------|
| 群体大小 $K$ | 4 | 影响 GRPO bias ~ $1/\sqrt{K}$ |
| PPO 剪切 $\epsilon$ | 0.2 | 影响 surrogate（**不**影响 $\rho$） |
| KL 系数 $\beta$ | 0.01 | 控制 advisor 更新步长 |
| 负载均衡 $\alpha$ | 0.01 | 影响 $J - \tilde J$ 差距 |
| Entropy bonus $\lambda_H$ | 0.01 | 保证 (P-floor) |
| Softmax floor $\alpha_{\mathrm{floor}}$ | 0.01 | $p_{\min} \approx 8 \times 10^{-4}$ |
| GAE $\lambda$ | 0.95 | step-level credit |
| 折扣 $\gamma$ | 0.99 | |
| Router 温度 $T_r$ | 1.0 | 推断时 → 0 |
| Routing noise $\sigma_{\mathrm{noise}}$ | 0.1 | 训练时启用（(P-expert)） |
| 学习率（cosine） | $3 \times 10^{-4} \to 1 \times 10^{-5}$ | 注：理论要求 R-M；cosine 是工程妥协 |
| Batch | 256 (= 64 任务 × 4 group) | |
| $\theta_{\mathrm{old}}$ 同步周期 $T_{\mathrm{sync}}$ | 4 epoch | 影响 PPO bias |

### 9.3 LOO-GRPO 损失伪代码

```python
def moeh_loss(theta, batch, theta_old, K, eps_clip, beta, alpha, lam_H):
    """
    batch.rewards:  [B, K, T]
    batch.actions:  [B, K, T]
    batch.states:   [B, K, T, ...]
    """

    # 1. Step-level GAE per rollout
    values   = value_net(batch.states)                      # [B, K, T]
    deltas   = batch.rewards + gamma * values[..., 1:] - values[..., :-1]
    A_gae    = compute_gae(deltas, gamma * lam)             # [B, K, T]

    # 2. LOO group-relative advantage
    sum_A    = A_gae.sum(dim=1, keepdim=True)               # over K
    mu_loo   = (sum_A - A_gae) / (K - 1)                    # [B, K, T]
    sigma    = A_gae.std(dim=1, keepdim=True) + eps_sigma
    A        = (A_gae - mu_loo) / sigma                     # [B, K, T]

    # 3. PPO clipped surrogate (clips surrogate, not rho itself)
    logp_new = log_pi(theta, batch.states, batch.actions)
    logp_old = log_pi(theta_old, batch.states, batch.actions)
    rho      = (logp_new - logp_old).exp()
    L_unc    = rho * A
    L_clip   = rho.clamp(1 - eps_clip, 1 + eps_clip) * A
    L_grpo   = torch.minimum(L_unc, L_clip).mean()

    # 4. KL regularizer
    L_kl     = kl_div(pi_old, pi_new).mean()

    # 5. Switch-style load balancing
    f, P     = gate_statistics(batch.states, theta)
    L_aux    = N * (f * P).sum()

    # 6. Entropy bonus (for P-floor)
    L_ent    = entropy(pi_new).mean()

    # 7. Total: minimize negative reward + regularizers
    return -L_grpo + beta * L_kl + alpha * L_aux - lam_H * L_ent
```

### 9.4 训练监控仪表盘

按重要性排序的监控指标（建议接入 W&B 或 TensorBoard）：

| 监控 | 公式 | 告警阈值 |
|------|------|---------|
| `J_avg` | episode return 滚动均值 | 下降趋势告警 |
| `grad_norm` | $\|\hat g_t\|_2$ | $> 100$ 告警（gradient explosion） |
| `kl_step` | $D_{\mathrm{KL}}(\pi_{\theta_{\mathrm{old}}} \| \pi_\theta)$ | $> 0.05$ 告警 |
| `clip_frac` | 被剪切样本占比 | $> 30\%$ 告警 |
| `sigma_t_p10` | $\sigma_t$ 第 10 分位 | $< 10^{-3}$ 告警（reward variance 退化） |
| `expert_usage_entropy` | $H(f)$ | $< 0.5 \log N$ 告警（expert collapse） |
| `effective_p_min` | $\inf \pi_\theta(v\|x)$ | $< 10^{-4}$ 告警（(P-floor) 失效） |
| `kappa_hat` | probing $\hat \kappa$ | $< 0.05$ 告警（(P-hint) 失效） |
| `ratio_p99` | $\rho^{(i)}_t$ 第 99 分位 | $> 100$ 告警 |

---

## §10 与相关工作对比

| 维度 | CB-MDP (Memonto) | H-LDP / MoEH（本工作） |
|------|-----------------|---------------------|
| 抽象层 | 内嵌 memory 操作 | 完全抽象 $(c, e)$ 选择 |
| Augmented state | $(s, M)$ | $\mathcal{X}$（H-LDP）/ 6 模态（MoEH） |
| Action | $k$-subset of $M$ | $(c, e)$（H-LDP）/ verb hint（MoEH） |
| LLM 角色 | 消费 retrieved memory | 一般冻结算子 $\pi_{\mathrm{LLM}}$ |
| 转移层数 | 双层 | 一般框架：单层（$\pi_\theta + \pi_{\mathrm{LLM}}$ 复合）；MoEH 实例：三层（advisor + LLM + env） |
| Policy 形态 | 单 retrieval policy | 任意（MoEH 用 top-$k$ MoE） |
| Credit | TD(λ) + 计数衰减 RM | GAE-λ + LOO-GRPO + mixture grad split |
| 收敛工具 | RM 引理 | Robbins-Siegmund + biased oracle 框架 |
| Identifiability | 隐式（memory 由 agent 决定） | 显式 (I) 假设 |
| 正则化-目标关系 | 直接对 $V^\pi$ | 明确分离 $J$ 与 $\tilde J$ |

**核心扩展**：

1. H-LDP 把 Memonto 的"memory-MDP"抽象提升到**任意 $(c, e)$ 控制**层面；
2. 把"双层转移"统一为"复合 $\pi_\theta + \pi_{\mathrm{LLM}}$"，避免对 LLM 角色硬编码；
3. 引入显式 **(I) 可辨识性**条件，处理"信号经过 LLM"这一关键路径；
4. 把所有具体算法的偏差-方差控制工作打包到**抽象 oracle**假设 (O1)–(O3)，使收敛性证明算法无关；
5. **正确分离** $J$（真目标）与 $\tilde J$（正则化目标），避免 v0.1 的"固定 $\alpha$ 仍收敛到 $J$"错误。

---

## 附录 A：符号速查表

| 符号 | 含义 |
|------|------|
| $x$ | RCM 状态 |
| $\mathcal{V}$ | Advisor verb 集合（$|\mathcal{V}|=12$） |
| $v$ | Advisor hint verb |
| $a'$ | Realized action (verb + content) |
| $\pi_\theta$ | Advisor 策略 |
| $\pi_{\mathrm{LLM}}$ | 冻结 LLM 算子 |
| $\pi_\theta^{\mathrm{eff}}$ | 有效策略 |
| $\phi(x)$ | 多模态融合 $\in \mathbb{R}^{896}$ |
| $g_e(x), \tilde g_e(x)$ | Router gate / 重归一化 |
| $\pi_e(v\|x)$ | Expert $e$ 输出 |
| $\hat A^{(i)}_t, \hat A^{(i),\mathrm{LOO}}_t$ | GRPO / LOO advantage |
| $\rho^{(i)}_t$ | 重要性比 |
| $J(\theta), \tilde J(\theta)$ | 真实目标 / 正则化目标 |
| $\delta_t$ | Oracle 偏差 |
| $\kappa$ | 可辨识性常数 |
| $p_{\min}$ | 策略下界 |
| $p_e$ | Expert 访问下界 |

---

## 附录 B：关键引理证明

### B.1 引理 1：朴素 GRPO 偏差分解

**设置**：$\mu_t = \frac{1}{K} \sum_j R^{(j)}$（含 $R^{(i)}$ 自身），$\sigma_t$ 同。$b^* = V^{\pi_{\theta_{\mathrm{old}}}}$。

**(i) Centering bias**：

$$
\mathbb{E}[\mu_t | x_t] - b^*(x_t) = \mathbb{E}[R^{(i)}|x_t] - b^*(x_t) = 0
$$

但 $\mu_t$ 与 $R^{(i)}$ 不独立（$\mu_t \ni R^{(i)} / K$）。条件期望

$$
\mathbb{E}[R^{(i)} - \mu_t | v_t^{(i)}, x_t] = \mathbb{E}[R^{(i)}|v_t^{(i)}, x_t] \cdot (1 - 1/K) - \frac{K-1}{K} \mathbb{E}[R^{(j)}|x_t]
$$

设 $Q(x, v) = \mathbb{E}[R | x, v]$（true Q），$V = \mathbb{E}_v[Q]$，则

$$
\mathbb{E}[R^{(i)} - \mu_t | v_t^{(i)}, x_t] = \frac{K-1}{K} (Q(x_t, v_t^{(i)}) - V(x_t)) = \frac{K-1}{K} A^*(x_t, v_t^{(i)})
$$

故梯度估计

$$
\mathbb{E}[\hat A^{(i)} \nabla \log \pi | x_t] = \frac{K-1}{K} \cdot \frac{1}{\sigma_t + \epsilon_\sigma} \mathbb{E}_v[A^*(x, v) \nabla \log \pi(v|x)]
$$

与真实梯度比较，centering bias 系数为 $(K-1)/K$，相对偏差 $O(1/K)$。

**(ii) Scaling bias**：$1/(\sigma_t + \epsilon_\sigma)$ 是随机变量，对动作 $v$ 弱相关。可写

$$
\frac{1}{\sigma_t + \epsilon_\sigma} = \mathbb{E}[\sigma_t^{-1}] + \zeta_t, \quad \mathbb{E}[\zeta_t] \to 0, \mathrm{Var}(\zeta_t) = O(1/K)
$$

故 scaling 引入额外 $O(1/\sqrt{K})$ 偏差。

合并：$\delta^{\mathrm{naive}} = O(1/K) + O(1/\sqrt{K}) + O(\|b^*\|_\infty)$，被 $O(1/\sqrt{K})$ 主导。$\square$

### B.2 引理 2：LOO-GRPO 偏差

LOO baseline $\mu^{-i}_t = \frac{1}{K-1} \sum_{j \neq i} R^{(j)}$ 与 $R^{(i)}$（条件于 $\theta_{\mathrm{old}}, x_t$）独立，故

$$
\mathbb{E}[R^{(i)} - \mu^{-i}_t | v^{(i)}, x_t] = Q(x_t, v^{(i)}) - V(x_t) = A^*(x_t, v^{(i)})
$$

即 LOO centering 是**精确无偏的** baseline。

剩余偏差只来自 $\hat \sigma_t$ scaling，由 $\hat \sigma_t$ 的方差 $O(1/K)$ 控制：

$$
\delta^{\mathrm{LOO}} \leq C_\sigma \cdot \mathrm{Var}(\hat \sigma_t)^{1/2} = O(1/\sqrt{K})
$$

虽然 $1/\sqrt{K}$ 仍不 summable，但**系数 $C_\sigma$ 远小于朴素版**（少了 centering 项）。$\square$

### B.3 Top-$k$ 不可微集的测度

**引理 5**：$\{\theta : \mathcal{E}_k(x)$ 在 $\theta$ 处跳变$\}$ 是 $\Theta$ 中的有限超平面并，故 Lebesgue 测度为零。

**证明**：跳变发生当且仅当 router logits 中 top-$k$ 与第 $k+1$ 个 logit 相等：

$$
\exists\, e_1 \in \mathcal{E}_k, e_2 \notin \mathcal{E}_k : \;\; (W_r)_{e_1} \phi(x) = (W_r)_{e_2} \phi(x)
$$

每一对 $(e_1, e_2)$ 对应一个超平面 $\{\theta : (W_r)_{e_1 - e_2} \phi(x; \theta) = 0\}$，是 $\Theta$ 的 codimension-1 子流形，Lebesgue 测度零。$\binom{N}{2}$ 对的有限并仍测度零。$\square$

**推论**：SGD 轨迹 $\{\theta_t\}$ 几乎必然不命中此集合，故梯度几乎处处定义。

### B.4 引理 3：可辨识性下推

由 $\pi_\theta^{\mathrm{eff}}(a'|x) = \sum_v \pi_\theta(v|x) \pi_{\mathrm{LLM}}(a'|x, \mathrm{render}(v))$：

$$
\|\pi_{\theta_1}^{\mathrm{eff}} - \pi_{\theta_2}^{\mathrm{eff}}\|_{\mathrm{TV}} = \tfrac12 \int \big|\sum_v (\pi_{\theta_1}(v) - \pi_{\theta_2}(v)) \pi_{\mathrm{LLM}}(a'|x, v)\big| da'
$$

通过反向 Pinsker / minimum-distance 论证（设 $\Delta = \pi_{\theta_1} - \pi_{\theta_2}$），可下界为：

$$
\geq \kappa \cdot \tfrac12 \sum_v |\Delta(v)| = \kappa \|\pi_{\theta_1} - \pi_{\theta_2}\|_{\mathrm{TV}}
$$

详细推导依赖 $\pi_{\mathrm{LLM}}$ 的条件分布几何，省略。$\square$

### B.5 定理 3：邻域收敛证明

参考 H-LDP 定理 1 证明（附录 B.2）。当 $\delta_t$ 不 summable 但有界 $\delta_t \leq \delta_\infty$：

由 Robbins-Siegmund 引理修改版，可得

$$
\liminf_t \|\nabla \tilde J(\theta_t)\| \leq 2 \delta_\infty \quad \text{a.s.}
$$

证明类似 H-LDP 主定理；省略。$\square$

---

## 附录 C：相对 v0.1 的修正映射表

| # | v0.1 位置 | 问题 | 本版位置 / 修正 |
|---|----------|------|----------------|
| 1 | §6.2 引理 1 | "$\frac{1}{K}\sum \hat A = 0$"被误用作策略梯度无偏性 | §5.2 + 引理 1（附录 B.1）：正确表述为有偏，§5.3 引入 LOO 修正 |
| 2 | §8.2 引理 3 | "Surrogate gradient ≈ true PG"未证明 | §7.4：分解为 PPO/GRPO/topk 三项，逐项给出偏差表达式 |
| 3 | §8.1 (A6) | "$|\rho| \leq 1+\epsilon$ 由 PPO clipping" | §5.4：明确 clipping 限制 surrogate 而非 ratio；引入 (P-floor) 给出 ratio bound |
| 4 | §8.1 (A7) | "$\sigma_t \geq \sigma_{\min}$ a.s." 过强 | §7.3：改用 $\epsilon_\sigma$ 构造性下界 |
| 5 | §8.1 (A4) | Top-$k$ 与 $C^1$ 假设冲突 | §3.3 + 附录 B.3：理论以 dense soft mixture 为对象；top-$k$ 跳变集测度零 |
| 6 | §8.5 | 可辨识性仅作讨论 | §3.6 (P-hint) + §7.2 引理 3：列为正式假设并下推到 (I) |
| 7 | §8.1 (A3) | "冻结 LLM ⇒ MDP 平稳"不够 | §2.4 (R4)：要求**所有**随机源时间齐次 |
| 8 | §8.1 (A1) | "状态空间紧致"无 topology | H-LDP §3.2 (R1)–(R6)：替换为 horizon/reward/token 有界 |
| 9 | §8.4 主定理 | RM 与 $O(T^{-1/2})$ 速率混用 | H-LDP §9.1+§9.3：明确分离两个独立定理 |
| 10 | 附录 B.2 | Borel-Cantelli 不成立 | H-LDP 附录 B.2：使用 Robbins-Siegmund 正确证明 |
| 11 | §6.6 / §8.4 | "收敛到 $J$"虽 fixed $\alpha > 0$ | §5.6 + §7.5 + 推论 1：正确表述为收敛到 $\tilde J$，并给 $J$ 偏离上界 |
| 12 | §8.4 | "Adam 不影响主要结论" | §7.7 + H-LDP §9.5：明确 SGD 与 Adam 分离，Adam 需补 (Adam-1)/(Adam-2) |
| 13 | §4 定理 1 | "Capacity-Prompt 解耦定理" | §4 命题 1：降级为 Proposition，补三条 caveat |
| 14 | §6.5 | "load balancing 保证所有 expert 被更新" | §6.3 (P-expert)：显式假设访问下界，通过 noisy router 或 dense 实现 |
| 15 | §3.2 vs §5.5 | 有限动作 vs free-form content 张力 | §2.2：明确分离 $\mathcal{A}_{\mathrm{adv}}$（有限）与 $\mathcal{A}_{\mathrm{real}}$（一般），理论用前者 |

---

## 文档版本

| 版本 | 日期 | 变更 |
|------|------|------|
| v0.1 | 2026-05-31 | 初稿 |
| v0.2 | 2026-05-31 | **重大修订**：fit 进 H-LDP 一般框架；修复附录 C 列出的 15 个数学问题；新增 §8 实验设计；新增 LOO-GRPO 修正 |

