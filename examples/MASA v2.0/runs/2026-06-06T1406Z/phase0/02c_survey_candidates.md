# Survey Candidates

**run_dir**: `.`
**topic**: automated literature survey agents with citation graph expansion
**date**: 2026-06-06

## Surveys / Reviews / Taxonomies

Papers that are themselves surveys, reviews, tutorials, or taxonomies relevant to the automated literature survey + citation graph expansion topic.

| # | arXiv ID | Year | Type | Title | Relevance |
|---|----------|------|------|-------|-----------|
| 1 | 2402.08565v2 | 2024 | **review** | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | Reviews AI tools for semi-automating SLR; covers screening & extraction phases across 21+11 tools. Strong overview of state-of-play for LLM-assisted review automation. |
| 2 | 2501.10326v2 | 2025 | **survey** | Large language models for automated scholarly paper review: A survey | Surveys LLM-based automated scholarly paper review (ASPR); covers methods, datasets, online systems, performance, and challenges. Adjacent to our topic but focused on paper *review* rather than *survey generation*. |
| 3 | 2409.04600v1 | 2024 | **review** | The emergence of Large Language Models (LLM) as a tool in literature reviews: an LLM automated systematic review | LLM-assisted systematic review of 172 papers on using LLMs for literature reviews. Finds ChatGPT/GPT most dominant (73.2%), most automation at search (34.9%) and data extraction (31.4%) stages. |
| 4 | 2412.15249v2 | 2024 | **review** | LitLLMs, LLMs for Literature Review: Are we there yet? | Zero-shot eval of LLMs for retrieval + writing literature reviews; proposes keyword-extraction & re-ranking pipeline. Assesses current capabilities and gaps. |
| 5 | 2508.06401v3 | 2025 | **survey** | A Systematic Literature Review of Retrieval-Augmented Generation: Techniques, Metrics, and Challenges | Systematic review of 128 RAG papers (2020–2025). Catalogs datasets, architectures, evaluation practices. Relevant as context for RAG-based survey agents. |
| 6 | 2402.19473v6 | 2024 | **survey** | Retrieval-Augmented Generation for AI-Generated Content: A Survey | Broad RAG survey covering augmentation methodologies, applications across modalities, benchmarks, and limitations. Foundational reading for RAG component of survey agents. |
| 7 | 2202.01110v2 | 2022 | **survey** | A Survey on Retrieval-Augmented Text Generation | Earlier RAG survey; covers generic paradigm, tasks (dialogue, MT, other generation). Dated but useful for tracing RAG foundations. |
| 8 | 2002.06961v2 | 2020 | **survey** | Citation Recommendation: Approaches and Datasets | First dedicated survey on citation recommendation. Covers approaches, datasets, evaluation methods. Relevant for citation graph *retrieval* component of survey agents. |
| 9 | 2306.14905v1 | 2023 | **taxonomy** | PRISMA-DFLLM: An Extension of PRISMA for Systematic Literature Reviews using Domain-specific Finetuned Large Language Models | Proposes a methodological framework extending PRISMA with domain-finetuned LLMs. Provides reporting guidelines checklist. Relevant as a taxonomy for structured SLR workflows that could be automated. |
| 10 | 2510.03120v1 | 2025 | **taxonomy/benchmark** | SurveyBench: How Well Can LLM(-Agents) Write Academic Surveys? | Fine-grained evaluation framework for automated survey generation; features 11,343 arXiv papers, 4,947 surveys, multi-faceted metrics (outline quality, content quality, non-textual richness). Essential benchmark paper. |

## Related Overview / System Papers (non-survey but citation-worthy)

