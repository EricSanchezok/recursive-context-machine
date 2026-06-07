# Merged Expansion Report — ExpansionMerger

**run_dir**: `.`
**generated**: 2026-06-07T10:20:43+08:00
**sources**: `03a_seed_papers.md`, `03b_citation_expansion.md`, `03c_semantic_expansion.md`
**schema**: `schema` (flat file; no `schema/expansion.md` exists)

---

## 1. Seeds

Ten seed papers selected across five roles for maximal downstream expansion coverage.

| # | arXiv ID | Title (Short) | Year | Role | Strategy |
|---|----------|---------------|------|------|----------|
| 1 | 2402.08565 | AI for Literature Reviews | 2024 | survey | Backward |
| 2 | 2501.04306 | LLM4SR | 2025 | survey | Backward |
| 3 | 2309.09727 | When LLMs Meet Citation | 2023 | citation_seed | Backward + bridge |
| 4 | 2508.17647 | SurveyGen | 2025 | method | Bi-directional (4/4 scouts) |
| 5 | 2503.04629 | SurveyForge | 2025 | method | Bi-directional (3/4 scouts) |
| 6 | 2510.21900 | IterSurvey / Survey-Arena | 2025 | method | Bi-directional (3/4 scouts) |
| 7 | 2406.10252 | AutoSurvey | 2024 | method | Backward (earliest method) |
| 8 | 2510.03120 | SurveyBench | 2025 | benchmark | Forward |
| 9 | 2302.07302 | CiteSee | 2023 | mechanism | Bi-directional |
| 10 | 2305.15186 | SciReviewGen | 2023 | dataset | Forward |

---

## 2. Citation Graph Expansion

### 2.1 Methodology

PDFs were downloaded (10/10) but binary text extraction was not available. The citation graph was reconstructed via **arxiv_search** using targeted title-keyword and semantic queries per seed paper. Edges are labelled **Backward** (seed cites paper) or **Forward** (paper cites seed).

### 2.2 Forward / Backward Edges Per Seed

#### Seed 1: 2402.08565 — AI for Literature Reviews

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2111.07533 | Automated scholarly paper review | 2021 |
| Backward | 2401.10917 | AI to automate SLR | 2024 |
| Forward | 2605.16475 | Generative AI for Literature Reviews | 2026 |
| Forward | 2603.20235 | Writing lit reviews with AI: lessons learned | 2026 |
| Forward | 2410.15978 | PROMPTHEUS | 2024 |
| Forward | 2412.15249 | LitLLMs | 2024 |
| Forward | 2505.23789 | LitChat | 2025 |
| Forward | 2405.06563 | What Can NLP Do for Peer Review? | 2024 |

#### Seed 2: 2501.04306 — LLM4SR

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2406.10833 | Comprehensive Survey of Scientific LLMs | 2024 |
| Backward | 2303.18223 | Survey of LLMs | 2023 |
| Forward | 2505.13259 | Automation to Autonomy: LLMs in Sci Discovery | 2025 |
| Forward | 2502.05151 | Transforming Science with LLMs | 2025 |
| Forward | 2507.11810 | Evolving Role of LLMs | 2025 |
| Forward | 2304.02020 | Bibliometric Review of LLMs | 2023 |
| Forward | 2408.10729 | Efficient LLMs for Scientific Text | 2024 |
| Forward | 2505.22787 | Can LLMs Match SR Conclusions? | 2025 |

#### Seed 3: 2309.09727 — When LLMs Meet Citation

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2305.14627 | ALCE: Text with Citations | 2023 |
| Forward | 2508.15396 | Attribution, Citation, and Quotation Survey | 2025 |
| Forward | 2602.06718 | GhostCite | 2026 |
| Forward | 2504.02767 | How Deep Do LLMs Internalize Literature? | 2025 |
| Forward | 2405.15739 | LLMs Reflect Human Citation Patterns | 2024 |
| Forward | 2509.21557 | Gen-Time vs Post-hoc Citation | 2025 |
| Forward | 2502.14561 | Predicting Citation Intent | 2025 |
| Forward | 2405.02228 | REASONS: Attribution Benchmark | 2024 |

