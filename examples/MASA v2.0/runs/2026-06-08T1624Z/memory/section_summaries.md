# Section Summaries

## s1 — Introduction and Scope
Introduces ASG as a rapidly growing field (2024–2026) with four research questions: architectural evolution, evidence support, evaluation state, and blind spots. Three contributions: first systematic audit of the evidence gap, documentation of the evaluation comparability crisis, and identification of five unaddressed blind spots. Roadmap of 7 sections.

## s2 — Evolution Overview
Five overlapping phases from pre-LLM infrastructure through single-agent, multi-agent, graph-aware, and benchmarking eras. Highlights the cross-phase pattern of metric dispersion.

## s2.3 — Phase 2: Multi-Agent Explosion (Late 2024–2025)
Describes six multi-agent systems: Agentic AutoSurvey (8.18/10 vs 4.77/10 — the only controlled comparison), SciSage (reflect-while-writing, +32% Citation F1), MATC (5-agent hierarchical with error-mitigation taskforces), KMCA (mixture-of-experts with minigraphs), AutoSurvey2 (parallel sections), InsightAgent (human-centered). Notes the uncontrolled complexity this phase added.

## s2.4 — Phase 3: Graph Awareness and Iterative Refinement (2025)
Describes two parallel developments: SurveyG's three-layer hierarchical citation graph (Foundation/Development/Frontier) as the only system where graph drives both retrieval and organization, and iterative refinement (SurveyGen-I's coarse-to-fine, IterSurvey's recurrent-outline). Notes the shared absence of convergence criteria.

## s2.5 — Phase 4: Evaluation Maturation (2025–2026)
Five dedicated ASG benchmarks: SurveyBench (11,343 topics, quiz-driven), SurveyEval (7 subjects, 3 dimensions), SurGE (1M papers, 4 dimensions), SurveyLens (10 disciplines, discipline-aware), DeepSurvey-Bench (academic value). Notes the irony that no Phase 1–3 system has been evaluated on any of them.

## s2.1 — Phase 0: Pre-LLM Foundations (2012–2023)
Established citation graph infrastructure (Semantic Scholar), cascading citation expansion, hybrid search strategies, PageRank influence propagation, and agent reasoning frameworks (ReAct, LATS). None could generate text — providing the gap that Phase 1 addressed.

## s2.2 — Phase 1: Single-Agent Pipeline (2024–Early 2025)
Describes five single-agent systems establishing the Outline→Retrieve→Draft→Refine template. AutoSurvey (4.77/10 baseline), SurveyX (AttributeTree), SurveyGen (quality-aware retrieval), SurveyForge (first citation chaining), Meow (outline refinement). Highlights the cognitive bottleneck driving transition to Phase 2.

## s3 — Core Architectures Overview
Three architectural paradigms — single-agent pipelines, multi-agent systems, iterative refinement — with a systematic audit showing the only controlled comparison is Agentic AutoSurvey vs. AutoSurvey.

## s3.1 — Single-Agent Pipelines
Comparison table of 5 systems (AutoSurvey, SurveyX, SurveyGen, SurveyForge, Meow) across planning mechanism, retrieval, iteration, and graph awareness. Each targets a different bottleneck but all share the single-agent cognitive ceiling (4.77/10 baseline).

## s3.2 — Multi-Agent Pipelines
Comparison table of 6 systems (Agentic AutoSurvey, SciSage, MATC, KMCA, AutoSurvey2, InsightAgent) across agent count, coordination, graph awareness, error handling. The 8.18/4.77 comparison is the field's strongest evidence. Notes that none achieve hierarchical graph awareness.

## s3.3 — Iterative Refinement
Comparison of SurveyGen-I (coarse-to-fine retrieval, memory-guided) and IterSurvey (recurrent-outline, self-evaluation). Both lack convergence criteria — the iteration loop is unbounded without a stopping condition.

