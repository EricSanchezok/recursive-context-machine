# Extended Query Plan Analysis — Gap Coverage Rationale

**run_dir**: `.`
**source**: `01_query_plan.md`, `02_candidate_pool.md`, `03_expansion.md`, `03a_seed_papers.md`
**generated**: 2026-06-07T10:22:30+08:00

---

## 1. Motivation

The main query plan (22 queries) and expansion phase (95 new candidates from 10 seeds + 7 semantic queries) together produced a pool of ~170 papers. Despite broad coverage, several systematic gaps remain. This document analyzes the gap coverage of the 13 extended queries in `01b_query_plan_extended.md`.

---

## 2. Gap-by-Gap Analysis

### Gap 1: Seed Selection & Graph Traversal Strategy

| Aspect | Detail |
|--------|--------|
| **Current state** | Only 1 paper (2403.09295, Seed-based Retrieval Evaluation) addresses how to choose initial seed papers for citation graph traversal. No papers on comparative traversal strategies (BFS vs DFS vs best-first vs random-walk). |
| **Why it matters** | Every citation-based survey agent must make an initial seed selection decision. The quality of the final survey is bounded by the quality of the initial seed set. Without understanding seed selection strategies, the survey cannot evaluate this critical decision point. |
| **Covering query** | **EM-04** — "Seed paper selection and traversal strategies for citation network analysis" |
| **Coverage adequacy** | High — directly targets the decision point. Expected to surface papers from bibliometrics, network science, and IR that evaluate seed selection algorithms applicable to citation graphs. |

### Gap 2: Interactive / Human-in-the-Loop Survey Systems

| Aspect | Detail |
|--------|--------|
| **Current state** | ~4 scattered papers (PROMPTHEUS, CRUISE-Screening, TOBY, InteractiveSurvey). No coordinated retrieval of this sub-family. Interactive survey generation is qualitatively different from fully autonomous generation — user steering changes the architecture and evaluation protocol. |
| **Why it matters** | The F-02 frontier query in the main plan touched on interactive systems but only as an emerging direction. Interactive systems represent a distinct architectural family with different design constraints (steerability, user intent modeling, feedback loops). |
| **Covering query** | **EM-01** — "Interactive human-guided scientific literature exploration and survey generation" |
| **Coverage adequacy** | High — designed to pull together the scattered papers and surface additional interactive architectures. |

### Gap 3: Citation Graph Infrastructure & Open Citation Data

| Aspect | Detail |
|--------|--------|
| **Current state** | Only Semantic Scholar Literature Graph (1805.02262) and OpenAlex (2205.01833) in the pool. Missing: OpenCitations Corpus (COCI), Crossref Event Data, Microsoft Academic Graph (legacy), CORE, Unpaywall. |
| **Why it matters** | Citation graph-based survey agents are only as good as the citation graph they traverse. The availability, coverage, latency, and licensing of citation infrastructure directly affect system design. |
| **Covering query** | **EB-01** — "Open citation graph datasets and scholarly knowledge graph infrastructure" |
| **Coverage adequacy** | High — targets infrastructure explicitly. Expected to surface COCI, Crossref, and other open citation databases. |

### Gap 4: Systematic Review Automation (Pre-LLM & Hybrid)

| Aspect | Detail |
|--------|--------|
| **Current state** | No papers on traditional or hybrid SLR automation tools (ASReview, Rayyan, Covidence, DistillerSR). The current pool focuses exclusively on LLM-based survey generation. |
| **Why it matters** | Traditional SLR automation tools represent a decade of research on rigorous literature selection (PRISMA workflows, dual-screening, conflict resolution). LLM-based systems can learn from these workflows. Hybrid approaches (LLM + traditional screening) are a promising direction not yet represented. |
| **Covering query** | **EM-06** — "Automated systematic review screening and literature selection tools" |
| **Coverage adequacy** | Medium-High — targets the sub-field directly. Some papers may be pre-arXiv or in medical informatics venues. arXiv coverage may be incomplete for this area. |

### Gap 5: Contradiction & Conflict Detection Across Papers

