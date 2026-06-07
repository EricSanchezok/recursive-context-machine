# Paper Fetch Report

**run_dir**: `.`
**generated**: 2026-06-07T10:36:02+08:00
**agent**: PaperFetcher

---

## Summary

| Metric | Count |
|--------|-------|
| **Total papers targeted** | **72** |
| Downloaded successfully | 67 |
| Already existed | 5 |
| Failed | 0 |
| Skipped (no arXiv ID) | 0 |

### Target Breakdown

| Source | Method Papers | Benchmark Papers | Total |
|--------|--------------|-----------------|-------|
| Main pool (02_candidate_pool.md) | 33 | 18 | 51 |
| Extended pool (02b_candidate_pool_extended.md) | 10 | 11 | 21 |
| **Total unique** | **43** | **29** | **72** |

---

## Already Existed (5 papers)

These PDFs were already present in `pdfs/` and were not re-downloaded.

| arXiv ID | Title | File |
|----------|-------|------|
| 2406.10252 | AutoSurvey | pdfs/2406.10252.pdf |
| 2503.04629 | SurveyForge | pdfs/2503.04629.pdf |
| 2508.17647 | SurveyGen | pdfs/2508.17647.pdf |
| 2510.03120 | SurveyBench | pdfs/2510.03120.pdf |
| 2510.21900 | IterSurvey / Survey-Arena | pdfs/2510.21900.pdf |

---

## Downloaded (67 papers)

### Category A: Core Survey Generation Methods (Main Pool — 29 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2510.26012 | AutoSurvey2 | pdfs/2510.26012.pdf |
| 2509.18661 | Agentic AutoSurvey | pdfs/2509.18661.pdf |
| 2510.07733 | SurveyG (Hierarchical Citation Graph) | pdfs/2510.07733.pdf |
| 2502.14776 | SurveyX (AttributeTree) | pdfs/2502.14776.pdf |
| 2508.14317 | SurveyGen-I | pdfs/2508.14317.pdf |
| 2506.12689 | SciSage / SurveyScope | pdfs/2506.12689.pdf |
| 2504.08762 | InteractiveSurvey | pdfs/2504.08762.pdf |
| 2511.17689 | ARISE (Rubric-Guided Iterative) | pdfs/2511.17689.pdf |
| 2504.18496 | DimInd (Facets/Taxonomies/Syntheses) | pdfs/2504.18496.pdf |
| 2402.01788 | LitLLM | pdfs/2402.01788.pdf |
| 2408.07884 | Instruct LLMs Step by Step | pdfs/2408.07884.pdf |
| 2412.15249 | LitLLMs (evaluation protocol) | pdfs/2412.15249.pdf |
| 2407.01796 | ReClaim (Interleaved Reference-Claim) | pdfs/2407.01796.pdf |
| 2504.00824 | ScholarCopilot | pdfs/2504.00824.pdf |
| 2402.16063 | Citation-Enhanced Generation | pdfs/2402.16063.pdf |

### Category C: Additional Method Papers from SurveyScout (Main Pool — 7 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2509.19370 | Meow (Metadata-Driven Outline Writing) | pdfs/2509.19370.pdf |
| 2410.15978 | PROMPTHEUS (Human-Centered SLR Pipeline) | pdfs/2410.15978.pdf |
| 2504.14822 | InsightAgent (Systematic Review in Hours) | pdfs/2504.14822.pdf |
| 2403.08399 | System for SLR using multiple AI agents | pdfs/2403.08399.pdf |
| 2411.18583 | Automated Lit Review using NLP + RAG | pdfs/2411.18583.pdf |
| 2407.20906 | Auto Review Generation (PDH catalysis) | pdfs/2407.20906.pdf |
| 2312.09948 | GEAR-Up (Query Expansion + KG) | pdfs/2312.09948.pdf |

### Category E: Citation Graph Mechanisms (Main Pool — 2 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2005.11401 | RAG for Knowledge-Intensive NLP Tasks | pdfs/2005.11401.pdf |
| 2408.16444 | SurveySum (Multi-Article Summarization Dataset) | pdfs/2408.16444.pdf |

