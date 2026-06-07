# Survey Outline: Automated Literature Survey Agents with Citation Graph Expansion

## Narrative Threads

### Thread 1: The Semantic–Structural Tension
Every survey generation system faces a fundamental choice: build on *what papers say* (semantic embeddings, full-text retrieval) or *where papers sit* (citation graph position, network topology). Phase 1 (2015–2020) chose structure with GNNs and graph embeddings. Phases 2–3 (2023–2025) abandoned it for pure semantic retrieval, achieving impressive results but losing the complementary signals that graphs encode—intellectual lineage, community structure, role differentiation (foundational vs. frontier). Phase 5 (2025–2026) is rediscovering structure, but integration remains shallow: no system uses graphs simultaneously for retrieval, organization, validation, and narrative tracing. This thread runs through all sections, culminating in the argument that reunifying semantic and structural signals is the field's central unsolved design problem.

### Thread 2: The Evaluation Comparability Crisis
Progress is unmeasurable because nearly every system invents its own benchmark (LitQA, ScholarQABench, SurveyScope, SurveyBench, SurGE, Survey-Arena, ReportBench, DeepSurvey-Bench, SurveyLens, SGSimEval, SurveyEval—at least 11 distinct benchmarks with zero shared rubrics, metrics, or topics). Claims like "superhuman," "human-competitive," and "dramatically better" rest on evaluation protocols that are incommensurable across papers. This fragmentation infects every claim in the literature: a +32% citation F1 on SurveyScope cannot be compared to an 8.18/10 quality score on a custom rubric. This thread connects the field's inability to track genuine progress with its proliferation of unsubstantiated claims.

### Thread 3: The Bottleneck Transfer Problem
Better retrieval does not guarantee better surveys. PaSa's RL-optimized search achieves +37.78% recall@20. LitFM's structure-aware pretraining achieves +28.1% precision. Both are impressive on their respective retrieval benchmarks. But no study tests whether these gains transfer to survey quality. Conversely, multi-agent architectures (SciSage, Agentic AutoSurvey) improve citation F1 and quality scores, but they operate on whatever the retrieval agent returns—if the retrieval is biased or incomplete, no amount of agent coordination can fix it. The field operates with an untested assumption: that improving retrieval components linearly improves survey outcomes. A parallel cost dimension compounds this: massive datastores (OpenScholar: 45M papers) and multi-agent coordination incur computational costs that are rarely compared across systems, making efficiency–quality trade-offs invisible.

### Thread 4: The Critical-Analytic Blind Spot
Every evaluation measures surface quality: coherence, coverage, citation accuracy, structural organization. No system or benchmark measures whether a generated survey provides *original analysis*—identifying contradictions in the literature, proposing new taxonomies, critiquing methodological weaknesses, or suggesting future research directions. These are precisely the functions that justify surveys as a scholarly genre, distinguishing them from annotated bibliographies or literature summaries. The field has optimized for "looks like a survey" while ignoring "provides the scholarly value of a survey." This blind spot is the most consequential gap in the literature and motivates the future directions we propose.

---

## Section 1: Introduction and Scope

- **Narrative Arc**: Motivates the survey, defines scope (automated literature survey generation with citation graph expansion), positions the work relative to existing surveys, states contributions, and provides roadmap.
- **Refinement Guidelines**:
  1. Open with the explosion of scientific publishing (2M+ papers/year [source: STM Global Brief 2023 or equivalent]) and the resulting need for automated survey generation
  2. Define the dual challenge: (a) retrieving the right papers from a vast and growing corpus, and (b) synthesizing them into a coherent, critical narrative
  3. **Position relative to existing surveys**: Distinguish this survey from (a) "When LLMs Meet Citation" (2309.09727) which reviews LLM-for-citation bidirectionally, (b) the citation recommendation survey (2002.06961), and (c) "Emergence of LLM as a Tool in Literature Reviews" (2409.04600). This survey is unique in: (i) its focus on citation graph expansion as a specific retrieval strategy, (ii) its critical assessment of unsubstantiated claims, and (iii) its five-phase evolution narrative tracing the semantic–structural tension.
  4. **Add cross-domain context paragraph**: Position this survey relative to established scientometric frameworks (PRISMA for systematic review quality assurance), text evaluation metrics (SummEval for summarization quality), and scientometric citation analysis (citation normalization, field classification). Connect: PRISMA provides screening methodology that automated systems should but do not adopt; SummEval provides multi-dimensional summarization evaluation that survey evaluation benchmarks could adapt; scientometrics provides citation bias and normalization methods that graph-aware systems ignore.
  5. State the three contributions: (i) a critical taxonomy of 135+ papers across 6 architectural paradigms, (ii) a narrative evolution tracing the semantic–structural tension across five phases, and (iii) a rigorous assessment of unsubstantiated claims, evaluation comparability crisis, and methodological blind spots
  6. Provide a roadmap enumerating each section's purpose