| Aspect | Detail |
|--------|--------|
| **Current state** | No papers address how survey agents should handle conflicting evidence across papers. The current pool assumes a coherent narrative can be synthesized from supporting citations only. |
| **Why it matters** | A credible survey must acknowledge and analyze conflicting findings. Current automated survey systems (AutoSurvey, SurveyGen, SurveyForge) largely ignore this capability. Understanding contradiction detection methods is essential for advancing the field. |
| **Covering query** | **EM-07** — "Detecting contradictions and conflicting findings across scientific papers" |
| **Coverage adequacy** | Medium — contradiction detection is a niche sub-field. May surface papers from NLP (scientific claim verification, stance detection) and meta-science (reproducibility analysis). |

### Gap 6: Citation Intent & Function Classification

| Aspect | Detail |
|--------|--------|
| **Current state** | Citation attribution benchmarks (CiteRAG, FActScore, REASONS) exist, but no dedicated retrieval targets **why** citations are made (support, contrast, background, extension, future-work). |
| **Why it matters** | For a citation-graph survey agent to make relevance judgments, it must understand the function of each citation. A supporting citation has different evidential weight than a contrasting one. Citation intent classification is a prerequisite for quality-aware citation synthesis. |
| **Covering query** | **EM-05** — "Citation intent classification and citation function analysis in scientific text" |
| **Coverage adequacy** | High — a well-established NLP sub-field with ample arXiv papers. Expected to surface 8–10 relevant papers. |

### Gap 7: Multi-Paper Scientific Summarization (Without Citation Graphs)

| Aspect | Detail |
|--------|--------|
| **Current state** | No papers on general scientific multi-document summarization (Multi-XScience, MS^2, SciSummNet). The pool focuses on survey-specific methods that include citation traversal. |
| **Why it matters** | Multi-paper summarization is the core generation task in survey writing, decoupled from retrieval. Benchmarks and techniques from this sub-field (hierarchical summarization, facet-aware summarization) may transfer directly to survey section generation. |
| **Covering query** | **EB-02** — "Multi-document summarization datasets and benchmarks for scientific literature" |
| **Coverage adequacy** | Medium-High — a moderate arXiv presence. Many papers may be in ACL/NAACL proceedings rather than arXiv. |

### Gap 8: Cost-Quality Tradeoffs & Budget-Aware Retrieval

| Aspect | Detail |
|--------|--------|
| **Current state** | No papers address when to stop citation expansion, how many citations to retrieve per seed, or economic constraints of LLM-based traversal. |
| **Why it matters** | LLM-based citation traversal is expensive (API costs for every expansion step). Practical deployment requires understanding the cost-quality Pareto frontier. Termination criteria are a fundamental design parameter. |
| **Covering query** | **EP-01** — "Cost-quality tradeoffs and budget-aware retrieval for literature search" |
| **Coverage adequacy** | Medium — this is an emerging concern. Relevant papers may appear in ML systems or HCI venues. arXiv coverage likely limited but growing. |

### Gap 9: LLM-Based Scholarly Writing Tools (Broader Context)

| Aspect | Detail |
|--------|--------|
| **Current state** | Survey generation is treated as an isolated task. Broader scholarly writing tools (thesis writing, grant writing, AI-assisted paper drafting) are absent. |
| **Why it matters** | Survey writing shares architectural patterns with other scholarly writing tasks: outline generation, evidence gathering, citation management, iterative revision. These broader tools may contain transferable innovations. |
| **Covering query** | **EM-02** — "Large language model agents for academic paper writing and scholarly synthesis" |
| **Coverage adequacy** | High — a rapidly growing arXiv sub-field. Expected to surface 10–12 relevant papers. |

### Gap 10: Scientific Knowledge Graphs for Paper Discovery

| Aspect | Detail |
|--------|--------|
| **Current state** | KG traversal papers exist (CD-01 in main plan) but no systematic retrieval of scientific KG construction or KG-based paper discovery. |
| **Why it matters** | Scientific KGs (ORKG, PaperWithCode, SciGraph) offer structured representations of research findings that can complement citation graph traversal. Semantic relationships between concepts can guide paper discovery beyond citation links. |
| **Covering query** | **ES-01** — "Knowledge graph construction and retrieval for scientific literature discovery" |
| **Coverage adequacy** | Medium — surveys of this sub-field exist (ES-01 is itself a survey query) and will provide taxonomic coverage. |

### Gap 11: General LLM Agent Tool-Use Frameworks for Research