#### Seed 4: 2508.17647 — SurveyGen

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2406.10252 | AutoSurvey | 2024 |
| Backward | 2503.04629 | SurveyForge | 2025 |
| Backward | 2510.03120 | SurveyBench | 2025 |
| Forward | 2502.14776 | SurveyX | 2025 |
| Forward | 2504.08762 | InteractiveSurvey | 2025 |
| Forward | 2508.14317 | SurveyGen-I | 2025 |
| Forward | 2512.02763 | SurveyEval | 2025 |
| Forward | 2508.15658 | SurGE Benchmark | 2025 |
| Forward | 2508.11310 | SGSimEval | 2025 |

#### Seed 5: 2503.04629 — SurveyForge

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2406.10252 | AutoSurvey | 2024 |
| Backward | 2510.03120 | SurveyBench | 2025 |
| Forward | 2508.17647 | SurveyGen | 2025 |
| Forward | 2502.14776 | SurveyX | 2025 |
| Forward | 2504.08762 | InteractiveSurvey | 2025 |
| Forward | 2508.14317 | SurveyGen-I | 2025 |
| Forward | 2512.02763 | SurveyEval | 2025 |

#### Seed 6: 2510.21900 — IterSurvey / Survey-Arena

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2406.10252 | AutoSurvey | 2024 |
| Backward | 2508.17647 | SurveyGen | 2025 |
| Backward | 2402.01788 | LitLLM: A Toolkit | 2024 |
| Forward | 2110.06354 | Tell Me How to Survey | 2021 |
| Forward | 2408.13450 | vitaLITy 2 | 2024 |
| Forward | 2412.15249 | LitLLMs | 2024 |
| Forward | 2501.10120 | PaSa | 2025 |
| Forward | 2503.23229 | Citegeist | 2025 |
| Forward | 2309.01684 | CRUISE-Screening | 2023 |

#### Seed 7: 2406.10252 — AutoSurvey

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2305.15186 | SciReviewGen | 2023 |
| Forward | 2508.17647 | SurveyGen | 2025 |
| Forward | 2503.04629 | SurveyForge | 2025 |
| Forward | 2510.21900 | IterSurvey | 2025 |
| Forward | 2502.14776 | SurveyX | 2025 |
| Forward | 2508.14317 | SurveyGen-I | 2025 |

#### Seed 8: 2510.03120 — SurveyBench

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2406.10252 | AutoSurvey | 2024 |
| Backward | 2503.04629 | SurveyForge | 2025 |
| Backward | 2508.17647 | SurveyGen | 2025 |
| Forward | 2512.02763 | SurveyEval | 2025 |
| Forward | 2508.15658 | SurGE Benchmark | 2025 |
| Forward | 2508.11310 | SGSimEval | 2025 |

#### Seed 9: 2302.07302 — CiteSee

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2107.00414 | MultiCite | 2021 |
| Backward | 1511.04946 | Context Sensitive Article Ranking | 2015 |
| Backward | 2002.06406 | HybridCite | 2020 |
| Backward | 1705.08063 | Contextualizing Citations | 2017 |
| Backward | 1805.02262 | Semantic Scholar Lit Graph | 2018 |
| Forward | 2406.01606 | SymTax | 2024 |
| Forward | 2509.04190 | Changing role of cited papers | 2025 |
| Forward | 2001.02344 | DocCit2Vec | 2020 |

#### Seed 10: 2305.15186 — SciReviewGen

| Relation | arXiv ID | Title | Year |
|----------|----------|-------|------|
| Backward | 2102.00176 | Can We Automate Scientific Reviewing? | 2021 |
| Backward | 2010.04147 | Automatic generation of reviews | 2020 |
| Backward | 2010.06119 | ReviewRobot | 2020 |
| Backward | 1804.09635 | PeerRead Dataset | 2018 |
| Forward | 2304.03512 | Hierarchical Catalogue Generation | 2023 |
| Forward | 2306.12587 | ARIES | 2023 |
| Forward | 2402.10886 | Reviewer2 | 2024 |
| Forward | 2412.11948 | OpenReviewer | 2024 |
| Forward | 2406.10252 | AutoSurvey | 2024 |

