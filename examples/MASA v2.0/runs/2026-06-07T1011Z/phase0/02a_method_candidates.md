# Method & Mechanism Candidate Pool

**run_dir**: `.`
**generated**: 2026-06-07T10:14:22+08:00
**scout_agent**: masa-method-scout

**Total unique candidates**: 48 (after deduplication across 6 queries)

---

## Candidate Entries

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 1 | 2510.07733 | 2025 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph for Automated Survey Generation | CM-01 | **method** | Core method — hierarchical citation graph with multi-agent validation; 3-layer taxonomy (Foundation/Development/Frontier) |
| 2 | 2509.18661 | 2025 | Agentic AutoSurvey: Let LLMs Survey LLMs | CM-01, CM-05 | **method** | Multi-agent (4 specialist agents) for survey generation; 12-dimension evaluation; processes 75–443 papers per topic |
| 3 | 2510.26012 | 2025 | AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys | CM-01, CM-05, M-02 | **method** | Multi-stage pipeline with parallel section generation, iterative refinement, real-time retrieval |
| 4 | 2411.06159 | 2024 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | CM-01 | **mechanism** | Knowledge minigraph construction + multi-path summarization via LLMs |
| 5 | 2402.01788 | 2024 | LitLLM: A Toolkit for Scientific Literature Review | CM-01 | **method** | RAG-based toolkit with keyword extraction, re-ranking, and related-work generation |
| 6 | 2403.02574 | 2024 | ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary | CM-01 | **mechanism** | Reflective Incremental Mechanism mimicking human workflow; G-Score metric |
| 7 | 2408.07884 | 2024 | Instruct Large Language Models to Generate Scientific Literature Survey Step by Step | CM-01 | **method** | Step-by-step prompt design: title → abstract → headings → content |
| 8 | 2406.10252 | 2024 | AutoSurvey: Large Language Models Can Automatically Write Surveys | CM-01, CM-05 | **method** | Foundational AutoSurvey pipeline: retrieval → outline → subsection drafting → integration → evaluation |
| 9 | 2504.08762 | 2025 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | CM-01, CM-05, M-02 | **method** | User-steerable generation; reference categorisation, outline refinement through interface |
| 10 | 2412.15249 | 2024 | LitLLMs, LLMs for Literature Review: Are we there yet? | CM-01 | **method** | Zero-shot decomposition: retrieval (keyword extraction → search → re-ranking) → planning → generation |
| 11 | 2507.15245 | 2025 | SPAR: Scholar Paper Retrieval with LLM-based Agents for Enhanced Academic Search | M-01 | **mechanism** | RefChain-based query decomposition and evolution for scholarly retrieval |
| 12 | 1805.02262 | 2018 | Construction of the Literature Graph in Semantic Scholar | M-01 | **theory** | Foundational infrastructure — 280M+ node citation graph for algorithmic discovery |
| 13 | 2402.08339 | 2024 | Interleaved snowballing: Reducing the workload of literature curators | M-01, M-04 | **mechanism** | Formalised snowballing algorithm with reduced curator workload via candidate filtering |
| 14 | 1205.1143 | 2012 | Recommendation on Academic Networks using Direction Aware Citation Analysis | M-01, M-04 | **mechanism** | Direction-aware citation recommendation (forward/backward tuning) with relevance feedback |
| 15 | 2106.01560 | 2021 | CitationIE: Leveraging the Citation Graph for Scientific Information Extraction | M-01, M-04 | **mechanism** | Graph-augmented text representations for scientific IE |
| 16 | 2209.13243 | 2022 | IdeaReader: A Machine Reading System for Understanding the Idea Flow of Scientific Publications | M-01 | **mechanism** | Reference/citation clustering → key paper extraction → literature review generation |
| 17 | 2306.03535 | 2023 | SciLit: A Platform for Joint Scientific Literature Discovery, Summarization and Citation Generation | M-01, M-03 | **mechanism** | Two-stage pre-fetch + re-ranking for literature search; abstractive citing sentence generation |
| 18 | 2302.07302 | 2023 | CiteSee: Augmenting Citations in Scientific Papers with Persistent and Personalized Historical Context | M-01 | **mechanism** | Personalised historical context for inline citation understanding |
| 19 | 2408.02508 | 2024 | PUREsuggest: Citation-based Literature Search and Visual Exploration with Keyword-controlled Rankings | M-01 | **mechanism** | Citation-based suggestions with visual explanations and keyword steering |
| 20 | 2005.11401 | 2020 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks | CM-04 | **method** | Foundational RAG paper — parametric + non-parametric memory for generation |
| 21 | 2407.01796 | 2024 | Ground Every Sentence: Improving Retrieval-Augmented LLMs with Interleaved Reference-Claim Generation | CM-04 | **mechanism** | Sentence-level attribution in RAG; ReClaim method for fine-grained citations |
| 22 | 2406.11460 | 2024 | TRACE the Evidence: Constructing Knowledge-Grounded Reasoning Chains for Retrieval-Augmented Generation | CM-04 | **mechanism** | Knowledge-grounded reasoning chains for multi-hop QA with RAG |
| 23 | 2403.05313 | 2024 | RAT: Retrieval Augmented Thoughts Elicit Context-Aware Reasoning in Long-Horizon Generation | CM-04 | **mechanism** | Iterative CoT revision with retrieval; relevant for iterative survey refinement |
| 24 | 2304.14732 | 2023 | Search-in-the-Chain: Interactively Enhancing Large Language Models with Search for Knowledge-intensive Tasks | CM-04 | **mechanism** | Chain-of-Query with IR verification; LLM-IR interaction loop |
| 25 | 2410.11217 | 2024 | On the Capacity of Citation Generation by Large Language Models | CM-04 | **mechanism** | Citation quality evaluation metrics; Generate-then-Refine for citation enhancement |
| 26 | 2601.14949 | 2026 | What Should I Cite? A RAG Benchmark for Academic Citation Prediction (CiteRAG) | CM-04 | **benchmark** | First comprehensive RAG benchmark for citation prediction; multi-level retrieval |
| 27 | 2504.00824 | 2025 | ScholarCopilot: Training Large Language Models for Academic Writing with Accurate Citations | CM-04 | **method** | Dynamic retrieval-token-gated citation generation; Qwen-2.5-7B trained on 500K arXiv papers |
| 28 | 2510.13095 | 2025 | Retrieval-in-the-Chain: Bootstrapping Large Language Models for Generative Retrieval | CM-04 | **mechanism** | Reasoning-augmented GR with structured CoT and iterative refinement |
| 29 | 2402.16063 | 2024 | Citation-Enhanced Generation for LLM-based Chatbots | CM-04 | **mechanism** | Post-hoc citation generation with NLI-based verification; training-free plugin |
| 30 | 2510.21900 | 2025 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | CM-05, M-02 | **method** | Recurrent outline generation with planning agent; paper cards for faithful grounding |
| 31 | 2504.18496 | 2025 | Facets, Taxonomies, and Syntheses: Navigating Structured Representations in LLM-Assisted Literature Review (DimInd) | CM-05 | **method** | Multi-level compression (papers → tables → taxonomies → narrative); human evaluation with 23 researchers |
| 32 | 2306.11832 | 2023 | QuOTeS: Query-Oriented Technical Summarization | CM-05 | **mechanism** | Query-focused extractive summarization for scientific document composition |
| 33 | 2511.17689 | 2025 | ARISE: Agentic Rubric-Guided Iterative Survey Engine for Automated Scholarly Paper Generation | CM-05 | **method** | Modular LLM agents with rubric-guided iterative refinement loop; quality score 92.48 |
| 34 | 2408.16444 | 2024 | SurveySum: A Dataset for Summarizing Multiple Scientific Articles into a Survey Section | CM-05, M-02 | **benchmark** | Dataset and pipelines for multi-article survey section summarization |
| 35 | 2503.04629 | 2025 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | CM-05, M-02 | **method** | Outline heuristic analysis + memory-driven generation + SurveyBench evaluation |
| 36 | 1806.00089 | 2018 | Cascading Citation Expansion | M-04 | **mechanism** | Iterative citation expansion via Dimensions API; coverage maximisation through successive expansion |
| 37 | 2512.22159 | 2025 | Oignon: Citation Graph Tool | M-04 | **mechanism** | Open-source citation graph tool with dual-path recency-weighted ranking |
| 38 | 2110.06595 | 2021 | Refcat: The Internet Archive Scholar Citation Graph | M-04 | **theory** | Large-scale citation graph dataset (1.3B citations) with exact/fuzzy matching |
| 39 | 1208.5464 | 2012 | Finding Communities in Site Web-Graphs and Citation Graphs | M-04 | **theory** | Fast community detection algorithm for citation graphs |
| 40 | 1501.04894 | 2015 | CITEX: A new citation index to measure the relative importance of authors and papers | M-04 | **theory** | Iterative author/paper scoring algorithm on publication graph |
| 41 | 1812.11252 | 2018 | Towards Finding Non-obvious Papers: An Analysis of Citation Recommender Systems | M-04 | **mechanism** | Power-law analysis of citation projection graphs; non-obvious paper recommendation |
| 42 | 2311.06785 | 2023 | Depth and Breadth of Research Area Coverage and Its Impact on Publication Citation | M-04 | **theory** | Study of how coverage breadth/depth affects citation impact |
| 43 | 2508.17647 | 2025 | SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models | M-02 | **method** | QUAL-SG quality-aware RAG framework; 4,200 human-written surveys dataset |
| 44 | 2506.12689 | 2025 | SciSage: A Multi-Agent Framework for High-Quality Scientific Survey Generation | M-02 | **method** | Reflect-when-you-write paradigm with hierarchical Reflector agent; SurveyScope benchmark |
| 45 | 2508.14317 | 2025 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing | M-02 | **method** | Coarse-to-fine retrieval; memory-guided generation for coherence across subsections |
| 46 | 2502.14776 | 2025 | SurveyX: Academic Survey Automation via Large Language Models | M-02 | **method** | AttributeTree preprocessing + online retrieval + re-polishing; citation quality enhancement |
| 47 | 1708.00247 | 2017 | Query expansion techniques for information retrieval: A survey | M-03 | **theory** | Comprehensive survey of QE techniques (1960–2017); taxonomy and methodology |
| 48 | 2301.11069 | 2023 | BERT-Embedding and Citation Network Analysis based Query Expansion Technique for Scholarly Search (QeBERT) | M-03 | **mechanism** | BERT-embedding + citation network analysis for pseudo-relevance feedback expansion |

