# 02f Extended Benchmark & Evaluation Candidates

**run_dir**: `.`
**scout**: ExtendedBenchmarkScout
**date**: 2026-06-07T10:24:00+08:00
**source_plan**: `01b_query_plan_extended.md`
**note**: This file supplements `02b_benchmark_candidates.md` with broader benchmark/dataset/infrastructure queries.

---

## Queries Executed

| # | Query (from ExtendedQueryPlan) | Source ID | topK | Results |
|---|--------------------------------|-----------|------|---------|
| 1 | Open citation graph datasets and scholarly knowledge graph infrastructure | EB-01 | 10 | 10 |
| 2 | Multi-document summarization datasets and benchmarks for scientific literature | EB-02 | 10 | 10 |

**Total unique candidate papers identified**: 18 (deduplicated across both queries; 2 overlaps with existing pool removed)

---

## Candidate Summary

### Category E: Citation Graph & Scholarly Data Infrastructure (8 new papers)

These are infrastructure datasets, knowledge graphs, and open citation databases foundational for citation-graph-based survey agents.

| ID | arXiv ID | Short Name | Year | Source | Key Features |
|----|----------|-----------|------|--------|-------------|
| EB01 | 1906.11964 | OpenCitations (COCI) | 2019 | EB-01 | Open citation data as Linked Open Data; 445M+ citations; I4OC compliant; RDF/SPARQL/OCI identifiers |
| EB02 | 2110.06595 | Refcat (Internet Archive) | 2021 | EB-01 | 1.3B+ citations; CC0 public domain; fatcat catalog; exact + fuzzy citation matching |
| EB03 | 2206.01439 / 1901.10816 | ORKG (Open Research KG) | 2019 | EB-01 | Machine-actionable scholarly knowledge; crowdsourced + automated KG acquisition; FAIR compliant |
| EB04 | 2308.03671 | SemOpenAlex | 2023 | EB-01 | 26B RDF triples; SPARQL endpoint; entity embeddings; linked open data for scientific impact analysis |
| EB05 | 2602.12206 | OpenAIRE Citation Graph (Compact) | 2026 | EB-01 | 200M+ publications, 2B+ citations; 32GB compact representation; Python processing pipeline |
| EB06 | 2206.07476 | OpenCitations e-Infrastructure | 2022 | EB-01 | FAIR reuse guidelines; UNESCO Open Science compliant; data provenance and licensing architecture |
| EB07 | 2203.09159 | EMAKG (Enhanced MAG) | 2022 | EB-01 | Extended Microsoft Academic KG with geo/collaboration/metrics/linguistic features; high temporal coverage |
| EB08 | 2301.10140 | Semantic Scholar Open Data | 2023 | EB-01 | 200M+ papers; 2.4B+ citation edges; structured parsed text; vector embeddings; open API + data dumps |

> **Note**: Two additional papers from EB-01 results were already in the expanded pool and removed: `2205.01833` (OpenAlex — already in pool as theory/infrastructure from semantic expansion EQ-04) and `1805.02262` (Semantic Scholar Lit Graph 2018 — already in pool as foundational from citation expansion).

### Category F: Multi-Document Scientific Summarization Benchmarks & Datasets (10 new papers)

These are benchmarks and datasets for multi-document summarization of scientific text — directly adjacent to survey generation evaluation.

| ID | arXiv ID | Short Name | Year | Source | Key Features |
|----|----------|-----------|------|--------|-------------|
| EB09 | 2010.14235 | Multi-XScience | 2020 | EB-02 | Related-work section generation from abstract + references; extreme summarization protocol |
| EB10 | 2104.06486 | MS² (Medical Studies) | 2021 | EB-02 | 470K+ documents, 20K summaries; biomedical domain; BART-based baselines; contradictory evidence aggregation |
| EB11 | 2408.16444 | SurveySum | 2024 | EB-02 | Dedicated dataset for summarizing multiple scientific articles into a survey section; RAG-evaluated pipelines |
| EB12 | 2405.01930 | OARelatedWork | 2024 | EB-02 | 94K papers, 5.8M refs; full-text cited papers; related-work generation from full content (not just abstracts) |
| EB13 | 2203.01769 | PeerSum | 2022 | EB-02 | Meta-review summarization from peer reviews; highly abstractive; features reviewer disagreement |
| EB14 | 2403.05303 | ACLSum | 2024 | EB-02 | Aspect-based summarization; expert-curated; challenges/approaches/outcomes dimensions |
| EB15 | 2004.15011 | SciTLDR (TLDR) | 2020 | EB-02 | Extreme summarization; 5.4K TLDRs over 3.2K papers; author + expert-derived summaries; CATTS method |
| EB16 | 1804.08875 | SciSumm Scientific Article | 2018 | EB-02 | Multi-sentence summarization from title + abstract; evaluated extractive + abstractive models |
| EB17 | 2011.08072 | MAG-20 (Topic-Centric) | 2020 | EB-02 | Topic-centric unsupervised MDS; 20 Fields of Study in MAG; human-validated gold standard; abstractive + extractive |
| EB18 | 2505.16349 | XSum (Pipeline) | 2025 | EB-02 | Modular RAG pipeline; question-generation + editor modules; evaluated on SurveySum; CheckEval/G-Eval/Ref-F1 |