### Category F: Benchmarks & Evaluation (Main Pool — 17 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2508.15658 | SurGE | pdfs/2508.15658.pdf |
| 2512.02763 | SurveyEval | pdfs/2512.02763.pdf |
| 2601.15307 | DeepSurvey-Bench | pdfs/2601.15307.pdf |
| 2602.11238 | SurveyLens | pdfs/2602.11238.pdf |
| 2508.11310 | SGSimEval | pdfs/2508.11310.pdf |
| 2503.08506 | ReviewBench / Review-CoT | pdfs/2503.08506.pdf |
| 2601.14949 | CiteRAG (Citation Prediction Benchmark) | pdfs/2601.14949.pdf |
| 2407.12861 | CiteME (Citation Attribution) | pdfs/2407.12861.pdf |
| 2305.14251 | FActScore (Atomic Factuality) | pdfs/2305.14251.pdf |
| 2403.18802 | LongFact + SAFE | pdfs/2403.18802.pdf |
| 2509.25868 | ReFACT (Scientific Confabulation) | pdfs/2509.25868.pdf |
| 2204.04991 | TRUE (Factual Consistency Meta-Eval) | pdfs/2204.04991.pdf |
| 2412.13612 | LLMs for Auto Lit Review (Multi-dim eval) | pdfs/2412.13612.pdf |
| 2310.04480 | Auto-survey Challenge | pdfs/2310.04480.pdf |
| 2308.10410 | Wikipedia-style Survey Eval | pdfs/2308.10410.pdf |
| 2411.16638 | Factuality Metrics Critique | pdfs/2411.16638.pdf |

### Category H: Frontier Signals — Live Updating (Main Pool — 1 paper)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2510.15624 | Build Your Personalized Research Group | pdfs/2510.15624.pdf |

### Category J: Frontier Signals — Multi-Agent Orchestration (Main Pool — 5 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2603.13327 | DOVA (Deliberation-First Multi-Agent) | pdfs/2603.13327.pdf |
| 2603.03005 | OrchMAS (Multi-Agent Scientific Experts) | pdfs/2603.03005.pdf |
| 2507.07257 | Open Source Planning & Control (30 agents) | pdfs/2507.07257.pdf |
| 2509.20175 | Federation of Agents (Semantics-Aware) | pdfs/2509.20175.pdf |
| 2410.21784 | MARCO (Multi-Agent Real-time Chat Orchestration) | pdfs/2410.21784.pdf |

### Category XM: Extended Methods (Extended Pool — 10 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2409.13740 | PaperQA2 | pdfs/2409.13740.pdf |
| 2411.14199 | OpenScholar | pdfs/2411.14199.pdf |
| 2603.14629 | ResearchPilot | pdfs/2603.14629.pdf |
| 2312.07559 | PaperQA | pdfs/2312.07559.pdf |
| 2311.12315 | AcademicGPT | pdfs/2311.12315.pdf |
| 2404.07738 | ResearchAgent | pdfs/2404.07738.pdf |
| 2010.04147 | AutoReviewGen | pdfs/2010.04147.pdf |
| 2006.12166 | ASReview | pdfs/2006.12166.pdf |
| 2309.01684 | CRUISE-Screening | pdfs/2309.01684.pdf |
| 2510.26750 | ProfOlaf | pdfs/2510.26750.pdf |

### Category XC/XD: Extended Benchmarks — Citation & Contradiction (Extended Pool — 3 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2107.00414 | MultiCite (Multi-sentence Citation Dataset) | pdfs/2107.00414.pdf |
| 2004.14974 | SciFact (Scientific Claim Verification) | pdfs/2004.14974.pdf |
| 2311.09182 | ContraDoc (Self-contradiction Dataset) | pdfs/2311.09182.pdf |

### Category XI: Extended Benchmarks — Multi-Document Summarization (Extended Pool — 8 papers)

| arXiv ID | Title (short) | File |
|----------|--------------|------|
| 2010.14235 | Multi-XScience (Related-work Generation) | pdfs/2010.14235.pdf |
| 2405.01930 | OARelatedWork (94K papers, full-text) | pdfs/2405.01930.pdf |
| 2104.06486 | MS² (Medical Studies Summarization) | pdfs/2104.06486.pdf |
| 2203.01769 | PeerSum (Meta-review Summarization) | pdfs/2203.01769.pdf |
| 2403.05303 | ACLSum (Aspect-based Summarization) | pdfs/2403.05303.pdf |
| 2004.15011 | SciTLDR (Extreme Summarization) | pdfs/2004.15011.pdf |
| 2011.08072 | MAG-20 (Topic-Centric MDS) | pdfs/2011.08072.pdf |
| 2407.18940 | LitSearch (Retrieval Benchmark) | pdfs/2407.18940.pdf |

---

## Download Notes

1. **No failures** — all 67 targeted PDFs were successfully retrieved from arXiv.
2. **2203.01769 (PeerSum)** — initial download failed with HTTP 404. Retried with version suffix `v1` and succeeded.
3. **5 pre-existing method/benchmark PDFs** were skipped (AutoSurvey, SurveyForge, SurveyGen, SurveyBench, IterSurvey).
4. **5 pre-existing non-target PDFs** (CiteSee, SciReviewGen, LLM Citation Survey, AI for Lit Reviews, LLM4SR) were present but not relevant to this fetch.
5. **Total PDFs in `pdfs/` directory: 77** (67 newly downloaded + 10 pre-existing).
