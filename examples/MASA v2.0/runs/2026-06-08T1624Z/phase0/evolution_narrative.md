# Evolution Narrative — Automated Survey Generation

**Generated**: 2026-06-08
**Agent**: EvolutionMapper
**Input**: phase0/paper_taxonomy.md + 20 paper profiles

---

## Critical Arc

The automated survey generation (ASG) field has undergone four distinct phases in just over two years (2024–2026), progressing from a proof-of-concept single-agent pipeline through a multi-agent explosion, a graph-awareness awakening, and into evaluation maturation. Each phase genuinely advanced the state of the art — the best systems now produce surveys that human evaluators rate 8+/10 — yet the field is held back by three interconnected failures: an evaluation comparability crisis where every paper reports on a different metric+dataset combination; a persistent graph-awareness gap where only 5 of 35 core method papers use citation graph structure for anything beyond keyword search; and a systematic avoidance of convergence criteria, making it impossible to know when quality has plateaued. The field is currently rich in architectural variation but poor in diagnostic evidence, producing systems that are increasingly complex without demonstrable understanding of *why* complexity helps.

---

## Phase 0: Pre-LLM Foundations (2012–2023)

### Technical Approach

Before LLMs made automated *writing* feasible, a pre-existing infrastructure of citation graph analysis, retrieval methodology, and evaluation metrics had already been developed. These papers are not ASG systems themselves, but they provide the building blocks that every subsequent system depends on.

### Key Contributions

- **1205.1143 (Direction Aware Citation Analysis, 2012)**: Direction-aware recommendation on academic networks — formalized how citation directionality affects relevance.
- **1310.8224 (Transitive Reduction of Citation Networks, 2013)**: Showed that citation networks can be simplified by removing transitive edges, a technique later relevant for graph-based retrieval.
- **1407.5107 (PageRank beyond the Web, 2014)**: Extended PageRank to citation analysis, establishing influence propagation as a core ranking paradigm.
- **1805.02262 (Semantic Scholar Literature Graph, 2018; updated 2301.10140, 2023)**: Infrastructure-level construction of the literature graph — the backbone dataset for any citation-aware retrieval.
- **1810.00826 (How Powerful are GNNs?, 2018)**: Foundational GNN expressiveness theory, later employed by Temporal GNN Paper Recommendation (2408.15371) and H2CGL (2305.01572) for citation network modeling.
- **1806.00089 (Cascading Citation Expansion, 2018)**: The technique of cascading forward/backward citation expansion — directly used by SciSage and SurveyForge for citation chaining.
- **1904.07579 (Go Wide, Go Deep, 2019)**: Influence dispersion trees for quantifying impact — formalized the notion that citation depth matters.
- **2004.09741 (Hybrid Search Strategies for SLRs, 2020)**: Demonstrated that hybrid (embedding + keyword + citation chaining) search outperforms any single strategy for systematic reviews.
- **2210.03629 (ReAct, 2022) and 2310.04406 (LATS, 2023)**: Agent reasoning frameworks (ReAct: reasoning+acting; LATS: tree search for agent planning) that later ASG systems implicitly rely on for multi-step planning.
- **2402.08339 (Interleaved Snowballing, 2024)**: Reduced curator workload by interleaving forward/backward snowballing — a practical methodology directly applicable to ASG retrieval.

### What This Phase Genuinely Achieved

- Established that **hybrid search** (embedding + keyword + citation) outperforms any single retrieval strategy for literature review.
- Built the **infrastructure** (Semantic Scholar Literature Graph, OpenAIRE Citation Graph) that makes citation-aware retrieval possible at scale.
- Developed **GNN-based citation prediction** methods that, while not used by ASG systems yet, demonstrate the feasibility of modeling citation dynamics.
- Formalized **citation expansion strategies** (snowballing, cascading, interleaved) that later ASG systems adopted piecemeal.

### Unfulfilled Claims