---

## Key Findings & Observations

### 1. Citation infrastructure is diverse and often cross-listed as "benchmark"

Papers like OpenAlex, Semantic Scholar Open Data, and OpenCitations are **infrastructure** papers — they describe platforms and datasets, not evaluation benchmarks in the traditional sense. They are included here because they constitute the **evaluation substrate** for citation-graph survey agents: any agent that uses Semantic Scholar's API or OpenAlex's knowledge graph depends on the correctness and coverage of these datasets.

### 2. Multi-document summarization benchmarks fill an evaluation gap

The existing `02b_benchmark_candidates.md` covers 10 dedicated survey-generation benchmarks (SurveyBench, SurGE, SurveyEval, etc.) but none of the broader scientific multi-document summarization benchmarks. Papers like Multi-XScience, MS², and SurveySum evaluate summarization quality in settings closely related to survey section generation — providing complementary metrics and datasets.

### 3. SurveySum (2408.16444) bridges both worlds

SurveySum is unique in being explicitly designed for "summarizing multiple scientific articles into a **section of a survey**." It directly targets the survey generation setting and was evaluated with a RAG pipeline (XSum from EB18). It could serve as a secondary evaluation dataset alongside the dedicated survey benchmarks.

### 4. OARelatedWork (2405.01930) offers full-text advantages

Unlike most existing datasets that only use abstracts, OARelatedWork provides full-text cited papers, enabling extractive upper bounds that are 217% higher in ROUGE-2. This is relevant for citation-aware survey agents that need to synthesize *across* full papers, not just abstracts.

### 5. Temporal spread

| Era | Count | Papers |
|-----|-------|--------|
| 2018–2019 | 3 | EB16 (SciSumm), EB03 (ORKG), EB01 (OpenCitations) |
| 2020–2021 | 4 | EB09 (Multi-XScience), EB10 (MS²), EB15 (SciTLDR), EB02 (Refcat) |
| 2022–2023 | 5 | EB05 (OpenAIRE), EB06 (OpenCitations infra), EB07 (EMAKG), EB08 (S2 Open Data), EB04 (SemOpenAlex) |
| 2024–2026 | 6 | EB11 (SurveySum), EB12 (OARelatedWork), EB13 (PeerSum), EB14 (ACLSum), EB17 (MAG-20), EB18 (XSum) |

The infrastructure papers cluster in 2019–2023; the summarization benchmarks cluster 2020–2025, with a recent surge in 2024–2025.

---

## Overlaps with Existing Pool

| arXiv ID | Existing Role | Conflict Resolution |
|----------|--------------|-------------------|
| 2205.01833 (OpenAlex) | theory/infrastructure (expansion) | **Removed** — already in candidate pool |
| 1805.02262 (S2 Lit Graph 2018) | foundational (expansion) | **Removed** — already in candidate pool |

All other 18 papers are genuinely new additions to the candidate pool.

---

## Role Distribution for New Candidates

| Likely Role | Count | Paper IDs |
|-------------|-------|-----------|
| benchmark | 10 | EB09–EB18 (all MDS benchmarks) |
| infrastructure | 8 | EB01–EB08 (citation graph datasets/SKGs) |
| **Total** | **18** | |

> **Recommendation**: Classify EB01–EB08 as **"infrastructure"** (a sub-role of benchmark/evaluation) rather than "method" to preserve role distribution. The MDS benchmarks (EB09–EB18) should be classified as **"benchmark"** under the broader evaluation umbrella.