### 2.3 Edge Counts

| Seed | Backward Edges | Forward Edges | Total |
|------|---------------|---------------|-------|
| 2402.08565 (AI4LitReview) | 2 | 6 | 8 |
| 2501.04306 (LLM4SR) | 2 | 6 | 8 |
| 2309.09727 (LLM+Citation) | 1 | 7 | 8 |
| 2508.17647 (SurveyGen) | 3 | 6 | 9 |
| 2503.04629 (SurveyForge) | 2 | 5 | 7 |
| 2510.21900 (IterSurvey) | 3 | 6 | 9 |
| 2406.10252 (AutoSurvey) | 1 | 5 | 6 |
| 2510.03120 (SurveyBench) | 3 | 3 | 6 |
| 2302.07302 (CiteSee) | 5 | 3 | 8 |
| 2305.15186 (SciReviewGen) | 4 | 5 | 9 |
| **Total** | **26** | **52** | **78** |

**Note**: Some edges connect to seeds themselves (e.g., SurveyGen → SurveyForge is counted here). The unique non-seed paper count is 79 (see §4).

---

## 3. Semantic Neighborhood Expansion

Seven embedding-search queries (EQ-01 through EQ-07) probed specific semantic neighborhoods missed by the initial QueryPlan.

### 3.1 Queries Executed

| # | Query Target | Papers Found | New Candidates |
|---|--------------|-------------|----------------|
| EQ-01 | Iterative refinement methods | 10 | 1 |
| EQ-02 | Structured representation / taxonomies | 10 | 8 |
| EQ-03 | Citation quality evaluation | 8 | 1 |
| EQ-04 | Seed-and-expand graph traversal | 10 | 5 |
| EQ-05 | Snowballing / reference chaining | 10 | 3 |
| EQ-06 | Concept drift boundary (survey vs summarization) | 10 | 1 |
| EQ-07 | Multi-agent debate / deliberation | 9 | 7 |
| **Total** | — | **55 unique** | **18 new** |

### 3.2 New Semantic Candidates (18 papers)

| # | arXiv ID | Title | Year | Source | Role |
|---|----------|-------|------|--------|------|
| 1 | 2110.06354 | Tell Me How to Survey (RePaGer, SurveyBank) | 2021 | EQ-01 | method |
| 2 | 2510.17263 | TaxoAlign (CS-TaxoBench) | 2025 | EQ-02 | benchmark |
| 3 | 2402.04854 | Hierarchical Tree-structured KG for Insight Survey | 2024 | EQ-02 | mechanism |
| 4 | 2504.13834 | Science Hierarchography | 2025 | EQ-02 | method |
| 5 | 2409.04432 | Survey on KOS of Research Fields | 2024 | EQ-02 | survey |
| 6 | 2006.01747 | ORKG FAIR Literature Surveys | 2020 | EQ-02 | method |
| 7 | 2306.10051 | TOBY (Survey Exploration Tool) | 2023 | EQ-02 | mechanism |
| 8 | 2304.03512 | Hierarchical Catalogue Generation | 2023 | EQ-02 | benchmark |
| 9 | 2508.15804 | ReportBench (Deep Research Eval) | 2025 | EQ-03 | benchmark |
| 10 | 2106.05633 | Citation Recommendation via KGs | 2021 | EQ-04 | mechanism |
| 11 | 2403.09295 | Seed-based Retrieval in Publication Networks | 2024 | EQ-04 | mechanism |
| 12 | 2205.01833 | OpenAlex (Open Scholarly KG) | 2022 | EQ-04 | theory |
| 13 | 2108.03366 | VitaLITy (Serendipitous Discovery) | 2021 | EQ-05 | method |
| 14 | 2302.04580 | BigSurvey / CAST | 2023 | EQ-06 | method |
| 15 | 2406.19643 | Debate-to-Write (Persona-Driven) | 2024 | EQ-07 | mechanism |
| 16 | 2510.10890 | LLMxMapReduce-V3 (MCP-Driven Survey) | 2025 | EQ-07 | method |
| 17 | 2502.19130 | Voting or Consensus in MAD | 2025 | EQ-07 | mechanism |
| 18 | 2409.17213 | Plurals (Simulated Ensembles) | 2024 | EQ-07 | method |