- "GNN-based citation recommendation will transform literature discovery" — GNN-based methods (Temporal GNN, H2CGL, GCN Citation Count Prediction) produced strong prediction results but were never integrated into any operational ASG system. The field chose embedding + keyword hybrid retrieval over learned citation graph models.
- "Interleaved snowballing reduces curator workload by X%" — The methodology paper (2402.08339) showed merit but no ASG system has implemented interleaved snowballing; most use simpler forward/backward chaining.

### Limitation That Drove Transition to Phase 1

None of these mechanism papers could *generate text*. They could recommend, rank, predict, and analyze citation networks, but the actual writing of a survey required human effort. The arrival of instruction-tuned LLMs capable of synthesizing text from retrieved papers created the missing generation component, enabling the first end-to-end ASG systems.

---

## Phase 1: The Single-Agent Pipeline (2024 — Early 2025)

### Technical Approach

A single LLM orchestrates a sequential pipeline: plan the survey outline, retrieve relevant papers using embedding similarity, draft each section, and optionally refine post-hoc. The entire cognitive load — planning, retrieval, writing, quality assessment — is borne by one model instance. The key architectural insight is that survey generation can be decomposed into stages, each prompted separately, rather than attempted in one shot.

### Key Contributions

- **2406.10252 (AutoSurvey, 2024)**: **Foundational seed paper**. Introduced the Outline → Retrieve → Draft → Refine pipeline that defined the field's architectural template. Demonstrated that LLMs can produce coherent surveys end-to-end, with post-hoc refinement improving quality. Uses embedding-only retrieval; no citation graph awareness.
- **2502.14776 (SurveyX, 2025)**: Replaced simple outline with **AttributeTree** structured pre-processing — a more granular survey planning mechanism that decomposes the topic into attributes before generating. Hybrid retrieval (embedding + keyword).
- **2508.17647 (SurveyGen, 2025)**: Introduced **quality-aware retrieval** — retrieves papers based on predicted quality contribution rather than just relevance. Trained on 4,200+ human-written surveys. Quality estimation feedback loop for iterative improvement.
- **2503.04629 (SurveyForge, 2025)**: First single-agent system to add **citation graph awareness** via a Scholar Navigation Agent that follows citation trails (bfs citation chaining). Memory-guided outline evolves as new papers are discovered.
- **2509.19370 (Meow, 2025)**: End-to-end outline writing with multi-round outline refinement before section generation. Adds iterative improvement at the outline stage.
- **2410.15978 (PROMPTHEUS, 2024)**: Human-centered pipeline for SLRs, introducing human-in-the-loop at key checkpoints. Bridges Phase 1 and the Hybrid Interactive sub-phase.

### Reported Performance

| Paper | Metric | Value | Dataset |
|-------|--------|-------|---------|
| AutoSurvey | Custom quality rating | Baseline (established benchmark) | Custom 10 topics |
| Agentic AutoSurvey (multi-agent extension) | Survey quality | 8.18/10 | Custom evaluation |
| AutoSurvey (baseline for above) | Survey quality | 4.77/10 | Same custom eval |
| SurveyGen | Quality estimation accuracy | Trained on 4,200+ human surveys | Custom corpus |
| SurveyForge | Citation coverage | Improved via bfs chaining | Custom eval |

### What This Phase Genuinely Achieved

- **Proved feasibility**: LLMs can generate coherent, structured surveys from a set of retrieved papers — a non-trivial capability that was not obvious in 2023.
- **Established the pipeline template**: The decompose-into-stages pattern (outline → retrieve → draft → refine) became the default architecture for the field, adopted (with variations) by every subsequent system.
- **Demonstrated that structure matters**: SurveyX's AttributeTree and Meow's end-to-end outline both showed that better planning produces better surveys.
- **Introduced quality awareness**: SurveyGen's insight that retrieval should be quality-weighted rather than relevance-only was a genuine conceptual advance.

### Unfulfilled Claims

