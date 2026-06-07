# Extended QueryPlan — Broader Retrieval Queries

**run_dir**: `.`
**source**: `01_query_plan.md`, `02_candidate_pool.md`, `03_expansion.md`, `03a_seed_papers.md`
**generated**: 2026-06-07T10:22:00+08:00

---

## Overview

This extended query plan generates a secondary set of broader arXiv retrieval queries designed to fill gaps identified during the expansion phase (§6 of `03_expansion.md`). These queries are intentionally broader than the main query plan (22 queries) to cast a wider net across underrepresented sub-topics, adjacent fields, and omitted method families.

**Total extended queries**: 13

**Gap areas addressed**:
1. Seed selection & graph traversal strategy
2. Interactive/human-in-the-loop survey systems
3. Citation graph infrastructure & open citation data
4. Systematic review automation (pre-LLM & hybrid)
5. Contradiction & conflict detection across papers
6. Citation intent & function classification
7. Multi-paper scientific summarization
8. Cost-quality tradeoffs & budget-aware retrieval
9. LLM-based scholarly writing tools (broader than surveys)
10. Scientific knowledge graphs for paper discovery
11. General LLM agent tool-use frameworks for research
12. Citation graph & scholarly datasets
13. Temporal citation dynamics & literature obsolescence

---

## Extended Method Queries (3)

Broader architectural patterns for survey and literature-synthesis systems that were underrepresented in the main pool.

### EM-01: Human-in-the-loop interactive literature survey systems

| Field | Value |
|-------|-------|
| **ID** | EM-01 |
| **Query** | Interactive human-guided scientific literature exploration and survey generation |
| **target_dimension** | extended_method |
| **expected_gain** | Pulls together the scattered family of interactive systems (PROMPTHEUS, CRUISE-Screening, TOBY, InteractiveSurvey) into a coherent sub-field |
| **negative_filter** | Pure end-user literature reading tools without survey generation |
| **rationale** | Gap #2 — only ~4 papers scattered across pool; no systematic retrieval of this sub-family |
| **top_k** | 12 |

### EM-02: LLM agents for academic scholarly writing and synthesis

| Field | Value |
|-------|-------|
| **ID** | EM-02 |
| **Query** | Large language model agents for academic paper writing and scholarly synthesis |
| **target_dimension** | extended_method |
| **expected_gain** | Captures broader scholarly writing tools (thesis writing, grant writing, paper drafting) whose architectures may transfer to survey generation |
| **negative_filter** | Non-scholarly text generation (creative writing, code generation) |
| **rationale** | Gap #9 — survey writing is one type of scholarly writing; broader writing-agent architectures likely share building blocks |
| **top_k** | 12 |

### EM-03: General-purpose LLM agent tool use for research tasks

| Field | Value |
|-------|-------|
| **ID** | EM-03 |
| **Query** | LLM agents with tool use for scientific research and data analysis |
| **target_dimension** | extended_method |
| **expected_gain** | Captures agent frameworks (e.g., CodeAct, OpenHands, AutoGPT variants) adapted for research workflows; tool-use patterns may transfer to citation traversal |
| **negative_filter** | Agents for game-playing or web navigation without research focus |
| **rationale** | Gap #11 — general agent-tool architectures for research are a source of transferable orchestration patterns |
| **top_k** | 10 |

---

## Extended Mechanism Queries (4)

Specific techniques and sub-processes that were absent or thin in the current pool.

### EM-04: Seed selection and traversal strategy for citation graph mining

| Field | Value |
|-------|-------|
| **ID** | EM-04 |
| **Query** | Seed paper selection and traversal strategies for citation network analysis |
| **target_dimension** | extended_mechanism |
| **expected_gain** | Targets the critical decision point of how to choose initial seeds and which graph traversal strategy (BFS, DFS, best-first, random-walk) to use |
| **negative_filter** | Pure bibliometric ranking without traversal strategy |
| **rationale** | Gap #1 — only 1 paper (2403.09295) dedicated to seed selection; a foundational gap for citation-graph survey agents |
| **top_k** | 12 |

### EM-05: Citation intent and citation function classification methods

| Field | Value |
|-------|-------|
| **ID** | EM-05 |
| **Query** | Citation intent classification and citation function analysis in scientific text |
| **target_dimension** | extended_mechanism |
| **expected_gain** | Uncovers why citations are made (background, support, contrast, extension, future-work) — critical for survey agents making relevance judgments |
| **negative_filter** | Citation count analysis or impact metrics without intent |
| **rationale** | Gap #6 — citation intent is mentioned in passing but no dedicated retrieval targets this sub-field; essential for quality citation synthesis in surveys |
| **top_k** | 10 |

### EM-06: Systematic review automation and screening tools

| Field | Value |
|-------|-------|
| **ID** | EM-06 |
| **Query** | Automated systematic review screening and literature selection tools |
| **target_dimension** | extended_mechanism |
| **expected_gain** | Captures traditional SLR automation (ASReview, Rayyan, PRISMA-based tools) that inform hybrid LLM+traditional approaches |
| **negative_filter** | Manual systematic review methodology without automation |
| **rationale** | Gap #4 — pre-LLM and hybrid SLR automation methods are absent from the pool; they define the state of the art in rigorous literature selection |
| **top_k** | 10 |

### EM-07: Contradiction and conflict detection across scientific papers

| Field | Value |
|-------|-------|
| **ID** | EM-07 |
| **Query** | Detecting contradictions and conflicting findings across scientific papers |
| **target_dimension** | extended_mechanism |
| **expected_gain** | Directly addresses how survey agents should handle conflicting evidence — a missing capability in current automated survey systems |
| **negative_filter** | General fact-checking or stance detection not focused on scientific literature |
| **rationale** | Gap #5 — no papers in the pool address how survey agents deal with contradictions; this is a known weakness of existing systems |
| **top_k** | 10 |