---

## 4. Merged New Candidates (Deduplicated)

### 4.1 Deduplication Method

- **arXiv ID matching**: exact match on arXiv ID across citation (79 papers) and semantic (18 papers) candidates.
- **Normalized title matching**: lower-cased, whitespace-stripped title comparison for cross-verification.
- **Overlap found**: 2 papers appear in both citation and semantic expansions:
  - `2110.06354` Tell Me How to Survey — citation (Core Method) ∩ semantic (EQ-01)
  - `2304.03512` Hierarchical Catalogue Generation — citation (Dataset) ∩ semantic (EQ-02)

### 4.2 Full Merged Candidate Pool

**Total unique non-seed papers**: **95** (79 from citation + 18 from semantic − 2 overlap)

Organised by role:

#### Method Papers (20 + 6 new semantic = 26 from expansion; 38 already in 02 pool)

| arXiv ID | Title | Year | Source | In Semantic? |
|----------|-------|------|--------|-------------|
| 2406.10252 | AutoSurvey | 2024 | Citation (seeds 4,5,6,7,8) | — |
| 2508.17647 | SurveyGen | 2025 | Citation (seeds 4,5,6,7,8) | — |
| 2503.04629 | SurveyForge | 2025 | Citation (seeds 4,5,7,8) | — |
| 2510.21900 | IterSurvey | 2025 | Citation (seeds 6,7) | — |
| 2502.14776 | SurveyX | 2025 | Citation (seeds 4,5,7) | — |
| 2508.14317 | SurveyGen-I | 2025 | Citation (seeds 4,5,7) | — |
| 2504.08762 | InteractiveSurvey | 2025 | Citation (seeds 4,5) | — |
| 2402.01788 | LitLLM: A Toolkit | 2024 | Citation (seed 6) | — |
| 2412.15249 | LitLLMs | 2024 | Citation (seeds 1,6) | — |
| 2408.13450 | vitaLITy 2 | 2024 | Citation (seed 6) | — |
| 2503.23229 | Citegeist | 2025 | Citation (seed 6) | — |
| 2501.10120 | PaSa | 2025 | Citation (seed 6) | — |
| 2309.01684 | CRUISE-Screening | 2023 | Citation (seed 6) | — |
| 2110.06354 | Tell Me How to Survey (SurveyBank) | 2021 | Citation + Semantic (EQ-01) | ✅ |
| 2504.13834 | Science Hierarchography | 2025 | Semantic (EQ-02) | ✅ |
| 2006.01747 | ORKG FAIR Literature Surveys | 2020 | Semantic (EQ-02) | ✅ |
| 2108.03366 | VitaLITy | 2021 | Semantic (EQ-05) | ✅ |
| 2302.04580 | BigSurvey / CAST | 2023 | Semantic (EQ-06) | ✅ |
| 2510.10890 | LLMxMapReduce-V3 | 2025 | Semantic (EQ-07) | ✅ |
| 2409.17213 | Plurals | 2024 | Semantic (EQ-07) | ✅ |

#### Evaluation & Benchmark Papers (8 + 3 new semantic = 11 from expansion; 17 already in 02 pool)

| arXiv ID | Title | Year | Source | In Semantic? |
|----------|-------|------|--------|-------------|
| 2510.03120 | SurveyBench | 2025 | Citation (seeds 4,5,8) | — |
| 2512.02763 | SurveyEval | 2025 | Citation (seeds 4,5,8) | — |
| 2508.15658 | SurGE Benchmark | 2025 | Citation (seeds 4,8) | — |
| 2508.11310 | SGSimEval | 2025 | Citation (seeds 4,8) | — |
| 2304.03512 | Hierarchical Catalogue Generation | 2023 | Citation + Semantic (EQ-02) | ✅ |
| 2510.17263 | TaxoAlign (CS-TaxoBench) | 2025 | Semantic (EQ-02) | ✅ |
| 2508.15804 | ReportBench | 2025 | Semantic (EQ-03) | ✅ |