- "LLMs Can Automatically Write Surveys" (AutoSurvey title) — The surveys were coherent but the paper's own evaluation showed they fell significantly short of human-written quality (baseline 4.77/10). The claim was directional, not achieved.
- "Quality-aware retrieval improves survey quality" (SurveyGen) — The quality estimation model was trained on 4,200+ surveys, but no controlled ablation showed whether quality-aware retrieval outperforms standard hybrid retrieval when evaluation methods are held constant.
- "Citation chaining solves coverage problems" (SurveyForge) — The bfs citation chaining demonstrably expands coverage, but without a systematic audit, it's unclear whether it adds relevant or peripheral papers.

### Limitation That Drove Transition to Phase 2

The single-agent pipeline approach hit a ceiling: one model handles planning, retrieval, writing, and quality assessment. This creates an inherent cognitive bottleneck — the model cannot specialize, cannot parallelize, and cannot provide independent quality feedback. AutoSurvey's own evaluation (4.77/10) made the gap to acceptable quality evident. Multi-agent architectures emerged as the natural response: distribute the cognitive load across specialized agents.

---

## Phase 2: Multi-Agent Explosion (Late 2024 — 2025)

### Technical Approach

Multiple specialized LLM agents with distinct roles (Planner, Searcher, Writer, Reviewer, Refiner) coordinate through structured communication to produce surveys. The division of labor enables parallel work, specialized expertise, and independent quality feedback loops. Architectures vary from flat (all agents equal, shared task board) to hierarchical (manager orchestrates specialized taskforces) to mixture-of-experts (multiple parallel analyses synthesized by coordinator).

### Key Contributions

- **2404.17017 (AutoGenesisAgent, 2024)**: Self-generating multi-agent systems for complex tasks — precursor concept that multi-agent architectures can be automatically configured.
- **2411.06159 (KMCA / MPSA, 2024)**: Mixture of Knowledge Minigraph Agents. First system to combine **multi-agent architecture with graph awareness** — multiple KMCA agents each analyze a minigraph subgraph, then a Multi-Perspective Synthesis Agent integrates. Configurable agent count.
- **2506.12689 (SciSage, 2025)**: 4-agent system (Searcher, Writer, Reflector, Refiner) with **reflect-while-writing** — the Writer pauses periodically for real-time reflection, preventing error accumulation. Searcher uses forward/backward citation chaining. Reports +32% Citation F1 improvement on SurveyScope.
- **2508.04306 (MATC, 2025)**: 5-agent hierarchical system (Manager + 4 specialized taskforces: Explorer, Validator, Analyzer, Reviewer). **Error-mitigation architecture** — explicit taskforces for detecting and correcting errors. Uses bfs citation chaining for coverage.
- **2509.18661 (Agentic AutoSurvey, 2025)**: 4-agent system (Planner, Researcher, Writer, Reviewer) with **shared task board coordination**. Reports 8.18/10 quality score vs 4.77/10 for AutoSurvey. Most direct head-to-head comparison in the field.
- **2510.26012 (AutoSurvey2, 2025)**: 5-agent system with **parallel section generation** — Planner → Retriever → Parallel Writers → Consolidator → Refiner. Multi-LLM evaluation for quality assessment.
- **2504.14822 (InsightAgent, 2025)**: 6-agent system (human orchestrator + 5 specialized agents). Reports **+27.2% quality improvement** and reduces timeline from months to 1.5 hours. Bfs citation tracking.
- **2510.10890 (LLM×MapReduce-V3, 2025)**: MCP-driven hierarchical modular system with MapReduce coordination. Configurable agent count. Human-in-the-loop support.
- **2408.06292 (The AI Scientist, 2024)**: Full discovery pipeline including survey generation. Configurable multi-agent system demonstrating that ASG can be embedded in broader autonomous research.

### Reported Performance

