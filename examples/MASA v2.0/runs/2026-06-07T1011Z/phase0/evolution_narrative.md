# Evolution Narrative

## Critical Arc

The field of automated survey generation has completed a rapid four-year transition from foundational retrieval-augmented generation (RAG) paradigms through single-agent pipelines and into a fragmented landscape of multi-agent architectures, graph-enhanced retrieval, and human-in-the-loop systems — but this architectural proliferation has outpaced the development of rigorous, standardized evaluation. While each successive phase has introduced genuine innovations (decomposition of survey writing into sub-tasks in 2024, multi-agent role specialization in 2025, deliberation-first reasoning in 2026), the field remains stuck in a regime where nearly every system defines its own quality rubric, evaluates on self-created benchmarks with small topical samples (typically 10–46 CS topics), and reports metrics that are incommensurable across papers. The central claim — that automated systems can produce survey-quality literature reviews — remains unsubstantiated by any standardized, multi-dimensional evaluation that measures factual accuracy, citation faithfulness, coverage breadth, analytical depth, and readability on a shared corpus against human-written gold standards.

## Phase 1: Foundations — RAG Paradigm and Dataset Infrastructure (2020–2023)

### Technical Approach
This phase established the core enabling technologies: retrieval-augmented generation for grounding LLM outputs in external knowledge, datasets for training and evaluating review generation, and factuality evaluation frameworks. Systems were primarily single-pass, single-agent pipelines designed for short-form QA or related-work generation rather than full surveys.

### Key Contributions
- **RAG (Lewis et al., 2005.11401)**: Foundational retrieve-then-generate paradigm combining parametric and non-parametric memory. Established the core architecture that virtually all subsequent survey systems build upon.
- **SciFact (Wadden et al., 2004.14974)**: First large-scale dataset for scientific claim verification (SUPPORT/REFUTE/NEI paradigm). Became the standard benchmark for citation-claim alignment.
- **Multi-XScience (Lu et al., 2010.14235)**: Dataset for multi-document related-work generation, establishing the evaluation format of target text + source papers.
- **SciReviewGen (Kim et al., 2305.15186)**: Large-scale dataset of 10K+ reviews paired with 690K cited papers, the largest of its kind and the standard training/evaluation resource for review generation.
- **FActScore (Min et al., 2305.14251)**: Atomic fact decomposition for fine-grained factuality evaluation. Became the de facto standard for factuality assessment in survey generation.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| RAG | QA Accuracy | SOTA at publication | Multiple QA datasets | Short-form QA, not survey generation |
| SciFact | Claim verification F1 | Baseline established | SciFact (1,409 pairs) | Biomedical domain only |
| SciReviewGen | ROUGE-L | Baseline established | SciReviewGen (10K reviews) | Reference-based evaluation on curated dataset |

### What This Phase Genuinely Achieved
Established the fundamental building blocks: the RAG architecture for grounding generation in retrieved knowledge, datasets enabling training and evaluation of review generation, and factuality evaluation methodology. These remain the infrastructure on which the entire field depends. The phase did not produce end-to-end survey generation, but without it, no later system would exist.

### Unfulfilled Claims
- **RAG claimed to reduce hallucination** via retrieved context — true for short-form QA, but the claim was never tested on multi-paragraph survey-length generation where hallucination patterns differ.
- **FActScore claimed to provide a general factuality metric** — but atomic decomposition quality is LLM-dependent, and the framework was never validated for survey-length text with hundreds of citations.

### Limitation That Drove Transition to Next Phase
All Phase 1 systems addressed short-form tasks (QA, claim verification, related-work generation). No system could produce a coherent multi-section survey (>1,000 words) with structured narrative, integrated citations, and topical organization. The transition to Phase 2 was driven by the recognition that LLMs (particularly GPT-4) had sufficient reasoning capacity to attempt this larger task — but the Phase 1 datasets and methods provided no direct pathway; a fundamentally new decomposition strategy was required.

## Phase 2: The Single-Agent Pipeline Emerges (2024)