- **Reference Papers**: [2309.09727, 2002.06961, 2409.04600, 2004.05904, 2203.17239, 1501.05462]
  - Added 2004.05904 (clustering comparison), 2203.17239 (citation bias), 1501.05462 (scientometrics survey) for cross-domain positioning

## Section 2: The Evolution Arc — Five Phases of Automated Survey Generation

- **Narrative Arc**: Traces the complete chronological arc from citation graph foundations (Phase 1, 2015–2020) through single-agent pipelines (Phase 2, 2023–2024), multi-agent architectures (Phase 3, 2024–2025), iterative/RL systems (Phase 4, 2025), and graph re-integration (Phase 5, 2025–2026). Each phase subsection ends with a quantitative trend table and a critical transition: what the phase promised but did not deliver.
- **Refinement Guidelines**:
  1. Organize chronologically as 5 subsections (2.1–2.5), one per phase
  2. Each subsection must include: (a) the dominant technical approach, (b) key innovations with citations, (c) a quantitative performance summary table, (d) what the phase genuinely achieved, (e) a critical "unfulfilled claims" paragraph, and (f) the limitation that drove the transition to the next phase
  3. **Cross-phase comparison table (CRITICAL — required at end of Section 2)**: Include a comprehensive table with columns: phase name, time period, representative systems, graph awareness level, iteration strategy, claimed performance metric + value, evaluation benchmark used, computational cost profile (datastore size, agent count, API calls per survey if available), and number of papers in the pool from that phase. This table is the single most important comparative artifact in the survey. It must be placed after subsection 2.5 and before Section 3.
  4. **Metric consolidation**: First mention of each core metric (SciSage +32%, LitFM +28.1%, PaSa +37.78%, Agentic AutoSurvey 8.18/10) goes here in §2.3–2.5. Later sections (3, 4, 5) should cross-reference these values rather than re-stating them in full. Only §3.4 (Bottleneck Transfer) may synthesize them in a dedicated table.
  5. Weave Threads 1 and 2 into the phase transitions: show how the semantic–structural tension drove each transition and how evaluation differences make cross-phase comparison impossible
- **Reference Papers**: [phase boundary papers: 1805.02262, 2004.07180, 2406.10252, 2312.07559, 2506.12689, 2509.18661, 2501.10120, 2510.07733, 2410.03761]

### Subsection 2.1: Citation Graph Foundations (2015–2020)
- Key papers: Semantic Scholar Literature Graph (1805.02262), SPECTER (2004.07180), LitFM (2409.12177), HiGTL (2410.03761)
- Develops Thread 1: establishes what structural signals Phase 1 captured and why they could not synthesize

### Subsection 2.2: Single-Agent Survey Pipelines (2023–Early 2024)
- Key papers: AutoSurvey (2406.10252), PaperQA (2312.07559), PaperQA2 (2409.13740), STORM (2402.14207), OpenScholar (2411.14199)
- Develops Threads 1 (abandoning graphs for semantics) and 2 (nearly every system uses a different benchmark)
- **Fix vague attribution**: Avoid phrasing like "claims that would later attract scrutiny" without citation. Replace with specific citations and concrete descriptions (e.g., "AutoSurvey's 'competitive with human-written' claim [2406.10252] was later critiqued by [2508.15658, 2601.15307] for conflating structural coherence with critical depth")