---

## Query Coverage Summary

| Source Query | Queried | Unique Candidates |
|-------------|---------|------------------|
| CM-01 (LLM agents for survey gen) | ✓ | 10 (#1–10) |
| CM-04 (RAG + citation chaining) | ✓ | 10 (#20–29) |
| CM-05 (Iterative retrieval/synthesis) | ✓ | 6 (#30–35) + 5 overlap |
| M-01 (Forward/backward chaining) | ✓ | 9 (#11–19) |
| M-02 (Citation-aware evidence aggregation) | ✓ | 6 (#43–46, #33–35 overlap) |
| M-03 (Query expansion strategies) | ✓ | 2 (#47–48) + overlapped #17 |
| M-04 (Citation graph expansion) | ✓ | 7 (#36–42) + 3 overlap |

---

## Risks & Notes

- Several older papers (2012, 2015, 2017, 2018) captured from M-03 and M-04 queries — these are theory/infrastructure works rather than current method papers. They may be demoted during scoring.
- The multi-query strategy successfully cross-validated: 6 papers appear across multiple source queries (SurveyG, Agentic AutoSurvey, AutoSurvey2, InteractiveSurvey, SurveyForge, IterSurvey).
- Top systems captured: **AutoSurvey** family (3 variants), **SurveyG**, **SurveyForge**, **SurveyX**, **ARISE**, **SciSage**, **SurveyGen**, **IterSurvey**, **InteractiveSurvey**, **ScholarCopilot**, **ChatCite**, **LitLLM**, **SPAR**, and citation infrastructure works (**Semantic Scholar graph**, **Refcat**, **PUREsuggest**).