### Technical Approach
The defining innovation of 2024 was task decomposition: breaking survey writing into sequential stages (retrieval → outline → section drafting → integration → evaluation), each executed by a single LLM via prompt engineering. This was the year of the single-agent pipeline, with AutoSurvey as the archetype. Systems were predominantly single-pass (no iteration between stages), used hybrid keyword+embedding retrieval without citation graph awareness, and evaluated on small self-created benchmarks (typically 10 topics).

### Key Contributions
- **AutoSurvey (2406.10252)**: First end-to-end survey generation pipeline decomposing survey writing into retrieval, outline, drafting, integration, and evaluation stages. Demonstrated that LLMs can produce coherent multi-section surveys (>5,000 words). Established the baseline architecture that most subsequent systems either extend or react against.
- **PaperQA2 (2409.13740)**: Achieved superhuman performance on scientific QA with multi-step verification and contradiction detection. Not a survey system per se, but demonstrated that LLM-based systems could outperform PhD scientists on literature synthesis tasks.
- **OpenScholar (2411.14199)**: Specialized retriever trained on 45M scientific papers with citation-supervised fine-tuning. Showed that domain-specific training significantly improves citation accuracy compared to general-purpose models.
- **ReClaim (2407.01796)**: Per-sentence iterative retrieval-generate-verify loop ensuring fine-grained citation attribution. Introduced the iteration paradigm at the micro-level, though not scaled to full surveys.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| AutoSurvey | ROUGE-L | ~0.35 | Self-created benchmark (10 CS topics) | Compared against human-written surveys |
| AutoSurvey | Human win-rate | ~40% | Self-created benchmark | Humans preferred system over human baseline in 40% of cases |
| PaperQA2 | QA accuracy | Superhuman | Multiple scientific QA benchmarks | Outperforms human PhD scientists on QA |
| OpenScholar | Citation accuracy | SOTA at publication | Multiple scientific QA datasets | Domain-specific training improves citation quality |
| ReClaim | Citation precision | Improved vs post-hoc | Multiple QA datasets | Per-sentence verification reduces false citations |

### What This Phase Genuinely Achieved
Demonstrated that single-LLM pipelines could produce coherent multi-section surveys on specialized topics — a non-trivial achievement. Established the stage-decomposition template (retrieval→outline→drafting→integration) that later multi-agent systems would inherit and specialize. PaperQA2's superhuman QA result was a genuine milestone, proving that LLM-based scientific literature synthesis could exceed human expert performance on well-defined tasks.

### Unfulfilled Claims
- **AutoSurvey's ~40% human win rate** is reported as a positive result, but means humans preferred the human-written baseline in 60% of cases. The claim "LLMs can automatically write surveys" is technically true but the quality gap to human surveys was and remains substantial.
- **PaperQA2 claim of "superhuman"** applies to short-form QA, not to survey generation. No evidence was provided that the system could produce a full multi-section survey comparable to a human-written review.
- **ReClaim's sentence-level verification** was demonstrated on QA and summarization tasks. The claim that this scales to full survey generation (100+ sentences, 50+ citations) was never demonstrated. The computational cost of per-sentence retrieval for surveys is prohibitive.

### Limitation That Drove Transition to Next Phase
Single-agent pipelines had a fundamental ceiling: one LLM driving all stages could not simultaneously achieve deep retrieval, coherent organization, accurate citation, and critical synthesis. The single-pass design meant retrieval gaps discovered during writing could not be addressed. The lack of citation graph awareness meant papers were discovered via keyword/embedding search only, missing the citation chaining benefits that human reviewers rely on. These limitations drove two distinct responses in 2025: multi-agent architectures (specializing different LLMs for different sub-tasks) and graph-enhanced retrieval (using citation structure for paper discovery).

## Phase 3: Architectural Proliferation (2025)

### Technical Approach
2025 saw an explosion of architectural diversity as the field fragmented into five distinct approaches: (1) **Multi-agent pipelines** with specialized agent roles (Planner, Retriever, Writer, Reviewer) and iterative refinement loops; (2) **Graph-enhanced retrieval** systems making citation graph traversal a core component; (3) **Hybrid interactive** systems with human-in-the-loop; (4) **Iterative refinement** systems where multi-round generation is the defining pattern; and (5) continued single-agent pipelines with improved preprocessing (AttributeTree, coarse-to-fine retrieval). Simultaneously, benchmark papers proliferated as the field recognized the evaluation crisis — but each benchmark defined different metrics, protocols, and reference sets, deepening the comparability problem rather than solving it.

