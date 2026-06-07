# Section Summaries

## s1: Introduction and Scope
Establishes the need for automated survey generation given the scale of scientific publishing (citing STM Global Brief 2023). Defines the dual challenge of retrieval and synthesis. Positions this survey relative to three existing surveys (2309.09727, 2002.06961, 2409.04600), highlighting three unique aspects: focus on citation graph expansion, critical assessment of claims, and five-phase evolution narrative. Adds cross-domain context connecting to PRISMA, SummEval, and scientometric bias corrections. States three contributions — taxonomy of 135+ papers, five-phase narrative, critical assessment — and provides a roadmap.

## s2.1: Citation Graph Foundations (2015–2020)
Covers Phase 1: pre-LLM graph-based methods. Details Semantic Scholar Literature Graph (280M+ nodes), SPECTER citation-informed contrastive embeddings, LitFM graph transformer (+28.1% precision), Context-Aware Citation Rec (+28% MAP), and HiGTL taxonomy induction. Includes performance table. Establishes that graph structure encodes intellectual lineage, role differentiation, community boundaries, and temporal patterns — signals orthogonal to text content. Ends with the limitation that drove Phase 2: graphs could retrieve but not synthesize.

## s2.2: Single-Agent Survey Pipelines (2023–Early 2024)
Covers Phase 2: the LLM revolution. Details AutoSurvey's canonical Outline→Retrieve→Draft→Refine pipeline, PaperQA's BFS citation chaining, PaperQA2's contradiction detection and "superhuman" claim (critiqued by 2508.11310, 2601.15307), STORM's multi-perspective questioning, and OpenScholar's 45M-paper datastore. Includes a 5-system comparison table. Critically assesses the "human-competitive" and "superhuman" claims. Develops Thread 1 (abandoning graphs for semantics) and Thread 2 (each system uses a different benchmark).

## s2.3: Multi-Agent Architectures (Late 2024–2025)
Covers Phase 3: multi-agent systems. Details SciSage's reflect-while-writing (4 agents, +32% citation F1), Agentic AutoSurvey's shared task board (4 agents, 8.18/10), MATC's error-mitigation taskforces, and InsightAgent's human-centered design. Includes a 4-system comparison table. Critically assesses that baselines differ across papers, no controlled ablation studies exist, and multi-agent coordination amplifies rather than solves retrieval gaps. Develops Thread 3 and Thread 2.

## s2.4: Iterative and RL-Guided Systems (2025)
Covers Phase 4: learning from feedback. Details PaSa's RL-optimized search policy (+37.78% recall), AURA's adaptive questioning, IterSurvey's recurrent outline generation, and SurveyGen-I's adaptive planning. Includes a 4-system comparison table. Analyzes the bottleneck transfer problem: RL optimizes retrieval objectives, not survey quality, and the transfer is asserted but untested.

## s2.5: Citation Graph Re-integration (Current Frontier, 2025–2026)
Covers Phase 5: the return to structure. Details SurveyG's three-layer hierarchical graph with horizontal/vertical traversal, Graphs of Research's 2-hop citation DAG as SFT training data, Science Hierarchography's hybrid embedding+LLM clustering, and LitFM. Includes a performance table. Exposes four open problems: shallow graph-LLM integration, ignored temporal dynamics, arbitrary hierarchy granularity, and the absence of graph-aware multi-agent systems.

## s2.cross: Cross-Phase Comparison Table
The single most important comparative artifact in the survey. Comprehensive table spanning all 5 phases with columns: phase name, time period, representative systems, graph awareness level, iteration strategy, claimed metric + value, evaluation benchmark, computational cost profile, paper count. Post-table analysis notes: (1) cross-phase comparison is impossible due to benchmark proliferation, (2) graph awareness was lost then regained, (3) computational cost reporting is almost entirely absent.

## s3.1: Graph-Enhanced Retrieval — Structure as Signal
Deep-dive into graph-enhanced retrieval mechanisms. Explains four signals graphs encode that text cannot: intellectual lineage, role differentiation, community boundaries, temporal evolution. Details LitFM's graph transformer (attention over text + neighbor tokens simultaneously), SurveyG's horizontal/vertical traversal, HiGTL's GNN+clustering, and SPECTER's contrastive embeddings. Includes a 4-method comparison table with cost profiles. Concludes that graph-aware retrieval produces better representations but stops at retrieval — synthesis is left to graph-blind methods.

