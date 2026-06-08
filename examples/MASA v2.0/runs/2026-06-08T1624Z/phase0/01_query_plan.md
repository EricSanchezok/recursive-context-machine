# Query Plan — MASA Pipeline

**Generated**: 2026-06-08
**Topic**: Automated literature survey generation using multi-agent systems and citation-graph expansion
**run_dir**: `.`

Generated from SurveySpec (`00_survey_spec.md`).

---

## Query Index

| # | Type | Query String | Rationale |
|---|------|-------------|-----------|
| 1 | `core_method` | `multi-agent survey generation 2025` | Core architecture pattern: multiple LLM agents collaborating to produce a survey |
| 2 | `foundational` | `single agent text generation pipeline` | Paired with #1 — pre-multi-agent era of single-agent text pipeline |
| 3 | `core_method` | `graph neural network literature expansion citation` | GNN-based citation-graph traversal for survey breadth |
| 4 | `foundational` | `citation network traversal survey early work` | Paired with #3 — earlier non-GNN citation graph methods |
| 5 | `core_method` | `LLM based literature review pipeline` | Core method: LLM-native pipeline for automated review generation |
| 6 | `foundational` | `automatic related work generation pre LLM` | Paired with #5 — pre-LLM approaches (e.g., SciSumm, CLAIR) |
| 7 | `mechanism` | `citation graph expansion breadth depth traversal` | Specific retrieval mechanism: how graphs are traversed |
| 8 | `mechanism` | `paper retrieval forward backward citation traversal` | Forward/backward chaining in citation graph expansion |
| 9 | `mechanism` | `PageRank influence propagation scientific literature` | Influence-based expansion algorithms for literature |
| 10 | `problem` | `automated literature survey generation challenges` | Problem framing: what makes automated surveys hard |
| 11 | `problem` | `citation fabrication hallucination survey generation` | Key limitation: hallucinated citations in generated surveys |
| 12 | `problem` | `survey coverage coherence factuality gap` | Quality problem: trade-offs across coverage, coherence, factuality |
| 13 | `benchmark` | `SurveyBench automated survey evaluation` | Benchmark for evaluating automated survey systems |
| 14 | `benchmark` | `Multi-Survey benchmark literature review` | Multi-Survey benchmark for survey evaluation |
| 15 | `benchmark` | `survey quality metrics ROUGE BERTScore factuality` | Evaluation metrics used to assess generated surveys |
| 16 | `survey` | `automated survey generation survey methods overview` | Existing survey papers on survey-generation methods |
| 17 | `survey` | `LLM agent survey architecture comparison` | Overview of LLM agent architectures for survey tasks |
| 18 | `citation_seed` | `citation graph theory scientific literature network` | Theoretical grounding for citation networks |
| 19 | `citation_seed` | `Garfield citation indexing history` | Foundational citation theory — Garfield's work |
| 20 | `frontier` | `multi-agent scientific discovery automated research 2025` | Cutting-edge: multi-agent for full research workflows |
| 21 | `frontier` | `graph neural network citation analysis science 2025` | Frontier: GNN applied to citation/intent analysis |
| 22 | `frontier` | `LLM evaluation citation accuracy fact verification` | Frontier: verifying citation accuracy in LLM-generated text |
| 23 | `boundary` | `generic RAG retrieval augmented generation pipeline` | Boundary work: generic RAG that is NOT survey-specific |
| 24 | `boundary` | `text summarization benchmark CNN Dailymail` | Boundary work: pure summarisation without survey structure |
| 25 | `cross_domain` | `PRISMA systematic review methodology guidelines` | Systematic review methodology from biomedicine |
| 26 | `cross_domain` | `information foraging theory literature search` | Information foraging theory — user search behaviour model |
| 27 | `cross_domain` | `discourse structure scientific writing taxonomy` | Discourse structure analysis for scientific survey writing |

---

## Design Notes

### Temporal Balance Rule
Every `core_method` query has a `foundational` partner (1↔2, 3↔4, 5↔6) to ensure pre-2023 foundation papers are retrieved alongside recent work.

### Diversity Coverage
- **Methods**: multi-agent, GNN-based, LLM-native pipeline, PageRank, BFS/DFS traversal
- **Benchmarks**: SurveyBench, Multi-Survey, human evaluation
- **Metrics**: ROUGE, BERTScore, FactKB, entity precision, coverage, coherence
- **Limitations**: hallucination, citation fabrication, shallow coverage, evaluation subjectivity
- **Cross-domain**: PRISMA, information foraging, discourse structure
- **Foundational**: Transformer/Vaswani, BERT/Devlin, GPT-3/Brown, Garfield citation theory

### Search Strings
Short English search strings used throughout because arXiv embedding search works best with focused, non-boolean queries.

---

## Summary

| Type | Count |
|------|-------|
| `core_method` | 3 |
| `foundational` | 3 |
| `mechanism` | 3 |
| `problem` | 3 |
| `benchmark` | 3 |
| `survey` | 2 |
| `citation_seed` | 2 |
| `frontier` | 3 |
| `boundary` | 2 |
| `cross_domain` | 3 |
| **Total** | **27** |

Representative IDs: `1` (core_method: multi-agent survey), `7` (mechanism: citation graph expansion), `13` (benchmark: SurveyBench), `19` (citation_seed: Garfield), `25` (cross_domain: PRISMA)