### Key Contributions
- **ARISE (2511.17689)**: Rubric-guided iterative refinement with 5 specialized agents (Planning, Retrieval, Drafting, Evaluation, Revision). Evaluation→Revision loop continues until quality threshold (92.48) is met. First system to formalize quality criteria as a driver of the generation loop.
- **SciSage (2506.12689)**: Reflect-when-you-write paradigm with hierarchical Reflector Agent (local paragraph-level + global survey-level). Real-time quality reflection during drafting rather than post-hoc evaluation. Introduced Citation F1 metric.
- **SurveyForge (2503.04629)**: Memory-driven generation with cross-section context buffer and bidirectional citation chaining. Systematic analysis of outline heuristics from human-written surveys.
- **SurveyG (2510.07733)**: Three-tier hierarchical citation graph (Foundation/Development/Frontier) for organizing papers by their role in the research lineage. First system to model citation graph depth rather than treating all retrieved papers uniformly.
- **IterSurvey (2510.21900)**: Recurrent outline refinement with paper card grounding ensuring faithful citation attribution. Paired with Survey-Arena pairwise comparison framework.
- **Agentic AutoSurvey (2509.18661)**: Four-agent architecture with 12-dimensional quality rubric and iterative review-refine loop. Demonstrated at scale (75–443 papers per topic).
- **SurveyBench (2510.03120)**: Comprehensive benchmark with 11K+ arXiv papers and 4.9K+ human-written surveys across multiple disciplines. Introduced quiz-driven evaluation for answerability testing.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| ARISE | Overall quality score | 92.48 | Self-evaluation | System-defined rubric; relationship to human judgment unestablished |
| ARISE | Quality threshold reached | Yes | Iterative refinement loop | Convergence within limited iterations |
| SciSage | Citation F1 | SOTA at publication | SurveyScope (46 papers, 11 CS domains) | New metric introduced |
| SciSage | Overall quality | Top-ranked | SurveyBench, SurveyScope | Combined evaluation across two benchmarks |
| SurveyForge | Win rate vs human | Reported | SurveyBench (100 human surveys) | Compared across outline, content, citation dimensions |
| SurveyG | Coverage | Improved over flat retrieval | Self-evaluation on CS topics | Hierarchical graph improves coverage breadth |
| SurveyG | Organization quality | Higher than non-hierarchical | Human evaluation | Three-tier structure preferred by evaluators |
| IterSurvey | Citation faithfulness | High | Self-evaluation | Paper cards reduce hallucinated citations |
| Agentic AutoSurvey | Coverage breadth | 75–443 papers/topic | Self-evaluation on LLM topics | Scales with topic breadth |

### What This Phase Genuinely Achieved
The architectural innovations of 2025 were substantial and real. Multi-agent systems demonstrated that role specialization (separating retrieval, writing, evaluation into distinct agents) produces better outputs than monolithic single-agent pipelines — particularly through iterative quality feedback loops. Graph-enhanced retrieval showed that citation chaining discovers papers that keyword/embedding search misses, with SurveyG's hierarchical tiering providing a principled way to organize discovered papers by their role in the research narrative. The rubric-guided approach (ARISE, SciSage, Agentic AutoSurvey) moved evaluation from post-hoc assessment to an integral driver of the generation process. Iterative refinement (IterSurvey, ReClaim) demonstrated that multi-round generation with explicit quality signals converges to better outputs than single-pass approaches.

### Unfulfilled Claims
- **ARISE's 92.48 quality score** is the most striking unvalidated claim in the phase. This is a system-defined score from a self-constructed rubric with no established correlation to human-judged survey quality. The same system could define a 100-point rubric and report a higher number. Without human calibration, this number is meaningless as a cross-system comparison value.
- **SciSage "SOTA at publication"** is based on evaluation on its own SurveyScope benchmark (46 papers, 11 CS domains). No contemporaneous system was evaluated on the same benchmark, so the SOTA claim is untestable.
- **SurveyG's "improved coverage"** and **"higher organization quality"** are reported without confidence intervals, effect sizes, or comparison against the strongest baseline (which would be SurveyForge or ARISE, not "flat retrieval").
- **SurveyForge's "win rate vs human"** — the paper reports comparisons but the actual values are context-dependent. Without standardizing the human-written reference set, win-rate comparisons across papers are apples-to-oranges.
- **Virtually every system claims to address citation accuracy/hallucination**, but none conducts a systematic hallucination audit using FActScore or a similar independent methodology. Citation accuracy claims are self-evaluated, not independently verified.