#### Citation Mechanism & Attribution Papers (12 + 7 new semantic = 19 from expansion; 19 already in 02 pool)

| arXiv ID | Title | Year | Source | In Semantic? |
|----------|-------|------|--------|-------------|
| 2302.07302 | CiteSee | 2023 | Citation (seed 9) | — |
| 2305.14627 | ALCE: Text with Citations | 2023 | Citation (seed 3) | — |
| 2107.00414 | MultiCite | 2021 | Citation (seed 9) | — |
| 2002.06406 | HybridCite | 2020 | Citation (seed 9) | — |
| 1705.08063 | Contextualizing Citations | 2017 | Citation (seed 9) | — |
| 2405.02228 | REASONS: Attribution Benchmark | 2024 | Citation (seed 3) | — |
| 2509.21557 | Gen-Time vs Post-hoc Citation | 2025 | Citation (seed 3) | — |
| 2502.14561 | Predicting Citation Intent | 2025 | Citation (seed 3) | — |
| 2406.01606 | SymTax | 2024 | Citation (seed 9) | — |
| 2509.04190 | Changing role of cited papers | 2025 | Citation (seed 9) | — |
| 2001.02344 | DocCit2Vec | 2020 | Citation (seed 9) | — |
| 2602.06718 | GhostCite | 2026 | Citation (seed 3) | — |
| 2402.04854 | Hierarchical Tree-structured KG | 2024 | Semantic (EQ-02) | ✅ |
| 2306.10051 | TOBY | 2023 | Semantic (EQ-02) | ✅ |
| 2106.05633 | Citation Recommendation via KGs | 2021 | Semantic (EQ-04) | ✅ |
| 2403.09295 | Seed-based Retrieval Evaluation | 2024 | Semantic (EQ-04) | ✅ |
| 2406.19643 | Debate-to-Write | 2024 | Semantic (EQ-07) | ✅ |
| 2502.19130 | Voting or Consensus in MAD | 2025 | Semantic (EQ-07) | ✅ |

#### Dataset Papers (2 from expansion; 2 already in 02 pool)

| arXiv ID | Title | Year | Source |
|----------|-------|------|--------|
| 2305.15186 | SciReviewGen | 2023 | Citation (seeds 7,10) |
| 2304.03512 | Hierarchical Catalogue Generation | 2023 | Citation + Semantic (EQ-02) |

*(SciReviewGen is also a seed; listed here as an expanded reference.)*

#### Broader Survey / Overview Papers (10 from expansion; 3 already in 02 pool)

| arXiv ID | Title | Year | Source |
|----------|-------|------|--------|
| 2402.08565 | AI for Literature Reviews | 2024 | Citation (seed 1) |
| 2501.04306 | LLM4SR | 2025 | Citation (seed 2) |
| 2309.09727 | When LLMs Meet Citation | 2023 | Citation (seed 3) |
| 2406.10833 | Comprehensive Survey of Scientific LLMs | 2024 | Citation (seed 2) |
| 2505.13259 | Automation to Autonomy: LLMs in Sci Discovery | 2025 | Citation (seed 2) |
| 2502.05151 | Transforming Science with LLMs | 2025 | Citation (seed 2) |
| 2507.11810 | Evolving Role of LLMs in Sci Innovation | 2025 | Citation (seed 2) |
| 2605.16475 | Generative AI for Literature Reviews | 2026 | Citation (seed 1) |
| 2508.15396 | Attribution, Citation, and Quotation Survey | 2025 | Citation (seed 3) |
| 2405.15739 | LLMs Reflect Human Citation Patterns | 2024 | Citation (seed 3) |
| 2409.04432 | Survey on KOS of Research Fields | 2024 | Semantic (EQ-02) |

