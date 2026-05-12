# CM — Context Machine

## 1. Mathematical Definition

### Definition 1 (CM)

A context machine is a triple:

$$\mathcal{M} = (\mathcal{C}, \mathcal{E}, \Phi)$$

where:

- $\mathcal{C}$ — context space
- $\mathcal{E}$ — environment space
- $\Phi: \mathcal{C} \times \mathcal{E} \to \mathcal{C} \times \mathcal{E}$ — state transition map

### Definition 2 (State)

Let $x = (c, e) \in \mathcal{C} \times \mathcal{E}$ denote the system state.

### Definition 3 (State Transition)

$$\Phi(c, e) = (c', e')$$

where:

$$c' = \pi(c, e)$$
$$e' = \omega(c', e)$$

- $\pi: \mathcal{C} \times \mathcal{E} \to \mathcal{C}$ — context engineering
- $\omega: \mathcal{C} \times \mathcal{E} \to \mathcal{E}$ — environment transition

$\pi$ executes first. $\omega$ depends on the result of $\pi$.

### Definition 4 (Computation)

Given an initial state $x_0 = (c_0, e_0)$, the computation of $\mathcal{M}$ is the sequence:

$$x_{t+1} = \Phi(x_t), \quad t = 0, 1, 2, \dots$$

The computation terminates at $x_n$ when $x_n \in \mathcal{H}$, where $\mathcal{H} \subset \mathcal{C} \times \mathcal{E}$ is the set of terminal states.

### Symbol Table

| Symbol | Meaning |
|--------|---------|
| $\mathcal{M}$ | Context machine instance |
| $\mathcal{C}$ | Context space |
| $\mathcal{E}$ | Environment space |
| $c \in \mathcal{C}$ | Context |
| $e \in \mathcal{E}$ | Environment |
| $x = (c, e)$ | System state |
| $\Phi$ | State transition map |
| $\pi$ | Context engineering |
| $\omega$ | Environment transition |
| $x_t$ | State at time $t$ |
| $\mathcal{H}$ | Terminal state set |

### Properties

**Property 1 (Markov property).** $x_{t+1}$ depends only on $x_t$, not on history.

**Property 2 (Determinism).** $\Phi$ is a function; $x_{t+1}$ is uniquely determined by $x_t$.

---

## 2. Refinement

### 2.1 Context Space $\mathcal{C}$

$\mathcal{C}$ is the set of all finite sequences of fragments:

$$\mathcal{C} = \mathcal{F}^*$$

A fragment $f \in \mathcal{F}$ is an atomic unit of context. Each fragment carries a unique identifier $i \in \mathbb{N}$, assigned upon insertion. Identifiers are stable: $\text{replace}$ preserves the id, $\text{remove}$ destroys it.

A fragment has an immutable role:

$$role(f) \in \{ \text{System}, \text{User}, \text{Assistant} \}$$

The role is an intrinsic property of the fragment and cannot be modified by $\pi$.

The empty context is denoted $c_0 = [\ ]$.

### 2.2 Environment Space $\mathcal{E}$

$\mathcal{E}$ is an abstract state space representing the external world accessible to the machine. Its internal structure is deliberately unspecified — $\mathcal{E}$ may include file systems, sandboxes, network state, tool registries, or any other external resource.

$\mathcal{E}$ provides a resource pool:

$$e.resources = \{ (id_1, f_1), (id_2, f_2), \dots \}$$

Resources are pre-constructed fragments (prompt templates, tool definitions, memory snippets) that $\pi$ can select and insert into $c$.

### 2.3 Pending Queue $p$

The pending queue $p \in \mathcal{F}^*$ holds fragments produced by $\omega$ that have not yet been consumed by $\pi$. It acts as a buffer between $\omega$ and $\pi$.

The head of the queue is denoted $head(p)$. $\pi$ may only consume from the head — fragments must be processed in order.

### 2.4 Context Engineering $\pi$

$\pi$ constructs a new context from the current context, environment, and pending queue:

$$\pi(c, e, p) = c'$$

$\pi$ operates in two modes:

**Consumption mode** ($p \neq [\ ]$). $\pi$ must consume pending fragments. Available actions:

| Action | Effect |
|--------|--------|
| $\text{append}$ | Append $head(p)$ to $c$; remove from $p$ |
| $\text{insert}(i)$ | Insert $head(p)$ after fragment $i$; remove from $p$ |
| $\text{remove}(i)$ | Remove fragment $i$ from $c$ |

**Free mode** ($p = [\ ]$). $\pi$ may freely construct context from $e.resources$ or halt. Available actions:

| Action | Effect |
|--------|--------|
| $\text{append}(k)$ | Append resource $k$ from $e.resources$ to $c$ |
| $\text{insert}(i, k)$ | Insert resource $k$ after fragment $i$ |
| $\text{remove}(i)$ | Remove fragment $i$ from $c$ |
| $\text{replace}(i, k)$ | Replace fragment $i$ with resource $k$ |
| $\text{halt}$ | Terminate $\pi$; trigger $\omega$ |

**Constraint.** $\text{halt}$ is only legal when $p = [\ ]$. The pending queue must be empty before the machine can invoke $\omega$.

### 2.5 Environment Transition $\omega$

$\omega$ encapsulates the LLM interaction cycle:

1. The new context $c'$ is fed to the language model.
2. The language model produces output (text, tool calls).
3. Tool calls are executed against the current environment $e$.
4. The environment is updated to $e'$.
5. The language model output and tool results are placed into $p$ as new fragments.

$$\omega(c', e) = (e', p_{new})$$

The language model itself is treated as a fixed function — it is not part of the machine's state. Only its input ($c'$) varies across steps.

### 2.6 Full State Transition

With the pending queue, the state transition becomes:

$$\Phi(c, e, p) = (c', e', p')$$

where:

$$c' = \pi(c, e, p)$$
$$(e', p') = \omega(c', e)$$

### 2.7 Action Space for Reinforcement Learning

$\pi$'s action space is discrete and finite:

| Mode | Action | Parameter space |
|------|--------|----------------|
| Consumption | $\text{append}$ | 1 |
| Consumption | $\text{insert}(i)$ | $|c|$ |
| Consumption | $\text{remove}(i)$ | $|c|$ |
| Free | $\text{append}(k)$ | $|e.resources|$ |
| Free | $\text{insert}(i, k)$ | $|c| \times |e.resources|$ |
| Free | $\text{remove}(i)$ | $|c|$ |
| Free | $\text{replace}(i, k)$ | $|c| \times |e.resources|$ |
| Free | $\text{halt}$ | 1 |

Reward is delayed: it is assigned after $\omega$ completes and distributed across all $\pi$ steps within the same $\Phi$ cycle.