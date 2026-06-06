# ExtendedSurveyCandidates — Survey/Review/Tutorial/Taxonomy Papers from Extended Queries

**run_dir**: `.`
**source**: `01b_query_plan_extended.md` (13 extended queries)
**generated**: 2026-06-06T12:48:00+08:00
**total_candidates**: 12 (after deduplication against existing 82-paper pool)

---

## Legend

| Column | Description |
|--------|-------------|
| `#` | Sequential unique ID (E-001 onward) |
| `arXiv ID` | Base arXiv identifier |
| `Title` | Short title |
| `Year` | Publication year |
| `paper_type` | survey / review / tutorial / taxonomy / overview |
| `source_query` | Which extended query ID surfaced this |
| `duplicate_in_pool` | Whether this paper already appears in 02_candidate_pool.md |
| `notes` | Inclusion rationale and linkage to the survey generation task |

---

## Candidate Papers

### E-001: Survey on Multi-Document Summarization: Systematic Literature Review
- **arXiv ID**: `2312.12915`
- **Title**: Survey on Multi-Document Summarization: Systematic Literature Review
- **Year**: 2023
- **paper_type**: **survey**
- **source_query**: EQ-02 (multi-document summarization)
- **duplicate_in_pool**: No
- **notes**: Systematic literature review of multi-document summarization methods. Directly relevant as MDS forms the methodological precursor to survey generation. Identifies open challenges (accuracy, evaluation) that persist in the survey generation context.

### E-002: PubMed and Beyond: Biomedical Literature Search in the Age of Artificial Intelligence
- **arXiv ID**: `2307.09683`
- **Title**: PubMed and Beyond: Biomedical Literature Search in the Age of Artificial Intelligence
- **Year**: 2023
- **paper_type**: **survey**
- **source_query**: EQ-05 (biomedical literature review automation)
- **duplicate_in_pool**: No
- **notes**: Survey of 36 biomedical literature search tools covering evidence-based medicine, gene-related retrieval, semantic search, recommendation, and concept mining. Includes discussion of LLM-era advances. Valuable as boundary survey for understanding the retrieval tool landscape.

### E-003: Facets, Taxonomies, and Syntheses: Navigating Structured Representations in LLM-Assisted Literature Review
- **arXiv ID**: `2504.18496`
- **Title**: Facets, Taxonomies, and Syntheses: Navigating Structured Representations in LLM-Assisted Literature Review
- **Year**: 2025
- **paper_type**: **taxonomy / overview**
- **source_query**: EQ-06 (interactive human-AI collaborative literature synthesis)
- **duplicate_in_pool**: No
- **notes**: Presents DimInd system that scaffolds literature review through faceted comparison tables, taxonomies of concepts, and narrative syntheses. Evaluated with 23 researchers. Directly addresses the taxonomy-creation aspect of survey generation — highly relevant as a taxonomy paper of structured literature representations.

### E-004: Towards Continuous Systematic Literature Review in Software Engineering
- **arXiv ID**: `2206.04177`
- **Title**: Towards Continuous Systematic Literature Review in Software Engineering
- **Year**: 2022
- **paper_type**: **review**
- **source_query**: EQ-07 (living systematic reviews)
- **duplicate_in_pool**: No
- **notes**: Proposes and evaluates the Continuous Systematic Literature Review (CSLR) concept and process. Addresses the living/continuous review paradigm. Provides BPMN process model for continuous evidence updating — directly relevant to the living survey frontier dimension.

### E-005: Continuous Systematic Literature Review: An Approach for Open Science
- **arXiv ID**: `2108.12922`
- **Title**: Continuous Systematic Literature Review: An Approach for Open Science
- **Year**: 2021
- **paper_type**: **review**
- **source_query**: EQ-07 (living systematic reviews)
- **duplicate_in_pool**: No
- **notes**: Positional paper proposing CSLR concept with open science practices. Companion to E-004. Relevant for understanding the process engineering of living/continuous review updates.

### E-006: Automating Systematic Literature Reviews with NLP and Text Mining: a Systematic Literature Review
- **arXiv ID**: `2211.15397`
- **Title**: Automating Systematic Literature Reviews with Natural Language Processing and Text Mining: a Systematic Literature Review
- **Year**: 2022
- **paper_type**: **survey**
- **source_query**: EQ-09 (systematic review automation ML)
- **duplicate_in_pool**: No
- **notes**: SLR of 29 studies on NLP/TM automation of SLR steps (study selection, quality assessment, data extraction, synthesis). Identifies gaps in data extraction, monitoring, and quality assessment. Provides taxonomy of automation approaches. Directly relevant as a survey of the pre-LLM automation landscape.

### E-007: Hidden Citations Obscure True Impact in Science
- **arXiv ID**: `2310.16181`
- **Title**: Hidden Citations Obscure True Impact in Science
- **Year**: 2023
- **paper_type**: **overview / analysis**
- **source_query**: EQ-08 (citation analysis)
- **duplicate_in_pool**: No
- **notes**: Uses unsupervised ML to identify hidden citations (textual credits without formal references). Shows hidden citations outnumber standard citations for influential discoveries. Relevant for understanding citation coverage limitations in citation-graph methods (M-05).

