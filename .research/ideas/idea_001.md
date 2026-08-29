## Core Insight

Agent harness 面临 capacity-prompt coupling:经验越多 → prompt 越长 → attention 稀释 + 成本上升。现有方案(summarization / retrieval / compression)全部非参数化。MoEH 用 1M 参数的 MoE advisor,每步把状态编码成 embedding,输出 12 维动作先验分布 π(s),通过 logit mixing(logit_final = λ·logit_prior + (1-λ)·logit_executor)干预 7B executor。经验被压进权重,prompt 长度恒定,长 horizon 不退化。

架构:冻结 embedding(qwen3-embedding-0.6b)→ 128-dim adapter → 896-dim fused state → 2-layer MLP router(top-2)→ 4 个 expert MLP(896→128→12)→ weighted mix → 12 维动作先验。总参数 ~1.0M。

训练:Phase B BC pretrain(NLL + 0.01×load_balancing, lr=1e-4)→ Phase C GRPO fine-tune(k=8 rollouts, lr=5e-5, 20 轮)。

## Novelty Analysis

核心缺口:**没有现有工作训练一个独立小模型,为 LLM agent 提供 step-level 动作分布先验**。

- 相关工作分成四类,全部不重叠:
  - (a) 直接训练完整 LLM(ART, Agent-BRACE)
  - (b) 静态 skill 注入(PA-MoE——phase-level,非 step-level)
  - (c) 自然语言建议(Advisor Models, ExecTune——语义而非机制)
  - (d) 程序化上下文压缩(FoldGRPO, MEM1——非参数化)
- 定位:MoEH = 参数化动作先验 + MoE 路由 + GRPO,四者组合无先例

主要主张(可证伪):参数化动作先验把经验与 prompt 长度解耦 → 200 turns 成功率不低于 10 turns(配对 t 检验 p<0.05)。

支持主张(条件):MoE 架构优于 dense/linear 先验(若 Linear 在 2% 内则放弃 MoE claim)。

非贡献声明:不 claim 在线进化(offline + 可选周期微调)、不 claim 跨 executor 泛化、不 claim emergent specialization(只测量)。

## Feasibility

- 计算预算 363 GPU-hours,~5 周
- 主 benchmark: Terminal-Bench 2.0(89 tasks, SOTA GPT-5.2 63%, 目标 +5%~+10%)
- Phase A 数据:RCM 无 MoEH 跑 TB2 + SWE-bench Lite,期望 2000-3500 成功步
- 已知风险:TB2 太小(用 TerminalTraj 50K 补)、executor 不暴露 logits(降级 NL prior)、Linear 追平(显式 drop condition)

## Open Issues (from analysis)

1. **12 维动作空间未定义**——最大空洞,决定信息量与 novelty 边界
2. **Decoupling 逻辑漏洞**——embedding 模型自身有上下文窗口,长历史必然截断
3. **名字叫 Evolving 但不 claim evolving**——命名与内容不符
4. **Advisor 基线不公平**——1M 参数 NL advisor 是残废版稻草人
5. **Equifinality in MoE 未防御**——需加路由消融(学习 vs 固定 vs 随机)
6. **GRPO 信用分配**——任务级稀疏奖励,无过程奖励
7. **自身过拟合 harness**——GRPO 只在 20 个 TB2 任务上训练,须用 RCM 多 harness 变体收集数据

---

## Update — 2026-08-18T09:31:18Z

导入完成。已基于提案 + 基线设计 + 论文表做了完整分析,识别 7 个关键弱点:
1. 12 维动作空间未定义(最大空洞)
2. Decoupling 逻辑漏洞(embedding 模型自身有上下文窗口)
3. 名字叫 Evolving 但不 claim evolving
4. Advisor 基线不公平(1M 参数 NL advisor 是稻草人)
5. Equifinality in MoE 未防御(需路由消融)
6. GRPO 信用分配问题(任务级稀疏奖励)
7. 自身过拟合 harness(GRPO 只在 20 个 TB2 任务训练,须用 RCM 多 harness 变体收集数据)

文献库:23 篇论文已导入(19 core + 4 related),gap G01 已注册,idea_001 已与 Advisor Models/HARBOR/FoldGRPO 建立关系边。
