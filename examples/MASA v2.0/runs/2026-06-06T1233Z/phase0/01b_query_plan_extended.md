# ExtendedQueryPlan — Broader Retrieval Queries

**run_dir**: `.`
**source**: `01_query_plan.md`, `02_candidate_pool.md`, `00_survey_spec.md`
**generated**: 2026-06-06T12:44:32+08:00

---

## Overview

This extended query plan generates 13 broader queries to cast a wider net beyond the 22 focused queries in the main plan. Each query targets an underrepresented sub-topic, alternative terminology family, adjacent field, or boundary area identified by gap analysis of the existing candidate pool.

**Total extended queries**: 13

---

## Extended Queries

### EQ-01: Related work generation with large language models
- **Query**: "related work generation with large language models"
- **Category**: `extended_method`
- **Rationale**: The NLP community predominantly uses "related work" framing rather than "survey generation." The current pool contains only one related-work paper (2505.19647), suggesting a large body of missed literature. Captures methods for generating related work sections from paper corpora.

### EQ-02: Multi-document summarization for scientific literature
- **Query**: "multi document summarization scientific literature"
- **Category**: `extended_method`
- **Rationale**: Multi-document summarization (MDS) shares retrieval, synthesis, and attribution methods with survey generation. The survey generation field evolved partly from MDS. Current pool has no MDS papers, creating a gap in the methodological lineage.

### EQ-03: Knowledge grounded long form text generation
- **Query**: "knowledge grounded long form text generation"
- **Category**: `extended_method`
- **Rationale**: Broader method family that subsumes survey generation as a specific instance. Reveals general-purpose architectures (planning, drafting, factuality) that may transfer to survey tasks. Current pool focuses exclusively on survey-specific systems.

### EQ-04: Scientific paper generation and writing with LLMs
- **Query**: "scientific paper generation with language models"
- **Category**: `extended_method`
- **Rationale**: Adjacent writing task (full paper generation, not just surveys) that shares many architectural components: outline planning, evidence retrieval, multi-section coherence. Bridges survey generation with broader scientific writing automation.

### EQ-05: Biomedical literature review automation with LLMs
- **Query**: "biomedical literature review automation language models"
- **Category**: `extended_boundary`
- **Rationale**: Medical/biomedical community has a separate literature on systematic review automation with LLMs. Different evaluation standards (clinical relevance), but methodologically relevant for planning, retrieval, and synthesis patterns. Current pool has only one medical paper (2504.14822).

### EQ-06: Interactive human AI collaborative literature synthesis
- **Query**: "interactive human AI collaborative literature synthesis"
- **Category**: `extended_mechanism`
- **Rationale**: Captures interactive systems where humans guide retrieval/outline/drafting. The current frontier dimension has only InteractiveSurvey (2504.08762) and one medical Agent paper. Many tools emphasize human-in-the-loop but use different terminology.

### EQ-07: Living systematic reviews continuous evidence monitoring
- **Query**: "living systematic reviews continuous evidence monitoring"
- **Category**: `extended_survey`
- **Rationale**: Living review automation is a mature field in evidence-based medicine. Current pool has only 2 frontier papers (Evolving Literature Analysis, vitaLITy 2). This query captures the broader automation literature for updating surveys incrementally.

### EQ-08: Citation analysis and bibliometric foundations for survey coverage
- **Query**: "citation analysis bibliometric coverage literature discovery"
- **Category**: `extended_mechanism`
- **Rationale**: Pre-LLM citation analysis, bibliometric coupling, and co-citation analysis form the foundation for citation graph expansion methods (M-05 in main plan). Current pool has only 3 citation-graph papers (LitFM, PUREsuggest, CitationIE), missing the bibliometrics literature.

### EQ-09: Systematic review automation with machine learning
- **Query**: "systematic review automation machine learning text mining"
- **Category**: `extended_boundary`
- **Rationale**: Pre-LLM systematic review automation using ML (screening, extraction, risk-of-bias assessment). Provides context for LLM era advances and identifies persistent challenges. Current pool excludes this entirely per scope, but understanding the boundary strengthens the survey.

### EQ-10: Hallucination and factuality in long form text generation
- **Query**: "hallucination factuality long form text generation evaluation"
- **Category**: `extended_problem`
- **Rationale**: Broader problem framing than survey-specific hallucination (P-01). Captures general factuality evaluation frameworks (e.g., FEVER, TRUE, FActScore lineage) that apply to survey evaluation. Current pool has FActScore but misses the broader factuality eval literature.

### EQ-11: Structure aware retrieval for multi section document generation
- **Query**: "structure aware retrieval multi section document generation"
- **Category**: `extended_mechanism`
- **Rationale**: Captures retrieval strategies that account for document structure (sections, subsections, hierarchies). Critical for survey generation where different sections need different evidence. Current pool has no dedicated structure-aware retrieval papers.

### EQ-12: Educational adaptive survey and review generation
- **Query**: "educational survey generation adaptive learning materials"
- **Category**: `extended_problem`
- **Rationale**: Captures survey/synthesis generation for educational purposes (curriculum, learning materials). Different use case but shares method families. Helps assess whether educational surveys face different architectural requirements.

### EQ-13: Evaluation metrics for automated text quality beyond lexical overlap
- **Query**: "evaluation metrics text quality beyond lexical overlap"
- **Category**: `extended_benchmark`
- **Rationale**: Captures broader text quality evaluation (factual consistency, coherence, informativeness, reader alignment). Current benchmarks rely heavily on ROUGE/BLEU variants and survey-specific benchmarks, missing the broader NLG evaluation literature.

---

## Query Distribution Summary

| Category | Count | Purpose |
|----------|-------|---------|
| extended_method | 4 | Broader architectural/methodological families |
| extended_mechanism | 3 | Broader technique families (retrieval, citation) |
| extended_problem | 2 | Broader challenges (hallucination, education) |
| extended_benchmark | 1 | Broader evaluation approaches |
| extended_survey | 1 | Broader living/continuous review surveys |
| extended_boundary | 2 | Domain-specific and pre-LLM boundaries |
| **Total** | **13** | |

---

## Notes

- All queries are short English strings suitable for arXiv semantic search (embedding-indexed).
- These queries intentionally overlap with main-plan queries at the edges; downstream scouts should deduplicate by arXiv ID.
- Queries EQ-05 (biomedical) and EQ-09 (systematic review ML) serve as boundary probes — candidates from these queries should be included only if they demonstrate method transfer to the general survey generation task.
- If any query yields zero results, downstream scouts may fall back to related main-plan queries within the same dimension.