### Limitation That Drove Transition to Frontier
The fragmentation of 2025 exposed two deep problems. First, **evaluation had become anarchic**: every system defined its own metrics, benchmarks, and evaluation protocols, making it impossible to determine which architectural innovation actually moved the needle. Second, **no system had yet addressed the core intellectual challenge of survey writing**: critical synthesis — analyzing, comparing, and reconciling conflicting findings across papers. Systems were producing descriptive surveys (summarizing what papers say) rather than analytical surveys (evaluating evidence, identifying debates, assessing methodological quality). The frontier systems in 2026 begin to address this through deliberation-first and domain-expert architectures, but the evaluation crisis remains unresolved.

## Phase 4: Current Frontier — Deliberation and Domain Expertise (2026)

### Technical Approach
The frontier phase (2026) is characterized by systems that invert the traditional retrieve-then-generate pipeline: deliberation happens before retrieval, with agents formulating information needs based on structured discussion of the research topic. DOVA's deliberation-first paradigm and OrchMAS's domain-specialized expert agents represent complementary approaches to the same insight — that quality synthesis requires understanding the intellectual landscape before gathering papers, not after. These systems also show a trend toward local-first deployment (ResearchPilot on consumer hardware) and more granular evaluation benchmarks (SurveyLens for discipline adaptation, DeepSurvey-Bench for academic depth).

### Key Contributions
- **DOVA (2603.13327)**: Deliberation-first multi-agent architecture where agents with different research perspectives (empiricist, theoretician, methodologist, domain specialist) discuss the topic before any retrieval occurs. Inverts the standard retrieve-then-deliberate pattern.
- **OrchMAS (2603.03005)**: Domain-specialized expert agents with pre-configured knowledge profiles, orchestrated by a Coordinator agent. Explicit modeling of domain expertise at the agent level rather than relying on general-purpose LLM knowledge.
- **ResearchPilot (2603.14629)**: Local-first multi-agent system (4 agents: Retriever, Analyst, Writer, Reviewer) running on consumer hardware. Demonstrates that multi-agent survey generation can work without cloud API dependencies.

### Open Problems This Phase Exposes (but does not solve)

- **Evaluation fragmentation worsens**: New benchmarks (SurveyLens, DeepSurvey-Bench, CiteRAG) are added to an already crowded landscape without any consolidation or standardization effort. The field now has more benchmarks than there are methodological approaches.
- **Deliberation quality is unmeasured**: DOVA argues that deliberation before retrieval improves output, but provides no way to measure whether the deliberation itself is productive. Without a deliberation quality metric, the claim that "deliberation-first improves retrieval relevance" is circular.
- **Domain expertise vs. general capability**: OrchMAS's domain-specialized agents require upfront configuration of knowledge profiles. How this scales to new domains — and whether the expertise is genuinely different from what a general-purpose LLM already knows — is not addressed.
- **Citation graph awareness remains shallow**: No 2026 system uses citation graph traversal beyond BFS. None attempts multi-hop reasoning over citation chains, temporal analysis of research trajectories, or methodological lineage tracking. The field has not progressed beyond single-hop forward/backward chaining.

## Critical Assessment

### Claim vs. Evidence Gap