### Subsection 2.3: Multi-Agent Architectures (Late 2024–2025)
- Key papers: SciSage (2506.12689), Agentic AutoSurvey (2509.18661), MATC (2508.04306), InsightAgent (2504.14822)
- Develops Thread 3 (multi-agent coordination cannot fix incomplete retrieval) and Thread 2 (baselines not standardized)

### Subsection 2.4: Iterative and Reinforcement-Learning-Guided Systems (2025)
- Key papers: PaSa (2501.10120), IterSurvey (2510.21900), AURA (2510.27126), SurveyGen-I (2508.14317)
- Develops Thread 3 (bottleneck transfer: optimizing retrieval ≠ better surveys)

### Subsection 2.5: Citation Graph Re-integration (Current Frontier, 2025–2026)
- Key papers: SurveyG (2510.07733), Graphs of Research (2605.14790), Science Hierarchography (2504.13834), LitFM (2409.12177)
- Develops Thread 1 (rediscovering structure) and sets up gaps for Section 5

## Section 3: Architectural Deep Dive — Graph-Aware Retrieval vs. Pipeline-Based Generation

- **Narrative Arc**: This section is the analytical heart of the survey. It places three architectural paradigms—graph-enhanced retrieval, single-agent pipelines, and multi-agent architectures—in direct critical comparison. The organizing question is not "which is better?" but "what structural signals does each paradigm capture, what does it miss, and do current integration attempts (e.g., SurveyG's hierarchical graph, SciSage's BFS chaining) genuinely bridge the gap?" The section develops Threads 1 and 3 through systematic comparison tables and mechanism-level analysis, and introduces a computational cost dimension that is rarely discussed but practically decisive.
- **Refinement Guidelines**:
  1. Organize into 3 subsections (3.1–3.3), one per paradigm—but the section should interleave comparisons rather than presenting them in isolation
  2. For each paradigm: explain the mechanism (not just the purpose—HOW does SurveyG's horizontal traversal work? HOW does SciSage's reflect-while-writing prevent error accumulation?), present a comparison table across 6–8 dimensions including computational cost (datastore size, API calls per survey, token budget), include pros/cons, and cite quantitative results
  3. **Metric handling (avoid redundancy)**: This section should cross-reference metrics introduced in §2 rather than re-stating them in full. Exception: §3.4's bottleneck transfer table may synthesize metrics from multiple sources.
  4. Add a Subsection 3.4 that directly confronts the bottleneck transfer problem (Thread 3): present a table showing retrieval performance vs. survey quality for every system that reports both (none do—make this gap explicit)
  5. **Baseline absolute values in §3.4 table**: For each system, include not just the improvement metric (e.g., +32%, +37.78%) but also the baseline absolute value (e.g., single-agent baseline citation F1, GPT-4o recall@20, AutoSurvey 4.77/10). This provides scale context: a +32% improvement from a 50% baseline is different from a +32% improvement from a 10% baseline.
  6. Include a critical analysis paragraph for each paradigm examining whether the "performance improvement" claims survive scrutiny: e.g., Agentic AutoSurvey's 8.18 vs 4.77—is this architecture or confounds?
  7. Use Thread 1 throughout: contrast graph-aware approaches (what structural signals they encode) with pipeline approaches (what they miss by ignoring graphs)
- **Reference Papers**: [cross-paradigm comparison: 2510.07733, 2409.12177, 2406.10252, 2506.12689, 2509.18661, 2409.13740]

### Subsection 3.1: Graph-Enhanced Retrieval — Structure as Signal
- Deep-dive: LitFM graph transformer mechanism, SurveyG hierarchical traversal, SPECTER citation-informed embeddings, HiGTL taxonomy induction
- Key question: What does graph structure encode that text cannot?
- Computational cost: GNN training overhead vs. inference-only embedding lookup

### Subsection 3.2: Single-Agent Pipelines — Semantic Retrieval at Scale
- Deep-dive: AutoSurvey's outline→retrieve→draft→refine pipeline, PaperQA2's contradiction detection, OpenScholar's 45M-paper datastore
- Key question: Does massive datastore scale compensate for graph blindness?
- Computational cost: OpenScholar's 45M paper storage vs. lightweight alternatives

