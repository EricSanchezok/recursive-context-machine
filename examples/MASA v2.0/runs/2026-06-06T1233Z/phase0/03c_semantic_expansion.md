# SemanticExpansion — Embedding-Search Neighbor Discovery

**run_dir**: `.`
**generated**: 2026-06-06T12:40:06+08:00
**source**: `03a_seed_papers.md` (10 seeds), `00_survey_spec.md`, `01_query_plan.md`, `02_candidate_pool.md` (82 known candidates)
**method**: 8 concept-rich embedding queries targeting method gaps, benchmark frontiers, and semantic neighbors of the seed set.

---

## Query Design

| # | Query String | Rationale | Target Dimension |
|---|-------------|-----------|------------------|
| Q1 | STORM-like outline-driven survey writing systems with retrieval and citation | Recover the STORM anchor (named in spec, absent from candidate pool) + similar outline-driven systems | Method — missing anchor |
| Q2 | Agentic retrieval for scientific literature synthesis and survey composition | Beyond keyword RAG; recursive/agentic retrieval patterns for survey synthesis | Method — agentic retrieval |
| Q3 | Factuality and citation accuracy in long-form generated text evaluation | Builds on FActScore + ReClaim seeds; surfaces citation-evaluation metrics and benchmarks | Evaluation — citation factuality |
| Q4 | Graph-aware literature review generation using citation networks | Citation graph expansion dimension; beyond LitFM into graph-aware synthesis | Mechanism — graph expansion |
| Q5 | Evaluation frameworks and benchmarks for machine-generated scientific surveys | Benchmarks beyond SurveyBench; evaluation protocol diversity | Benchmark — coverage gaps |
| Q6 | Multi-agent writing workflows for collaborative academic paper drafting | Multi-agent coordination for writing; complements MATC, LiRA, Minigraph Agents | Mechanism — multi-agent writing |
| Q7 | Interactive human-AI survey generation with user steering and customization | Interactive/user-steerable dimension from frontier queries | Frontier — user steering |
| Q8 | Long-context synthesis and structured review generation with retrieval augmentation | Context handling, planning and coherence for long-form survey writing | Mechanism — long-context synthesis |

---

## Results — New Candidates (Deduplicated)

Papers already in `02_candidate_pool.md` (82 candidates) are marked **known** and excluded from the new count. Papers flagged **drift** are conceptually tangential.

### New High-Signal Candidates (15)