| Paper | Metric | Value | Dataset |
|-------|--------|-------|---------|
| Agentic AutoSurvey | Survey quality (1-10) | 8.18/10 | Custom (same as AutoSurvey baseline) |
| AutoSurvey (baseline) | Survey quality (1-10) | 4.77/10 | Same custom evaluation |
| SciSage | Citation F1 | +32% improvement | SurveyScope |
| InsightAgent | Quality improvement | +27.2% | Custom evaluation |
| InsightAgent | Time reduction | Months → 1.5 hours | Custom evaluation |

### What This Phase Genuinely Achieved

- **Demonstrated that multi-agent architectures outperform single-agent baselines**: The only controlled comparison (Agentic AutoSurvey vs. AutoSurvey, 8.18 vs. 4.77) shows a 71% improvement. This is the field's most important quantitative result.
- **Introduced real-time quality feedback**: SciSage's reflect-while-writing prevents error accumulation — a genuine architectural insight that addresses the "garbage in, garbage out" problem of long-form generation.
- **Proved that agent specialization yields measurable gains**: Systems with high specialization (SciSage, Agentic AutoSurvey, MATC) consistently outperform low-specialization pipelines.
- **Established graph-aware multi-agent systems**: KMCA and MATC showed that citation graph traversal can be integrated into multi-agent coordination, not just single-agent retrieval.

### Unfulfilled Claims

- "Multi-agent systems outperform single-agent" — The only controlled comparison is Agentic AutoSurvey vs. AutoSurvey. All other comparisons are across different datasets, metrics, and evaluation protocols. The claim is supported by one strong data point, not a systematic finding.
- "+32% Citation F1 improvement" (SciSage) — The improvement is reported on SurveyScope, which is not a standardized benchmark. It's unclear whether this translates to overall survey quality improvement.
- "Months → 1.5 hours" (InsightAgent) — This conflates automation speed with quality. A human systematic review takes months *because* of depth and rigor; reproducing that depth in 1.5 hours is impressive only if the quality is comparable, which the paper does not rigorously demonstrate.
- "Error-mitigation taskforces solve the hallucination problem" (MATC) — MATC has an explicit error-mitigation architecture but evaluates on custom metrics without a hallucination audit. The claim is structural, not evidenced.

### Limitation That Drove Transition to Phase 3

Multi-agent systems became more complex without becoming more *grounded*. Adding agents increases coordination overhead, prompt engineering complexity, and computational cost — but without citation graph awareness, even the best multi-agent systems retrieve papers using embedding similarity, which has no structural understanding of the research landscape. The field recognized that citation graph structure could provide the missing grounding for both retrieval and survey organization.

---

## Phase 3: Graph Awareness and Refinement (2025)

### Technical Approach

Two parallel developments characterize this phase. First, **citation graph traversal** becomes a first-class retrieval mechanism, moving beyond embedding/keyword/hybrid to include bfs citation chaining and hierarchical graph construction. Second, **iterative refinement** centers the system architecture around the improvement loop itself, treating survey generation as a convergence process rather than a pipeline.

### Key Contributions

**Graph-Enhanced Retrieval:**

- **2510.07733 (SurveyG, 2025)**: **Most architecturally novel paper in the field**. Constructs a three-layer hierarchical citation graph (Foundation/Development/Frontier) and uses both horizontal traversal (within layers) and vertical traversal (between layers). The graph structure directly maps to the survey outline — the first system where the citation graph determines both what to retrieve and how to organize it.
- **2409.12177 (LitFM, 2024)**: Retrieval-augmented structure-aware foundation model for citation graphs. Integrates graph structure into the retrieval model itself, rather than as a post-hoc traversal.
- **2503.04629 (SurveyForge, 2025)**: Memory-guided outline with Scholar Navigation Agent following citation trails (bfs chaining). Bridges single-agent and graph-aware approaches.
- **2506.12689 (SciSage, 2025)**: Citation-aware Searcher agent with forward/backward chaining in a multi-agent context.
- **2508.04306 (MATC, 2025)**: Exploration taskforce using citation chaining for coverage in multi-agent context.

**Iterative Refinement:**