### Subsection 3.3: Multi-Agent Architectures — Division of Labor, Amplification of Gaps
- Deep-dive: SciSage's reflect-while-writing mechanism, Agentic AutoSurvey's shared task board, MATC's error-mitigation taskforces
- Key question: Does multi-agent coordination improve quality, or just redistribute the same limitations across more agents?
- Computational cost: N agents × per-agent API calls; token overhead of inter-agent communication

### Subsection 3.4: The Bottleneck Transfer Problem — Retrieval Gains ≠ Survey Gains
- Comparative analysis: PaSa's +37.78% recall (baseline: GPT-4o recall@20) vs. SciSage's +32% citation F1 (baseline: single-agent citation F1) vs. LitFM's +28.1% precision (baseline: text-only embedding precision)
- Key argument: These metrics measure different constructs and may not transfer; the field needs end-to-end evaluations linking retrieval improvements to survey quality
- Introduces cost–efficiency trade-off: expensive retrieval gains may not justify marginal survey quality improvements

## Section 4: Beyond Pipeline Design — Interaction, Iteration, and Learning

- **Narrative Arc**: Pipeline architecture is not the only axis of variation. This section examines three alternative strategies for improving survey quality: human-in-the-loop interaction (Section 4.1), procedural iterative refinement (Section 4.2), and reinforcement learning for search/generation policies (Section 4.3). These strategies cut across architectural categories and raise a deeper question: should the field focus on designing better static pipelines, or on learning better dynamic behaviors? The section develops Thread 3 (bottleneck transfer) by asking whether RL-optimized search policies address the right objective, and Thread 4 (critical-analytic blind spot) by showing how current iteration objectives all optimize surface quality. **A synthesis subsection (4.4) directly compares the three approaches on scalability, quality ceiling, and cost per survey.**
- **Refinement Guidelines**:
  1. Organize into 4 subsections (4.1–4.4)
  2. For each approach, include a comparison table across approaches: interaction mode, scalability, quality improvement metric, citation accuracy, time cost, human effort required
  3. Connect each approach to the core tension: does interaction/iteration/RL actually address the semantic–structural gap, or does it optimize within existing paradigms?
  4. Include critical analysis: InsightAgent's 27.2% quality improvement is impressive but requires human orchestrator—does it scale? PaSa's RL policy is paradigm-shifting but optimizes for recall, not critical synthesis
  5. **Subsection 4.4 (NEW)**: A cross-approach synthesis comparing HITL, procedural iteration, and RL on three axes: scalability (papers per unit time), quality ceiling (maximum achievable quality), and cost per survey (token budget, human hours, API calls). The goal is to identify which approach is best suited for which deployment scenario, and to show that none address the critical-analytic blind spot.
- **Reference Papers**: [cross-cutting: 2504.14822, 2510.21900, 2501.10120, 2510.27126, 2006.12166, 2505.23789]

### Subsection 4.1: Human-in-the-Loop Systems — Quality Through Oversight
- Key papers: InsightAgent, ASReview, LitChat, FAST², AiReview
- Mechanism-level: How InsightAgent's human orchestrator interacts with 6 agents; ASReview's active learning stopping criterion
- Cost dimension: Human effort hours vs. quality improvement

### Subsection 4.2: Procedural Iterative Refinement — Self-Evaluation Loops
- Key papers: IterSurvey, SurveyGen-I, SurveyGen
- Mechanism-level: How recurrent outline generation adapts to content; self-evaluation convergence properties
- Risk: LLM self-evaluation overconfidence → locally optimal but globally flawed surveys

### Subsection 4.3: Reinforcement Learning for Search and Generation Policies
- Key papers: PaSa, AURA, Text2Grad, RL4F
- Mechanism-level: PaSa's RL-optimized search policy (epsilon-greedy exploration over citation-follow vs. keyword-refine vs. author-search); AURA's adaptive questioning for information gain
- Develops Thread 3 (RL optimizes the wrong objective for survey quality) and Thread 4 (no RL objective captures critical-analytic depth)

