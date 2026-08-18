# Literature Context: MoEH: Mixture-of-Evolving-Harness for LLM Agent Systems

## Key Papers
| Slug | One-line Thesis | Relevance | Tags |
|------|----------------|-----------|------|
| adityataparia_2026_learning_to_configure_agentic_ai_systems | (see adityataparia_2026_learning_to_configure_agentic_ai_systems.md) | core | harness-automation, closest-work |
| biswasengupta_2026_harbor_automated_harness_optimization | (see biswasengupta_2026_harbor_automated_harness_optimization.md) | core | harness-automation, closest-work |
| carlosjimenez_2023_swebench_can_language_models_resolve_realworld_github_issues | (see carlosjimenez_2023_swebench_can_language_models_resolve_realworld_github_issues.md) | core | benchmark |
| haebinseong_2026_last_harness_youll | (see haebinseong_2026_last_harness_youll.md) | core | harness-automation, closest-work |
| jianhaoruan_2026_aorchestra_automating_subagent_creation_for_agentic_orchestration | (see jianhaoruan_2026_aorchestra_automating_subagent_creation_for_agentic_orchestration.md) | core | harness-automation, closest-work |
| joykiratsingh_2026_agentbrace_decoupling_beliefs_from_actions_in_longhorizon_tasks_via_verbalized_state_uncertainty | (see joykiratsingh_2026_agentbrace_decoupling_beliefs_from_actions_in_longhorizon_tasks_via_verbalized_state_uncertainty.md) | core | advisor-models, closest-work |
| mikeamerrill_2026_terminalbench_benchmarking_agents_on_hard_realistic_tasks_in_command_line_interfaces | (see mikeamerrill_2026_terminalbench_benchmarking_agents_on_hard_realistic_tasks_in_command_line_interfaces.md) | core | benchmark |
| muhammadrashid_2025_swepolybench_a_multilanguage_benchmark_for_repository_level_evaluation_of_coding_agents | (see muhammadrashid_2025_swepolybench_a_multilanguage_benchmark_for_repository_level_evaluation_of_coding_agents.md) | core | benchmark |
| parthasawa_2025_how_to_train_your_advisor_steering_blackbox_llms_with_advisor_models | (see parthasawa_2025_how_to_train_your_advisor_steering_blackbox_llms_with_advisor_models.md) | core | advisor-models, baseline |
| paulbrookes_2025_evolving_excellence_automated_optimization_of_llmbased_agents | (see paulbrookes_2025_evolving_excellence_automated_optimization_of_llmbased_agents.md) | core | harness-automation, closest-work |
| sethkarten_2026_continual_harness_online_adaptation_for_selfimproving_foundation_agents | (see sethkarten_2026_continual_harness_online_adaptation_for_selfimproving_foundation_agents.md) | core | harness-automation, evolving, closest-work |
| shengtianyang_2026_phaseaware_mixture_of_experts_for_agentic_reinforcement_learning | (see shengtianyang_2026_phaseaware_mixture_of_experts_for_agentic_reinforcement_learning.md) | core | moe-agents, closest-work |
| tianshixu_2026_adapting_the_interface_not_the_model_runtime_harness_adaptation_for_deterministic_llm_agents | (see tianshixu_2026_adapting_the_interface_not_the_model_runtime_harness_adaptation_for_deterministic_llm_agents.md) | core | harness-automation, runtime-adaptation, closest-work |
| vijaylingam_2026_exectune_effective_steering_of_blackbox_llms_with_guide_models | (see vijaylingam_2026_exectune_effective_steering_of_blackbox_llms_with_guide_models.md) | core | advisor-models, closest-work |
| weiweisun_2025_scaling_longhorizon_llm_agent_via_contextfolding | (see weiweisun_2025_scaling_longhorizon_llm_agent_via_contextfolding.md) | core | context-compression, baseline |

## Active Gaps
G01. 没有现有工作训练一个独立小模型,为 LLM agent 提供 step-level 动作分布先验。现有方案要么训练完整 LLM(GRPO)、要么静态 skill 注入、要么输出 NL 建议、要么程序化压缩上下文——没有"参数化动作先验"这一格。 — source: parthasawa_2025_how_to_train_your_advisor_steering_blackbox_llms_with_advisor_models

## Known Baselines
(populate from papers/*.md Key Results sections)

## Open Questions
(populate from papers/*.md Open Questions sections)

## Time-Critical Signals
(papers and findings from last 3 months — check dates)

## Active Chains
没有现有工作训练一个独立小模型,为 LLM agent 提供 step-level 动作分布先验。现有方案要么训练完整 LLM(GRPO)、要么静态 skill 注入、要么输出 NL 建议、要么程序化压缩上下文——没有"参数化动作先验"这一格。 ← idea_001