- **2508.14317 (SurveyGen-I, 2025)**: Coarse-to-fine retrieval with adaptive planning and memory-guided writing. The plan evolves as new papers are discovered — the survey's structure is emergent rather than predetermined.
- **2510.21900 (IterSurvey, 2025)**: Recurrent outline with self-evaluation loop. The outline adapts as content is generated, evaluated with Survey-Arena. Pure iteration — no graph awareness, but iteration is the central architectural pattern.

### Reported Performance

| Paper | Metric | Value | Dataset |
|-------|--------|-------|---------|
| SurveyG | Survey structure quality | "Hierarchical graph improves organization" | Custom evaluation |
| SciSage | Citation F1 | +32% improvement | SurveyScope |
| IterSurvey | Evaluation via Survey-Arena | Metric not specified | Survey-Arena |

### What This Phase Genuinely Achieved

- **Demonstrated that graph structure can drive survey organization**: SurveyG is the only system where the citation graph directly determines the survey's outline — a qualitatively different approach from embedding similarity + outline prompt.
- **Introduced hierarchical graph awareness**: The Foundation/Development/Frontier layer model is an intuitive and plausible mapping of how scientific fields evolve.
- **Proved that iteration can be the central pattern**: SurveyGen-I and IterSurvey showed that treating survey generation as a convergence process (rather than a pipeline) enables emergent structure discovery.
- **Created the conceptual bridge between retrieval and organization**: Prior systems treated retrieval (find relevant papers) and organization (structure the survey) as separate stages. SurveyG unifies them through graph structure.

### Unfulfilled Claims

- "Hierarchical graph improves organization" (SurveyG) — This is asserted, not measured against a non-graph baseline. There is no ablation study showing that the graph structure causes better organization rather than being correlated with it.
- "Coarse-to-fine retrieval adapts to discovered content" (SurveyGen-I) — The adaptation mechanism is described but not evaluated for convergence. How many iterations are needed? When does quality plateau? No criteria specified.
- "Self-evaluation loop converges to high quality" (IterSurvey) — Survey-Arena provides an evaluation, but without convergence criteria or stopping rules, the loop is unbounded. The system could iterate forever without guaranteed improvement.

### Limitation That Drove Transition to Phase 4

As systems proliferated — single-agent, multi-agent, graph-aware, iterative — it became impossible to compare them. Each paper used a different evaluation protocol: custom topics, custom metrics, custom human evaluation rubrics. The field needed standardized benchmarks to make claims measurable. This recognition drove the rapid emergence of dedicated ASG evaluation frameworks.

---

## Phase 4: Evaluation Maturation and Benchmarking (2025–2026)

### Technical Approach

Dedicated benchmarks, evaluation frameworks, and datasets specifically designed for automated survey generation. Unlike earlier phases where each system self-evaluated, these benchmarks provide standardized corpora, metrics, and evaluation protocols. Three distinct evaluation paradigms emerge: quiz-based (evaluate via QA on survey content), dimension-rating (rate specific quality dimensions), and discipline-aware (customize evaluation by field).

### Key Contributions

- **2510.03120 (SurveyBench, 2025)**: **Most comprehensive benchmark**. Quiz-driven evaluation using 11,343 arXiv topics and 4,947 human surveys. Measures whether generated surveys contain accurate factual knowledge via QA accuracy.
- **2512.02763 (SurveyEval, 2025)**: Multi-subject evaluation across 7 subjects. Three dimensions: overall quality, outline coherence, reference accuracy.
- **2508.15658 (SurGE, 2025)**: 4-dimension evaluation (coverage, accuracy, structure, readability) using a 1M-paper corpus. Largest evaluation corpus.
- **2602.11238 (SurveyLens, 2026)**: Discipline-aware evaluation with 1,000 human surveys across 10 disciplines. First benchmark to recognize that survey quality standards vary by field.
- **2601.15307 (DeepSurvey-Bench, 2026)**: Academic value evaluation focusing on information value and scholarly communication quality — moves beyond surface metrics.
- **2508.11310 (SGSimEval, 2025)**: Multi-faceted benchmark specifically for ASG systems. Comprehensive but less widely cited.
- **2406.10291 (ResearchArena, 2024)**: 3-stage agent evaluation (paper discovery, selection, organization) with 12M-paper offline environment — evaluates retrieval and organization independently of generation.