### Subsection 4.4 (NEW): Cross-Approach Synthesis — Scalability, Quality Ceiling, and Cost
- Compare three approaches on: scalability (papers/hour), quality ceiling (maximum reportable score), cost per survey (human hours, API calls, token budget), citation accuracy, and graph awareness
- Show that HITL achieves highest quality (InsightAgent: +27.2%) but worst scalability; RL achieves best scalability but optimizes narrow objectives; procedural iteration sits in between
- Key gap: No approach measures or optimizes critical-analytic depth—all three optimize for surface quality dimensions that existing benchmarks capture
- Reference papers: 2504.14822, 2510.21900, 2501.10120, 2510.27126, 2504.04193

## Section 5: Critical Assessment — Claims, Gaps, and Blind Spots

- **Narrative Arc**: This is the analytical core of the survey. It systematically evaluates the field's claims against available evidence, diagnoses the methodological weaknesses that make progress untrackable, and identifies the blind spots that the field is collectively ignoring. The section does not merely catalog problems—it argues that the evaluation comparability crisis (Thread 2) and the critical-analytic blind spot (Thread 4) are causally linked: because no benchmark measures critical-analytic depth, the field optimizes for what it can measure (coherence, coverage, citation accuracy) and has no incentive to address what it cannot measure.
- **Refinement Guidelines**:
  1. Present the Claim vs. Evidence Gap table from the evolution narrative, expanded with additional cross-references
  2. Diagnose the 5 methodological weaknesses (no shared benchmark, non-standardized human eval, unvalidated LLM-as-judge, missing ablations, unaudited citation hallucination) — add a 6th: **incomparable computational cost reporting** (no system reports token budgets, API costs, or total compute in a standardized way, making practical comparisons impossible)
  3. Dedicate Subsection 5.3 to the evaluation comparability crisis — enumerate all 11+ benchmarks, map their task types, metrics, and evaluation protocols; show that they are incommensurable
  4. Subsection 5.4: Blind spots — critical-analytic depth, citation hallucination rates, domain transferability, temporal recency bias, prestige/status bias
  5. Subsection 5.5: The root cause argument — connect Threads 2 and 4: the field cannot address what it cannot measure, and it cannot measure what it does not define
  6. **Metric discipline**: §5.1's claim-evidence table should reference metrics by their first-introduction location in §2, not re-state full metric details. E.g., "SciSage's +32% citation F1 improvement (§2.3)" rather than re-stating the full benchmark context.
- **Reference Papers**: [cross-phase relevant: 2301.13298, 2508.11310, 2601.15307, 2508.15658, 2602.11238, 2510.03120, 2512.02763, 2305.18554, 2402.12046, 2401.03545, 2411.05584, 2508.12735]

### Subsection 5.1: Claim vs. Evidence Gap Analysis
- Systematic evaluation of 7 major claims from the literature (AutoSurvey "human-competitive", PaperQA2 "superhuman", OpenScholar "8B beats GPT-4o", multi-agent "dramatically better", graph-aware "28.1% precision", PaSa "37.78% recall", SurveyG "better organization")
- **Use "nearly every" language**: "Nearly every system evaluates on its own benchmark" (paperQA/PaperQA2 share LitQA/LitQA2, so "every" is inaccurate)

### Subsection 5.2: Methodological Weaknesses Across All Phases
- 6 cross-cutting weaknesses with specific paper examples: no shared benchmark, non-standardized human eval, unvalidated LLM-as-judge, missing ablation studies, unaudited citation hallucination rates, incomparable computational cost reporting

### Subsection 5.3: The Evaluation Comparability Crisis
- Map of 11+ benchmarks (LitQA, LitQA2, ScholarQABench, SurveyScope, Survey-Arena, SurveyBench, SurGE, ReportBench, DeepSurvey-Bench, SurveyLens, SGSimEval, SurveyEval); demonstration of incommensurability; proposed resolution path (shared benchmark, standardized rubric, leaderboard)

### Subsection 5.4: Blind Spots — What the Field Is Not Looking At
- Critical-analytic depth, citation hallucination rates, domain transferability, temporal recency bias, prestige/status bias (Matthew Effect)