## s3.2: Single-Agent Pipelines — Semantic Retrieval at Scale
Deep-dive into single-agent pipeline mechanisms. Details AutoSurvey's outline-driven staging, PaperQA2's contradiction detection algorithm (claim extraction, cross-paper alignment, flagging of conflicts), OpenScholar's 45M-paper datastore, STORM's multi-perspective questioning, and SurveyX's AttributeTree. Includes a 5-system comparison table. Analyzes whether massive scale compensates for graph blindness: scale helps but cannot overcome terminological barriers that citation graphs would capture.

## s3.3: Multi-Agent Architectures — Division of Labor, Amplification of Gaps
Deep-dive into multi-agent mechanisms. Details SciSage's reflect-while-writing (Writer pauses every N sentences for Reflector evaluation before continuing), Agentic AutoSurvey's shared task board protocol (parallel agent work), and MATC's error-mitigation taskforces (Exploitation, Exploration, Experience, Self-Correction). Includes a 4-system comparison table with overhead costs. Critical analysis: improvements are genuine but confounded — no controlled ablation studies, baselines differ, and multi-agent coordination amplifies retrieval gaps.

## s3.4: The Bottleneck Transfer Problem — Retrieval Gains ≠ Survey Gains
Presents the bottleneck transfer argument. Builds a table showing retrieval metrics vs. survey quality metrics across all paradigms with baseline absolute values. Explains three structural reasons transfer may fail: selection matters more than recall, recall-coherence trade-offs, and unmeasured quality dimensions. Adds a cost-efficiency argument: expensive retrieval gains (massive datastores, RL training) may not justify marginal survey quality improvements. Crystallizes Thread 3 as an untested hypothesis.

## s4.1: Human-in-the-Loop Systems — Quality Through Oversight
Details HITL mechanisms: InsightAgent (human orchestrator + 5 agents, +27.2% quality, months→1.5h), ASReview (active learning stopping criterion, 80-95% effort reduction), LitChat (conversational KG construction), FAST² (self-correcting classifier). Includes a 4-system comparison table. Critical analysis: HITL achieves the highest quality but at scalability cost — the human bottleneck that InsightAgent introduces is a fundamental limitation. Even human-guided systems do not measure critical-analytic depth.

## s4.2: Procedural Iterative Refinement — Self-Evaluation Loops
Details procedural self-evaluation mechanisms: IterSurvey's recurrent outline generation (outline adapts to discovered content), SurveyGen-I's coarse-to-fine retrieval with adaptive planning and memory, and SurveyGen's quality-aware RAG with 4,200+ survey dataset. Includes a 3-system comparison table. Critical analysis: self-evaluation relies on the LLM to detect its own errors — known overconfidence bias means the refinement loop may converge to a locally optimal but globally flawed survey.

## s4.3: Reinforcement Learning for Search and Generation Policies
Details RL approaches: PaSa's epsilon-greedy search policy (citation follow, keyword refine, author search; recall@20 reward; synthetic trajectories; +37.78% recall), AURA's adaptive questioning (LSDE metric), Text2Grad's span-level gradients from NL feedback, and RL4F's Generator+Critic architecture. Includes a 4-system comparison table with training compute costs. Critical analysis: RL optimizes for what is measurable (recall, information gain) rather than what matters (critical-analytic depth, scholarly value).

## s4.4: Cross-Approach Synthesis — Scalability, Quality Ceiling, and Cost
NEW synthesis comparing HITL (§4.1), procedural iteration (§4.2), and RL (§4.3) on scalability, quality ceiling, cost, citation accuracy, graph awareness, and critical-analytic depth. Key finding: HITL achieves highest quality (+27.2%) but does not scale; RL scales best but optimizes narrow objectives; procedural iteration sits between. Universal blind spot: no approach measures critical-analytic depth. Provides deployment recommendations per scenario.

## s5.1: Claim vs. Evidence Gap Analysis
Presents a systematic evaluation of 7 major claims across the literature: AutoSurvey "human-competitive," PaperQA2 "superhuman," OpenScholar "8B beats GPT-4o," multi-agent "dramatically better," LitFM "28.1% precision," PaSa "37.78% recall," SurveyG "better organization." Includes a summary table with: Claim, Paper, Evidence Type, Gap. Identifies three structural patterns: benchmark proliferation enables inflated claims (with exception noted for PaperQA/PaperQA2 sharing LitQA/LitQA2), retrieval and generation claims are decoupled, and evaluation rubrics measure surface quality only.

## s5.2: Methodological Weaknesses Across All Phases
Diagnoses 6 cross-cutting weaknesses: (1) no shared benchmark (11+ incommensurable benchmarks), (2) non-standardized human evaluation, (3) unvalidated LLM-as-judge, (4) missing ablation studies, (5) unaudited citation hallucination rates, and (6) incomparable computational cost reporting. For each weakness, provides specific paper examples and explains consequences. Shows that these weaknesses reinforce each other: the lack of a shared benchmark enables non-standardized evaluation, which makes LLM-as-judge an attractive shortcut, which eliminates incentives for ablations.

