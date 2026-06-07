# Extended Query Plan Analysis — Gap Coverage Rationale

Generated: 2026-06-07T18:02Z  
Run dir: `.`  

---

## Background

The main expansion phase (03_expansion.md) produced **69 unique papers** across 9 categories. While coverage is strong in survey agent systems (22 papers), citation graph embeddings (10 papers), and benchmarks (10 papers), several important gap areas remain. This document explains what each extended query targets and why.

---

## Gap 1: Graph/Tree-based Literature Analysis Methods (Queries E1, E2)

**Why this gap matters:** The 03 expansion has strong coverage of *flat* citation graph methods (SPECTER embeddings, co-citation, bibliographic coupling) but nearly nothing on *hierarchical* or *tree-structured* traversal. SurveyG (2510.07733) is the only paper exploring a 3-layer hierarchical graph. The broader concept of constructing a topic tree or taxonomy and then traversing it top-down to guide survey section generation is underexplored. Similarly, graph neural networks for citation graph reasoning are only tangentially covered (LitFM, CitationIE).

**What E1 targets:** Systems that decompose a survey topic into a hierarchical structure (tree, DAG, or multi-layer graph) and then traverse that structure to allocate retrieval and generation resources per subtopic.

**What E2 targets:** Representation learning methods that use GNN architectures (GCN, GraphSAGE, GAT) on citation graphs to produce paper embeddings for retrieval. Catches papers that frame citation graph reasoning as a graph learning problem rather than a retrieval problem.

---

## Gap 2: RAG for Scientific Literature Survey (Queries E3, E4)

**Why this gap matters:** The 03 expansion has many RAG-adjacent papers (PaperQA, OpenScholar, etc.), but they are treated as end-to-end systems. The *methodology* of retrieval-augmented generation specifically for survey synthesis — as distinct from general-purpose RAG or QA-focused RAG — is underexplored. Key questions: How many retrieval hops are optimal? How should retrieved passages be aggregated and deduplicated across sections? Does iterative re-retrieval improve coverage or just amplify bias?

**What E3 targets:** Papers that frame survey section writing as a constrained RAG task: given a topic outline, retrieve supporting papers, and synthesize a coherent narrative. Catches ablations and design studies of RAG hyperparameters for survey writing.

**What E4 targets:** Iterative/multi-hop retrieval strategies where the query is refined after each retrieval round (e.g., "which subtopics are missing?", "which claims need more evidence?"). This is distinct from single-shot retrieval used by many survey agents.

---

## Gap 3: Foundation Models for Scientific Literature (Covered by seeds + E2)

**Why this gap is partially addressed:** SPECTER (2004.07180) and SciBERT (1903.10676) are already in the seed set and well-represented through their citation trees. The 03 expansion captures their influence through papers that cite or extend them (CitationIE, LitFM, etc.). Additional coverage comes from E2 (GNN-based methods that often build on or extend these foundation models). No separate query is needed for this gap because the foundation models are already anchor papers in the pool.

---

## Gap 4: Citation Quality and Noise (Query E5)

**Why this gap matters:** The 03 expansion produced 6 citation attribution papers, but only one (2508.12735) specifically addresses citation *noise* — the phenomenon where papers cite irrelevant or tangentially related work. Automated surveys that rely on citation graph expansion are vulnerable to noise amplification: an initial off-topic citation can pull the entire expansion off-topic. Methods for detecting, quantifying, and mitigating citation noise are critical for survey quality.

**What E5 targets:** Papers on citation quality assessment, citation noise detection, hallucination measurement in citation contexts, and automated citation verification. Catches methods that could serve as guardrails during graph expansion.

---

## Gap 5: Temporal Bias (Query E6)

**Why this gap matters:** The 03 expansion shows a strong 2025 recency skew (54% of semantic candidates from 2025). While the expansion report flags this as Risk 2, no papers in the pool specifically address how to counteract temporal bias during graph expansion. Methods for temporal normalization (field- and age-normalized citation counts), recency-aware retrieval weighting, and time-decay modelling could inform graph expansion policy design.

**What E6 targets:** Papers on temporal normalization of citation impact, recency bias in automated retrieval, and methods for maintaining temporal balance in citation graph traversal.

---

## Gap 6: Cross-domain Transfer (Query E7)

