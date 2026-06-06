# MethodScout — Method & Mechanism Candidate Pool

**run_dir**: `.`
**source_plan**: `01_query_plan.md`
**generated**: 2026-06-06T12:35:18+08:00
**total_candidates**: 37 (after deduplication of repeated titles)

---

## Core Method Queries (3 queries × topK = 10)

### CM-01: "LLM agent architectures for automated survey generation"

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 1 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | CM-01 | core_method | Multi-agent framework (4 agents) for automated survey generation; scores 8.18/10 vs AutoSurvey's 4.77/10 |
| 2 | 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph | 2025 | CM-01 | core_method | Hierarchical citation graph with 3-layer organisation (Foundation/Development/Frontier) |
| 3 | 2510.10890 | LLM×MapReduce-V3: Enabling Interactive In-Depth Survey Generation through MCP-Driven Modular Agent System | 2025 | CM-01 | core_method | Hierarchically modular agent system using MCP servers; human-in-the-loop |
| 4 | 2503.04629v1 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | 2025 | CM-01 | core_method | Outline heuristics from human-written outlines + scholar navigation agent for retrieval |
| 5 | 2502.14776v2 | SurveyX: Academic Survey Automation via Large Language Models | 2025 | CM-01 | core_method | Two-phase system (Preparation+Generation); AttributeTree pre-processing; re-polishing |
| 6 | 2406.10252v2 | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | CM-01 | citation_seed | Foundational system; retrieval + outline generation + subsection drafting by specialized LLMs |
| 7 | 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing | 2025 | CM-01 | core_method | Coarse-to-fine retrieval; adaptive planning; memory mechanism for coherence across subsections |
| 8 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | 2025 | CM-01 | core_method | User-customizable intermediate components; online retrieval + user uploads |

### CM-02: "Multi-agent systems for academic literature review writing"

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 9 | 2510.05138 | LiRA: A Multi-Agent Framework for Reliable and Readable Literature Review Generation | 2025 | CM-02 | core_method | Multi-agent workflow emulating human literature review; outperforms AutoSurvey and MASS-Survey |
| 10 | 2603.14629 | ResearchPilot: A Local-First Multi-Agent System for Literature Synthesis and Related Work Drafting | 2026 | CM-02 | core_method | Open-source self-hostable multi-agent system; DSPy, SQLite, Qdrant architecture |
| 11 | 2508.04306v1 | Multi-Agent Taskforce Collaboration: Self-Correction of Compounding Errors in Long-Form Literature Review Generation | 2025 | CM-02 | mechanism | MATC framework; three collaboration paradigms (exploration, exploitation, experience) for error mitigation |
| 12 | 2411.06159v3 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | 2024 | CM-02 | mechanism | Collaborative knowledge minigraph agents (CKMAs); prompt-based KMCA and MPSA agents |
| 13 | 2505.19647v1 | Select, Read, and Write: A Multi-Agent Framework of Full-Text-based Related Work Generation | 2025 | CM-02 | mechanism | Selector/Reader/Writer agents with graph-aware reading order strategies |
| 14 | 2504.14822v2 | Completing A Systematic Review in Hours instead of Months with Interactive AI Agents | 2025 | CM-02 | frontier | InsightAgent; human-centered interactive agent with visualisation; medical domain |

### CM-03: "Retrieval-augmented generation for survey paper composition"

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 15 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys | 2025 | CM-03 | core_method | Multi-stage pipeline with parallel section generation, iterative refinement, and real-time retrieval |
| 16 | 2508.17647v1 | SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models | 2025 | CM-03 | benchmark | Large-scale dataset (4,200+ surveys); QUAL-SG framework with quality-aware retrieval indicators |
| 17 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow | 2025 | CM-03 | core_method | Recurrent outline generation; paper cards; review-and-refine loop; Survey-Arena benchmark |
| 18 | 2402.01788v2 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | CM-03 | mechanism | RAG-based toolkit; keyword extraction from user abstracts; re-ranking pipeline |
| 19 | 2504.10861v2 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution | 2025 | CM-03 | boundary | Scientific QA system with attribution; not survey generation but relevant synthesis method |

---

## Mechanism Queries (3 queries × topK = 10)

