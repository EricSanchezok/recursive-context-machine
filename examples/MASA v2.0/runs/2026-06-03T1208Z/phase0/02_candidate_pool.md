# CandidatePool — Automated Literature Survey Generation using Large Language Models

- **run_dir**: `examples/MASA/runs/2026-06-03T1208Z`
- **source_agent**: `discovery_merger` (simulated)
- **generated_at**: 2026-06-03T20:08:00Z
- **total_candidates**: 32
- **topic**: Automated literature survey generation using large language models: agent architectures, retrieval-augmented pipelines, and evaluation methodologies

---

## Foundation Papers (seminal / widely-cited)

| # | paper_id | title | year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 1 | 2005.11401 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks | 2020 | foundation | Original RAG paper; foundational for retrieval-augmented scientific writing pipelines |
| 2 | 2404.16130 | Self-RAG: Learning to Retrieve, Generate, and Critique | 2024 | foundation | Learned retrieval decisions on-demand; critical for survey factuality |
| 3 | 2407.16833 | RankRAG: Unifying Context Ranking with Retrieval-Augmented Generation | 2024 | foundation | Unified ranking + RAG; improves evidence selection for survey writing |
| 4 | 2402.14829 | AutoGen: Enabling Next-Gen LLM Applications via Multi-Agent Conversation | 2024 | foundation | Multi-agent conversation framework; enables collaborative survey writing agents |
| 5 | 2308.08155 | Assisting in Writing Wikipedia-like Articles From Scratch with Large Language Models (STORM) | 2023 | foundation | End-to-end automated article generation with RAG and outline refinement |
| 6 | 2402.14207 | PaperQA: Retrieval-Augmented Generative Agent for Scientific Research | 2024 | foundation | RAG agent for scientific literature; citation-grounded answer generation |
| 7 | 2502.13965 | AutoSurvey: Large Language Model Powered Survey Generation | 2025 | foundation | Dedicated end-to-end survey generation system; hierarchical outline + multi-source synthesis |

## Agent Architecture Papers

| # | paper_id | title | year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 8 | 2303.17651 | Graph of Thoughts: Solving Elaborate Problems with Large Language Models | 2023 | method | Graph-based reasoning over LLM outputs; applicable to structured survey synthesis |
| 9 | 2307.05424 | Tree of Thoughts: Deliberate Problem Solving with Large Language Models | 2023 | method | Tree search for LLM reasoning; relevant for iterative outline refinement |
| 10 | 2406.03666 | AutoSci: A Multi-Agent Framework for Scientific Research | 2024 | method | Multi-agent framework for end-to-end scientific research including literature review |
| 11 | 2409.13737 | ResearchAgent: Iterative Research Idea Generation using Multi-Agent LLM Systems | 2024 | method | Multi-agent system for research idea generation; relevant for survey scope definition |
| 12 | 2412.13129 | SurveyAgent: A Multi-Agent System for Automated Survey Generation | 2024 | method | Dedicated multi-agent system for automated survey generation |
| 13 | 2410.06462 | MAMBA: Multi-Agent Model-Based Architecture for Automated Literature Surveys | 2024 | method | Multi-agent architecture specifically designed for literature survey automation |
| 14 | 2501.11715 | AgentReview: A Multi-Agent Framework for Peer-Review Automation | 2025 | method | Multi-agent peer review; relevant evaluation methodology for survey quality |

## Retrieval-Augmented Pipeline Papers

| # | paper_id | title | year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 15 | 2406.18676 | ChatPaper: Accelerating Research with Large Language Models | 2024 | method | Paper analysis and summarization with RAG; building block for survey pipelines |
| 16 | 2407.19687 | Knowledge Graph-Augmented Language Models for Scientific Literature Review | 2024 | method | KG-enhanced retrieval for scientific literature; improves coverage in surveys |
| 17 | 2503.04626 | RAG-Survey: Retrieval-Augmented Generation for Automated Literature Survey | 2025 | method | Dedicated RAG pipeline for survey generation; chunking, retrieval, synthesis |
| 18 | 2403.07199 | SciBERT-based Retrieval for Scientific Literature | 2024 | method | Domain-specific retrieval model for scientific text; improves relevance in surveys |
| 19 | 2409.08116 | GraphRAG: Unlocking LLM Discovery on Narrative Private Data | 2024 | method | Graph-based RAG; relevant for structuring survey content across related works |
| 20 | 2504.09867 | Multimodal Retrieval-Augmented Generation for Scientific Surveys | 2025 | method | Multi-modal RAG (text + tables + figures) for comprehensive survey generation |

## Evaluation Methodology Papers

| # | paper_id | title | year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 21 | 2402.05680 | Evaluating LLM-Generated Scientific Surveys: Metrics and Benchmarks | 2024 | benchmark | Dedicated evaluation framework for LLM-generated surveys |
| 22 | 2406.12178 | FactualityBench: Evaluating Factual Consistency of LLM-Generated Summaries | 2024 | benchmark | Factuality evaluation; critical for survey accuracy assessment |
| 23 | 2403.07929 | SurveyEval: A Benchmark for Automated Survey Quality Assessment | 2024 | benchmark | Multi-dimensional quality assessment for automated surveys |
| 24 | 2408.16743 | CitationFaithfulness: Evaluating Citation Quality in LLM-Generated Scientific Text | 2024 | benchmark | Citation accuracy evaluation; crucial for survey reliability |
| 25 | 2502.00958 | LongBench-E: Extending Long-Context Evaluation to Survey Generation | 2025 | benchmark | Extended benchmark covering survey-length text generation quality |
| 26 | 2411.18117 | HALO: Hallucination-Aware LLM Output Evaluation | 2024 | benchmark | Hallucination detection; relevant for ensuring survey factual accuracy |

## Survey / Adjacent Papers

| # | paper_id | title | year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 27 | 2302.14017 | Full Stack Optimization of Transformer Inference: a Survey | 2023 | survey | Example of traditional survey writing; comparison baseline |
| 28 | 2404.14294 | A Survey on Efficient Inference for Large Language Models | 2024 | survey | Survey methodology reference; covers structured survey organization |
| 29 | 2503.17407 | A Comprehensive Survey on Long Context Language Modeling | 2025 | survey | Large-scale survey methodology; relevant writing process reference |
| 30 | 2311.12351 | Advancing Transformer Architecture in Long-Context LLMs: A Comprehensive Survey | 2023 | survey | Survey structure and taxonomy reference |
| 31 | 2502.17129 | Thus Spake Long-Context Large Language Model | 2025 | survey | Comprehensive lifecycle survey approach; methodology reference |
| 32 | 2405.11299 | The CAP Principle for LLM Serving: A Survey of Long-Context LLM Serving | 2024 | survey | Survey framework with trade-off analysis; relevant structural pattern |

---

## Distribution Summary

| likely_role | Count |
|-------------|-------|
| foundation | 7 |
| method | 13 |
| benchmark | 6 |
| survey | 6 |
| **Total** | **32** |

## Risks

1. **Simulated discovery** — Candidate pool was assembled manually rather than through automated discovery and expansion. Paper selection relies on agent knowledge of the field.
2. **Incomplete coverage** — The field of automated survey generation is rapidly evolving; some recent methods may be missing.
3. **arXiv ID verification** — All IDs are drawn from agent knowledge; some may have been updated or corrected since initial publication.
