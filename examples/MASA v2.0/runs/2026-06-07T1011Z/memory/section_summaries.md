# Section Summaries

## s1 — Introduction
Sets up the survey's scope, central tension (architectural innovation outpacing evaluation infrastructure), and four narrative threads (Evaluation Crisis, Automation–Control Tension, Citation Shallowness, Depth–Breadth Trade-off). Provides a roadmap of the 7 sections.

## s2.1 — Core Enabling Methods: RAG and Retrieval Paradigm
Describes the foundational RAG paradigm (Lewis et al., 2005.11401), its retrieve-then-generate mechanism, and why its single-pass, passage-level assumptions are insufficient for multi-section survey generation. Establishes Thread 3 (citation shallowness) by noting RAG's passage-level retrieval.

## s2.2 — Datasets and Evaluation Infrastructure
Surveys 6 key Phase 1 datasets (SciFact, Multi-XScience, SciReviewGen, FActScore, SciTLDR, MS²) with a comparison table. Shows that none provides evaluation infrastructure for full multi-section surveys — the root of Thread 1 (Evaluation Crisis).

## s2.3 — Limitations That Drove the Transition
Synthesizes the Phase 1→2 transition: short-form methods don't scale to long-form surveys. Three specific gaps (single-pass retrieval, flat summarization, no quality control) drove the need for stage decomposition.

## s3.1 — The Stage-Decomposition Template
Deep dive into AutoSurvey as the archetype (5-stage pipeline), compared with SurveyX, LitLLM, Instruct LLMs Step by Step, and DimInd. Includes a 5-column comparison table. Thread 2 (Automation–Control Tension) emerges: single-agent maximizes automation, minimizes control.

## s3.2 — Training Paradigms: Fine-Tuned vs. Zero-Shot
Compares OpenScholar, ScholarCopilot, AcademicGPT (fine-tuned) with AutoSurvey, SurveyX, PaperQA2 (zero-shot). Includes comparison table across training data, retrieval, citation accuracy, and portability. Thread 3: neither paradigm addresses citation graph reasoning.

## s3.3 — Phase 2 Assessment
Establishes Phase 2's ceiling: single-pass design, no citation graph awareness, single-LLM bottleneck. These limitations drove Phase 3's multi-agent and graph-enhanced responses.

## s4.1 — Multi-Agent Architectures (Updated Round 2)
Deep dive comparing 8 multi-agent systems (added AutoSurvey2 with parallel section generation and real-time re-retrieval). 6-column comparison table expanded. Prompt-deep specialization critique strengthened with falsifiability framing (controlled experiment proposal). DOVA deliberation quality cross-referenced with rubric-quality problem. Thread 2 deepens.

## s4.2 — Graph-Enhanced Retrieval (Updated Round 4)
Deep dive comparing 5 graph-enhanced systems (SurveyForge, SurveyG, SurveyGen, ProfOlaf, GEAR-Up). Explains SurveyG's three-tier mechanism. Critical assessment expanded with structured multi-hop barrier analysis: (a) engineering barrier — scalable path-finding at citation-graph scale requires efficient algorithms; (b) relevance degradation — noise accumulates per hop; (c) infrastructural gap — no claim-level citation graph exists; (d) evaluation gap — no benchmark measures multi-hop citation accuracy. Cites SciAtlas (2605.22878) for the "superficial keyword matching lacks topological reasoning" framing. Thread 3 (Citation Graph Shallowness) fully developed.

## s4.3 — Human-in-the-Loop and Iterative Refinement (Updated Round 2)
Compares hybrid interactive (InteractiveSurvey, PROMPTHEUS, CRUISE-Screening) and iterative refinement (ReClaim, IterSurvey) systems. ReClaim's computational cost quantified: ~10 API calls/sentence, ~1,000 calls for a 100-sentence survey (~$50-100). Exposes Thread 4 (Depth–Breadth Trade-off) with concrete cost estimates.

## s4.4 — The Evaluation Benchmark Explosion
Surveys 8+ benchmarks (SurveyBench, SurveyEval, SurveyScope, Survey-Arena, SurGE, SGSimEval, SurveyLens, DeepSurvey-Bench, CiteRAG) with a 6-column comparison table. Diagnoses the 4-way fragmentation crisis: metrics, references, human evaluation, and benchmark proliferation. Central development of Thread 1.

## s5 — Current Frontier: Deliberation, Domain Expertise, Local Deployment (Updated Round 4)
Deep dive into DOVA (3-phase deliberation mechanism, cross-reference to §4.1 rubric-quality problem), OrchMAS (4-stage hierarchical orchestration with caveats), and ResearchPilot (quantization specifics: 4/8-bit, 7B-13B models, 60-75% cloud performance estimate). Cross-phase comparison table. Critical assessment expanded with quality control paradigm comparison: deliberation-first (DOVA) vs. rubric-guided iteration (ARISE) vs. reflect-when-you-write (SciSage) — each solves a different quality problem and introduces a different failure mode. None addresses the fundamental question of whether in-loop quality control improves the final survey against human expert judgment. Thread 3 (citation shallowness) and Thread 4 (depth–breadth) critiqued.

## s6 — Critical Assessment (Updated Round 4)
The analytical core: (a) Claim vs. Evidence Gap table (7 claims) with new "Claim Source" column citing specific paper sections, (b) 8 methodological weaknesses (added reproducibility crisis — none of 12+ systems provide reproducible code, contrast with Reproducible Pipeline 2508.04612 achieving F1>0.85; computational cost blindness — no system reports standardized compute costs) with aggregated field-wide statistics (mean ~20 topics, median ~15, max ~100 across 12 systems), "most surveyed papers" instead of "77", (c) Evaluation Comparability Crisis diagnosis, (d) 7 blind spots. Converges all four narrative threads.

## s7 — Future Directions (Updated Round 2)
Six directions (merged former direction 6 into direction 3 as structured output extension), each grounded in Section 6 gaps, with prioritization. New cross-cutting trade-off analysis: multi-hop reasoning vs. Depth–Breadth Trade-Off, domain adaptation vs. evaluation comparability, local deployment vs. evaluation protocol support.
