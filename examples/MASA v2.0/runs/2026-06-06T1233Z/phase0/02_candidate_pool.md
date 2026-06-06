# CandidatePool — Merged Candidate Pool

**run_dir**: `.`
**generated**: 2026-06-06T12:36:53+08:00
**source_scouts**: MethodScout, BenchmarkScout, SurveyScout, FrontierScout
**source_scout_artifacts**: 02a_method_candidates.md, 02b_benchmark_candidates.md, 02c_survey_candidates.md, 02d_frontier_candidates.md
**deduplication**: arXiv ID (base ID without version suffix) first, then normalized title
**total_unique_candidates**: 87

---

## Legend

| Column | Description |
|--------|-------------|
| `#` | Sequential unique ID in this pool |
| `arXiv ID` | Base arXiv identifier (version suffix stripped for dedup) |
| `Title` | Short title |
| `Year` | Publication year |
| `likely_role` | Categorical role for downstream use (categorical only, no numeric scores) |
| `source_agent` | Which scout(s) contributed this candidate |
| `source_query` | Original query ID(s) from the QueryPlan |
| `notes` | Brief inclusion rationale |

### Role Categories

| Role | Meaning |
|------|---------|
| `core_method` | Primary architectural/system paper for automated survey generation |
| `mechanism` | Specific technique (attribution, refinement, retrieval, coordination) |
| `benchmark` | Dedicated evaluation benchmark, dataset, or metric for survey generation |
| `evaluation` | General evaluation framework or metric applicable to survey quality |
| `survey` | Existing survey/review/taxonomy paper of the field |
| `citation_seed` | Named system or tool retained for citation/reference purposes |
| `frontier` | Emerging direction (living surveys, interactive generation) |
| `boundary` | Adjacent area worth noting but likely out-of-scope for the final survey |

---

## Candidate Pool