### Subsection 5.5: The Root Cause — You Cannot Optimize for What You Do Not Measure
- Argues that the evaluation crisis and the critical-analytic blind spot are causally linked; proposes defining and measuring critical-analytic depth as the single most important missing capability

## Section 6: Future Directions — Integrating Graph, Multi-Agent, and Learned Policies

- **Narrative Arc**: The field has all the ingredients for a next-generation system—graph-aware embeddings (LitFM), hierarchical traversal (SurveyG), multi-agent coordination (SciSage, Agentic AutoSurvey), learned search policies (PaSa), and human-in-the-loop refinement (InsightAgent)—but no system combines them. This section sketches the architecture of a genuinely integrated system and identifies the research challenges that stand in the way. It also proposes a reimagined evaluation framework that measures what matters (critical-analytic depth, bias awareness, field-situatedness) rather than what is easy to measure.
- **Refinement Guidelines**:
  1. Organize into 3-4 forward-looking proposals, each grounded in a gap from Section 5
  2. Do not invent speculative technologies—every proposal should cite existing work that partially addresses the gap
  3. **Add prioritization comparison**: After the introduction of §6, include a brief comparison of the four proposals on two axes: **impact** (how much would this solve? rated H/M/L) and **feasibility** (how hard is it to build? rated H/M/L). This helps readers understand trade-offs and which direction to invest in first.
  4. Proposal 1: Deep graph-LLM integration (combine LitFM, SurveyG, PaSa, SciSage into a unified architecture — explain how each component would interact, with cost–efficiency estimates)
  5. Proposal 2: Learned traversal with stopping criteria (extend PaSa's RL policy to hierarchical graph traversal; reference Temporal GNN 2408.15371 for dynamic graphs)
  6. Proposal 3: Multi-dimensional evaluation (critical-analytic depth, bias awareness, field-situatedness, citation hallucination audit, standardized cost reporting)
  7. End with a call for a community-wide evaluation framework (shared benchmark, standardized rubric, leaderboard)
  8. Weave all 4 threads into the conclusion, showing how each would be resolved in a next-generation system
- **Reference Papers**: [future-looking: 2605.14790, 2512.16661, 2408.15371, 2504.13834, 2602.11238, 2508.11310, 2601.15307, 2510.17853]

### Subsection 6.1: Deep Graph-LLM Integration — Towards a Unified Architecture
- Sketch combining LitFM (retrieval backbone), SurveyG (organizational scaffold), PaSa (learned traversal), SciSage (multi-agent coordination), and CiteGuard (citation validation)
- Discuss cost–quality trade-offs: what does adding each component cost?

### Subsection 6.2: Learned Traversal Policies for Hierarchical Graphs
- Extending PaSa-style RL to hierarchical graph traversal with domain-dependent stopping criteria

### Subsection 6.3: A Reimagined Evaluation Framework
- Beyond surface quality: critical-analytic depth, bias awareness, field-situatedness, citation hallucination audit, standardized cost reporting

### Subsection 6.4: The Path to Community-Wide Benchmarking
- Shared benchmark across disciplines, standardized human evaluation rubric, leaderboard-based comparison

## Section 7: Conclusion

- **Narrative Arc**: Summarizes the survey's findings, restates the four narrative threads, and issues a call to action for the community to address the evaluation crisis and the critical-analytic blind spot before further architectural innovation.
- **Refinement Guidelines**:
  1. Briefly recap the evolution arc (Section 2) — 2-3 sentences
  2. Restate the four narrative threads as the field's central unresolved challenges — 1-2 sentences per thread, synthesizing how they interact
  3. Argue that architectural innovation without evaluation standardization and critical-analytic measurement will continue to produce incommensurable results
  4. **Include a concrete vision**: What would the field look like in 2 years if it addresses these challenges? A shared leaderboard with standardized cost reporting. A graph-aware multi-agent system generating surveys that identify research gaps, not just restructure known content. Evaluation rubrics that distinguish surface quality from critical-analytic depth.
  5. End with a forward-looking statement about the potential of integrated graph-aware multi-agent systems
- **Reference Papers**: [] (no new references — synthesizes existing)