**Why this gap matters:** Nearly all 22 survey agent systems in the 03 expansion were evaluated on CS/NLP/ML literature. Whether these methods transfer to biomedicine, physics, social sciences, or humanities is unknown. The survey spec explicitly mentions "disciplinary adoption" as an expected dimension. Catch papers that adapt survey agents to new domains or compare performance across disciplines.

**What E7 targets:** Papers that apply automated survey generation to non-CS domains (biomedicine, social sciences, physics) and report domain-specific challenges or adaptations.

---

## Gap 7: Multi-agent Architectures (Query E8)

**Why this gap matters:** The 03 expansion has only two papers explicitly using multi-agent frameworks (Agentic AutoSurvey, SciSage). Given the popularity of multi-agent LLM systems in 2024–2025, there are likely more multi-agent survey architectures that were not captured by seed-based expansion. This query casts a wide net to catch them.

**What E8 targets:** Papers with explicit multi-agent designs for survey generation — including specialized roles (planner, searcher, writer, reviewer, integrator, discriminator), inter-agent communication protocols, and agent coordination strategies.

---

## Gap 8: Post-training Optimization (Query E9)

**Why this gap matters:** Only PaSa (2501.10120) uses reinforcement learning for paper search optimization. The broader category of using RLHF, reward modelling, preference optimization, or supervised fine-tuning to improve survey generation quality is absent from the pool. This is a potential oversight: many systems may train their generators on curated human-written surveys.

**What E9 targets:** Papers that use RL, RLHF, DPO, or other training-time optimization methods for improving survey generation, citation retrieval, or outline planning.

---

## Gap 9: Human-in-the-loop (Query E10)

**Why this gap matters:** InteractiveSurvey (2504.08762) is the only system allowing user interaction during survey generation. Given that survey generation is typically a high-stakes academic task, human oversight (outline approval, reference verification, section-level feedback) is a natural design requirement that many systems may address.

**What E10 targets:** Systems that allow user intervention during the survey generation process — including outline editing, reference addition/removal, section rewriting, and iterative refinement loops.

---

## Gap 10: Evaluation Methodology (Query E11)

**Why this gap matters:** While 10 benchmark papers are in the pool, most evaluate automatically (statistical metrics, overlap with reference surveys). Papers on *human evaluation protocol design* for generated surveys — including inter-annotator agreement, evaluation rubrics, and quality dimension definitions — are underrepresented.

**What E11 targets:** Studies of human evaluation methodology for automatically generated surveys, including annotation guidelines, evaluation dimensions (coverage, faithfulness, readability, originality), and quality assessment protocols.

---

## Gap 11: Systematic Review Alignment (Query E12)

**Why this gap matters:** The survey spec includes PRISMA 2020 and systematic review methodology as adjacent context. While 3 systematic review screening tools are in the pool (ASReview, CRUISE, DenseReviewer), the broader relationship between automated survey generation and established evidence synthesis methodology is not captured. Catches papers that explicitly compare, align, or integrate automated survey generation with PRISMA guidelines.

**What E12 targets:** Papers that connect LLM-based survey automation to systematic review methodology, including PRISMA-based evaluation of generated surveys, Cochrane handbook alignment, and methodological comparisons between automated and traditional review processes.

---

## Summary

| Gap | Severity | Queries | Catch |
|-----|----------|---------|-------|
| Graph/tree-based analysis | **High** | E1, E2 | Hierarchical traversal, GNN citation embeddings |
| RAG for literature survey | **High** | E3, E4 | Survey-specific RAG methodology, iterative retrieval |
| Foundation models | Medium | (seeds cover) | SPECTER, SciBERT already in pool |
| Citation quality/noise | **High** | E5 | Citation noise detection, hallucination metrics |
| Temporal bias | **High** | E6 | Temporal normalization, recency-aware retrieval |
| Cross-domain transfer | Medium | E7 | Non-CS domain applications |
| Multi-agent architectures | Medium | E8 | Specialized agent roles for survey generation |
| Post-training optimization | Medium | E9 | RLHF, DPO for survey quality |
| Human-in-the-loop | Low | E10 | Interactive survey refinement |
| Human evaluation methodology | Medium | E11 | Evaluation rubrics, annotation protocols |
| Systematic review alignment | Low | E12 | PRISMA alignment, methodology comparison |