---

## Extended Problem Queries (2)

Known limitations and open challenges that were not captured in the main problem queries.

### EP-01: Cost and quality tradeoffs in citation graph traversal

| Field | Value |
|-------|-------|
| **ID** | EP-01 |
| **Query** | Cost-quality tradeoffs and budget-aware retrieval for literature search |
| **target_dimension** | extended_problem |
| **expected_gain** | Captures analyses of when to stop citation expansion, how many citations to retrieve, and economic constraints — a practical deployment concern |
| **negative_filter** | General API cost analysis without literature search context |
| **rationale** | Gap #8 — no papers address termination criteria or budget constraints; critical for practical survey agent deployment |
| **top_k** | 10 |

### EP-02: Temporal coverage and recency-aware citation expansion

| Field | Value |
|-------|-------|
| **ID** | EP-02 |
| **Query** | Temporal coverage and literature obsolescence in citation graph surveys |
| **target_dimension** | extended_problem |
| **expected_gain** | Addresses how survey agents handle outdated citations, literature drift, and recency bias — a concrete limitation identified in the expansion |
| **negative_filter** | General citation aging studies without survey automation context |
| **rationale** | Gap #13 — the F-01 queries surfaced only 2 papers on longitudinal surveys; the temporal dynamics of citation-based surveys deserve dedicated retrieval |
| **top_k** | 10 |

---

## Extended Benchmark & Dataset Queries (2)

Evaluation infrastructure that was missing or thin.

### EB-01: Citation graph and scholarly data infrastructure datasets

| Field | Value |
|-------|-------|
| **ID** | EB-01 |
| **Query** | Open citation graph datasets and scholarly knowledge graph infrastructure |
| **target_dimension** | extended_benchmark |
| **expected_gain** | Captures missing infrastructure papers: OpenCitations (COCI), Crossref citation graph, Microsoft Academic Graph, CORE, Unpaywall |
| **negative_filter** | Private/proprietary citation databases without open access |
| **rationale** | Gap #3 — only Semantic Scholar and OpenAlex in current pool; citation infrastructure is foundational for any graph-based survey agent |
| **top_k** | 10 |

### EB-02: Multi-document summarization benchmarks for scientific text

| Field | Value |
|-------|-------|
| **ID** | EB-02 |
| **Query** | Multi-document summarization datasets and benchmarks for scientific literature |
| **target_dimension** | extended_benchmark |
| **expected_gain** | Captures general scientific summarization benchmarks (Multi-XScience, MS^2, SciSummNet) that are related to but distinct from survey-specific benchmarks |
| **negative_filter** | News summarization or generic text summarization without scientific focus |
| **rationale** | Gap #7 — scientific multi-document summarization techniques and benchmarks are a direct adjacent field that could inform survey section generation |
| **top_k** | 10 |

---

## Extended Survey Queries (1)

Broader surveys of adjacent fields.

### ES-01: Survey of knowledge graphs and semantic retrieval for scientific discovery

| Field | Value |
|-------|-------|
| **ID** | ES-01 |
| **Query** | Knowledge graph construction and retrieval for scientific literature discovery |
| **target_dimension** | extended_survey |
| **expected_gain** | Captures surveys of scientific KG construction (ORKG, PaperWithCode, SciGraph) and KG-based retrieval that overlaps with citation graph traversal |
| **negative_filter** | General KG surveys without scientific literature focus |
| **rationale** | Gap #10 — scientific KGs for paper discovery are adjacent to citation graphs; existing surveys of KG4Sci can provide broader context |
| **top_k** | 10 |

---

## Extended Boundary Query (1)

Boundary probing to ensure the extended pool does not drift too far.

### EBX-01: Traditional bibliometric citation network analysis

| Field | Value |
|-------|-------|
| **ID** | EBX-01 |
| **Query** | Bibliometric citation network analysis and co-citation clustering methods |
| **target_dimension** | extended_boundary |
| **expected_gain** | Captures traditional bibliometric methods that serve as a boundary guard — related enough to be confused with LLM-based citation traversal but should be demoted during scoring |
| **negative_filter** | Must explicitly exclude via scout classifier if no LLM or automation component |
| **rationale** | Gap guard — the line between bibliometric citation analysis and LLM-guided citation traversal is blurry; this query helps calibrate the boundary |
| **top_k** | 8 |

---

## Extended Query Distribution Summary

| Type | Count | Purpose |
|------|-------|---------|
| extended_method | 3 | Broader method patterns (interactive systems, scholarly writing, agent tool-use) |
| extended_mechanism | 4 | Missing techniques (seed selection, citation intent, SLR automation, contradiction) |
| extended_problem | 2 | Unaddressed limitations (cost-quality, temporal coverage) |
| extended_benchmark | 2 | Missing infrastructure (citation datasets, scientific summarization benchmarks) |
| extended_survey | 1 | Adjacent field surveys (scientific KGs) |
| extended_boundary | 1 | Traditional bibliometric guard |
| **Total** | **13** | |

---

## Notes

1. All queries are short English strings suitable for arXiv embedding-based semantic search.
2. These queries are **broader** than the main query plan queries — they deliberately explore boundary areas and underrepresented sub-topics.
3. Extended queries are designed to supplement, not replace, the main 22-query plan. Downstream scouts should execute both sets.
4. The extended queries target a total of **~136 candidate papers** (sum of all top_k values), of which an estimated 60–80 will be genuinely new additions to the pool.
5. Gap areas 1–13 (listed in §Overview) are mapped to specific queries; no gap is left unattended.
6. The extended boundary query (EBX-01) serves as a negative guard — it will surface papers that should be demoted if they lack any automation or LLM component.