| # | arXiv ID | Title | Year | Likely Role | Source Query | Rationale |
|---|----------|-------|------|-------------|--------------|-----------|
| E01 | **2402.14207** | STORM: Assisting in Writing Wikipedia-like Articles from Scratch with LLMs | 2024 | `core_method` | Q1 | **Missing anchor system** named in SurveySpec. Outline-driven pre-writing via multi-perspective question asking. Foundational for the outline-driven survey generation lineage. |
| E02 | **2506.12689** | SciSage: A Multi-Agent Framework for High-Quality Scientific Survey Generation | 2025 | `core_method` | Q1, Q2, Q5 | Multi-agent reflect-when-you-write paradigm. +1.73 coherence, +32% citation F1. Releases **SurveyScope** benchmark (46 high-impact papers, 11 CS domains). |
| E03 | **2312.07559** | PaperQA: Retrieval-Augmented Generative Agent for Scientific Research | 2023 | `core_method` | Q2 | **Missing anchor system** named in SurveySpec. RAG agent for full-text scientific articles. Outperforms existing LLMs on science QA (LitQA benchmark). |
| E04 | **2411.14199** | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | 2024 | `core_method` | Q2, Q8 | Retrieval-augmented LM over 45M open-access papers. ScholarQABench benchmark. Citation accuracy on par with human experts. Open-source. |
| E05 | **2507.10522** | DeepResearch^{Eco}: A Recursive Agentic Workflow for Complex Scientific QA | 2025 | `mechanism` | Q2 | Recursive, depth-controlled agentic workflow for scientific synthesis. Up to 21× source integration improvement. Domain-specific (ecology). |
| E06 | **2305.14627** | ALCE: Enabling LLMs to Generate Text with Citations | 2023 | `benchmark` | Q3 | **First benchmark** for automatic LLM citation evaluation (ALCE). Fluency, correctness, and citation quality metrics. Strong correlation with human judgement. |
| E07 | **2403.18802** | Long-form Factuality in LLMs (SAFE) | 2024 | `evaluation` | Q3 | Google DeepMind's **SAFE** evaluator + LongFact benchmark. LLM agent outperforms crowdsourced human annotators (72% agreement, wins 76% of disagreements). 20× cheaper. |
| E08 | **2406.19276** | VERISCORE: Evaluating Factuality of Verifiable Claims | 2024 | `evaluation` | Q3 | Distinguishes verifiable vs. unverifiable claims. 8 long-form tasks, 16 models. More nuanced than FActScore for tasks with non-verifiable content. |
| E09 | **2504.18496** | DimInd: Facets, Taxonomies, and Syntheses — Navigating Structured Representations in LLM-Assisted Literature Review | 2025 | `mechanism` | Q8 | Interactive system scaffolding lit review across large collections. Multiple compression levels (papers → tables → taxonomies → syntheses). 23-participant evaluation. |
| E10 | **2503.00751** | RAPID: Efficient Retrieval-Augmented Long Text Generation with Writing Planning and Information Discovery | 2025 | `mechanism` | Q8 | Outline-first retrieval-augmented generation. Reduces hallucination via attribute-constrained search. FreshWiki-2024 benchmark. Adjacent to STORM for survey writing. |
| E11 | **2403.05313** | RAT: Retrieval Augmented Thoughts for Context-Aware Reasoning in Long-Horizon Generation | 2024 | `mechanism` | Q8 | Iterative CoT revision with retrieval. 13.6%–42.8% improvement on long-horizon tasks (code, math, creative writing, planning). Applicable to iterative survey drafting. |
| E12 | **2404.11588** | Related Work and Citation Text Generation: A Survey | 2024 | `survey` | Q4 | **Existing survey** of the related work generation (RWG) task. Historical overview, key approaches, and ongoing challenges. Complements seed survey (2401.10917). |
| E13 | **2604.03141** | Beyond Precision: Importance-Aware Recall for Factuality Evaluation | 2026 | `evaluation` | Q3 | Addresses recall gap in factuality evaluation. LLMs perform better on precision than recall. Importance-aware weighting. |
| E14 | **2510.12839** | FaStFACT: Faster, Stronger Long-Form Factuality Evaluations | 2025 | `evaluation` | Q3 | Chunk-level claim extraction with confidence-based pre-verification. Document-level evidence. Fast and strong alignment with human evaluation. |
| E15 | **2411.02448** | REC: Rate, Explain and Cite — Enhanced Explanation and Attribution in Automatic Evaluation | 2024 | `evaluation` | Q3 | Fine-tuned general-purpose LLM auto-evaluator (REC-8B/12B/70B). Ratings + explanation + verifiable citation. Outperforms state-of-the-art LLMs as evaluator. |

### Drift Candidates (Low Signal — Not Counted in New Total)

Papers returned by queries but with weak alignment to automated survey generation. Listed here for auditability.

| arXiv ID | Title | Query | Drift Reason |
|----------|-------|-------|-------------|
| 2510.27126 | AURA: RL for Adaptive Conversational Surveys | Q7 | Questionnaire survey, not literature survey generation |
| 2507.17718 | AI Telephone Surveying | Q7 | Telephone interview automation |
| 2412.17049 | Modular Conversational Agents for Surveys | Q7 | Social/transportation surveys |
| 2501.05985 | LLMs for Questionnaire Generation | Q7 | Questionnaire design |
| 2512.08646 | QSTN: Questionnaire Inference Framework | Q7 | Questionnaire response generation |
| 2305.08271 | SmartProbe: Virtual Moderator | Q7 | Market research surveys |
| 2505.01150 | AI-Driven Survey Question Generation | Q7 | Educational questionnaire research |
| 2502.20140 | Telephone Surveys Meet Conversational AI | Q7 | Telephone survey methodology |
| 2509.15568 | LiteLong: Long-Context Data Synthesis | Q8 | Long-context training data synthesis, not survey writing |
| 2410.09141 | ACER: Automatic Context Extension via Retrieval | Q8 | Context extension for LMs, not survey gen |
| 2405.10040 | SynthesizRR: Dataset Synthesis | Q8 | Classification dataset synthesis |
| 2502.15592 | Context Synthesis for Instruction Tuning | Q8 | Instruction tuning data |
| 2502.16684 | WildLong: Long-Context Instruction Data | Q8 | Instruction data synthesis |
| 2512.02589 | PaperDebugger: In-Editor Writing Assistant | Q6 | Paper editing environment, not survey generation |
| 2510.19600 | AutoPage: Paper-to-Page | Q6 | Conference webpage generation |
| 2502.09577 | Polymind: Parallel Diagramming | Q6 | Prewriting diagramming tool |
| 2508.17489 | Dynamic Collaborative Document Writing | Q6 | Voting/aggregation model for coauthoring theory |
| 2505.11336 | XtraGPT: Paper Revision | Q6 | Paper revision, not survey generation |
| 2512.01434 | CollabToolBuilder | Q6 | Tool-building framework |
| 2509.11826 | Collaborative Document Editing with AI Agents | Q6 | General collaborative editing in HCI |
| 2301.11223 | CitationSum: Citation-aware Summarization | Q4 | Single-paper summarization, not survey generation |
| 2402.13426 | Explaining Relationships Among Papers | Q4 | Citation text generation, not full surveys |
| 2306.03535 | SciLit: Literature Discovery Platform | Q4 | Literature discovery tool |
| 2302.07302 | CiteSee: Citation Augmentation | Q4 | Reading/annotation tool |
| 2411.15993 | Investigating Factuality in Long-Form Text | Q3 | Analysis of factuality decline, not evaluation framework |
| 2501.03200 | FACTS Grounding Leaderboard | Q3 | Grounding to provided context document, not citation eval |
| 2309.12455 | LongDocFACTScore | Q3 | Document summarization, not survey gen |

