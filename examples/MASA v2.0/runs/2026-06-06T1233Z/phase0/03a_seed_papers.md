# SeedPapers — Citation Graph Expansion Seed Set

**run_dir**: `.`
**generated**: 2026-06-06T12:38:43+08:00
**source**: `02_candidate_pool.md` (82 unique candidates)
**selection_strategy**: Prefer surveys, high-signal core method papers, evaluation anchors, and bridge mechanisms — maximizing forward/backward citation graph yield.
**total_seeds**: 10

---

## Seed Papers

| # | arXiv ID | Title | Year | Role in CandidatePool | Rationale for Seed Selection |
|---|----------|-------|------|-----------------------|------------------------------|
| 1 | 2406.10252 | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | `core_method` | **Foundational anchor**. Retrieval + outline + drafting architecture. Highly cited — its reference list captures the early method landscape and forward citations reveal derivative work. |
| 2 | 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | `core_method` | **Latest evolution** of the AutoSurvey lineage; multi-agent (4-agent) framework scoring 8.18/10. Bridges the foundational method era with the 2025 multi-agent wave. |
| 3 | 2503.04629 | SurveyForge: Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | 2025 | `core_method` | **Distinct architectural approach** — outline heuristics + scholar navigation agent. Adds a planning-perspective anchor not covered by AutoSurvey. |
| 4 | 2402.01788 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | `mechanism` | **Bridge candidate**. RAG-based toolkit with keyword extraction and re-ranking. Connects the RAG mechanism literature to the survey-generation task. |
| 5 | 2407.01796 | ReClaim: Ground Every Sentence with Interleaved Reference-Claim Generation | 2024 | `mechanism` | **Citation fidelity anchor**. Sentence-level citations at 90% accuracy. Essential for surfacing the citation-attribution sub-literature. |
| 6 | 2310.11511 | Self-RAG: Learning to Retrieve, Generate, and Critique through Self-Reflection | 2023 | `mechanism` | **Foundational retrieval+reflection** paper. Highly cited; connects survey generation to the broader Self-RAG/RankRAG retrieval lineage. |
| 7 | 2305.14251 | FActScore: Fine-grained Atomic Evaluation of Factual Precision | 2023 | `evaluation` | **Widely-adopted evaluation metric**. Atomic fact decomposition is used across multiple survey evaluation benchmarks. Captures the factual-consistency evaluation literature. |
| 8 | 2510.03120 | SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs? | 2025 | `benchmark` | **Dedicated survey-generation benchmark**. 11,343 topics, 4,947 surveys; multifaceted metric hierarchy. Captures the evaluation benchmarking literature. |
| 9 | 2305.15186 | SciReviewGen: Large-scale Dataset for Automatic Literature Review Generation | 2023 | `benchmark` | **Large-scale survey dataset**. 10,000+ reviews, 690K cited papers; reveals hallucination challenges. Captures the data-and-evaluation literature. |
| 10 | 2401.10917 | Artificial intelligence to automate the systematic review of scientific literature | 2024 | `survey` | **Existing survey of the field**. 15-year survey covering 34 primary studies. Its reference list captures the pre-LLM and early-LLM systematic review landscape. |

---

## Coverage Analysis

| Dimension | Seeds Addressing It | Notes |
|-----------|-------------------|-------|
| **Method / Architecture** | AutoSurvey (1), Agentic AutoSurvey (2), SurveyForge (3), LitLLM (4) | Spans single-agent, multi-agent, and hybrid approaches |
| **Retrieval / RAG** | LitLLM (4), ReClaim (5), Self-RAG (6), AutoSurvey (1) | From toolkit RAG to attribution to self-reflection |
| **Citation Attribution** | ReClaim (5), FActScore (7) | Sentence-level citation and atomic factuality |
| **Evaluation / Benchmark** | FActScore (7), SurveyBench (8), SciReviewGen (9) | Metrics, dedicated benchmark, and dataset |
| **Existing Surveys** | AI to automate systematic review (10) | Captures prior taxonomies and reference lists |
| **Frontier / Open Challenges** | Agentic AutoSurvey (2), SurveyBench (8) | Multi-agent coordination, evaluation standardization |

---

## Risks

1. **No `schema/expansion.md`** on disk — seed selection guided by SurveySpec (`00_survey_spec.md`), QueryPlan (`01_query_plan.md`), and CandidatePool (`02_candidate_pool.md`).
2. **Time range**: Self-RAG (2310.11511, 2023) and FActScore (2305.14251, 2023) are at the lower bound of the 2023–2025 window. Retained due to foundational status and high citation impact.
3. **No STORM seed**: STORM (a named system in the spec) is absent from the candidate pool. If STORM's arXiv paper (e.g., 2402.14207) is known, it should be added as a seed. The current set compensates with AutoSurvey (the other named anchor).
4. **PaperQA not seeded**: Similarly absent from the pool; SurveyForge + AutoSurvey + LitLLM cover the retrieval-for-survey space.