| Claimed Advance | Supporting Evidence | Assessment |
|-----------------|-------------------|------------|
| "AutoSurvey achieves human-competitive quality" | ROUGE-L ~0.35; 40% human win rate | ROUGE-L measures n-gram overlap, not analytical depth. 40% win rate means losing 60% of the time. The claim overstates the evidence. |
| "ARISE achieves 92.48 quality score" | Self-evaluation on system-defined rubric | Rubric dimensions and scoring are defined by the system; no human calibration or cross-validation. This is not a meaningful absolute score. |
| "PaperQA2 is superhuman" | Outperforms PhD scientists on scientific QA | QA is not survey generation. The "superhuman" claim applies narrowly to factual question answering, not to analytical synthesis. |
| "SciSage achieves SOTA Citation F1" | Evaluation on SurveyScope benchmark | No concurrent system evaluated on the same benchmark. SOTA claim is unfalsifiable without direct comparison. |
| "Multi-agent systems outperform single-agent" | Comparison across different benchmarks, not controlled | Each multi-agent paper uses a different evaluation setup (different topics, different metrics, different reference surveys). No controlled ablation comparing single vs multi-agent on identical conditions. |
| "Citation graph traversal improves coverage" | Self-evaluation showing more papers retrieved | The question is not quantity but quality — does graph traversal discover papers that are genuinely more relevant? No paper measures this. |
| "Survey generation is mature enough for practical use" | Fragmented evaluation, small topic samples | The largest evaluation in the corpus is ~100 surveys. Human-written surveys exist for thousands of topics. The field has not demonstrated scalability or reliability at practical deployment scale. |

### Methodological Weaknesses Across All Phases

1. **Self-evaluation epidemic**: Of the 12 representative papers analyzed in depth, 10 rely primarily on self-created evaluation benchmarks with no independent validation. Evaluation results are generated by the same team that built the system, using metrics and datasets they defined. There is no independent third-party evaluation in the entire corpus.

2. **Small topical samples**: AutoSurvey evaluated on 10 topics. SciSage on 46 papers across 11 domains. SurveyForge's SurveyBench covers 100 surveys. SurveyBench (2510.03120) expands to 4,947 surveys — but no generation system has been evaluated on more than ~100 topics. Every system's evaluation is at risk of overfitting to its specific evaluation set.

3. **Single-discipline bias**: Computer science dominates evaluation across all phases. Systems tested on CS topics cannot be assumed to generalize to biomedicine, social sciences, humanities, or engineering. Only SurveyBench includes multi-discipline reference surveys, and even then, generation systems are rarely evaluated on non-CS topics.

4. **No standardized evaluation benchmark**: Despite 8+ benchmark papers (SurveyBench, SurveyEval, SGSimEval, SurGE, SurveyLens, DeepSurvey-Bench, SurveyScope, Survey-Arena), no single benchmark has achieved consensus adoption. Each new system reports on a different combination of benchmarks, making cross-comparison impossible.

5. **Citation verification is superficial**: Systems claim "high citation accuracy" but verification is typically: (a) post-hoc and NLI-based, not during generation; (b) evaluated at the paper level, not the claim level; (c) self-evaluated, not independently audited. No system uses FActScore-style atomic decomposition for systematic citation audit.

6. **No measure of analytical depth**: The field evaluates coverage (are all relevant papers cited?), coherence (does the narrative flow?), and citation accuracy (are claims supported by cited papers?). No system or benchmark measures analytical depth — does the survey identify conflicting findings, evaluate methodological quality, identify open questions, or provide novel synthesis? These are the distinguishing features of high-quality human surveys, and they are entirely absent from current evaluation frameworks.

### Evaluation Comparability Crisis

The field suffers from a deep evaluation comparability crisis that prevents meaningful progress measurement:

- **Metric fragmentation**: ROUGE-L, BLEU, FActScore, Citation F1, rubric-based quality scores (dimension counts vary: 5 in ARISE, 12 in Agentic AutoSurvey), win-rate (vs human, vs baseline), user satisfaction, coverage breadth, quiz answerability — no two systems report the same set of metrics.

- **Reference survey fragmentation**: AutoSurvey uses its own human-written surveys on 10 topics. SurveyForge uses SurveyBench (100 human surveys). SciSage uses SurveyScope (46 papers). SurveyG uses self-evaluation on CS topics. No system evaluates against another system's reference set, making direct comparison impossible.

- **Human evaluation inconsistency**: When human evaluation is used (rarely), protocols vary wildly: win-rate comparison (AutoSurvey), Likert-scale ratings (SurveyG), user satisfaction surveys (InteractiveSurvey). None use standardized human evaluation guidelines (e.g., the Likert dimensions used in the NarrativeQA or SummEval communities).