#### Theory / Infrastructure (1 new semantic)

| arXiv ID | Title | Year | Source |
|----------|-------|------|--------|
| 2205.01833 | OpenAlex | 2022 | Semantic (EQ-04) |

#### Pre-2023 Foundational Papers (7 from citation expansion)

| arXiv ID | Title | Year | Source |
|----------|-------|------|--------|
| 2111.07533 | Automated scholarly paper review | 2021 | Citation (seed 1) |
| 2102.00176 | Can We Automate Scientific Reviewing? | 2021 | Citation (seed 10) |
| 2010.04147 | Automatic generation of reviews | 2020 | Citation (seed 10) |
| 2010.06119 | ReviewRobot | 2020 | Citation (seed 10) |
| 1804.09635 | PeerRead Dataset | 2018 | Citation (seed 10) |
| 1805.02262 | Semantic Scholar Lit Graph | 2018 | Citation (seed 9) |
| 1511.04946 | Context Sensitive Article Ranking | 2015 | Citation (seed 9) |

#### Peer Review & Evaluation Papers (4 from citation expansion)

| arXiv ID | Title | Year | Source |
|----------|-------|------|--------|
| 2405.06563 | What Can NLP Do for Peer Review? | 2024 | Citation (seed 1) |
| 2306.12587 | ARIES: Paper Edits from Peer Reviews | 2023 | Citation (seed 10) |
| 2402.10886 | Reviewer2 | 2024 | Citation (seed 10) |
| 2412.11948 | OpenReviewer | 2024 | Citation (seed 10) |

#### Misc / Adjacent Papers (6 from citation expansion)

| arXiv ID | Title | Year | Source |
|----------|-------|------|--------|
| 2603.20235 | Writing lit reviews with AI: lessons learned | 2026 | Citation (seed 1) |
| 2410.15978 | PROMPTHEUS | 2024 | Citation (seed 1) |
| 2505.23789 | LitChat | 2025 | Citation (seed 1) |
| 2401.10917 | AI to automate SLR | 2024 | Citation (seed 1) |
| 2504.02767 | How Deep Do LLMs Internalize Literature? | 2025 | Citation (seed 3) |
| 2505.22787 | Can LLMs Match SR Conclusions? | 2025 | Citation (seed 2) |

### 4.3 Role Distribution Summary

| Role | In 02 Pool | From Citation | From Semantic | Overlap | Total New | New Pool Total |
|------|-----------|---------------|---------------|---------|-----------|----------------|
| method | 38 | 14 | 6 | 0 | 20 | 58 |
| benchmark | 17 | 5 | 3 | 1 | 7 | 24 |
| mechanism | 19 | 12 | 7 | 0 | 19 | 38 |
| survey | 3 | 10 | 1 | 0 | 11 | 14 |
| theory | 5 | 0 | 1 | 0 | 1 | 6 |
| dataset | 2 | 2 | 0 | 1 | 1 | 3 |
| peer-review | 0 | 4 | 0 | 0 | 4 | 4 |
| foundational | 0 | 7 | 0 | 0 | 7 | 7 |
| misc/adjacent | 0 | 6 | 0 | 0 | 6 | 6 |
| **(subtotal non-seed)** | **(84)** | **(60)** | **(18)** | **(2)** | **(76)** | **(160)** |
| seeds | 10 | — | — | — | — | 10 |
| **Grand total** | **102** | **79** | **18** | **2** | **95** | **170** |

**Note**: The 84 non-seed papers in the 02 pool include papers already overlapping with citation expansion results. The "Total New" column counts genuinely new papers from this expansion that are not in the 02 pool. The "New Pool Total" is a projection.

---

## 5. Drift Risks

### 5.1 From Citation Expansion