### Factuality and Citation Evaluation Tools

- **2305.14251 (FActScore, 2023)**: Atomic fact precision — foundational for verifying survey factual accuracy.
- **2309.12455 (LongDocFACTScore, 2023)**: Long-document factuality — directly applicable to survey-length texts.
- **2406.19276 (VERISCORE, 2024)**: Verifiable claim evaluation — can verify whether survey claims cite appropriate evidence.
- **2403.18802 (SAFE, 2024)**: Long-form factuality evaluation applicable to surveys.
- **2510.17853 (CiteGuard, 2025)**: Faithful citation attribution — can detect whether citations support the claims they accompany.
- **2305.14627 (ALCE, 2023)**: Benchmark for LLM-generated text with citations.
- **2407.12861 (CiteME, 2024)**: Evaluates whether LLMs can accurately cite scientific claims.
- **2411.16638 (Do Auto Metrics Measure Factuality?, 2024)**: **Critical meta-evaluation** — questions whether automatic metrics can reliably measure factuality at all.

### What This Phase Genuinely Achieved

- **Created the infrastructure for apples-to-apples comparison**: For the first time, ASG systems can be evaluated on the same corpus with the same metrics. SurveyBench's 11,343 topics and SurGE's 1M-paper corpus provide sufficient scale.
- **Introduced quiz-based evaluation**: SurveyBench's approach — evaluate survey quality via QA on the survey's content — is a clever proxy for depth of understanding, partially avoiding the surface-form overlap problem of ROUGE/BERTScore.
- **Recognized discipline-specific quality standards**: SurveyLens's discipline-aware evaluation is a genuine advance — survey quality means different things in different fields.
- **Enabled factuality auditing**: The availability of FActScore, VERISCORE, CiteGuard, and CiteME provides the toolchain for systematic factuality evaluation that earlier phases lacked.

### Unfulfilled Claims

- "SurveyBench enables comprehensive evaluation" — With 11,343 topics, SurveyBench is large, but quiz-based evaluation measures factual recall, not survey quality (coherence, readability, insight, novelty). A survey that passes a QA test may still be a poor survey.
- "SurGE's 4-dimension evaluation captures survey quality" — The 4 dimensions (coverage, accuracy, structure, readability) are reasonable but the paper does not demonstrate inter-annotator agreement or correlation with human judgments.
- "Discipline-aware evaluation improves assessment" (SurveyLens) — This is intuitively true, but SurveyLens does not demonstrate that discipline-aware evaluation changes system rankings compared to discipline-agnostic evaluation.

### Open Problems This Phase Exposes (but does not solve)

- **No benchmark has been adopted as standard**: SurveyBench, SurveyEval, SurGE, SurveyLens, DeepSurvey-Bench, and SGSimEval all exist. No single benchmark is used by a majority of ASG papers. The field now has multiple standards — which means it still has none.
- **Quiz-based vs. dimension-rating vs. reference-free**: Which evaluation paradigm best captures survey quality? No paper compares evaluation paradigms against human judgment.
- **Factuality evaluation for surveys is unsolved**: CiteME shows that even state-of-the-art LLMs struggle with accurate citation. CiteGuard provides attribution metrics but hasn't been applied to ASG systems. The meta-evaluation paper (2411.16638) directly questions whether automatic metrics measure factuality at all.
- **No evaluation of survey insight or novelty**: All existing benchmarks evaluate coverage, accuracy, structure, and readability. None evaluate whether a survey provides *new insight* — the very quality that distinguishes excellent surveys from competent summaries.

---

## Critical Assessment