## s3.4 — The Controlled Comparison Gap
Systematic audit shows only one controlled pair exists (Agentic AutoSurvey vs. AutoSurvey). Analyzes what this single comparison cannot tell us: whether all multi-agent systems outperform single-agent, whether agent count correlates with quality, whether coordination pattern matters. The broader comparison vacuum extends to graph-aware and iterative systems.

## s4 — Graph Awareness Overview
Only 5 of 35 core method papers use citation graph structure. Design spectrum from bolt-on (citation chaining as supplementary retrieval) to backbone (graph determines both retrieval and organization).

## s4.1 — Citation Chaining as Retrieval Strategy
Compares SciSage, SurveyForge, and MATC — three systems using bfs citation chaining within broader pipelines. None evaluates the graph component in isolation. Graph-as-bolt-on: chaining adds retrieval depth but does not affect survey structure.

## s4.2 — SurveyG — Hierarchical Graph as Foundation
Deep-dive on SurveyG's three-layer hierarchical graph (Foundation/Development/Frontier) with horizontal + vertical traversal. The only system where graph structure maps directly to survey outline. Critically assesses the absence of an ablation study.

## s4.3 — Missed Opportunity — Learned Graph Representations
Contrasts LitFM's structure-aware model with GNN-based citation methods (Temporal GNN, H2CGL, Context-Aware Citation Rec) that have zero adoption in ASG. The field chose embedding+keyword hybrid over learned graph models.

## s5 — Critical Assessment Overview
Systematic audit of claims, methodological weaknesses, evaluation comparability crisis, and blind spots.

## s5.1 — Claim vs. Evidence Audit
Formal table auditing 6 major claims: multi-agent superiority (single data point), +32% Citation F1 (narrow, unablated), +27.2% quality (conflates speed with rigor), hierarchical graph (unablated), SurveyBench (limited paradigm), quality-aware retrieval (no controlled comparison). Each claim's required methodological improvement is specified.

## s5.2 — Methodological Weaknesses
Five cross-phase weaknesses: (1) custom evaluation universal, (2) no ablation studies, (3) human evaluation unreproducible, (4) no convergence criteria, (5) graph evaluation not isolated. Concrete examples from specific papers for each.

## s5.3 — Evaluation Comparability Crisis
Dispersion table showing every system's benchmark — only one pair (Agentic AutoSurvey vs. AutoSurvey) shares evaluation. Consequences: cannot rank, cannot attribute, cannot replicate, claims are unfalsifiable. Phase 4 benchmarks exist but no Phase 1–3 system has used them.

## s5.4 — Blind Spots
Five blind spots: (1) citation hallucination unmeasured despite available tools, (2) no evaluation of insight/novelty, (3) human ground truth idealized, (4) computational cost opaque, (5) no cross-lingual/cross-domain evaluation.

## s6 — Future Directions Overview
Six directions grounded in gaps from Sections 3–5: graph integration, convergence criteria, standardized evaluation, cost reporting, hallucination auditing, ablation culture.

## s6.1 — First-Class Citation Graph Integration
Proposes GNN-based graph integration combining SurveyG's hierarchical approach with LitFM's learned representations. Success criteria: ablation studies and benchmark evaluation.

## s6.2 — Convergence-Guaranteed Iterative Refinement
Proposes multi-objective optimization (coverage vs. redundancy, citation support vs. hallucination, insight vs. verbosity) with gradient-free optimization for principled stopping.

## s6.3 — Standardized Evaluation and Ablation Culture
Calls for community convergence on SurveyBench/SurGE/SurveyLens, mandatory cost and hallucination reporting, and ablation studies for every architectural claim.

## s7 — Conclusion
Summarizes three narrative threads. Architecture has outpaced evidence. The path forward requires reorientation from architectural exploration to diagnostic science: shared benchmarks, ablation studies, hallucination auditing, convergence criteria, and cost reporting.