## s5.3: The Evaluation Comparability Crisis
Maps all 11 evaluation benchmarks (LitQA, LitQA2, ScholarQABench, SurveyScope, Survey-Arena, SurveyBench, SurGE, ReportBench, DeepSurvey-Bench, SurveyLens, SGSimEval) with task type, metric, size, and protocol. Demonstrates incommensurability: no mathematical framework exists to compare results across benchmarks. Proposes a resolution path: shared benchmark, standardized rubric, automatic metrics benchmarked against human judgment, and leaderboard.

## s5.4: Blind Spots — What the Field Is Not Looking At
Identifies 5 blind spots: (1) critical-analytic depth — no system measures whether surveys provide original analysis; (2) citation hallucination rates — no systematic audit exists; (3) domain transferability — almost all systems evaluated on CS/AI only; (4) temporal recency bias — automated systems may amplify the 62% <5-year citation pattern; (5) prestige/status bias (Matthew Effect) — graph traversal preferentially discovers well-cited papers. Shows that these blind spots are mutually reinforcing.

## s5.5: The Root Cause — You Cannot Optimize for What You Do Not Measure
Argues the evaluation comparability crisis and critical-analytic blind spot are causally linked. Because no benchmark measures critical-analytic depth, the field optimizes for what it can measure. This is an epistemic gap: the field has not defined what "good" means for automated surveys. Proposes that defining and measuring critical-analytic depth is the single most important missing capability. Standardized cost reporting is the second. Together they would transform field incentives.

## s6.prio: Prioritization of Proposals
Brief prioritization table comparing four proposals on impact (H/M/L) and feasibility (H/M/L). Rows: deep graph-LLM integration (§6.1), learned traversal (§6.2), reimagined evaluation (§6.3), community benchmarking (§6.4). Key insight: reimagined evaluation has highest impact but lowest feasibility; learned traversal has highest feasibility. Recommends parallel pursuit with evaluation framework as critical enabler.

## s6.1: Deep Graph-LLM Integration — Towards a Unified Architecture
Sketches a unified architecture combining LitFM (retrieval backbone), SurveyG (organizational scaffold), PaSa (learned traversal), SciSage (multi-agent coordination), and CiteGuard (citation validation). Explains how each component would interact. Includes a cost-quality trade-off table estimating each component's computational cost and quality impact. Identifies three research challenges: unified RL policy for survey quality, computational tractability, and temporal dynamics.

## s6.2: Learned Traversal Policies for Hierarchical Graphs
Proposes extending PaSa's RL policy to hierarchical graph traversal with actions: horizontal expand, vertical ascend/descend, layer-switch, and stop. Reward function balances recall with survey-relevant objectives (coverage of foundations, diversity, recency-appropriateness). Domain-dependent stopping criteria would learn per-section-type thresholds. The key extension is aligning traversal objectives with survey quality rather than retrieval recall.

## s6.3: A Reimagined Evaluation Framework
Proposes 5 evaluation dimensions beyond surface quality: (1) critical-analytic depth (contradiction identification, gap analysis, novel synthesis, methodological critique), (2) bias awareness (temporal coverage, citation concentration, venue/author diversity), (3) field-situatedness (accuracy of settled vs. contested question identification), (4) citation hallucination audit (CiteGuard-style verification), and (5) standardized cost reporting (token budget, API calls, compute hours). For each dimension: definition, measurement protocol, and validation strategy.

## s6.4: The Path to Community-Wide Benchmarking
Calls for community-wide benchmarking modeled on GLUE/SuperGLUE. Proposes: adopt SurveyLens's discipline-aware design as foundation, standardize human evaluation rubric (extending SurGE and DeepSurvey-Bench), establish automatic metric baselines benchmarked against human judgment, require standardized cost reporting, create a per-dimension leaderboard, and hold annual shared tasks. Argues the first step is social: agreeing that critical-analytic depth is a core dimension worth measuring.

## s7: Conclusion
Expanded conclusion (400 words). Recaps the evolution arc from graphs to semantics back to graphs. Synthesizes four narrative threads and their causal interactions: semantic-structural tension (Thread 1), evaluation comparability crisis (Thread 2), bottleneck transfer problem (Thread 3), critical-analytic blind spot (Thread 4). Shows Threads 2 and 4 are causally linked. Presents concrete 2-year vision: shared leaderboard with cost reporting, graph-aware multi-agent system for gap identification, rubrics distinguishing surface quality from critical-analytic depth. Concludes that the next breakthrough will come from measurement infrastructure, not architectural innovation.