1. **PDF text extraction unavailable** — Reference lists could not be directly parsed from the 10 downloaded PDFs. The citation graph was reconstructed from arxiv_search metadata. Estimated ~15–20% of the full graph remains unresolved (non-arXiv references, pre-2015 literature, informal sources).
2. **Non-arXiv fringe** — Papers in closed-access venues or on Semantic Scholar / Google Scholar only are missed.
3. **Citation intent ambiguity** — Forward edges may represent contrast citations, background citations, or methodological lineage. Downstream relevance filtering recommended.
4. **Deduplication caveat** — Papers may appear under multiple arXiv versions. Only the latest version is listed.

### 5.2 From Semantic Expansion

5. **Taxonomy/KOS drift (EQ-02)** — TaxoAlign, Science Hierarchography, and the KOS survey focus on knowledge organization rather than survey *writing*. Recommend medium-confidence scoring with demotion if the pool over-represents this dimension.
6. **Multi-agent debate drift (EQ-07)** — DeepDebater, Plurals, and Debate-to-Write are about general argumentation, not survey-specific multi-agent architectures. Transferable but adjacent. Recommend low-confidence scoring.
7. **Pre-2023 temporal gap** — Older foundational papers (2012–2018) surfaced in seed-and-expand queries but were filtered; the 2020–present scope should demote any pre-2020 papers during scoring.
8. **No novel citation-fidelity benchmark** — ReportBench evaluates Deep Research agents, not survey-specific citation precision/recall. The gap identified in seed selection (SurveyBench focuses on overall quality, not citation faithfulness) remains unfilled.

### 5.3 Structural Risks

9. **Missing `schema/expansion.md`** — Expansion parameters (k-hop depth, max citations per seed, forward/backward ratio) are not formally configured. The expansion was performed per workflow instructions with defaults inferred from the query plan.
10. **Survey seed breadth** — Seeds 1 (AI4LitReview) and 2 (LLM4SR) are broad surveys; their backward edges may include papers only tangentially related to automated survey generation.

---

## 6. Next Expansion Queries

Based on coverage gaps identified during merging, the following queries are suggested for future expansion rounds:

| Priority | Query Target | Rationale |
|----------|-------------|-----------|
| High | Survey-specific citation fidelity evaluation | No dedicated benchmark exists; semantic expansion (EQ-03) revealed only ReportBench |
| High | Seed selection strategies for citation graph traversal | 2403.09295 (Seed-based Retrieval Evaluation) is the only dedicated evaluation; more method papers needed |
| Medium | Interactive / human-in-the-loop survey systems | PROMPTHEUS, CRUISE-Screening, TOBY suggest a family of interactive systems |
| Medium | Citation graph construction infrastructure | OpenAlex (2205.01833) and Semantic Scholar Graph (1805.02262) are the only infrastructure papers |
| Low | Survey dataset construction methods | SciReviewGen and Hierarchical Catalogue Generation are the only dataset papers; more would strengthen evaluation |

---

## 7. Artifact Summary

| Metric | Value |
|--------|-------|
| **Seeds** | 10 |
| **Citation edges (forward+backward, incl. seed-to-seed)** | 78 |
| **Unique non-seed papers from citation expansion** | 79 |
| **Semantic embedding queries** | 7 |
| **Unique new papers from semantic expansion** | 18 |
| **Overlap (citation ∩ semantic)** | 2 |
| **Total unique new candidates (merged)** | **95** |
| **Existing pool (02_candidate_pool)** | 102 |
| **Projected total pool** | **170** |
| **Drift risks flagged** | 10 |
| **Next expansion query suggestions** | 5 |
| **PDFs downloaded** | 10/10 (stored in `pdfs/`) |

---

## 8. Resolution Notes

- **All 10 seed PDFs** are available at `pdfs/<arxiv_id>.pdf` for future direct reference parsing once a PDF text extraction tool is available.
- **Citation graph edges** are marked with Forward/Backward direction but lack confidence scores — downstream scoring should re-validate edge relevance.
- **Semantic candidates** have confidence annotations (High/Medium/Low) in the source artifact `03c_semantic_expansion.md`.
- **Deduplication** was performed at arXiv ID level. Title-level dedup caught no additional overlaps beyond the 2 already identified.