### E-008: CRUISE-Screening: Living Literature Reviews Toolbox
- **arXiv ID**: `2309.01684`
- **Title**: CRUISE-Screening: Living Literature Reviews Toolbox
- **Year**: 2023
- **paper_type**: **overview / tool**
- **source_query**: EQ-07 (living systematic reviews)
- **duplicate_in_pool**: No
- **notes**: Web-based application for conducting living literature reviews. Connected to multiple search engines via API for periodic updates. Uses text classification and QA models for screening. Open-source. Relevant as a concrete tool implementing the living review paradigm.

### E-009: ASReview: Open Source Software for Efficient and Transparent Reviews
- **arXiv ID**: `2006.12166`
- **Title**: Open Source Software for Efficient and Transparent Reviews
- **Year**: 2020
- **paper_type**: **overview / tool**
- **source_query**: EQ-09 (systematic review automation ML)
- **duplicate_in_pool**: No
- **notes**: ML-aided pipeline using active learning for title/abstract screening. Demonstrated to yield far more efficient reviewing than manual approaches. Widely used in evidence-based medicine. Relevant as a standard pre-LLM benchmark for review automation efficiency.

### E-010: Long-form factuality in large language models
- **arXiv ID**: `2403.18802`
- **Title**: Long-form factuality in large language models
- **Year**: 2024
- **paper_type**: **overview / benchmark**
- **source_query**: EQ-10 (hallucination/factuality)
- **duplicate_in_pool**: No
- **notes**: Introduces LongFact prompt set (38 topics, thousands of questions) and SAFE evaluation method. Proposes extended F1 for long-form factuality. Benchmarks 13 models. Directly relevant as an overview of the factuality evaluation landscape for long-form text including surveys.

### E-011: WildHallucinations: Evaluating Long-form Factuality in LLMs with Real-World Entity Queries
- **arXiv ID**: `2407.17468`
- **Title**: WildHallucinations: Evaluating Long-form Factuality in LLMs with Real-World Entity Queries
- **Year**: 2024
- **paper_type**: **overview / benchmark**
- **source_query**: EQ-10 (hallucination/factuality)
- **duplicate_in_pool**: No
- **notes**: Evaluates 15 LLMs on 7,919 entities mined from real-world user-chatbot conversations. Finds LLMs hallucinate more on entities without Wikipedia pages. Relevant as a benchmark that exposes factuality challenges common to survey generation.

### E-012: BEAMetrics: A Benchmark for Language Generation Evaluation Evaluation
- **arXiv ID**: `2110.09147`
- **Title**: BEAMetrics: A Benchmark for Language Generation Evaluation Evaluation
- **Year**: 2021
- **paper_type**: **overview / benchmark**
- **source_query**: EQ-13 (evaluation metrics beyond lexical overlap)
- **duplicate_in_pool**: No
- **notes**: Provides unified comparison of automatic metrics against human judgements across diverse generation tasks, fluency/coherence/informativeness dimensions, and languages. Reveals task-dependent differences in metric performance. Relevant as an overview of the broader NLG evaluation landscape.

---

## Summary Statistics

| paper_type | Count |
|------------|-------|
| survey | 3 |
| review | 2 |
| taxonomy / overview | 1 |
| overview / analysis | 1 |
| overview / tool | 2 |
| overview / benchmark | 3 |
| **Total** | **12** |

## Query Coverage

| Query | Candidates Found |
|-------|-----------------|
| EQ-02 (multi-document summarization) | 1 (E-001) |
| EQ-05 (biomedical lit review automation) | 1 (E-002) |
| EQ-06 (interactive human-AI synthesis) | 1 (E-003) |
| EQ-07 (living systematic reviews) | 3 (E-004, E-005, E-008) |
| EQ-08 (citation analysis) | 1 (E-007) |
| EQ-09 (systematic review automation ML) | 2 (E-006, E-009) |
| EQ-10 (hallucination/factuality) | 2 (E-010, E-011) |
| EQ-13 (evaluation metrics) | 1 (E-012) |

**Queries with zero survey candidates**: EQ-01 (related work generation — all method papers), EQ-03 (knowledge-grounded generation — all method/dialogue papers), EQ-04 (scientific paper generation — already-in-pool survey + method papers), EQ-11 (structure-aware retrieval — all method papers), EQ-12 (educational survey generation — all question-generation method papers)

## Risks

1. Papers E-004, E-005 (living reviews in SE) are from the software engineering community and use different terminology/process models than the LLM-based survey generation literature. Method transfer assessment needed downstream.
2. Papers E-002 (biomedical search survey), E-006 (SLR automation survey), and E-009 (ASReview tool) are pre-LLM or early-LLM boundary work — relevant for context but may be out of strict scope.
3. Papers E-010, E-011, E-012 are evaluation benchmarks/overviews rather than surveys of methods. Included because they provide the broader evaluation context needed for the survey's benchmark dimension.
4. Paper E-003 (taxonomy) provides structured representations for LLM-assisted review — bridges the HCI and NLP perspectives; recommended for inclusion.