| # | arXiv ID | Year | Type | Title | Notes |
|---|----------|------|------|-------|-------|
| 11 | 2406.10252v2 | 2024 | **system** | AutoSurvey: Large Language Models Can Automatically Write Surveys | Foundational system paper for automated survey generation; outline → section drafting → integration pipeline. |
| 12 | 2510.26012 | 2025 | **system** | AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys | Follow-up with parallel section generation, iterative refinement, real-time retrieval, multi-LLM evaluation. |
| 13 | 2509.18661v1 | 2025 | **system** | Agentic AutoSurvey: Let LLMs Survey LLMs | Multi-agent framework (Paper Search, Topic Mining, Writer, Quality Evaluator). Scored 8.18/10 vs AutoSurvey's 4.77/10. |
| 14 | 2510.21900 | 2025 | **system** | Deep Literature Survey Automation with an Iterative Workflow | Recurrent outline generation with paper cards; planning agent incrementally retrieves/reads/updates. Survey-Arena pairwise benchmark. |
| 15 | 2503.04629v1 | 2025 | **system** | SurveyForge: On the Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | Outline analysis of human-written surveys + scholar navigation agent for memory-driven retrieval. |
| 16 | 2402.01788v2 | 2024 | **system** | LitLLM: A Toolkit for Scientific Literature Review | RAG-based toolkit: web search → keyword extraction → paper re-ranking → related work generation. |
| 17 | 2411.14199v1 | 2024 | **system** | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | 45M paper datastore, self-feedback inference loop, ScholarQABench. Outperforms GPT-4o by 5%. |
| 18 | 2312.07559v2 | 2023 | **system** | PaperQA: Retrieval-Augmented Generative Agent for Scientific Research | RAG agent for science QA; LitQA benchmark. Early influential paper in RAG-for-science. |
| 19 | 2110.06354v3 | 2021 | **dataset/task** | Tell Me How to Survey: Literature Review Made Simple with Automatic Reading Path Generation | Introduces Reading Path Generation (RPG) task + SurveyBank dataset. Graph-optimization-based approach. |
| 20 | 2504.08762v1 | 2025 | **system** | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | Interactive system allowing user customization of reference categorization, outline, and content. |
| 21 | 2407.13993v3 | 2024 | **system** | LLAssist: Simple Tools for Automating Literature Review Using Large Language Models | Open-source tool for extracting info and evaluating relevance to user-defined research questions. |
| 22 | 2408.02508v1 | 2024 | **system** | PUREsuggest: Citation-based Literature Search and Visual Exploration with Keyword-controlled Rankings | Citation-based suggestion + visual exploration; keyword-steerable ranking. Useful for citation traversal component. |
| 23 | 1805.02262v1 | 2018 | **infrastructure** | Construction of the Literature Graph in Semantic Scholar | Describes Semantic Scholar's 280M-node literature graph. Foundational infrastructure paper. |
| 24 | 1404.5322v1 | 2014 | **tool** | CitNetExplorer: A new software tool for analyzing and visualizing citation networks | Citation network analysis and visualization tool. Dated but relevant for citation network methodology. |
| 25 | 2502.16868v1 | 2025 | **system** | Graphy: Towards End-to-End Modeling, Exploring and Generating Report from Raw Data | End-to-end platform that transforms raw documents into structured graph of Fact/Dimension nodes. Offline Scrapper + online Surveyor. |

## Summary Statistics

| Category | Count |
|----------|-------|
| Surveys | 4 |
| Reviews | 3 |
| Taxonomies / Benchmarks | 2 |
| Related Overview (non-survey) | 16 |
| **Total candidates** | **25** |

## Notes for ReaderAgent

- The **survey/review papers** (#1–#8) are the most important citation seeds. They provide background, taxonomy, and related work for framing the survey brief.
- **SurveyBench** (#10) and **AutoSurvey/AutoSurvey2** (#11, #12) are the primary benchmark and baseline system for the evaluation dimension.
- **Agentic AutoSurvey** (#13) and **Deep Literature Survey** (#14) represent the latest multi-agent / iterative paradigms — prioritize these for frontier coverage.
- **OpenScholar** (#17) is particularly strong on citation accuracy and datastore construction at scale.
- Boundary items (e.g., Citation Recommendation survey #8, CitNetExplorer #24) are included to provide contrast and context but are not core to the primary anchor question.