| # | arXiv ID | Title | Year | likely_role | source_agent | source_query | notes |
|---|----------|-------|------|-------------|--------------|--------------|-------|
| 1 | 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | core_method | MethodScout, SurveyScout, FrontierScout | CM-01, CS, F-01 | Multi-agent 4-agent framework; scores 8.18/10 |
| 2 | 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph | 2025 | core_method | MethodScout, SurveyScout | CM-01, CS | 3-layer citation graph (Foundation/Development/Frontier) |
| 3 | 2510.10890 | LLM×MapReduce-V3: Interactive In-Depth Survey Generation via MCP | 2025 | core_method | MethodScout | CM-01 | Modular agent system using MCP servers; human-in-the-loop |
| 4 | 2503.04629 | SurveyForge: Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | 2025 | core_method | MethodScout, SurveyScout, BenchmarkScout | CM-01, CS | Outline heuristics + scholar navigation agent |
| 5 | 2502.14776 | SurveyX: Academic Survey Automation via Large Language Models | 2025 | core_method | MethodScout, SurveyScout | CM-01, CS | Two-phase system (Preparation+Generation); AttributeTree |
| 6 | 2406.10252 | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | core_method | MethodScout, SurveyScout, FrontierScout | CM-01, CS, F-01 | Foundational system; retrieval + outline + drafting |
| 7 | 2508.14317 | SurveyGen-I: Consistent Survey Generation with Evolving Plans | 2025 | core_method | MethodScout, SurveyScout | CM-01, CS | Coarse-to-fine retrieval; adaptive planning; memory mechanism |
| 8 | 2504.08762 | InteractiveSurvey: Personalized and Interactive Survey Generation | 2025 | core_method | MethodScout, FrontierScout | CM-01, F-01, F-02 | User-customizable intermediate components; online retrieval |
| 9 | 2510.05138 | LiRA: Multi-Agent Framework for Reliable and Readable Literature Review Generation | 2025 | core_method | MethodScout | CM-02 | Multi-agent workflow; outperforms AutoSurvey and MASS-Survey |
| 10 | 2603.14629 | ResearchPilot: Local-First Multi-Agent System for Literature Synthesis | 2026 | core_method | MethodScout | CM-02 | Open-source self-hostable; DSPy, SQLite, Qdrant |
| 11 | 2508.04306 | MATC: Multi-Agent Taskforce Collaboration for Long-Form Literature Review | 2025 | mechanism | MethodScout | CM-02 | Three collaboration paradigms for error mitigation |
| 12 | 2411.06159 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | 2024 | mechanism | MethodScout | CM-02 | Collaborative knowledge minigraph agents |
| 13 | 2505.19647 | Select, Read, and Write: Multi-Agent Full-Text Related Work Generation | 2025 | mechanism | MethodScout | CM-02 | Selector/Reader/Writer agents with graph-aware reading |
| 14 | 2504.14822 | Completing A Systematic Review in Hours with Interactive AI Agents | 2025 | frontier | MethodScout | CM-02 | InsightAgent; human-centered interactive; medical domain |
| 15 | 2510.26012 | AutoSurvey2: Next Level Automated Literature Surveys | 2025 | core_method | MethodScout, SurveyScout, FrontierScout | CM-03, CS, F-01 | Multi-stage pipeline; parallel section generation; real-time retrieval |
| 16 | 2508.17647 | SurveyGen: Quality-Aware Scientific Survey Generation with LLMs | 2025 | benchmark | MethodScout, SurveyScout, BenchmarkScout, FrontierScout | CM-03, CS, F-02 | 4,200+ human-written surveys dataset; QUAL-SG framework |
| 17 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | 2025 | core_method | MethodScout, SurveyScout, BenchmarkScout | CM-03, CS | Recurrent outline generation; Survey-Arena benchmark |
| 18 | 2402.01788 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | mechanism | MethodScout, SurveyScout, FrontierScout | CM-03, CS, F-01 | RAG-based toolkit; keyword extraction; re-ranking pipeline |
| 19 | 2504.10861 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution | 2025 | boundary | MethodScout | CM-03 | Scientific QA with attribution; not survey generation |
| 20 | 2510.11394 | VeriCite: Reliable Citations in RAG via Rigorous Verification | 2025 | mechanism | MethodScout | M-01 | Three-stage citation verification (NLI + evidence selection) |
| 21 | 2407.01796 | ReClaim: Ground Every Sentence with Interleaved Reference-Claim Generation | 2024 | mechanism | MethodScout | M-01 | Sentence-level citations; 90% citation accuracy |
| 22 | 2406.13663 | MIRAGE: Model Internals-based Answer Attribution for Trustworthy RAG | 2024 | mechanism | MethodScout | M-01 | Plug-and-play attribution using model internals; saliency methods |
| 23 | 2509.20859 | Concise and Sufficient Sub-Sentence Citations for RAG | 2025 | mechanism | MethodScout | M-01 | Sub-sentence citations; credit model filtering |
| 24 | 2410.11217 | On the Capacity of Citation Generation by Large Language Models | 2024 | mechanism | MethodScout | M-01 | Systematic analysis; Generate-then-Refine method |
| 25 | 2506.07671 | GaRAGe: Benchmark with Grounding Annotations for RAG Evaluation | 2025 | benchmark | MethodScout | M-01 | 2,366 questions, 35K+ annotations; grounding evaluation |
| 26 | 2505.16415 | ARC-JSD: Jensen-Shannon Divergence Driven Attribution Mechanistic Study | 2025 | mechanism | MethodScout | M-01 | JSD-driven context attribution without fine-tuning |
| 27 | 2502.12568 | CogWriter: Cognitive Writing Perspective for Constrained Long-Form Generation | 2025 | mechanism | MethodScout | M-03 | Hierarchical planning + parallel generation + monitoring |
| 28 | 2506.04180 | SuperWriter: Reflection-Driven Long-Form Generation with LLMs | 2025 | mechanism | MethodScout | M-03 | Structured thinking-through; hierarchical DPO + MCTS |
| 29 | 2303.17651 | Self-Refine: Iterative Refinement with Self-Feedback | 2023 | mechanism | MethodScout | M-03 | Foundational iterative refinement; single LLM loop |
| 30 | 2310.08185 | EIPE-text: Evaluation-Guided Iterative Plan Extraction for Long-Form Text | 2023 | mechanism | MethodScout | M-03 | QA-based evaluation for iterative plan extraction |
| 31 | 2409.12177 | LitFM: Retrieval Augmented Structure-aware Foundation Model for Citation Graphs | 2024 | mechanism | MethodScout | M-05 | First literature foundation model with graph retriever; 28.1% precision improvement |
| 32 | 2408.02508 | PUREsuggest: Citation-based Literature Search and Visual Exploration | 2024 | mechanism | MethodScout | M-05 | Interactive citation suggestion; keyword steering |
| 33 | 2106.01560 | CitationIE: Leveraging the Citation Graph for Scientific Information Extraction | 2021 | mechanism | MethodScout | M-05 | Citation graph for SciIE tasks |
| 34 | 2503.08506 | ReviewAgents: Bridging Human and AI-Generated Paper Reviews | 2025 | boundary | MethodScout | CM-02 | Paper review generation; adjacent multi-agent pattern |
| 35 | 2412.10571 | RAGONITE: Evidence Contextualization and Counterfactual Attribution for Conversational QA | 2024 | mechanism | MethodScout | M-01 | Counterfactual attribution for RAG; ConfQuestions benchmark |
| 36 | 2412.14457 | VISA: RAG with Visual Source Attribution | 2024 | mechanism | MethodScout | M-01 | Visual source attribution with bounding boxes in document screenshots |
| 37 | 2401.04259 | MARG: Multi-Agent Review Generation for Scientific Papers | 2024 | boundary | MethodScout | CM-02 | Multi-agent paper review generation; multi-LLM discussion |
| 38 | 2510.03120 | SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs? | 2025 | benchmark | BenchmarkScout, SurveyScout | B-01, S | Quiz-driven evaluation; 11,343 topics, 4,947 surveys; multifaceted metric hierarchy |
| 39 | 2508.15658 | SurGE: Benchmarking Computer Science Survey Generation | 2025 | benchmark | BenchmarkScout, SurveyScout | B-01, S | CS-domain; 1M+ retrieval pool; 4-dimension evaluation |
| 40 | 2512.02763 | SurveyEval: Comprehensive Evaluation of LLM-Generated Academic Surveys | 2025 | benchmark | BenchmarkScout, SurveyScout | B-01, S | 3-dimension; 7 subjects; LLM-as-a-Judge + human references |
| 41 | 2601.15307 | DeepSurvey-Bench: Evaluating Academic Value of Automatically Generated Surveys | 2026 | benchmark | BenchmarkScout | B-01 | Academic value dimensions (informational, scholarly, research guidance) |
| 42 | 2508.11310 | SGSimEval: Multifaceted Similarity-Enhanced Benchmark for ASG | 2025 | benchmark | BenchmarkScout, SurveyScout | B-01, S | LLM scoring + quantitative metrics + human preference |
| 43 | 2602.11238 | SurveyLens: Discipline-Aware Benchmark for Automatic Survey Generation | 2026 | benchmark | BenchmarkScout | B-01 | 1,000 human-written surveys across 10 disciplines; dual-lens evaluation |
| 44 | 2305.15186 | SciReviewGen: Large-scale Dataset for Automatic Literature Review Generation | 2023 | benchmark | BenchmarkScout | B-01 | 10,000+ reviews, 690,000 cited papers; reveals hallucination challenges |
| 45 | 2304.03512 | Hierarchical Catalogue Generation for Literature Review: A Benchmark | 2023 | benchmark | BenchmarkScout | B-01 | 7.6K catalogues, 389K reference papers; informativeness and structural similarity metrics |
| 46 | 2510.21192 | Gen-Review: Large-scale Dataset of AI-Generated (and Human-written) Peer Reviews | 2025 | benchmark | BenchmarkScout | B-01 | 81K LLM-generated reviews for ICLR 2018–2025; enables detection of AI-written reviews |
| 47 | 2305.14251 | FActScore: Fine-grained Atomic Evaluation of Factual Precision | 2023 | evaluation | BenchmarkScout | B-01, B-03 | Atomic fact decomposition; widely used for factual consistency in long-form text |
| 48 | 2401.15042 | PROXYQA: Alternative Framework for Evaluating Long-Form Text Generation with LLMs | 2024 | evaluation | BenchmarkScout | B-01, B-02 | Meta-questions with proxy-questions and pre-annotated answers; self-consistent |
| 49 | 2204.04991 | TRUE: Re-evaluating Factual Consistency Evaluation | 2022 | evaluation | BenchmarkScout | B-03 | Meta-evaluation of factual consistency metrics; 11 datasets; NLI + question-generation approaches |
| 50 | 2503.05712 | Automatic Evaluation Metrics for Artificially Generated Scientific Research | 2025 | evaluation | BenchmarkScout | B-01, B-02 | Citation count and review score prediction as auto-evaluation |
| 51 | 2305.11747 | HaluEval: Large-Scale Hallucination Evaluation Benchmark for LLMs | 2023 | evaluation | BenchmarkScout | B-03 | ChatGPT-based hallucination sampling; ~19.5% hallucination rate |
| 52 | 2501.08292 | HALoGEN: Fantastic LLM Hallucinations and Where to Find Them | 2025 | evaluation | BenchmarkScout | B-03 | 10,923 prompts across 9 domains; high-precision verifiers; error classification A/B/C |
| 53 | 2303.08896 | SelfCheckGPT: Zero-Resource Black-Box Hallucination Detection | 2023 | evaluation | BenchmarkScout | B-03 | Sampling-based hallucination detection; no external database; WikiBio dataset |
| 54 | 2412.15249 | LitLLMs: LLMs for Literature Review — Are we there yet? | 2024 | evaluation | BenchmarkScout, FrontierScout | B-02, F-01 | Zero-shot evaluation protocol; rolling test set to avoid contamination |
| 55 | 2310.04480 | Auto-survey Challenge | 2023 | evaluation | BenchmarkScout | B-02 | Competition framework for LLM survey generation and critique; simulated peer-review |
| 56 | 2306.17614 | Outcome-based Evaluation of Systematic Review Automation | 2023 | evaluation | BenchmarkScout | B-02 | Evaluates impact of included/excluded studies on review outcomes |
| 57 | 2402.08565 | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | 2024 | survey | SurveyScout | S-01 | Reviews 21 SLR tools + 11 LLM-based tools |
| 58 | 2401.10917 | Artificial intelligence to automate the systematic review of scientific literature | 2024 | survey | SurveyScout | S-01 | 15-year survey; 34 primary studies analyzed |
| 59 | 2503.01424 | From Hypothesis to Publication: Comprehensive Survey of AI-Driven Research Support | 2025 | survey | SurveyScout | S-01 | Broad survey covering knowledge synthesis, literature writing, and peer review |
| 60 | 2502.05151 | Transforming Science with LLMs: Survey on AI-assisted Scientific Discovery | 2025 | survey | SurveyScout | S-01 | LLM applications across research lifecycle including literature search and content generation |
| 61 | 2501.10326 | Large language models for automated scholarly paper review: A survey | 2025 | survey | SurveyScout | S-01 | Survey focused on automated peer review (adjacent but relevant for evaluation methodology) |
| 62 | 2409.04600 | The emergence of LLMs as a tool in literature reviews: an LLM automated systematic review | 2024 | survey | SurveyScout, FrontierScout | S-01, F-01 | Metareview of LLM usage in literature review creation; 172 studies |
| 63 | 2407.20906 | Automated Review Generation Method Based on Large Language Models | 2025 | citation_seed | SurveyScout | CS | Applied to propane dehydrogenation; hallucination <0.5% |
| 64 | 2411.18583 | Automated Literature Review Using NLP Techniques and LLM-Based RAG | 2024 | citation_seed | SurveyScout | CS | Compares spaCy, T5, GPT-3.5 for review generation |
| 65 | 2502.18791 | Can LLMs Help Uncover Insights about LLMs? Evolving Literature Analysis | 2025 | frontier | FrontierScout | F-01 | Semi-automated + updatable dataset for longitudinal tracking |
| 66 | 2408.13450 | vitaLITy 2: Reviewing Academic Literature Using Large Language Models | 2024 | frontier | FrontierScout | F-01 | RAG architecture for literature search and summarization |
| 67 | 2502.14743 | Multi-Agent Coordination across Diverse Applications: A Survey | 2025 | survey | FrontierScout | CD-01 | Survey of multi-agent coordination; identifies LLM-based MAS as promising direction |
| 68 | 2605.27466 | AgensFlow: Coordination-Policy Substrate for Multi-Agent Systems | 2026 | mechanism | FrontierScout | CD-01 | Learned routing for multi-agent coordination |
| 69 | 2507.17061 | Parallelism Meets Adaptiveness: Scalable Documents Understanding in Multi-Agent LLM Systems | 2025 | mechanism | FrontierScout | CD-01 | Dynamic task routing + bidirectional feedback |
| 70 | 2502.07350 | KABB: Knowledge-Aware Bayesian Bandits for Dynamic Expert Coordination | 2025 | mechanism | FrontierScout | CD-01 | Semantic understanding + dynamic expert selection |
| 71 | 2406.20041 | BMW Agents: Framework For Task Automation Through Multi-Agent Collaboration | 2024 | mechanism | FrontierScout | CD-01 | Multi-agent framework for complex task automation |
| 72 | 2509.20175 | Federation of Agents: Semantics-Aware Communication Fabric for Agentic AI | 2025 | mechanism | FrontierScout | CD-01 | Semantic routing + dynamic task decomposition |
| 73 | 2404.11943 | AgentCoord: Visually Exploring Coordination Strategy for LLM-based MAS | 2024 | mechanism | FrontierScout | CD-01 | Visual exploration for designing coordination strategies |
| 74 | 2603.22791 | ABSTRAL: Automatic Design of MAS Through Iterative Refinement | 2026 | mechanism | FrontierScout | CD-01 | Evolves MAS architecture as natural-language document via contrastive trace analysis |
| 75 | 2310.11511 | Self-RAG: Learning to Retrieve, Generate, and Critique through Self-Reflection | 2023 | mechanism | FrontierScout | CD-02 | Foundational Self-RAG; adaptive retrieval + reflection tokens for factuality |
| 76 | 2504.01018 | Self-Routing RAG: Binding Selective Retrieval with Knowledge Verbalization | 2025 | mechanism | FrontierScout | CD-02 | LLM dynamically decides external retrieval vs. parametric knowledge; 29% fewer retrievals |
| 77 | 2406.13779 | FoRAG: Factuality-optimized RAG for Web-enhanced Long-form QA | 2024 | mechanism | FrontierScout | CD-02 | Factuality optimization via doubly fine-grained RLHF |
| 78 | 2410.22954 | RA-RAG: RAG with Estimation of Source Reliability | 2024 | mechanism | FrontierScout | CD-02 | Cross-checks source reliability across multiple sources for robust RAG |
| 79 | 2505.10792 | Finetune-RAG: Fine-Tuning LLMs to Resist Hallucination in RAG | 2025 | mechanism | FrontierScout | CD-02 | Fine-tuning approach to resist hallucination under imperfect retrieval |
| 80 | 2406.13629 | InstructRAG: Instructing RAG via Self-Synthesized Rationales | 2024 | mechanism | FrontierScout | CD-02 | Explicit denoising via self-synthesized rationales; 8.3% accuracy improvement |
| 81 | 2411.01022 | Provenance: Light-weight Fact-checker for RAG Output | 2024 | mechanism | FrontierScout | CD-02 | NLI-based factuality checker for RAG output |
| 82 | 2005.11401 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks | 2020 | mechanism | FrontierScout | CD-02 | Foundational RAG paper — contextualizes Self-RAG/RankRAG era |
| 83 | 2411.14199 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | 2024 | core_method | extended_pool | Supplement-R2 | Retrieval-augmented scientific literature synthesis with 45M-paper datastore |
| 84 | 2504.18496 | DimInd: Facets, Taxonomies, and Syntheses for LLM-Assisted Literature Review | 2025 | core_method | extended_pool | Supplement-R2 | Facet-based LLM-assisted literature review with interactive synthesis |
| 85 | 2404.07738 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature | 2024 | frontier | extended_pool | Supplement-R2 | Multi-agent literature analysis + gap identification for scientific discovery |
| 86 | 2411.09255 | DAHL: Domain-specific Automated Hallucination Evaluation in Biomedicine | 2024 | evaluation | extended_pool | Supplement-R2 | Domain-specific hallucination evaluation for biomedical survey assessment |
| 87 | 2403.02574 | ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary | 2024 | frontier | extended_pool | Supplement-R2 | Interactive human-workflow-guided survey creation with conversational interface |
| 88 | 2402.14207 | STORM: Assisting in Writing Wikipedia-like Articles From Scratch with LLMs | 2024 | core_method | extended_pool | Supplement-R3 | Seminal hybrid system for knowledge-grounded long-form article generation; inspired survey generation systems |

