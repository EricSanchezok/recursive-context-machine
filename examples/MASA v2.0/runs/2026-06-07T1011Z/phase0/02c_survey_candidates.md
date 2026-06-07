# Survey Candidates — SurveyScout

**run_dir**: `.`
**source queries**: S-01 ("Survey of AI-powered literature review"), S-02 ("Taxonomy of automated academic writing"), plus 2 supplementary survey-oriented queries
**generated**: 2026-06-07T10:13:24+08:00
**total candidates**: 26 (across 4 queries, topK=10 each, deduplicated)

---

## Existing Surveys & Reviews (anchor citations)

These papers survey the field of automated literature review / survey generation itself. They are the primary citation seeds for the MASA survey.

| # | arXiv ID | Title | Type | Notes |
|---|----------|-------|------|-------|
| 1 | 2402.08565v2 | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | **survey** | Comprehensive review of AI in SLRs; analyses 21 SLR tools + 11 recent LLM tools; taxonomy of 23 traditional + 11 AI features |
| 2 | 2401.10917v1 | Artificial intelligence to automate the systematic review of scientific literature | **survey** | Historical survey of AI techniques over 15 years for SLR automation; 34 primary studies reviewed |
| 3 | 2501.04306v1 | LLM4SR: A Survey on Large Language Models for Scientific Research | **survey** | Systematic survey of LLMs across 4 research stages: hypothesis discovery, experiment planning, scientific writing, peer reviewing |
| 4 | 2409.04600v1 | The emergence of LLMs as a tool in literature reviews: an LLM automated systematic review | **review** | LLM-assisted systematic review (3,788 articles → 172 eligible); finds ChatGPT/GPT most dominant architecture (73.2%) |
| 5 | 2504.18496v1 | Facets, Taxonomies, and Syntheses: Navigating Structured Representations in LLM-Assisted Literature Review | **taxonomy** | Interactive system (DimInd) for faceted comparison, taxonomy construction, narrative synthesis from large paper collections |

---

## Core Automated Survey Systems (method papers with citation potential)

These are the primary architectural contributions — each proposes a new automated survey pipeline. They are `method` papers that the MASA survey will analyse and compare.

| # | arXiv ID | Title | Type | Key Innovation |
|---|----------|-------|------|----------------|
| 6 | 2406.10252v2 | AutoSurvey: Large Language Models Can Automatically Write Surveys | **method** | Systematic retrieval + outline gen + subsection drafting by specialised LLMs + integration/refinement |
| 7 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys | **method** | Multi-stage pipeline with parallel section gen, iterative refinement, real-time retrieval, multi-LLM evaluation |
| 8 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | **method** | 4-agent framework (Paper Search, Topic Mining, Writer, Evaluator); scored 8.18/10 vs AutoSurvey's 4.77/10 |
| 9 | 2502.14776v2 | SurveyX: Academic Survey Automation via Large Language Models | **method** | Two-phase (Preparation + Generation) with AttributeTree preprocessing and re-polishing |
| 10 | 2503.04629v1 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | **method** | Outline by analysing human-written patterns + scholar navigation agent for memory-driven retrieval |
| 11 | 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing | **method** | Coarse-to-fine retrieval + adaptive planning + memory mechanism for cross-subsection coherence |
| 12 | 2508.17647v1 | SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models | **method** | 4,200+ human-written survey dataset; QUAL-SG framework with quality-aware retrieval indicators |
| 13 | 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph for Automated Survey Generation | **method** | Hierarchical citation graph (Foundation/Development/Frontier layers) + horizontal/vertical traversal |
| 14 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | **method** | Recurrent outline generation; paper cards for paper-level grounding; review-and-refine loop |
| 15 | 2509.19370v1 | Meow: End-to-End Outline Writing for Automatic Academic Survey | **method** | First metadata-driven outline writing framework; two-stage SFT + RL training for outline generation |
| 16 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | **method** | Personalised/interactive; users customise references, outline, content through interface |
| 17 | 2410.15978v2 | PROMPTHEUS: A Human-Centered Pipeline to Streamline SLRs with LLMs | **method** | End-to-end SLR automation: systematic search + extraction + BERTopic + summarisation |
| 18 | 2408.07884v1 | Instruct Large Language Models to Generate Scientific Literature Survey Step by Step | **method** | Step-by-step prompt design (title → abstract → headings → content); low cost (0.1 RMB/survey) |
| 19 | 2504.14822v2 | InsightAgent: Completing A Systematic Review in Hours instead of Months with Interactive AI Agents | **method** | Multi-agent + visualisation; user study with 9 medical professionals; 79.7% human-quality in 1.5h |
| 20 | 2402.01788v2 | LitLLM: A Toolkit for Scientific Literature Review | **method** | RAG-based toolkit; web search → re-ranking → related work section generation |
| 21 | 2403.08399v1 | System for systematic literature review using multiple AI agents: Concept and an empirical evaluation | **method** | Multi-AI agent model for full SLR automation; search string → retrieval → filtering → summarisation |
| 22 | 2411.18583v1 | Automated Literature Review Using NLP Techniques and LLM-Based RAG | **method** | Compares spaCy, T5, GPT-3.5 RAG for review generation; GUI for best system |
| 23 | 2407.20906v5 | Automated Review Generation Method Based on Large Language Models | **method** | Applied to PDH catalysis domain; 343 articles analysed; hallucination risk <0.5% verified |
| 24 | 2312.09948v1 | GEAR-Up: Generative AI and External Knowledge-based Retrieval Upgrading Scholarly Article Searches | **method** | Query expansion using LLMs + knowledge graphs for systematic review search |

---

## Evaluation & Benchmark Papers

| # | arXiv ID | Title | Type | Notes |
|---|----------|-------|------|-------|
| 25 | 2510.03120v2 | SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs? | **benchmark** | 11,343 arXiv papers + 4,947 surveys; quiz-driven evaluation; outline/content/non-textual richness metrics |

---

## Key Findings

1. **Rich survey literature exists**: The field has at least 5 dedicated surveys/taxonomies (papers #1–5) that can serve as citation seeds and related-work sources.
2. **Architectural diversity**: Methods span single-agent pipelines (AutoSurvey, SurveyX), multi-agent frameworks (Agentic AutoSurvey, SurveyG), iterative workflows (IterSurvey), and human-in-the-loop systems (InteractiveSurvey, InsightAgent).
3. **Citation-graph-aware methods are rare**: Only SurveyG (#13) explicitly uses hierarchical citation graph traversal. Most systems rely on flat semantic retrieval.
4. **Evaluation is maturing**: SurveyBench (#25) and SurveyGen's dataset (#12) provide standardised benchmarks, addressing a key gap identified in the QueryPlan.
5. **Temporal coverage**: Most papers are from 2024–2025, indicating a very active research area.

## Deduplication Notes

- 2510.03120 appears in 3 versions (v1, v2, and no-version); unified as #25.
- 2510.26012, 2406.10252v2, 2509.18661v1, etc. appeared across multiple queries; each listed once.
- 2309.15004v1 (question generation from educational text) excluded — out of scope.