### Claim vs. Evidence Gap

| Claimed Advance | Supporting Evidence | Assessment |
|-----------------|-------------------|------------|
| "Multi-agent systems outperform single-agent systems" | Agentic AutoSurvey: 8.18/10 vs AutoSurvey: 4.77/10 on custom evaluation | This is the only controlled comparison in the entire literature. It is strong evidence, but it is a single data point. No replication exists. |
| "+32% Citation F1 improvement" (SciSage) | SurveyScope benchmark, +32% improvement over baseline | SurveyScope is a custom benchmark. The improvement is on citation F1 (a narrow metric), not on overall survey quality. |
| "+27.2% quality improvement, months → hours" (InsightAgent) | Custom evaluation with human-in-the-loop vs. manual systematic review | Conflates speed with rigor. A manual SLR takes months because of methodological rigor; reproducing that rigor in 1.5 hours requires demonstration of comparable depth, not just speed. |
| "Hierarchical graph improves organization" (SurveyG) | Qualitative assessment; no ablation study | The claim is structural. The paper does not show that removing graph structure degrades organization — it only shows the system as built. |
| "SurveyBench enables comprehensive ASG evaluation" | 11,343 topics + 4,947 human surveys | Large corpus, but quiz-based evaluation measures factual recall, not survey quality. A passage-retrieval baseline might achieve similar QA scores without generating a coherent survey. |
| "Quality-aware retrieval improves surveys" (SurveyGen) | Trained on 4,200+ human surveys | No controlled comparison showing quality-aware retrieval outperforms standard hybrid retrieval with the same evaluation method. |

### Methodological Weaknesses Across All Phases

**1. Custom evaluation is universal.** Every method paper in Phases 1–3 evaluates on custom topics with custom metrics and custom human evaluation rubrics. The only cross-paper comparison (Agentic AutoSurvey vs. AutoSurvey) is possible because the same research group evaluated their own new system against their own old system on their own metric. No independent third-party evaluation exists anywhere in the literature.

**2. No ablation studies.** The field has no tradition of systematic ablation. Does adding citation chaining improve coverage? Does multi-agent coordination improve quality? Does iterative refinement converge? These questions are answered by comparing full systems, not by controlled experiments where one component is removed. SurveyG asserts that hierarchical graph improves organization but does not test the system without the graph. Agentic AutoSurvey claims multi-agent helps but the comparison is against a single-agent baseline, not an ablation of individual agents.

**3. Human evaluation is unreproducible.** When human evaluation is used (Agentic AutoSurvey, InsightAgent), the rubric, annotator qualifications, inter-annotator agreement, and annotation instructions are not consistently reported. Human evaluation is treated as a black-box quality oracle.

**4. No convergence criteria in iterative systems.** SurveyGen-I and IterSurvey both emphasize iteration without specifying when to stop. The iteration loops are unbounded — the system could iterate forever without guaranteed improvement. This is not just a practical issue; it means the systems lack a well-defined objective function.

**5. Graph awareness is evaluated in isolation.** The 5 papers that use citation graph traversal (SurveyG, SciSage, SurveyForge, MATC, LitFM) each evaluate their graph component differently, making cross-system comparison of graph awareness effectiveness impossible.

### Evaluation Comparability Crisis

The field's most intractable problem. Here is concrete evidence from the profile benchmarks arrays:

| System | Benchmark Used | Metric | Dataset Size |
|--------|---------------|--------|-------------|
| AutoSurvey | Custom | Quality rating (1-10) | 10 topics |
| Agentic AutoSurvey | Custom (same as AutoSurvey) | Quality rating (1-10) | Same 10 topics |
| SciSage | SurveyScope | Citation F1 | Not specified |
| SurveyG | Custom | Structure quality (qualitative) | Not specified |
| SurveyGen | Custom (4,200+ training surveys) | Quality estimation accuracy | Custom |
| InsightAgent | Custom | Quality improvement %, time reduction | Custom |
| IterSurvey | Survey-Arena | Not specified | Not specified |

