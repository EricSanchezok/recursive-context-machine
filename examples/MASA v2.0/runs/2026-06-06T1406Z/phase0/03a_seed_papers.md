# Seed Papers for Citation Graph Expansion

**Selected by**: CitationSeedSelector (MASA pipeline)
**Date**: 2026-06-06
**run_dir**: `.`
**Total seeds**: 8

---

## Selection Rationale

The goal of the seed set is to produce a rich, complementary citation graph when expanded forward and backward. Seeds are selected to:

1. **Span the timeline** (2023–2025) so expansion covers early foundations and recent advances.
2. **Cover the anchor question** — how citation graph structure guides paper discovery and synthesis.
3. **Be well-cited hubs** — papers that many systems cite (backward expansion) or that many systems compare against (forward expansion).
4. **Include benchmarks** — forward expansion from benchmark papers captures every system evaluated on them.
5. **Cover complementary sub-areas** — avoid overlap in citation neighborhoods.

---

## Seed Papers

### Seed 1: AutoSurvey
- **arXiv ID**: 2406.10252v2
- **Year**: 2024
- **Role**: core_method / citation_seed
- **Why this seed**: Foundational paper for automated survey generation. Introduced the outline → section drafting → integration pipeline. All 2025 systems (AutoSurvey2, SurveyForge, Agentic AutoSurvey, etc.) compare against it. Has the richest forward citation graph of any paper in the candidate pool. Backward citations connect to RAG and NLP foundations.
- **Expected expansion**: Forward → all descendant survey agent systems; Backward → RAG, LLM writing, scientific summarization literature.

### Seed 2: PaperQA
- **arXiv ID**: 2312.07559v2
- **Year**: 2023
- **Role**: citation_seed
- **Why this seed**: Early and highly influential RAG agent for scientific research. Introduced full-text retrieval + relevance assessment + RAG answers for science. Widely cited as an antecedent of the survey agent paradigm. Bridges our topic to the broader scientific RAG literature (LitQA benchmark).
- **Expected expansion**: Forward → all RAG-for-science systems including OpenScholar; Backward → retrieval, RAG, scientific QA foundations.

### Seed 3: OpenScholar
- **arXiv ID**: 2411.14199v1
- **Year**: 2024
- **Role**: core_method
- **Why this seed**: Large-scale retrieval-augmented LM operating on a 45M-paper datastore. Citation accuracy on par with human experts; outperforms GPT-4o by 5%. A key comparison point for all subsequent systems. Its scale and strong results make it a citation hub.
- **Expected expansion**: Forward → newer systems comparing against it; Backward → large-scale retrieval, datastore construction, scientific LMs.

### Seed 4: SurveyG
- **arXiv ID**: 2510.07733
- **Year**: 2025
- **Role**: core_method
- **Why this seed**: Directly addresses the primary anchor question — organizes citation graphs into three layers (Foundation/Development/Frontier) for survey generation. Introduces horizontal search within layers and vertical depth traversal. The most citation-graph-centric method in the pool.
- **Expected expansion**: Forward → future citation-graph-aware survey systems; Backward → citation graph traversal methods, graph-based retrieval, snowballing literature.

### Seed 5: SurveyForge
- **arXiv ID**: 2503.04629v1
- **Year**: 2025
- **Role**: core_method
- **Why this seed**: Cross-scout consensus (3 scouts: Method, Survey, Frontier). Unique outline-first approach — learns outline structure from human-written surveys, then uses scholar navigation agent for memory-driven retrieval. Bridges outline generation and citation traversal.
- **Expected expansion**: Forward → systems adopting outline-first or memory-driven approaches; Backward → outline generation, human survey methodology, iterative retrieval.

### Seed 6: Agentic AutoSurvey
- **arXiv ID**: 2509.18661v1
- **Year**: 2025
- **Role**: core_method
- **Why this seed**: Cross-scout consensus (3 scouts). Four specialized agents (Paper Search, Topic Mining, Writer, Quality Evaluator). Processes 75–443 papers per topic; reported score 8.18/10 vs AutoSurvey's 4.77/10. Strong quantitative benchmark presence makes it a forward-citation target.
- **Expected expansion**: Forward → multi-agent survey systems, quality evaluator components; Backward → multi-agent architectures, LLM-as-judge evaluation, agentic workflows.

### Seed 7: SurveyBench
- **arXiv ID**: 2510.03120
- **Year**: 2025
- **Role**: benchmark
- **Why this seed**: Cross-scout consensus (3 scouts). Built from 11,343 arXiv topics and 4,947 high-quality surveys. The primary evaluation framework used by new survey generation systems. Forward expansion captures every system evaluated on this benchmark — the most efficient way to discover relevant new papers.
- **Expected expansion**: Forward → all systems evaluated on SurveyBench; Backward → evaluation methodology, quiz-based assessment, human evaluation protocols.

### Seed 8: LitFM
- **arXiv ID**: 2409.12177v1
- **Year**: 2024
- **Role**: mechanism
- **Why this seed**: Cross-scout consensus (3 scouts: Method, Benchmark, Frontier). Novel graph retriever that navigates citation graphs by integrating graph structure during training and inference. 28.1% retrieval precision improvement. Benchmark datasets on 3 academic fields. Bridges survey agents to the citation graph retrieval literature.
- **Expected expansion**: Forward → graph-retrieval-augmented systems; Backward → graph neural networks for citation graphs, structure-aware retrieval, benchmark datasets.

---

## Summary

| # | arXiv ID | Year | Paper | Role | Expansion Value |
|---|----------|------|-------|------|-----------------|
| 1 | 2406.10252v2 | 2024 | AutoSurvey | core_method / citation_seed | Richest forward graph — all descendants |
| 2 | 2312.07559v2 | 2023 | PaperQA | citation_seed | Bridges to broader scientific RAG literature |
| 3 | 2411.14199v1 | 2024 | OpenScholar | core_method | Large-scale comparison hub |
| 4 | 2510.07733 | 2025 | SurveyG | core_method | Directly on anchor question — citation graph hierarchy |
| 5 | 2503.04629v1 | 2025 | SurveyForge | core_method | Outline-first + memory-driven retrieval |
| 6 | 2509.18661v1 | 2025 | Agentic AutoSurvey | core_method | Multi-agent, strongest scores |
| 7 | 2510.03120 | 2025 | SurveyBench | benchmark | Forward expansion finds all evaluated systems |
| 8 | 2409.12177v1 | 2024 | LitFM | mechanism | Graph retriever — bridges to graph retrieval literature |

**Time coverage**: 2023 (PaperQA) → 2024 (AutoSurvey, OpenScholar, LitFM) → 2025 (SurveyG, SurveyForge, Agentic AutoSurvey, SurveyBench)

**Sub-area coverage**:
- Survey generation pipelines: AutoSurvey, SurveyG, SurveyForge, Agentic AutoSurvey
- Retrieval/infrastructure: PaperQA, OpenScholar, LitFM
- Evaluation: SurveyBench
- Citation graph traversal: SurveyG, LitFM
