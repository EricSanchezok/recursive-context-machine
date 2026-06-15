# Learnable Harness：一般数学框架

> 本文从最一般的"Harness 控制 LLM 调用"范式出发，建立一个**不依赖任何具体系统**（RCM、Memonto、DSPy、ReAct 等）的数学框架，用于刻画**可学习 Harness**的决策过程、目标函数与收敛性条件。
>
> 具体方案（如 MoEH）在配套文档 [`evolve-harness-math.md`](./evolve-harness-math.md) 中作为本框架的**实例化**给出，并逐条验证本文提出的假设条件。

---

## 目录

- [§1 引言：Harness 是什么](#1-引言harness-是什么)
- [§2 LLM 作为冻结的随机算子](#2-llm-作为冻结的随机算子)
- [§3 H-LDP：Harnessed LLM Decision Process](#3-h-ldpharnessed-llm-decision-process)
- [§4 有效策略与可辨识性](#4-有效策略与可辨识性)
- [§5 优化目标](#5-优化目标)
- [§6 策略梯度的一般形式](#6-策略梯度的一般形式)
- [§7 信用分配的一般视角](#7-信用分配的一般视角)
- [§8 学习算法的抽象描述](#8-学习算法的抽象描述)
- [§9 收敛性](#9-收敛性)
- [§10 实例化路径](#10-实例化路径)
- [附录 A：符号速查表](#附录-a符号速查表)
- [附录 B：Robbins-Siegmund 引理与证明细节](#附录-b)

---

## §1 引言：Harness 是什么

考虑一个由 LLM 驱动的决策系统：它通过若干次 LLM 调用逐步完成任务。每次 LLM 调用都需要事先准备好两类输入：

- **执行上下文** $c$（execution context）：直接进入 LLM 的提示，包括对话历史、检索到的记忆、当前指令、参考资料等。
- **执行环境** $e$（execution environment）：LLM 在本次调用中可调用的工具集、可访问的资源、当前活跃模型、采样温度等运行时配置。

LLM 在 $(c, e)$ 上抽样输出 $y$；$y$ 经环境（工具执行、文件改写、外部 evaluator）处理后改变系统状态 $x$。然后系统进入下一个调用周期，重新决定 $(c, e)$。

**Harness 就是负责"在每个调用周期之间决定 $(c, e)$"的控制器**。本质上：

> **Harness ≜ 一次 LLM 调用周期内"约束其执行环境与执行上下文"，并据"当前环境 + LLM 与环境的交互结果"优化下一次调用的执行环境与执行上下文，使得连续调用后能成功完成任务的工程过程。**

**Learnable Harness** 把这个控制过程**参数化**为 $\pi_\theta$，并通过实际交互产生的轨迹数据优化 $\theta$。本文的目的是把"learnable harness"严格表述为一个马尔可夫决策问题，并给出其收敛性所需的**最弱**假设集合。

### 1.1 已有范式的统一视角

不同已有工作可视为本框架的不同特化：

| 范式 | $(c, e)$ 如何被决定 | $\pi$ 是否可学 |
|------|-------------------|--------------|
| Fixed harness (ReAct, naive ReAct) | 模板 + 规则 | 否（$\pi$ 退化为常数） |
| Prompt search (DSPy, APE) | 在离散 prompt 候选集上 search | 仅 prompt 离散选择 |
| Memory-augmented (Memonto, CB-MDP) | $c$ 由 memory retrieval 决定 | retrieval policy 可学 |
| Action-decomposed (本系列, MoEH) | $(c, e)$ 由 advisor hint 引导 LLM 协同生成 | advisor 可学 |

本框架对这些都中立——我们只刻画"决定 $(c, e)$ 的可学习控制器"这件事，不规定其内部结构。

---

## §2 LLM 作为冻结的随机算子

**关键假设**：在 harness 训练期间，LLM 的权重**冻结**。我们把 LLM 视为一个不可微的、抽样的随机算子：

$$
\pi_{\mathrm{LLM}}\!:\;\mathcal{C} \times \mathcal{E} \to \Delta(\mathcal{Y})
$$

其中 $\mathcal{Y}$ 是 LLM 单次调用的输出空间（可以是 token 序列、结构化命令、tool-call trace 等的并集）。对任意 $(c, e)$，LLM 抽样 $y \sim \pi_{\mathrm{LLM}}(\cdot \mid c, e)$。

冻结假设的三层意义：

1. **平稳性**：$\pi_{\mathrm{LLM}}$ 与 $\theta$ 解耦，避免"LLM 自身漂移"破坏 MDP 的 time-homogeneity。
2. **可微性边界清晰**：harness 训练只对 $\theta$ 求梯度；LLM 内部计算为黑箱。
3. **可重复性**：理论分析中 $\pi_{\mathrm{LLM}}$ 是固定可测核，对给定 $(c, e)$ 给出固定的输出分布。

**注**：实际推理时 sampler 设置（temperature、top-p、stop tokens）也算作 $\pi_{\mathrm{LLM}}$ 的一部分。若训练期间这些参数变化，则 $\pi_{\mathrm{LLM}}$ 不再冻结，本文结论需重新验证。

---

## §3 H-LDP：Harnessed LLM Decision Process

### 3.1 形式化定义

**定义 1（H-LDP）.** 一个 **Harnessed LLM Decision Process** 是元组

$$
\mathcal{M}_{\mathrm{HLDP}} \;=\; \big\langle\, \mathcal{X},\; \mathcal{C},\; \mathcal{E},\; \mathcal{Y},\; \pi_{\mathrm{LLM}},\; T,\; R,\; \rho_0,\; \gamma \,\big\rangle
$$

满足：

- $\mathcal{X}$：**Harness 可观测状态空间**（可测空间）。每个 $x \in \mathcal{X}$ 编码 harness 在某一时刻"看到的所有信息"——历史 LLM 输出、环境反馈、任务进度等。
- $\mathcal{C}, \mathcal{E}, \mathcal{Y}$：执行上下文空间、执行环境空间、LLM 输出空间（均为可测空间）。
- $\pi_{\mathrm{LLM}}: \mathcal{C} \times \mathcal{E} \to \Delta(\mathcal{Y})$：冻结 LLM 算子（见 §2）。
- $T: \mathcal{X} \times \mathcal{Y} \to \Delta(\mathcal{X})$：**环境转移核**。给定上一步 harness 状态 $x$ 与 LLM 输出 $y$，给出下一步状态分布。$T$ 隐含全部非 LLM 随机源（工具执行、文件系统、外部 evaluator、reactor 内部 LLM 等）。
- $R: \mathcal{X} \times \mathcal{Y} \to [-R_{\max}, R_{\max}]$：即时奖励，$R_{\max} < \infty$。
- $\rho_0 \in \Delta(\mathcal{X})$：初始状态分布。
- $\gamma \in [0, 1)$：折扣因子。

**一个 H-LDP 步骤** 的时序：

```
                ┌─────────────┐
   x_t  ───────►│   Harness   │──── (c_t, e_t) ────┐
                │   π_θ       │                     │
                └─────────────┘                     ▼
                                            ┌──────────────┐
                                            │  LLM (冻结)  │
                                            │   π_LLM      │
                                            └──────┬───────┘
                                                   │ y_t
                                                   ▼
                                            ┌──────────────┐
                                            │  Environment │
                                            │       T      │──── x_{t+1}, r_t ──►
                                            └──────────────┘
```

形式上：

$$
(c_t, e_t) \sim \pi_\theta(\cdot \mid x_t),\quad
y_t \sim \pi_{\mathrm{LLM}}(\cdot \mid c_t, e_t),\quad
x_{t+1} \sim T(\cdot \mid x_t, y_t),\quad
r_t = R(x_t, y_t)
$$

### 3.2 正则性假设

我们陈述如下**最弱**的正则性集合，避免诸如"状态空间紧致"等需要在文本空间定义拓扑才严格的强假设：

- **(R1) Episode horizon 有界**：每个 episode 长度 $T \leq T_{\max} < \infty$（由 step cap 或 evaluator 保证）。
- **(R2) 奖励有界**：$\|R\|_\infty \leq R_{\max} < \infty$。
- **(R3) 可测性**：$T, \pi_{\mathrm{LLM}}, R, \pi_\theta$ 均为各自空间上的可测映射。
- **(R4) 时间齐次性**：**所有**随机源（LLM、tool 执行、reactor、evaluator、task sampler、缓存与网络）在训练期间共同诱导一个 time-homogeneous Markov kernel。
- **(R5) Token / 输入有界**：$\mathcal{C}$ 中元素 token 长度被截断到 $L_{\max} < \infty$；$\mathcal{E}$ 中元素结构有界。
- **(R6) 参数集合有界**：$\Theta \subseteq \mathbb{R}^d$ 是有界子集（或算法保证轨迹 $\{\theta_t\}$ 留在某有界集合内）。

> **注（与"状态空间紧致"的区别）**：直接说 $\mathcal{X}$ 紧致需先在 $\mathcal{X}$ 上定义拓扑，但 $\mathcal{X}$ 包含文本、文件系统等离散对象，拓扑不自然。(R1)–(R6) 是更易于工程验证、且数学上够用的替代方案。

> **(R4) 的工程含义**：若工具调用涉及外部网络、wall-clock 时间或可变缓存，必须在训练前固化（例如使用 frozen tool snapshots、固定 seed、离线 evaluator）。否则 $T$ 不再 time-homogeneous，理论结论失效。

---

## §4 有效策略与可辨识性

### 4.1 有效策略

Harness 策略 $\pi_\theta: \mathcal{X} \to \Delta(\mathcal{C} \times \mathcal{E})$ 与冻结 LLM 复合，诱导 $\mathcal{Y}$ 上的**有效策略**：

**定义 2（有效策略）.**

$$
\pi_\theta^{\mathrm{eff}}(y \mid x) \;:=\; \int_{\mathcal{C} \times \mathcal{E}} \pi_\theta(c, e \mid x) \cdot \pi_{\mathrm{LLM}}(y \mid c, e)\;\mathrm{d}(c, e)
$$

**有效转移核**：

$$
P_\theta(x' \mid x) \;=\; \int_{\mathcal{Y}} \pi_\theta^{\mathrm{eff}}(y \mid x) \cdot T(x' \mid x, y)\;\mathrm{d}y
$$

**有效访问分布** $d^{\pi_\theta}(x)$：在 $\rho_0$ 与 $P_\theta$ 下沿无穷长 episode 累积的折扣占有度量。

### 4.2 可辨识性

> **核心观察**：在 H-LDP 中，harness 的梯度信号必须**穿过 LLM** 才能影响 reward。若 LLM 对 $(c, e)$ 的变化不敏感（即 $\pi_{\mathrm{LLM}}(\cdot \mid c, e)$ 对 $(c, e)$ 几乎不变），则 harness 永远学不到东西——即使收敛也是收敛到任意 stationary point。

**定义 3（Harness 信号可辨识性）.** 称 H-LDP 在策略类 $\{\pi_\theta\}_{\theta \in \Theta}$ 上是 **$\kappa$-可辨识的**，若存在常数 $\kappa > 0$，使得对任意 $\theta_1, \theta_2 \in \Theta$ 与几乎所有 $x \in \mathcal{X}$：

$$
\big\| \pi_{\theta_1}^{\mathrm{eff}}(\cdot \mid x) - \pi_{\theta_2}^{\mathrm{eff}}(\cdot \mid x) \big\|_{\mathrm{TV}}
\;\geq\; \kappa \cdot \big\| \pi_{\theta_1}(\cdot \mid x) - \pi_{\theta_2}(\cdot \mid x) \big\|_{\mathrm{TV}}
$$

**(I)** **可辨识性假设**：H-LDP 是 $\kappa$-可辨识的，且 $\kappa > 0$。

**等价条件（局部，可微版本）**：在 $\theta_0$ 邻域内，

$$
\big\| \nabla_\theta \pi_\theta^{\mathrm{eff}}(\cdot \mid x) \big\|_{\theta_0} \;\geq\; \kappa \cdot \big\| \nabla_\theta \pi_\theta(\cdot \mid x) \big\|_{\theta_0}
$$

(I) 是后续 §9 收敛性结论"非退化"所需。若 (I) 不成立（$\kappa = 0$），算法仍可形式收敛，但收敛到的 stationary point 与 $J$ 的真实极值点关系不明。

> **可辨识性的工程操作化**：在实例化时（如 MoEH），需要通过 prompt 设计确保 LLM 对 harness 的"hint"是敏感的——例如把 hint 写得足够明确、放在 prompt 显著位置。可定义经验估计量
> $$\hat \kappa = \min_{\theta_1, \theta_2 \in \text{batch}} \frac{\|\hat \pi_{\theta_1}^{\mathrm{eff}} - \hat \pi_{\theta_2}^{\mathrm{eff}}\|_1}{\|\pi_{\theta_1} - \pi_{\theta_2}\|_1}$$
> 训练前先 probing 验证 $\hat \kappa > 0$。

---

## §5 优化目标

**期望折扣回报**：

$$
J(\theta) \;:=\; \mathbb{E}\!\left[\, \sum_{t=0}^{T-1} \gamma^t r_t \;\bigg|\; x_0 \sim \rho_0,\, (c_t, e_t) \sim \pi_\theta,\, y_t \sim \pi_{\mathrm{LLM}},\, x_{t+1} \sim T \,\right]
$$

由 (R1)–(R2)：$|J(\theta)| \leq R_{\max} / (1 - \gamma) < \infty$。

**目标**：

$$
\theta^* \;\in\; \arg\max_{\theta \in \Theta} J(\theta)
$$

由于 $J$ 通常非凸（神经网络参数化），我们只追求收敛到 **stationary point**，即 $\|\nabla_\theta J(\theta)\| = 0$ 的点。

---

## §6 策略梯度的一般形式

由 score-function（REINFORCE）梯度公式：

$$
\nabla_\theta J(\theta) \;=\; \mathbb{E}_{(x, c, e) \sim d^{\pi_\theta} \otimes \pi_\theta}\!\left[\, Q^{\pi_\theta}(x, c, e) \cdot \nabla_\theta \log \pi_\theta(c, e \mid x) \,\right]
$$

其中

$$
Q^{\pi_\theta}(x, c, e) \;=\; \mathbb{E}\!\left[\, \sum_{s=t}^{T-1} \gamma^{s-t} r_s \,\Big|\, x_t = x,\, (c_t, e_t) = (c, e),\, \text{thereafter follow } \pi_\theta \,\right]
$$

是 $(x, c, e)$ 处的 action-value，**已经边缘化 LLM 与环境的下游随机性**。

**核心困难**：$(c, e)$ 通常是组合或文本空间，$\nabla_\theta \log \pi_\theta(c, e \mid x)$ 不易直接计算。具体方案（如 MoEH）通过**动作因子化**（把 $\pi_\theta$ 分解为更小的可参数化分布）化解此难题。本框架对此保持中立。

### 6.1 Off-policy 与重要性采样

实际训练中，rollout 由旧策略 $\pi_{\theta_{\mathrm{old}}}$ 产生，参数更新发生在 $\pi_\theta$。**重要性比**：

$$
\rho_t(\theta) \;=\; \frac{\pi_\theta(c_t, e_t \mid x_t)}{\pi_{\theta_{\mathrm{old}}}(c_t, e_t \mid x_t)}
$$

期望梯度的 off-policy 形式：

$$
\nabla_\theta J(\theta) \;=\; \mathbb{E}_{(c, e) \sim \pi_{\theta_{\mathrm{old}}}}\!\left[\, \rho_t(\theta) \cdot Q^{\pi_\theta}(x, c, e) \cdot \nabla_\theta \log \pi_\theta(c, e \mid x) \,\right]
$$

这是所有 trust-region 方法（TRPO、PPO、GRPO）的起点。**$\rho$ 的有界性**是后续梯度方差控制的关键，由 §9 中的"策略下界"假设保证（而非由"PPO clipping"——见 §9.2 的辨析）。

---

## §7 信用分配的一般视角

H-LDP 的信用分配可在三个层面同时进行：

| 层面 | 问题 | 通用工具 |
|------|------|---------|
| **Episode-level** | 稀疏 terminal reward → 整个轨迹 | Monte-Carlo return, baseline |
| **Step-level** | 长 episode 内不同步贡献不同 | TD-error, GAE-λ, $n$-step return |
| **Decomposition-level** | $\pi_\theta$ 因子化为多模块，模块间 credit | Mixture gradient, factor-attribution |

本框架不固定具体机制；具体方案在实例化时选择。

**示例（GAE-λ）**：定义 step value $V^{\pi_\theta}(x_t) = \mathbb{E}[\sum_{s \geq t} \gamma^{s-t} r_s \mid x_t]$，TD 误差 $\delta_t = r_t + \gamma V(x_{t+1}) - V(x_t)$，则

$$
\hat A_t^{\mathrm{GAE}} \;=\; \sum_{l=0}^{T-t-1} (\gamma \lambda)^l \delta_{t+l}
$$

兼顾偏差 (low when $\lambda \to 0$) 与方差 (low when $\lambda \to 1$)。

---

## §8 学习算法的抽象描述

任意基于策略梯度的 harness 学习算法可统一写为：

$$
\theta_{t+1} \;=\; \theta_t \;+\; \eta_t \, \hat g_t
$$

其中 $\hat g_t$ 是 $\nabla_\theta J(\theta_t)$ 的某种估计量。**估计方式**决定了具体算法（REINFORCE / PPO / GRPO / actor-critic 等），但**收敛性**只依赖 $\hat g_t$ 满足的几条抽象条件。

### 8.1 Biased Gradient Oracle

**定义 4（$\delta_t$-biased, $G$-bounded gradient oracle）.** 称随机过程 $\{\hat g_t\}_{t \geq 0}$ 关于 $J$ 是一个 **$\delta_t$-biased, $G$-bounded gradient oracle**，若存在 $\mathcal{F}_t$-可适序列 $\delta_t \geq 0$ 与常数 $G < \infty$，使得：

- **(O1) 偏差有界**：$\big\| \mathbb{E}[\hat g_t \mid \mathcal{F}_t] - \nabla J(\theta_t) \big\| \leq \delta_t$ a.s.
- **(O2) 二阶矩有界**：$\mathbb{E}\!\big[\, \|\hat g_t\|^2 \;\big|\; \mathcal{F}_t \,\big] \leq G^2 < \infty$ a.s.
- **(O3) 偏差可控**：$\sum_{t=0}^{\infty} \eta_t \delta_t < \infty$ a.s.，且 $\sup_t \delta_t \leq \delta_{\max} < \infty$。

> **设计意图**：把所有具体算法的"偏差控制 + 方差控制"工作打包到 (O1)–(O3) 三条假设里。具体算法只需在自己的实例化文档中验证这三条，就能直接套用 §9 的收敛性结论。
>
> 注 (O3) 中"$\sup_t \delta_t \leq \delta_{\max}$"是技术性的，用于从 $\sum \eta_t \delta_t < \infty$ 推出 $\sum \eta_t \delta_t^2 < \infty$（见证明）。

### 8.2 学习率条件

**(LR) Robbins-Monro 学习率**：$\eta_t > 0$，$\sum_t \eta_t = \infty$，$\sum_t \eta_t^2 < \infty$。

> **典型选择**：$\eta_t = c / (t+1)$，$\eta_t = c / \sqrt{t+1}$ 不满足（$\sum \eta_t^2 = \infty$），$\eta_t = c / (t+1)^{0.6}$ 满足。
>
> 实践中常用的"warmup + cosine decay" 是 horizon-dependent 的，属于另一类有限时间结果（见 §9.2）。

---

## §9 收敛性

### 9.1 主定理（渐近 a.s. 收敛）

**定理 1（H-LDP 抽象收敛性）.** 设：

- $J: \Theta \to \mathbb{R}$ 在 $\Theta_0 \supset \{\theta_t\}_{t \geq 0}$ 上 $L_J$-smooth（即 $\nabla J$ 是 $L_J$-Lipschitz）；
- $\hat g_t$ 是 $\delta_t$-biased, $G$-bounded gradient oracle（满足 (O1)–(O3)）；
- $\eta_t$ 满足 (LR)。

则

$$
\boxed{\quad \liminf_{t \to \infty}\; \big\| \nabla_\theta J(\theta_t) \big\| \;=\; 0 \quad \text{a.s.} \quad}
$$

**证明**（基于 Robbins-Siegmund 引理；详细步骤见附录 B.2）：

由 $L_J$-smoothness：

$$
J(\theta_{t+1}) \;\geq\; J(\theta_t) + \eta_t \langle \nabla J(\theta_t), \hat g_t \rangle - \frac{L_J \eta_t^2}{2} \|\hat g_t\|^2
$$

取条件期望（条件于 $\mathcal{F}_t$），由 (O1)、(O2)：

$$
\mathbb{E}[J(\theta_{t+1}) \mid \mathcal{F}_t] \;\geq\; J(\theta_t) + \eta_t \langle \nabla J(\theta_t),\, \nabla J(\theta_t) + \varepsilon_t \rangle - \frac{L_J G^2}{2} \eta_t^2
$$

其中 $\|\varepsilon_t\| \leq \delta_t$。由 Cauchy-Schwarz + $ab \leq \tfrac12 a^2 + \tfrac12 b^2$：

$$
\eta_t |\langle \nabla J(\theta_t), \varepsilon_t \rangle| \;\leq\; \frac{\eta_t}{2} \|\nabla J(\theta_t)\|^2 + \frac{\eta_t \delta_t^2}{2}
$$

故

$$
\mathbb{E}[J(\theta_{t+1}) \mid \mathcal{F}_t] \;\geq\; J(\theta_t) + \frac{\eta_t}{2} \|\nabla J(\theta_t)\|^2 - \frac{\eta_t \delta_t^2}{2} - \frac{L_J G^2}{2} \eta_t^2
$$

设 $V_t = J^* - J(\theta_t) \geq 0$（$J^* = \sup_\theta J$，有限由 (R2)）。整理：

$$
\mathbb{E}[V_{t+1} \mid \mathcal{F}_t] \;\leq\; V_t - \underbrace{\frac{\eta_t}{2} \|\nabla J(\theta_t)\|^2}_{W_t} + \underbrace{\frac{\eta_t \delta_t^2}{2} + \frac{L_J G^2}{2} \eta_t^2}_{B_t}
$$

由 (O3) 中 $\sup_t \delta_t \leq \delta_{\max}$ 与 $\sum_t \eta_t \delta_t < \infty$：

$$
\sum_t \eta_t \delta_t^2 \;\leq\; \delta_{\max} \sum_t \eta_t \delta_t \;<\; \infty
$$

结合 (LR) 中 $\sum_t \eta_t^2 < \infty$，可知 $\sum_t B_t < \infty$ a.s.

应用 **Robbins-Siegmund 引理**（附录 B.1）：存在有限随机变量 $V_\infty$ 使 $V_t \to V_\infty$ a.s.，且 $\sum_t W_t < \infty$ a.s.

故 $\sum_t \eta_t \|\nabla J(\theta_t)\|^2 < \infty$ a.s. 结合 $\sum_t \eta_t = \infty$（由 (LR)），必有 $\liminf_t \|\nabla J(\theta_t)\| = 0$ a.s. $\square$

### 9.2 关于 PPO clipping 的精细说明

> **常见误解**：PPO clipping 把重要性比 $\rho$ 限制在 $[1-\epsilon, 1+\epsilon]$ 内。
>
> **事实**：PPO clipping 限制的是 **surrogate objective 中使用的 clipped ratio**：
> $$\mathrm{clip}(\rho, 1-\epsilon, 1+\epsilon)$$
> 而非实际比值 $\rho$。当 $\pi_{\theta_{\mathrm{old}}}(c, e \mid x)$ 极小时，$\rho$ 可任意大。

**正确的 ratio 界**需要额外假设：

- **(P-floor) 策略下界**：存在 $p_{\min} > 0$ 使得 $\pi_\theta(c, e \mid x) \geq p_{\min}$ 对所有"被采样"的 $(c, e, x)$ 一致成立。

由 (P-floor)：

$$
\rho_t(\theta) \;\leq\; \frac{1}{\pi_{\theta_{\mathrm{old}}}(c_t, e_t \mid x_t)} \;\leq\; \frac{1}{p_{\min}}
$$

(P-floor) 通常通过**熵正则化**（向 loss 加 $-\lambda H(\pi_\theta)$）或**显式 floor**（输出 softmax 后下界裁剪）实现。

**替代方案（KL trust region）**：

- **(KL-TR) KL 信赖域**：$D_{\mathrm{KL}}(\pi_{\theta_{\mathrm{old}}} \,\|\, \pi_\theta) \leq \Delta$ 对所有 $t$ 一致成立。

由 Pinsker 不等式：$\|\pi_{\theta_{\mathrm{old}}} - \pi_\theta\|_{\mathrm{TV}} \leq \sqrt{\Delta / 2}$，可推得 $|\rho - 1|$ 的一致 bound（依赖 $p_{\min}$）。

> **小结**：PPO clipping 单独**不**给出 (O2) 所需的 $\|\hat g\|$ 界；必须配以 (P-floor) 或 (KL-TR) 之一。在 §10 实例化文档中需明确选择哪一种。

### 9.3 有限时间速率（独立结果）

定理 1 给出渐近 a.s. 收敛；对**预设** horizon $T$ 的有限时间速率，需另立结论：

**定理 2（有限时间速率，固定步长）.** 设 $\hat g_t$ 满足 (O1)–(O2) 且 $\delta_t \leq \delta$ 一致。选取常数步长

$$
\eta \;=\; \min\!\left(\, \frac{1}{L_J},\; \sqrt{\frac{2 (J^* - J(\theta_0))}{L_J G^2 T}} \,\right)
$$

则

$$
\frac{1}{T} \sum_{t=0}^{T-1} \mathbb{E}\!\big[\, \|\nabla J(\theta_t)\|^2 \,\big] \;\leq\; \sqrt{\frac{8 L_J G^2 (J^* - J(\theta_0))}{T}} \;+\; \delta^2
$$

即收敛到一个 $O(1/\sqrt{T})$ + $O(\delta^2)$ 邻域。

> **说明**：定理 1（渐近）与定理 2（有限时间）是**两个独立结果**，不能从一个直接推出另一个。
>
> - 定理 1 在 (LR)（变步长）下成立，证明 a.s. 收敛但无速率；
> - 定理 2 在固定步长下成立，给出 expected 速率但不保证 a.s.
>
> 这是 nonconvex stochastic optimization 的标准分离。证明见 Bottou-Curtis-Nocedal (2018) Theorem 4.10。

### 9.4 收敛到 $J$ vs 收敛到正则化目标

若算法实际优化的是

$$
\tilde J(\theta) \;:=\; J(\theta) - \sum_i \alpha_i L_i^{\mathrm{reg}}(\theta)
$$

（如 entropy / KL / load-balancing 等正则项），则 $\hat g_t$ 是 $\nabla \tilde J$（不是 $\nabla J$）的 oracle。

定理 1 给出的是 $\liminf \|\nabla \tilde J(\theta_t)\| = 0$ a.s.。在 stationary point 处：

$$
\nabla J(\theta_\infty) \;=\; \sum_i \alpha_i \nabla L_i^{\mathrm{reg}}(\theta_\infty)
$$

即 $J$ 的稳定点与 $\tilde J$ 的稳定点**通常不重合**，相差 $O(\sum_i \alpha_i \|\nabla L_i^{\mathrm{reg}}\|)$。

**两种合法的处理方式**：

**方式 A（reframe 结论）**：陈述为"收敛到正则化目标 $\tilde J$ 的稳定点"。

**方式 B（衰减系数）**：使用 $\alpha_i(t) \to 0$，且要求 $\sum_t \eta_t \alpha_i(t) < \infty$。这样定理 1 可应用于 $J$ 本身，但实践中正则化失去效果。

> **不合法的处理方式**：固定 $\alpha_i > 0$ 同时声称收敛到 $J$ 的稳定点。这是常见错误。

### 9.5 对优化器的边界

定理 1、2 都基于 **SGD-style** 更新 $\theta_{t+1} = \theta_t + \eta_t \hat g_t$。**Adam 等自适应优化器**的收敛性需单独分析（见 Reddi-Kale-Kumar 2018 AMSGrad、Défossez et al. 2022）。

在 H-LDP 框架内若使用 Adam，需补充：

- **(Adam-1) Preconditioner 谱范数有上下界**：$\lambda_{\min}(P_t) \geq p_- > 0$, $\lambda_{\max}(P_t) \leq p_+ < \infty$。
- **(Adam-2) 经 preconditioner 调整后的有效步长仍满足 (LR) 的某种推广**。

实践中可平移 SGD 结果到 Adam，但严格性需另行论证。本文主体仅陈述 SGD 结果。

### 9.6 关于策略可辨识性的角色

注意：定理 1 在抽象 $J$ 上成立，**不**需要 (I) 可辨识性。但 (I) 决定了**所收敛到的稳定点是否有意义**：

- 若 (I) 成立（$\kappa > 0$）：$\nabla J(\theta) = 0$ 意味着策略已经在有意义的方向上没有改进空间。
- 若 (I) 失败（$\kappa = 0$）：$\nabla J(\theta) = 0$ 可能仅因为 LLM 对 $(c, e)$ 完全不敏感，"信号被吃掉"，与 $\theta$ 的实际优劣无关。

**故 (I) 是 learnable harness 框架的 "well-posedness" 条件**——是定理 1 的**前提补充**，不是定理本身的假设，但必须独立验证。

---

## §10 实例化路径

任何 learnable harness 算法只要：

1. **结构化定义**：给出具体的 $\mathcal{X}, \mathcal{C}, \mathcal{E}, \mathcal{Y}$ 与转移 $T$ 的形式；
2. **参数化**：给出 $\pi_\theta$ 的具体架构；
3. **梯度估计器**：给出 $\hat g_t$ 的具体公式（loss + estimator）；
4. **假设验证**：
   - 验证 (R1)–(R6) 与 (I)；
   - 证明 $\hat g_t$ 满足 (O1)–(O3)（通常通过 (P-floor) 或 (KL-TR)）；
5. **正则化处理**：若使用辅助正则项，明确按 §9.4 的方式 A 或 B 处理；
6. **优化器**：使用 SGD 直接套用定理 1；使用 Adam 需补 (Adam-1)、(Adam-2)。

完成以上 6 步即可直接套用 §9.1 的渐近收敛结论。

**实例对照**：

| 实例 | $(c, e)$ 因子化 | $\pi_\theta$ 形式 | 估计方式 | 关键假设 |
|------|----------------|------------------|----------|---------|
| Fixed harness | $c$ 模板 | $\pi$ 常数 | — | — |
| Memonto / CB-MDP | $c$ ← memory retrieval | 单 retrieval policy | TD(λ) + 计数衰减 RM | 离散 memory + 有限动作 |
| DSPy prompt search | $c \in$ 候选集 | softmax over candidates | black-box | 候选集有限 + bandit-style |
| **MoEH**（本系列） | $(c, e)$ ← advisor hint + LLM 协同 | MoE top-$k$ advisor | GRPO + GAE-λ | (P-floor) + (I) + 详见配套文档 |

> 配套文档 [`evolve-harness-math.md`](./evolve-harness-math.md) 把 MoEH 作为本框架的具体实例，**逐条**验证以上 6 步。

---

## 附录 A：符号速查表

| 符号 | 含义 |
|------|------|
| $\mathcal{X}$ | Harness 可观测状态空间 |
| $\mathcal{C}$ | 执行上下文空间 |
| $\mathcal{E}$ | 执行环境空间 |
| $\mathcal{Y}$ | LLM 输出空间 |
| $\pi_{\mathrm{LLM}}$ | 冻结 LLM 算子 |
| $T$ | 环境转移核 |
| $R$ | 即时奖励 |
| $\pi_\theta$ | Harness 策略 |
| $\pi_\theta^{\mathrm{eff}}$ | 有效策略（与 LLM 复合） |
| $P_\theta$ | 有效转移核 |
| $d^{\pi_\theta}$ | 有效状态访问分布 |
| $J(\theta)$ | 期望折扣回报 |
| $\tilde J(\theta)$ | 正则化目标 |
| $Q^{\pi_\theta}$ | 有效 action-value |
| $\rho_t(\theta)$ | 重要性比 |
| $\hat g_t$ | 梯度估计量 |
| $\delta_t$ | Oracle 偏差上界 |
| $G$ | Oracle 二阶矩上界 |
| $\eta_t$ | 学习率 |
| $\kappa$ | 可辨识性常数 |
| $p_{\min}$ | 策略下界 |
| $\gamma$ | 折扣因子 |

---

## 附录 B：Robbins-Siegmund 引理与证明细节

### B.1 Robbins-Siegmund 引理

**引理 B.1（Robbins-Siegmund, 1971）.** 设 $(V_t, A_t, B_t, W_t)_{t \geq 0}$ 是非负、$\mathcal{F}_t$-可适随机过程，满足

$$
\mathbb{E}[V_{t+1} \mid \mathcal{F}_t] \;\leq\; (1 + A_t) V_t - W_t + B_t, \quad t \geq 0
$$

且 $\sum_t A_t < \infty$, $\sum_t B_t < \infty$ a.s. 则：

1. 存在有限随机变量 $V_\infty \geq 0$ 使 $V_t \to V_\infty$ a.s.；
2. $\sum_t W_t < \infty$ a.s.

证明：原文 Robbins-Siegmund (1971) Theorem 1；亦见 Bertsekas-Tsitsiklis (1996) Proposition 4.2 与 Polyak (1987) Lemma 11.2。

### B.2 定理 1 证明细节

主体已在 §9.1 给出。三个易错点：

**(i)** 从 (O1) "**偏差 $\leq \delta_t$**" 到不等式中出现的 "$\delta_t^2$"：

由 $|\langle u, v \rangle| \leq \|u\| \|v\|$ 与 $2|ab| \leq a^2 + b^2$（取 $a = \|u\|$, $b = \|v\|$）：

$$
|\langle \nabla J(\theta_t), \varepsilon_t \rangle| \;\leq\; \|\nabla J(\theta_t)\| \cdot \|\varepsilon_t\| \;\leq\; \frac{1}{2} \|\nabla J(\theta_t)\|^2 + \frac{1}{2} \|\varepsilon_t\|^2
$$

乘以 $\eta_t$ 并用 $\|\varepsilon_t\| \leq \delta_t$ 即得 $\frac{\eta_t}{2} \delta_t^2$。

**(ii)** 从 $\sum_t \eta_t \delta_t < \infty$ + $\sup_t \delta_t \leq \delta_{\max}$ 推出 $\sum_t \eta_t \delta_t^2 < \infty$：

$$
\sum_t \eta_t \delta_t^2 \;=\; \sum_t \eta_t \delta_t \cdot \delta_t \;\leq\; \delta_{\max} \sum_t \eta_t \delta_t \;<\; \infty
$$

故 $\sum_t B_t < \infty$ a.s.（B_t 中两项都 summable）。

**(iii)** 从 $\sum_t \eta_t \|\nabla J(\theta_t)\|^2 < \infty$ + $\sum_t \eta_t = \infty$ 推出 $\liminf \|\nabla J\| = 0$ a.s.：

反证：若 $\liminf \|\nabla J(\theta_t)\|^2 \geq \varepsilon > 0$ a.s.，则存在 $T_0$ 使 $\forall t \geq T_0$，$\|\nabla J(\theta_t)\|^2 \geq \varepsilon / 2$。故

$$
\sum_{t \geq T_0} \eta_t \|\nabla J(\theta_t)\|^2 \;\geq\; \frac{\varepsilon}{2} \sum_{t \geq T_0} \eta_t \;=\; \infty
$$

矛盾。$\square$

> **注**：此处**不**用 Markov + Borel-Cantelli（那条路径要求 $\sum_t \mathbb{E} Z_t < \infty$，而我们只有 $\sum_t \eta_t \mathbb{E} Z_t < \infty$，弱得多）。Robbins-Siegmund 是正确的工具。

### B.3 (O1)–(O3) 的工程检查清单

具体算法实例化时，建议按下表检查每条假设：

| 假设 | 工程检查 |
|------|---------|
| (O1) | 计算 $\hat g_t$ 期望，对比 $\nabla J$ 的解析或近似形式，给出偏差上界 $\delta_t$ |
| (O2) | 检查 $\|\hat g_t\|^2$ 的逐项上界（需要 ratio bound + advantage bound + score bound） |
| (O3) | 检查偏差源（off-policy、normalization、regularization）是否分别 summable |
| (LR) | 选择 $\eta_t = c / t^p$ with $p \in (0.5, 1]$ 即可保证 |
| (P-floor) 或 (KL-TR) | 选其一并验证；通常 (P-floor) 通过 entropy regularizer 实现 |
| (I) | 训练前 probing 估计 $\hat \kappa$；运行时监控 |

---

## 文档版本

| 版本 | 日期 | 变更 |
|------|------|------|
| v0.1 | 2026-05-31 | 初稿：建立 H-LDP 一般框架、抽象 oracle 假设、Robbins-Siegmund 主定理；明确 PPO clipping、可辨识性、正则化目标等易错点 |