Only Agentic AutoSurvey and AutoSurvey share the same evaluation. Every other system's results are *incommensurable*. This means:
- We cannot rank systems by quality.
- We cannot determine which architectural choice (multi-agent vs. single-agent, graph-aware vs. embedding-only, iterative vs. pipeline) matters most.
- We cannot replicate any result.

The Phase 4 benchmarks (SurveyBench, SurveyEval, SurGE, SurveyLens) were explicitly designed to solve this crisis. Yet no Phase 3 or Phase 2 system has been evaluated on any of them. The benchmarks exist, but the systems have not been benchmarked.

### Blind Spots

**1. Citation hallucination is unmeasured.** Despite the proliferation of factuality metrics (FActScore, VERISCORE, CiteGuard, CiteME, LongDocFACTScore), no ASG system has been systematically audited for citation hallucination — the problem of claiming a paper supports a statement when it does not. This is arguably the most critical failure mode for a survey generation system, yet it is universally ignored in method papers.

**2. No field evaluates survey insight or novelty.** Every benchmark evaluates recall (did you cover the right papers?), accuracy (are your claims correct?), structure (is the organization coherent?), and readability (is the prose clear?). None evaluate the most important quality of a survey: does it provide new insight? Does it synthesize findings in a way that advances understanding? This is fundamentally harder to evaluate, but ignoring it means the field is optimizing for competent summary rather than genuine synthesis.

**3. The human ground truth is idealized.** Human-written surveys are treated as an unproblematic gold standard. But human surveys vary enormously in quality, have known biases (citation network effects, author self-citation, disciplinary conventions), and are themselves imperfect. The SurveyLens insight that quality standards vary by discipline is a step, but no benchmark accounts for the fact that human surveys are an *imperfect* reference.

**4. Computational cost is opaque.** No paper reports inference cost, token usage, API calls, or runtime. Multi-agent systems are inherently more expensive than single-agent systems, but the cost-quality trade-off is never quantified. A system that achieves 8.18/10 at 10x the cost of a 4.77/10 system may not be a net improvement for most use cases.

**5. No cross-lingual or cross-domain evaluation.** All ASG systems generate surveys in English, for computer science (primarily NLP/ML) topics. Whether these systems work for other languages, other scientific fields, or other document types (clinical guidelines, policy reviews, legal surveys) is unknown.

### What a Truly Next-Generation System Would Need

1. **Citation graph traversal as a first-class retrieval primitive**, not a bolt-on. Only SurveyG treats graph structure as central; the others treat citation chaining as one retrieval strategy among many. A next-generation system would build retrieval entirely around the citation graph, using graph neural networks or attention-over-graphs to learn which papers to retrieve and how to organize them.

2. **Convergence-guaranteed iterative refinement** with explicit stopping criteria. Current iterative systems loop without convergence. A formal objective function (maximize coverage while minimizing redundancy; maximize citation support while minimizing hallucination risk; maximize insight while minimizing verbosity) with gradient-free optimization over the iterative loop would be a genuine advance.

3. **Standardized evaluation on at least two of: SurveyBench, SurGE, and SurveyLens.** No system can credibly claim state-of-the-art without evaluation on a shared benchmark. The field must converge on at least one common evaluation protocol, ideally a community-organized leaderboard.

4. **Systematic citation hallucination auditing** using CiteGuard, VERISCORE, or similar tools, reported as a primary metric alongside coverage and quality. A survey that fabricates citations is useless regardless of its structure or readability.

5. **Controlled cost-quality reporting**: token usage, API costs, runtime, and quality scores on the same benchmark. Without this, the field cannot answer the most practical question: which system should I use given my budget and quality requirements?

6. **Ablation studies for every architectural choice**: single-agent → multi-agent, embedding-only → graph-aware, single-pass → iterative refinement — each tested in controlled settings with the same evaluation protocol. The field currently has architectural variation without architectural understanding.