---

## Summary Statistics

| Role | Count | Description |
|------|-------|-------------|
| core_method | 15 | Agentic AutoSurvey, SurveyG, SurveyForge, SurveyX, AutoSurvey, SurveyGen-I, InteractiveSurvey, LiRA, ResearchPilot, AutoSurvey2, IterSurvey, LLM×MapReduce-V3, OpenScholar, DimInd, STORM |
| mechanism | 34 | Attribution (VeriCite, ReClaim, MIRAGE, ARC-JSD, SubSentenceCit, CitCapacity); refinement (CogWriter, SuperWriter, Self-Refine, EIPE-text); citation graphs (LitFM, PUREsuggest, CitationIE); coordination (MATC, Select-Read-Write, Minigraph Agents, AgensFlow, BMW Agents, Federation of Agents, AgentCoord, ABSTRAL, Parallelism, KABB); RAG variants (Self-RAG, Self-Routing RAG, FoRAG, RA-RAG, Finetune-RAG, InstructRAG, Provenance, RAGONITE, VISA, RAG foundational); other (LitLLM) |
| benchmark | 11 | SurveyBench, SurGE, SurveyEval, DeepSurvey-Bench, SGSimEval, SurveyLens, SciReviewGen, HierCat, Gen-Review, GaRAGe, SurveyGen |
| evaluation | 11 | FActScore, PROXYQA, TRUE, AutoEvalMetrics, HaluEval, HALoGEN, SelfCheckGPT, LitLLMs, Auto-survey Challenge, Outcome-based Eval, DAHL |
| survey | 7 | AI for Lit Reviews, AI SLR automation, Hypothesis to Publication, Transforming Science with LLMs, LLM scholarly review survey, LLM meta-review, Multi-Agent Coordination Survey |
| frontier | 5 | Completing Systematic Review in Hours, Evolving Literature Analysis, vitaLITy 2, ResearchAgent, ChatCite |
| citation_seed | 2 | Automated Review Generation (propane), Automated Lit Review NLP |
| boundary | 3 | Ai2 Scholar QA, ReviewAgents, MARG |
| **Grand Total** | **88** | Unique candidates across all likely_role categories; some papers assigned one primary role despite appearing across multiple scouts |

---

## Risks

1. **`schema/candidate_pool.md` does not exist on disk** — the `schema` leaf is a flat file containing only the SurveySpec Contract. The CandidatePool format was inferred from the system prompt specification and conventions established by the scout artifacts (`02a_*`, `02b_*`, `02c_*`, `02d_*`).
2. **`schema/handoff.md` does not exist on disk** — handoff format inferred from system prompt requirements.
3. **Time-range boundary**: Some frontier and cross-domain candidates (RAG foundational 2005.11401, 2020; CitationIE 2106.01560, 2021) fall outside the spec's 2023–2025 window. Retained as contextual references; downstream scouts should filter by year if the time window is strict.
4. **Year normalization**: Several arXiv IDs lack explicit version suffixes. The `Year` column reflects the arXiv submission year noted in the scout artifact, which may differ from the "xx" prefix in the arXiv ID where the year format varies (e.g., 2601.* is 2026, 2204.* is 2022).