| Aspect | Detail |
|--------|--------|
| **Current state** | Multi-agent frameworks exist (CD-02 frontier) but general agent-tool architectures for research (CodeAct, OpenHands, research agents) are absent. |
| **Why it matters** | Survey agents are increasingly built on general-purpose agent frameworks. Understanding the base architectures (tool use, planning, memory) is essential for evaluating survey-specific design choices. |
| **Covering query** | **EM-03** — "LLM agents with tool use for scientific research and data analysis" |
| **Coverage adequacy** | High — a major arXiv trend. Risk of excessive breadth; scout classifier must filter for research-focused tool use. |

### Gap 12: Citation Graph & Scholarly Datasets

| Aspect | Detail |
|--------|--------|
| **Current state** | SciReviewGen and SurveyBench are the only datasets. Missing: citation graph datasets (COCI snapshots, S2ORC, CORE, DBLP citation subsets), full-text corpora (PDF collections). |
| **Why it matters** | Dataset availability bounds what research problems can be studied. Comprehensive knowledge of available datasets enables better evaluation design. |
| **Covering query** | Covered by **EB-01** (gap #3) — citation graph datasets and infrastructure are bundled in the same query. |
| **Coverage adequacy** | Adequate — the query targets both infrastructure and datasets. |

### Gap 13: Temporal Citation Dynamics & Literature Obsolescence

| Aspect | Detail |
|--------|--------|
| **Current state** | F-01 (longitudinal surveys) surfaced only 2 papers. The temporal dynamics of citation-based surveys (outdated citations, literature drift, recency-aware expansion) are unaddressed. |
| **Why it matters** | Citation graphs evolve. A seed paper's citation network changes over time. Understanding temporal dynamics — when citations become obsolete, how literature drifts, how to maintain freshness — is essential for live-updating survey agents. |
| **Covering query** | **EP-02** — "Temporal coverage and literature obsolescence in citation graph surveys" |
| **Coverage adequacy** | Medium — this crosses bibliometrics (citation aging studies) and survey automation. Few dedicated papers exist, making this both a gap and a research opportunity. |

---

## 3. Query-Level Coverage Matrix

| Query ID | Gap 1 | Gap 2 | Gap 3 | Gap 4 | Gap 5 | Gap 6 | Gap 7 | Gap 8 | Gap 9 | Gap 10 | Gap 11 | Gap 12 | Gap 13 |
|----------|-------|-------|-------|-------|-------|-------|-------|-------|-------|--------|--------|--------|--------|
| **EM-01** | | ✅ | | | | | | | | | | | |
| **EM-02** | | | | | | | | | ✅ | | | | |
| **EM-03** | | | | | | | | | | | ✅ | | |
| **EM-04** | ✅ | | | | | | | | | | | | |
| **EM-05** | | | | | | ✅ | | | | | | | |
| **EM-06** | | | | ✅ | | | | | | | | | |
| **EM-07** | | | | | ✅ | | | | | | | | |
| **EP-01** | | | | | | | | ✅ | | | | | |
| **EP-02** | | | | | | | | | | | | | ✅ |
| **EB-01** | | | ✅ | | | | | | | | | ✅ | |
| **EB-02** | | | | | | | ✅ | | | | | | |
| **ES-01** | | | | | | | | | | ✅ | | | |
| **EBX-01** | *(boundary guard for Gaps 3,4,6,13 — ensures bibliometric drift is bounded)* |

**Coverage summary**: All 13 gaps are covered by at least 1 dedicated query. Gaps 3 and 12 share EB-01. Gap 13 is the thinnest coverage (single query, moderate expected yield). All other gaps have at least medium-high coverage adequacy.

---

## 4. Risk Assessment

| Risk | Extended Query | Mitigation |
|------|---------------|------------|
| **Excessive breadth** (too many irrelevant papers) | EM-02, EM-03, ES-01 | Scout classifier must apply stringent relevance filters on all extended queries. Consider lowering top_k if yield is <30%. |
| **Pre-2020 papers dominating** | EM-06 (SLR tools), EBX-01 | Scout should apply temporal demotion for papers before scope range. |
| **Overlap with main query plan** | EM-01 (interactive) partially overlaps F-02 | Deduplicate by arXiv ID after retrieval. |
| **Non-arXiv content gap** | EM-06 (SLR tools often in medical informatics) | Accept that arXiv coverage is incomplete; flag as known limitation. |
| **Citation intent papers too niche** | EM-05 | May surface only 3–5 papers on arXiv; supplement with non-arXiv sources if available. |
| **Contradiction detection too distant** | EM-07 | Definitely adjacent; scout should assign low confidence scores and demote during scoring. |
| **Infrastructure papers too general** | EB-01 | Many infrastructure papers (Crossref, COCI) are not about survey generation; they provide enabling technology. Role assignment should be "infrastructure" or "theory" not "method". |

---

## 5. Expected Yield

| Query | top_k | Expected New | Confidence | Notes |
|-------|-------|-------------|------------|-------|
| EM-01 | 12 | 6–8 | Medium | Paper count limited by novelty of sub-field |
| EM-02 | 12 | 8–10 | High | Rapidly growing area |
| EM-03 | 10 | 5–7 | Medium | High breadth risk |
| EM-04 | 12 | 6–8 | Medium | Mix of ML and bibliometrics papers |
| EM-05 | 10 | 5–7 | Medium | Niche but established sub-field |
| EM-06 | 10 | 4–6 | Medium-Low | arXiv coverage limited |
| EM-07 | 10 | 4–5 | Low | Niche intersection |
| EP-01 | 10 | 3–5 | Low | Emerging concern |
| EP-02 | 10 | 3–5 | Low | Cross-field gap |
| EB-01 | 10 | 6–8 | Medium | Infrastructure papers on arXiv |
| EB-02 | 10 | 5–7 | Medium | Some overlap with survey benchmarks |
| ES-01 | 10 | 5–7 | Medium | Survey-of-surveys; broad relevance |
| EBX-01 | 8 | 4–5 | Medium | Boundary guard; many will be out-of-scope |
| **Total** | **134** | **64–88** | | **Projected new unique candidates** |

The extended query plan is expected to add an estimated **60–80 genuinely new candidates** to the pool, increasing total pool size from ~170 to **~230–250**.

---

## 6. Recommendations for Downstream Scouts

1. **Execute extended queries after main queries** — The extended queries are supplementary; main queries should have priority.
2. **Apply strict relevance filtering** on EM-02, EM-03, ES-01 — these have the highest breadth risk.
3. **Classify EB-01 papers as "infrastructure"** rather than "method" to preserve role distribution.
4. **Monitor EM-07 (contradiction) yield** — if <3 relevant papers, flag as a genuine research gap for the final brief.
5. **Use EBX-01 to calibrate the boundary** — the papers retrieved should form the negative guard for the scout classifier.
6. **Combine EB-01 + EB-02 results** for a consolidated "Evaluation Infrastructure" category.
7. **Allocate ~60% of scout capacity** to the 13 extended queries, with the rest reserved for re-querying main queries with different hyperparameters if needed.

---

## 7. Comparison with Initial Expansion Suggestions

The `03_expansion.md` §6 suggested 5 "next expansion queries":

| Expansion Suggestion | Covered By | Status |
|----------------------|-----------|--------|
| Survey-specific citation fidelity evaluation | EM-05 (partial) | No dedicated benchmark exists; EM-05 addresses citation intent which is related |
| Seed selection strategies | EM-04 | Directly covered |
| Interactive / HITL systems | EM-01 | Directly covered |
| Citation graph construction infrastructure | EB-01 | Directly covered |
| Survey dataset construction methods | EB-01 (partial) | Partially covered via EB-01 |

The extended query plan covers all 5 suggested directions and adds 8 new gap areas.

---

## 8. Summary

The 13 extended queries systematically cover 13 identified gaps in the current pool:

| Coverage Level | Count | Gaps |
|----------------|-------|------|
| **Strong** (direct query, high arXiv presence) | 5 | Seed selection (EM-04), Citation intent (EM-05), Interactive systems (EM-01), Scholarly writing (EM-02), Graph infrastructure (EB-01) |
| **Adequate** (direct query, moderate arXiv presence) | 4 | General tool-use (EM-03), SLR automation (EM-06), Scientific summarization (EB-02), Scientific KGs (ES-01) |
| **Weak** (niche sub-field, limited arXiv) | 4 | Contradiction detection (EM-07), Cost-quality (EP-01), Temporal dynamics (EP-02), Boundary guard (EBX-01) |

**Expected total new candidates**: 60–80 (projected pool: 230–250)