### M-01: "Citation-grounded evidence extraction and attribution in RAG"

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 20 | 2510.11394 | VeriCite: Towards Reliable Citations in Retrieval-Augmented Generation via Rigorous Verification | 2025 | M-01 | mechanism | Three-stage citation verification: initial generation, NLI verification, evidence selection, answer refinement |
| 21 | 2407.01796v2 | Ground Every Sentence: Improving Retrieval-Augmented LLMs with Interleaved Reference-Claim Generation (ReClaim) | 2024 | M-01 | mechanism | Sentence-level citations via interleaved reference-claim generation; 90% citation accuracy |
| 22 | 2406.13663v4 | MIRAGE: Model Internals-based Answer Attribution for Trustworthy RAG | 2024 | M-01 | mechanism | Plug-and-play attribution using model internals; detects context-sensitive tokens via saliency methods |
| 23 | 2509.20859v1 | Concise and Sufficient Sub-Sentence Citations for Retrieval-Augmented Generation | 2025 | M-01 | mechanism | Sub-sentence citations for RAG; credit model filtering; fine-tuning dataset |
| 24 | 2410.11217v1 | On the Capacity of Citation Generation by Large Language Models | 2024 | M-01 | mechanism | Systematic analysis of LLM citation generation; Generate-then-Refine method; new evaluation metrics |
| 25 | 2506.07671v1 | GaRAGe: A Benchmark with Grounding Annotations for RAG Evaluation | 2025 | M-01 | benchmark | Large RAG benchmark (2,366 questions, 35K+ annotations); fine-grained grounding evaluation |
| 26 | 2505.16415v2 | ARC-JSD: Attributing Response to Context — A Jensen-Shannon Divergence Driven Mechanistic Study | 2025 | M-01 | mechanism | JSD-driven context attribution without fine-tuning; identifies attention heads and MLP layers responsible |

### M-03: "Iterative refinement and multi-pass drafting for long-form generation"

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 27 | 2502.12568v3 | CogWriter: A Cognitive Writing Perspective for Constrained Long-Form Text Generation | 2025 | M-03 | mechanism | Cognitive Writing Theory applied to LLMs; hierarchical planning + parallel generation + monitoring/reviewing |
| 28 | 2506.04180v1 | SuperWriter: Reflection-Driven Long-Form Generation with LLMs | 2025 | M-03 | mechanism | SuperWriter-Agent with structured thinking-through; hierarchical DPO + MCTS for optimisation |
| 29 | 2303.17651v2 | Self-Refine: Iterative Refinement with Self-Feedback | 2023 | M-03 | mechanism | Foundational iterative refinement approach; single LLM as generator, refiner, and feedback provider |
| 30 | 2310.08185v1 | EIPE-text: Evaluation-Guided Iterative Plan Extraction for Long-Form Narrative Text Generation | 2023 | M-03 | mechanism | QA-based evaluation mechanism for iterative plan extraction and refinement |

### M-05: "Citation graph expansion techniques for literature coverage"

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 31 | 2409.12177v1 | LitFM: A Retrieval Augmented Structure-aware Foundation Model For Citation Graphs | 2024 | M-05 | mechanism | First literature foundation model with graph retriever for citation graphs; 28.1% precision improvement |
| 32 | 2408.02508v1 | PUREsuggest: Citation-based Literature Search and Visual Exploration with Keyword-controlled Rankings | 2024 | M-05 | mechanism | Interactive citation-based suggestion system; keyword steering; visual exploration |
| 33 | 2106.01560v1 | CitationIE: Leveraging the Citation Graph for Scientific Information Extraction | 2021 | M-05 | mechanism | Citation graph for augmenting text representations in SciIE tasks |

---

## Supplementary: Additional papers found across queries (unique, not listed above)

| # | arXiv ID | Title (truncated) | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------------------|------|-------------|-------------|-----------------|
| 34 | 2503.08506v3 | ReviewAgents: Bridging the Gap Between Human and AI-Generated Paper Reviews | 2025 | CM-02 | boundary | Paper review generation framework; relevant as adjacent task with similar multi-agent patterns |
| 35 | 2412.10571v3 | RAGONITE: Evidence Contextualization and Counterfactual Attribution for Conversational QA | 2024 | M-01 | mechanism | Counterfactual attribution for RAG; ConfQuestions benchmark |
| 36 | 2412.14457v1 | VISA: RAG with Visual Source Attribution | 2024 | M-01 | mechanism | Visual source attribution with bounding boxes in document screenshots |
| 37 | 2401.04259v1 | MARG: Multi-Agent Review Generation for Scientific Papers | 2024 | CM-02 | boundary | Multi-agent paper review generation; multi-LLM discussion for feedback |

---

## Summary

| Type | Count | Notes |
|------|-------|-------|
| Core method (architectures) | 14 | Agentic AutoSurvey, SurveyG, SurveyForge, SurveyX, LiRA, etc. |
| Mechanism (attribution, refinement, graphs) | 16 | VeriCite, ReClaim, MIRAGE, CogWriter, Self-Refine, LitFM, etc. |
| Benchmark/dataset | 2 | GaRAGe, SurveyGen dataset |
| Boundary/adjacent | 3 | ReviewAgents, MARG, Ai2 Scholar QA |
| **Total unique candidates** | **37** | After deduplication of repeated titles |

**Papers overlapping with citation seeds from spec**: AutoSurvey (2406.10252v2)

**Notable new named systems not in original spec**: Agentic AutoSurvey, SurveyG, SurveyForge, SurveyX, SurveyGen-I, LiRA, CogWriter, SuperWriter, LitFM, VeriCite, ReClaim, MIRAGE, InteractiveSurvey, AutoSurvey2
