# Survey Candidates

**run_dir**: `.`
**source**: `01_query_plan.md` (Survey Queries S-01, S-02 + supplementary survey-oriented queries)
**generated**: 2026-06-06T12:35:47+08:00

---

## Legend

| Tag | Meaning |
|-----|---------|
| REVIEW | A paper that reviews existing work in the area; may or may not be a systematic survey |
| SURVEY | A paper explicitly presenting itself as a survey of a field |
| TAXONOMY | A paper whose primary contribution is a classification or taxonomy |
| BENCHMARK | A paper proposing a benchmark dataset or evaluation framework |
| SYSTEM | A paper proposing a specific system/architecture for automated survey generation |

---

## Survey & Review Papers (6)

These are the papers that are themselves surveys, reviews, or taxonomies of automated literature review/survey generation.

| # | arXiv ID | Title | Type | Relevance |
|---|----------|-------|------|-----------|
| 1 | 2402.08565v2 | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | REVIEW | Directly surveys AI tools for systematic literature reviews; examines 21 SLR tools + 11 LLM-based tools |
| 2 | 2401.10917v1 | Artificial intelligence to automate the systematic review of scientific literature | SURVEY | Surveys AI techniques for SLR automation over 15 years; 34 primary studies analyzed |
| 3 | 2503.01424v3 | From Hypothesis to Publication: A Comprehensive Survey of AI-Driven Research Support Systems | SURVEY | Broad survey covering knowledge synthesis, literature writing, and peer review |
| 4 | 2502.05151v2 | Transforming Science with Large Language Models: A Survey on AI-assisted Scientific Discovery, Experimentation, Content Generation, and Evaluation | SURVEY | Broad survey of LLM applications across the entire research lifecycle, including literature search and content generation |
| 5 | 2501.10326v2 | Large language models for automated scholarly paper review: A survey | SURVEY | Survey focused on automated peer review (adjacent but relevant for evaluation methodology) |
| 6 | 2409.04600v1 | The emergence of Large Language Models (LLM) as a tool in literature reviews: an LLM automated systematic review | REVIEW | Systematic review of LLM usage in literature review creation; 172 studies analyzed |

---

## Benchmark & Evaluation Papers (4)

These establish evaluation frameworks and metrics for automated survey generation — valuable for the benchmark/evaluation dimension of the survey spec.

| # | arXiv ID | Title | Type | Relevance |
|---|----------|-------|------|-----------|
| 7 | 2508.15658v1 | Benchmarking Computer Science Survey Generation (SurGE) | BENCHMARK | Dedicated benchmark for CS survey generation; 1M+ paper corpus |
| 8 | 2510.03120 | SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs? | BENCHMARK | Fine-grained, quiz-driven evaluation benchmark; 11,343 topics, 4,947 surveys |
| 9 | 2508.11310v1 | SGSimEval: A Comprehensive Multifaceted and Similarity-Enhanced Benchmark for Automatic Survey Generation | BENCHMARK | Multi-faceted evaluation combining LLM scoring, quantitative metrics, and human preference |
| 10 | 2512.02763 | SurveyEval: Towards Comprehensive Evaluation of LLM-Generated Academic Surveys | BENCHMARK | 3-dimension evaluation (quality, outline, reference) across 7 subjects |

---

## Key System Papers (citation seeds) (12)

These are the named systems from the survey spec and additional systems discovered during search. Retained as citation seeds, not as surveys.

| # | arXiv ID | Title | System | Notes |
|---|----------|-------|--------|-------|
| 11 | 2406.10252v2 | AutoSurvey: Large Language Models Can Automatically Write Surveys | AutoSurvey | Foundational system; outline + subsection drafting + refinement |
| 12 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | Agentic AutoSurvey | Multi-agent (4 agents); 8.18/10 vs AutoSurvey 4.77/10 |
| 13 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Surveys | AutoSurvey2 | Multi-stage pipeline with parallel section generation |
| 14 | 2502.14776v2 | SurveyX: Academic Survey Automation via Large Language Models | SurveyX | Online retrieval + AttributeTree + re-polishing |
| 15 | 2503.04629v1 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation | SurveyForge | Outline heuristics + scholar navigation agent |
| 16 | 2508.17647v1 | SurveyGen: Quality-Aware Scientific Survey Generation with LLMs | SurveyGen / QUAL-SG | 4,200 human-written surveys dataset; quality-aware retrieval |
| 17 | 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph | SurveyG | 3-layer citation graph (Foundation/Development/Frontier) |
| 18 | 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans | SurveyGen-I | Coarse-to-fine retrieval + memory-guided generation |
| 19 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow | IterSurvey | Recurrent outline generation; iterative retrieval |
| 20 | 2402.01788v2 | LitLLM: A Toolkit for Scientific Literature Review | LitLLM | RAG-based related work generation |
| 21 | 2407.20906v5 | Automated Review Generation Method Based on Large Language Models | — | Applied to propane dehydrogenation; hallucination <0.5% |
| 22 | 2411.18583v1 | Automated Literature Review Using NLP Techniques and LLM-Based RAG | — | Compares spaCy, T5, GPT-3.5 for review generation |

---

## Candidate Summary

| Category | Count | Use |
|----------|-------|-----|
| Surveys & Reviews | 6 | Provide existing taxonomies, historical context, and related work coverage |
| Benchmarks & Evaluation | 4 | Support the evaluation dimension of the survey spec |
| System Papers (citation seeds) | 12 | Primary sources for architectural patterns and empirical results |
| **Total** | **22** | |