---

## Coverage Analysis

| Seed Dimension | New Candidates Addressing It | Gap Closed? |
|----------------|-----------------------------|-------------|
| **Method / Architecture** | STORM (E01), SciSage (E02), PaperQA (E03), OpenScholar (E04) | ✅ STORM and PaperQA — both named anchors in SurveySpec — now captured. SciSage adds reflect-when-you-write paradigm. |
| **Retrieval / RAG** | DeepResearch^{Eco} (E05), RAPID (E10), RAT (E11) | ⚠️ Recursive agentic retrieval (DeepResearch) and outline-first retrieval (RAPID) extend beyond seed RAG toolkit papers. |
| **Citation Attribution** | ALCE (E06), SAFE (E07), VERISCORE (E08), FaStFACT (E14), REC (E15) | ✅ Strong coverage of citation evaluation benchmarks and metrics. |
| **Evaluation / Benchmark** | ALCE (E06), SAFE (E07), VERISCORE (E08), FaStFACT (E14), Importance-Aware Recall (E13) | ✅ Evaluation methodology dimension substantially strengthened beyond FActScore + SurveyBench. |
| **Existing Surveys** | Related Work Generation Survey (E12) | ✅ Adds coverage of the RWG sub-literature, distinct from full-survey generation. |
| **Frontier / Open Challenges** | DimInd (E09) for interactive synthesis, DeepResearch (E05) for recursive depth | ⚠️ Interactive dimension partially filled by DimInd; user-steerable survey tools remain thin. |

---

## Summary

| Metric | Count |
|--------|-------|
| Queries executed | 8 |
| Total results inspected | 80 |
| New candidates (high signal) | **15** |
| Drift candidates (low signal) | 26 |
| Prior known (in 82-pool) | 39 |

### Top-5 Most High-Impact New Candidates

1. **STORM (2402.14207)** — Recovered the missing foundational anchor system named in SurveySpec.
2. **PaperQA (2312.07559)** — Recovered the missing RAG agent anchor named in SurveySpec.
3. **OpenScholar (2411.14199)** — Strong open-source retrieval-augmented LM; citation accuracy on par with human experts.
4. **SciSage (2506.12689)** — State-of-the-art multi-agent framework with SurveyScope benchmark.
5. **SAFE (2403.18802)** / **ALCE (2305.14627)** — Two foundational evaluation frameworks for long-form factuality and citation quality.

---

## Risks

1. **No `schema/expansion.md` or `schema/handoff.md` on disk** — SemanticExpander workflow inferred from SurveySpec, QueryPlan, and system prompt conventions.
2. **Query 7 (interactive/human-AI survey generation)** returned mostly questionnaire-survey methodology papers (AURA, SmartProbe, AI Telephone Surveying) rather than interactive literature survey generation systems. Only InteractiveSurvey (already in pool) was relevant. The interactive/frontier dimension remains thin despite DeepResearch^{Eco} and DimInd.
3. **Time range**: New candidate ALCE (2305.14627, 2023) and PaperQA (2312.07559, 2023) are at the lower bound of the 2023–2025 window. Retained for foundational citation evaluation and RAG agent status.
4. **Multi-agent writing queries (Q6)** returned many tangential papers (PaperDebugger, AutoPage, Polymind) that concern paper editing, conference webpages, or diagramming rather than survey generation. Only LiRA and MATC (already in pool) were directly relevant.
