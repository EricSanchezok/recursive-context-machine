# Extended Query Plan — Automated Literature Survey Agents with Citation Graph Expansion

Generated: 2026-06-07T18:02Z  
Run dir: `.`  
Phase: ExtendedQueryPlanner (broader arXiv retrieval queries for iterative supplementation)

---

## Context

This extended query plan is built after the main expansion phase (03_expansion.md) which produced **69 unique papers** covering citation graph infrastructure, embeddings, survey agent systems, benchmarks, attribution methods, and adjacent domains. The extended queries target underrepresented sub-topics, boundary areas, alternative terminologies, and foundational techniques that the initial expansion did not adequately capture.

**Gap areas addressed (adapted from template general gap areas to the literature survey domain):**

| Template Gap Area | Domain Adaptation | Applied to Literature Survey Topic |
|---|---|---|
| AST/tree-based code editing | **Graph/tree-based literature analysis** | Hierarchical citation graph traversal, tree-of-thought planning for survey structure, graph neural networks for citation retrieval |
| RAG-augmented code editing | **RAG for scientific literature survey** | Retrieval-augmented generation over paper corpora, iterative retrieval for survey completeness, in-context citation retrieval |
| Foundation code models | **Foundation models for scientific literature** | SciBERT, SPECTER, scientific document embeddings, citation-aware pretraining, large-scale scientific language models |

---

## Extended Query Program (12 queries)

### Category: extended_method (4 queries)

| # | Query | Target Gap | Rationale |
|---|-------|------------|-----------|
| **E1** | "hierarchical citation graph survey generation traversal" | Graph/tree-based literature analysis (#1) | SurveyG introduced a 3-layer hierarchical graph; broader concept of hierarchical traversal (top-down, bottom-up, horizontal) for survey planning is unexplored. Tests whether explicit tree-structured planning over citation graphs improves survey coherence. |
| **E2** | "graph neural network citation retrieval embedding science" | Graph/tree-based literature analysis (#2) | LitFM and CitationIE use graph structure for retrieval, but GNN-based methods for citation graph reasoning are not well represented. Broadens to GNN architectures that learn paper representations from citation graph topology. |
| **E3** | "retrieval augmented generation scientific paper survey synthesis" | RAG for scientific literature survey (#1) | Core RAG methods applied to scientific literature synthesis — query, retrieve, aggregate, generate. Focuses on the *survey synthesis* step specifically, not general-purpose RAG. Catches methods that frame survey writing as a constrained RAG task. |
| **E4** | "multi hop retrieval iterative search literature review automation" | RAG for scientific literature survey (#2) | Multi-hop and iterative retrieval strategies for literature reviews. Captures systems that do not just single-shot retrieve but iteratively refine queries based on intermediate findings — a key capability gap for deep citation graph expansion. |

### Category: extended_problem (3 queries)

| # | Query | Target Gap | Rationale |
|---|-------|------------|-----------|
| **E5** | "citation quality noise hallucination automated survey evaluation" | Citation quality and noise | The 03 expansion produced 6 attribution papers, but only 1 (2508.12735) addresses citation *noise* specifically. Broader query to catch methods for detecting and quantifying citation hallucination, misattribution, and citation noise in LLM-generated surveys. |
| **E6** | "temporal bias recency citation graph literature automated review" | Temporal bias in citation expansion | A known failure mode (see 03_expansion §4 Risk 2). Catches methods for temporal normalization, recency-aware retrieval, and time-decay modelling in citation graph traversal. |
| **E7** | "cross domain survey generation biomedicine computer science methodology" | Cross-domain transfer | Tests whether methods developed for CS survey automation transfer to biomedicine, physics, or social sciences. Catches domain-specific adaptations and evaluations. |

### Category: extended_mechanism (3 queries)

| # | Query | Target Gap | Rationale |
|---|-------|------------|-----------|
| **E8** | "multi agent collaboration literature survey generation framework" | Multi-agent architectures | Agentic AutoSurvey (2509.18661) and SciSage (2506.12689) use multi-agent designs. Broader query to catch other multi-agent frameworks for survey generation — including specialized roles (planner, searcher, writer, reviewer, integrator). |
| **E9** | "reinforcement learning feedback optimization survey generation retrieval" | Post-training optimization | PaSa (2501.10120) uses RL for paper search. Broader query to catch RLHF, reward modelling, and preference optimization methods applied to survey generation or citation retrieval. |
| **E10** | "human in the loop interactive literature survey refinement" | Human-in-the-loop systems | InteractiveSurvey (2504.08762) is the only paper covering user interaction during survey generation. Broader query to catch other interactive/iterative refinement systems, including those that accept human feedback on outlines, section content, or reference selection. |

### Category: extended_benchmark (1 query)

| # | Query | Target Gap | Rationale |
|---|-------|------------|-----------|
| **E11** | "human evaluation automated survey quality coverage faithfulness protocol" | Evaluation methodology | The 03 expansion produced 10 benchmark papers, but many evaluate automatically. This query targets *human evaluation protocols* specifically — inter-annotator agreement studies, evaluation rubrics, and quality dimensions that ground automated metrics. |

### Category: extended_boundary (1 query)

| # | Query | Target Gap | Rationale |
|---|-------|------------|-----------|
| **E12** | "systematic review automation tool comparison PRISMA Cochrane guideline" | Systematic review methodology alignment | Explicitly connects survey automation to established systematic review methodology (PRISMA, Cochrane). Catches methodological comparisons and tools that align automated survey generation with evidence synthesis guidelines. |

---

## Query Distribution by Gap Area

| Gap Area | Query IDs | Count |
|----------|-----------|-------|
| Graph/tree-based literature analysis | E1, E2 | **2** |
| RAG for scientific literature survey | E3, E4 | **2** |
| Foundation models for scientific literature | (covered by seed papers SPECTER, SciBERT; plus E2 on GNN) | **1** |
| Citation quality and noise | E5 | 1 |
| Temporal bias | E6 | 1 |
| Cross-domain transfer | E7 | 1 |
| Multi-agent architectures | E8 | 1 |
| Post-training optimization | E9 | 1 |
| Human-in-the-loop | E10 | 1 |
| Evaluation methodology | E11 | 1 |
| Systematic review alignment | E12 | 1 |

> **Note on Foundation Models for Scientific Literature:** SPECTER and SciBERT are already in the seed set and well-represented in the 03 expansion. Additional coverage is provided by E2 (GNN-based citation embeddings) which captures representation learning methods that build on or extend these foundation models.

---

## Execution Notes

1. **arXiv search only:** All queries are arXiv-searchable English short strings. No PDF processing required.
2. **Deduplication key:** arXiv ID. Any new paper found must be checked against the existing 69-entry pool (03_expansion.md §2).
3. **Recency handling:** Queries explicitly avoid year filters to catch older foundational works. Post-hoc temporal weighting is the responsibility of the Synthesizer.
4. **Priority order:** Run E1–E4 first (core gap areas), then E5–E8 (mechanism/problem gaps), then E9–E12 (boundary/benchmark).