- **Benchmark proliferation without consolidation**: 2025-2026 added 8+ new benchmarks — SurveyBench, SurveyEval, SGSimEval, SurGE, SurveyLens, DeepSurvey-Bench, SurveyScope, Survey-Arena. Each adds evaluation dimensions but none subsumes the others. The result is a fragmented landscape where every paper can find a benchmark on which its system performs well.

### Blind Spots

1. **Multi-hop citation reasoning**: No system traces claims through chains of citations (Paper A cites Paper B, which builds on Paper C) to verify that synthesized claims are faithful to the original evidence. Every system treats each citation as independent, ignoring the citation graph's reasoning structure.

2. **Temporal analysis of research trajectories**: No system tracks how a research area has evolved over time — identifying paradigm shifts, resolving debates, tracking methodological improvements. SurveyG's hierarchical tiers (Foundation/Development/Frontier) gesture at this but organize by citation graph position, not by temporal intellectual development.

3. **Methodological quality assessment**: Human-written surveys critically evaluate the methods of cited papers (e.g., "this study had a small sample size," "the experimental design did not control for X"). No automated system attempts this. The field has entirely ceded methodological critique to human authors.

4. **Figure and table generation**: Human surveys include taxonomies, comparison tables, methodology diagrams, trend plots. No generation system produces non-textual content. SurveyBench explicitly includes "non-textual quality" as an evaluation dimension — and every system fails it because none attempts it.

5. **Cross-lingual surveys**: All 77 papers in the corpus address English-language surveys. There is no exploration of generating surveys in other languages or covering non-English literature.

6. **Longitudinal trustworthiness**: A survey published today cites papers up to the retrieval date. How does the survey age as the field progresses? No system addresses survey maintenance — updating coverage as new papers are published, revising claims as evidence accumulates, retracting claims that are superseded.

7. **User trust and calibration**: No paper studies whether users trust (or should trust) the output of automated survey systems. Do users detect hallucinated citations? Do they notice gaps in coverage? Do they over-rely on system outputs? These are human factors questions the field has not engaged with.

### What a Truly Next-Generation System Would Need

1. **Unified evaluation protocol**: A standardized evaluation framework adopted by the field, with: (a) a shared corpus of reference surveys across multiple disciplines (at least 500 topics); (b) a multi-dimensional metric suite covering coverage, citation faithfulness (via atomic decomposition), analytical depth, organization quality, and readability; (c) independent third-party evaluation using held-out topics; (d) human evaluation following agreed guidelines with inter-annotator agreement reporting.

2. **Multi-hop citation reasoning**: Citation graph traversal beyond BFS, using multi-hop expansion with relevance filtering per hop, enabling the system to trace intellectual lineages and verify claim provenance through chains of citations. This requires modeling not just which papers cite which, but what claims each paper makes and how they relate to claims in cited papers.

3. **Analytical synthesis capability**: The ability to identify conflicting findings across papers, evaluate methodological quality, assess evidence strength, and produce genuine synthesis — not just descriptive summaries. This requires: (a) claim extraction and comparison across papers; (b) methodological feature extraction and comparison; (c) confidence/provenance tracking per claim.

4. **Domain-adaptive expertise**: Rather than a single general-purpose system, the ability to adapt to different disciplinary conventions — citation density norms, argumentation structures, evaluation standards vary significantly across fields. This could be achieved through discipline-specific retrieval configurations, prompt templates, and quality rubrics.

5. **Temporal grounding and maintenance**: A mechanism for updating surveys as new literature accumulates, including: (a) change detection (flagging when new papers change the evidentiary landscape); (b) selective re-generation (updating affected sections without rewriting the entire survey); (c) version tracking (maintaining an audit trail of what claims were made when, and how they have been revised).

6. **Non-textual content generation**: Automated generation of comparison tables, methodology taxonomies, and evidence maps that are characteristic of high-quality human surveys. This requires structured output capabilities beyond narrative generation.

7. **Calibrated uncertainty communication**: The system should express confidence in its claims — flagging when evidence is weak, when findings are contested, when it is synthesizing from partial information. Current systems produce uniformly confident prose regardless of the underlying evidence quality, which is misleading to readers.
