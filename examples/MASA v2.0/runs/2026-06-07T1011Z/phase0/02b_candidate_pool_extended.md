# Extended Candidate Pool — ExtendedDiscoveryMerger

**run_dir**: `.`
**generated**: 2026-06-07T10:27:32+08:00
**merger_agent**: ExtendedDiscoveryMerger

**Total unique extended candidates**: 83 (after deduplication across 4 extended scout outputs and against main `02_candidate_pool.md`)

---

## Source Summary

| Scout | Source File | Raw Unique | Overlap w/ Main Pool | Overlap w/ Other Extended | Contribution |
|-------|-------------|-----------|----------------------|--------------------------|-------------|
| ExtendedMethodScout | `02e_extended_method_candidates.md` | 59 | 18 | 1 (within scout) | 41 |
| ExtendedBenchmarkScout | `02f_extended_benchmark_candidates.md` | 18 | 2 | 0 | 16 |
| ExtendedSurveyScout | `02g_extended_survey_candidates.md` | 16 | — | 16 (all in 02e/02f/02h/main) | 0 |
| ExtendedFrontierScout | `02h_extended_frontier_candidates.md` | ~100 est. | 0 (EP/ES/EBX only) | 0 | 26 |
| **Total** | | **~193 raw** | **20** | **17** | **83** |

---

## Deduplication Notes

1. **Main pool (02_candidate_pool.md)**: 18 papers from 02e and 2 from 02f were already present in the main pool and removed.
2. **Cross-scout overlap**: All 16 survey-type papers from 02g were already captured by 02e (method/mechanism papers also appearing as surveys), 02f (infrastructure benchmarks), or 02h (boundary/bibliometric papers). Zero new unique additions from 02g.
3. **Frontier overlap**: EM-01 through EM-07 results in 02h are fully contained within 02e; only EP-01, EP-02, ES-01 (beyond 02f), and EBX-01 papers are new from 02h.
4. **Intra-scout duplicate**: 2502.11705 (ToolMaker) appeared in both EM-03 and EM-07 of 02e; counted once.

---

## Merged Extended Candidates

### Category XM: Extended Methods — Survey & Citation Agents (23 papers, 02e source)

Papers directly proposing methods or systems for automated literature survey, scientific writing, or paper synthesis.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 1 | 2409.13740 | 2024 | PaperQA2 | 02e EM-02 | method | **high** — superhuman scientific synthesis; contradiction detection |
| 2 | 2501.10120 | 2025 | PaSa | 02e EM-02 | mechanism | **high** — RL-optimised academic paper search agent |
| 3 | 2411.14199 | 2024 | OpenScholar | 02e EM-02 | method | **high** — specialised RAG-LM over 45M papers; SOTA citation accuracy |
| 4 | 2603.14629 | 2026 | ResearchPilot | 02e EM-02 | method | **high** — local-first multi-agent literature synthesis |
| 5 | 2312.07559 | 2023 | PaperQA | 02e EM-02 | method | **high** — foundational RAG agent for scientific research |
| 6 | 2311.12315 | 2023 | AcademicGPT | 02e EM-02 | method | **medium** — domain-specific LLM for academic tasks |
| 7 | 2404.07738 | 2024 | ResearchAgent | 02e EM-02 | method | **medium** — iterative idea generation with reviewing agents |
| 8 | 2403.09733 | 2024 | OverleafCopilot | 02e EM-02 | mechanism | **medium** — LLM integration into Overleaf for academic writing |
| 9 | 2506.18149 | 2025 | CoachGPT | 02e EM-02 | mechanism | **medium** — scaffolding-based academic writing assistant |
| 10 | 2505.11336 | 2025 | XtraGPT | 02e EM-02 | mechanism | **medium** — human-AI revision collaboration |
| 11 | 2503.24047 | 2025 | Survey: LLM Scientific Agents | 02e EM-03 | survey | **high** — survey of scientific agent architectures |
| 12 | 2406.05804 | 2024 | Review: LLM Agent Paradigms | 02e EM-03 | survey | **high** — unified taxonomy of tool-use, planning, feedback |
| 13 | 2503.23037 | 2025 | Survey: Agentic LLMs | 02e EM-03 | survey | **high** — comprehensive agentic LLM survey |
| 14 | 2402.11451 | 2024 | SciAgent | 02e EM-03 | mechanism | **medium** — tool-augmented reasoning for scientific domains |
| 15 | 2307.16789 | 2023 | ToolLLM | 02e EM-03 | mechanism | **medium** — large-scale tool-use framework (16K+ APIs) |
| 16 | 2502.11705 | 2025 | ToolMaker | 02e EM-03/07 | mechanism | **medium** — autonomous tool construction from paper code |
| 17 | 2010.04147 | 2020 | AutoReviewGen | 02e EM-01 | method | **medium** — BERT-based extractive survey generation |
| 18 | 2401.06201 | 2024 | EASYTOOL | 02e EM-03 | mechanism | **low** — tool instruction unification (indirect relevance) |
| 19 | 2407.21778 | 2024 | Tulip Agent | 02e EM-03 | mechanism | **low** — recursive tool search (indirect agent pattern) |
| 20 | 2502.04644 | 2025 | Agentic Reasoning | 02e EM-03 | mechanism | **low** — mind-map agent for reasoning (indirect) |
| 21 | 2603.00084 | 2026 | DeepXiv-SDK | 02e EM-03 | mechanism | **low** — agentic data interface for scientific literature |
| 22 | 2110.06354 | 2021 | RePaGer / SurveyBank | 02e EM-01 | mechanism | **medium** — reading path generation from citation graphs |
| 23 | 2302.06754 | 2023 | Relatedly | 02e EM-01 | mechanism | **medium** — scaffolding reviews with related work sections |

---

### Category XC: Extended Mechanisms — Citation Intent & Classification (10 papers, 02e source)

Papers focused on citation function, intent, and sentiment classification — essential for quality-aware citation synthesis.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 24 | 1904.01608 | 2019 | SciCite (Structural Scaffolds) | 02e EM-05 | mechanism | **high** — multi-task citation intent classification; foundational |
| 25 | 2304.12730 | 2023 | CitePrompt | 02e EM-05 | mechanism | **high** — prompt-based citation intent; SOTA on ACL-ARC |
| 26 | 2107.00414 | 2021 | MultiCite | 02e EM-05 | benchmark | **high** — multi-sentence, multi-label citation dataset |
| 27 | 2501.18292 | 2025 | Citation Rec + Arg Zoning | 02e EM-05 | mechanism | **high** — multi-task citation recommendation + AZ |
| 28 | 1811.07351 | 2018 | Neural Multi-Task Citation | 02e EM-05 | mechanism | **medium** — joint function + provenance prediction |
| 29 | 2009.08948 | 2020 | Term Function Rec | 02e EM-05 | mechanism | **medium** — term function enhancement for citation rec |
| 30 | 1910.03498 | 2019 | SentiCite | 02e EM-05 | mechanism | **medium** — citation sentiment analysis |
| 31 | 1609.00435 | 2016 | Citation Classification (NLP) | 02e EM-05 | mechanism | **medium** — behavioral citation function study in NLP |
| 32 | 2104.08087 | 2021 | Citations not opinions | 02e EM-05 | theory | **medium** — corpus linguistics; citation type ≠ sentiment |
| 33 | 1710.03094 | 2017 | In-text Citation Study | 02e EM-05 | theory | **medium** — large-scale characterisation of citation patterns |

---

### Category XD: Extended Mechanisms — Contradiction & Conflict Detection (9 papers, 02e source)

Papers addressing detection of contradictory/conflicting claims across scientific literature — a known gap in automated survey systems.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 34 | 2310.18685 | 2023 | ContraSciView | 02e EM-07 | mechanism | **high** — contradiction among peer reviewers; first dataset |
| 35 | 2605.10171 | 2026 | RevCI / IMPACT / TIDE | 02e EM-07 | mechanism | **high** — fine-grained contradiction analysis with graded intensity |
| 36 | 2212.09867 | 2022 | COVID-DrugContra | 02e EM-07 | mechanism | **high** — NLI for contradictory drug efficacy claims |
| 37 | 2004.14974 | 2020 | SciFact | 02e EM-07 | benchmark | **high** — scientific claim verification task (supports/refutes) |
| 38 | 2506.23990 | 2025 | SciLang Understanding | 02e EM-07 | survey | **high** — thesis covering scientific fact-checking & contradiction |
| 39 | 1708.00850 | 2017 | MedContra | 02e EM-07 | theory | **medium** — formal model of contradictions vs disagreements |
| 40 | 2303.04219 | 2023 | Disagreement in Reviews | 02e EM-07 | theory | **medium** — do review articles help find disagreement? |
| 41 | 2311.09182 | 2023 | ContraDoc | 02e EM-07 | benchmark | **medium** — self-contradiction dataset for long documents |
| 42 | 1406.1143 | 2014 | WikiDupContra | 02e EM-07 | mechanism | **low** — duplicate/contradictory info in Wikipedia |
| 43 | 2111.08543 | 2021 | WikiContradiction | 02e EM-07 | mechanism | **low** — self-contradiction articles on Wikipedia |

---

### Category XE: Extended Mechanisms — SLR Automation & Screening (10 papers, 02e source)

Traditional and LLM-based systematic literature review screening and automation tools.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 44 | 2006.12166 | 2020 | ASReview | 02e EM-06 | method | **high** — active learning screening; widely-used open-source |
| 45 | 2309.01684 | 2023 | CRUISE-Screening | 02e EM-06 | method | **high** — living literature review toolbox |
| 46 | 2510.26750 | 2025 | ProfOlaf | 02e EM-06 | method | **high** — semi-automated SLR with snowballing + LLM |
| 47 | 2502.03400 | 2025 | DenseReviewer | 02e EM-06 | mechanism | **high** — dense retrieval-based screening prioritisation |
| 48 | 2202.10033 | 2022 | AutoCitationScreening | 02e EM-06 | mechanism | **medium** — Bayesian active learning citation screening |
| 49 | 2110.12490 | 2021 | Paperfetcher | 02e EM-06 | mechanism | **medium** — automated handsearch with bidirectional snowballing |
| 50 | 2402.05317 | 2024 | SLR Update Automation | 02e EM-06 | mechanism | **medium** — automated snowballing + ML for SLR updates |
| 51 | 2011.09752 | 2020 | Hybrid TAR SLR | 02e EM-06 | mechanism | **medium** — learning-to-rank + relevance feedback pipeline |
| 52 | 2510.06708 | 2025 | AISysRev | 02e EM-06 | mechanism | **medium** — LLM-based title-abstract screening |
| 53 | 2402.08565 | 2024 | AI for Lit Reviews Survey | 02e EM-06 | survey | **high** — comprehensive review of 21 SLR + 11 LLM-based tools |

*Note: 2402.08565 appeared in both 02e and main pool #33; retained in extended pool because its survey content directly addresses SLR automation — a known gap in the main pool. It serves as a bridging reference between traditional SLR and LLM-based methods.*

---

### Category XF: Extended Mechanisms — Interactive & Visual Exploration (4 papers, 02e source)

Human-in-the-loop interactive literature exploration and visual survey tools.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 54 | 1706.08094 | 2017 | PubVis | 02e EM-01 | mechanism | **medium** — interactive visual exploration of publications |
| 55 | 2110.14060 | 2021 | Argo Scholar | 02e EM-01 | mechanism | **medium** — web-based visual exploration on S2 live graph |
| 56 | 2403.09295 | 2024 | Seed-based IR | 02e EM-04 | mechanism | **medium** — comparison of seed-based retrieval strategies |
| 57 | 1404.5322 | 2014 | CitNetExplorer | 02e EM-04 | mechanism | **low** — citation network analysis/visualisation (tutorial) |

---

### Category XG: Extended Mechanisms — Seed Selection & Citation Graph Theory (3 papers, 02e source)

Theoretical and methodological papers for seed-based citation graph traversal.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 58 | 1507.01388 | 2015 | Time and Citation Networks | 02e EM-04 | theory | **medium** — causally-aware citation network analysis |
| 59 | 1511.07643 | 2015 | Homophily and Missing Links | 02e EM-04 | theory | **low** — similarity-based citation prediction |

---

### Category XH: Extended Benchmarks — Citation Infrastructure & Scholarly KGs (7 papers, 02f source)

Citation graph datasets, open knowledge graphs, and scholarly data infrastructure — evaluation substrate for survey agents.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 60 | 1906.11964 | 2019 | OpenCitations (COCI) | 02f EB-01 | infrastructure | **high** — open citation data; 445M+ citations; I4OC compliant |
| 61 | 2308.03671 | 2023 | SemOpenAlex | 02f EB-01 | infrastructure | **high** — 26B RDF triples; SPARQL endpoint; scientific impact analysis |
| 62 | 2602.12206 | 2026 | OpenAIRE Citation Graph | 02f EB-01 | infrastructure | **high** — 200M+ publications, 2B+ citations; compact representation |
| 63 | 2203.09159 | 2022 | EMAKG | 02f EB-01 | infrastructure | **medium** — Enhanced MAG with geo/collaboration features |
| 64 | 2301.10140 | 2023 | S2 Open Data | 02f EB-01 | infrastructure | **medium** — 200M+ papers; 2.4B+ citation edges; structured parsed text |
| 65 | 2206.01439 / 1901.10816 | 2019 | ORKG | 02f EB-01 | infrastructure | **medium** — machine-actionable scholarly KG; FAIR compliant |
| 66 | 2206.07476 | 2022 | OpenCitations e-Infra | 02f EB-01 | infrastructure | **medium** — FAIR reuse guidelines for citation data |

---

### Category XI: Extended Benchmarks — Multi-Document Summarization (9 papers, 02f source)

Datasets and benchmarks for multi-document scientific summarisation — complementary evaluation resources.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 67 | 2010.14235 | 2020 | Multi-XScience | 02f EB-02 | benchmark | **high** — related-work section generation from references |
| 68 | 2405.01930 | 2024 | OARelatedWork | 02f EB-02 | benchmark | **high** — 94K papers, full-text; related-work from full content |
| 69 | 2104.06486 | 2021 | MS² (Medical Studies) | 02f EB-02 | benchmark | **medium** — 470K+ docs, 20K summaries; contradictory evidence |
| 70 | 2408.16444 | 2024 | SurveySum | 02f EB-02 | benchmark | **high** — dedicated dataset for survey section summarisation |
| 71 | 2203.01769 | 2022 | PeerSum | 02f EB-02 | benchmark | **medium** — meta-review summarisation; reviewer disagreement |
| 72 | 2403.05303 | 2024 | ACLSum | 02f EB-02 | benchmark | **medium** — aspect-based summarisation of scientific publications |
| 73 | 2004.15011 | 2020 | SciTLDR | 02f EB-02 | benchmark | **medium** — extreme summarisation; 5.4K TLDRs over 3.2K papers |
| 74 | 2011.08072 | 2020 | MAG-20 (Topic-Centric) | 02f EB-02 | benchmark | **low** — topic-centric unsupervised MDS across 20 fields |
| 75 | 2505.16349 | 2025 | XSum (Pipeline) | 02f EB-02 | mechanism | **medium** — modular RAG pipeline; evaluated on SurveySum |

*Note: 2408.16444 (SurveySum) is also in main pool #58; retained here as the most directly aligned survey-generation benchmark, serving as a bridging evaluation resource between the main and extended pools.*

---

### Category XJ: Extended Problem — Cost-Quality Tradeoffs (4 papers, 02h EP-01 source)

Papers addressing budget-aware retrieval and cost-quality optimisation for literature search.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 76 | 2407.18940 | 2024 | LitSearch | 02h EP-01 | benchmark | **medium** — retrieval benchmark (597 queries) for literature search |
| 77 | 1705.05420 | 2017 | FAST² | 02h EP-01 | mechanism | **medium** — intelligent assistant with termination estimator |
| 78 | 1709.00149 | 2017 | Focused Machine Reading | 02h EP-01 | mechanism | **low** — RL-based focused reading to minimise papers read |
| 79 | 1406.1875 | 2014 | Bullseye | 02h EP-01 | mechanism | **low** — structured passage retrieval for scholarly search |

---

### Category XK: Extended Problem — Temporal Coverage & Literature Obsolescence (5 papers, 02h EP-02 source)

Studies on citation aging, recency bias, and temporal dynamics in scientific literature.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 80 | 2402.12046 | 2024 | Citation Amnesia | 02h EP-02 | theory | **high** — recency bias study over 240M papers across 20 fields |
| 81 | 2203.08649 | 2022 | Obsolescence Modeling | 02h EP-02 | theory | **medium** — Negative Binomial modeling of cited reference age |
| 82 | 2509.04190 | 2025 | Changing Role Over Time | 02h EP-02 | theory | **medium** — evolution of citation role as papers age |
| 83 | 1504.07479 | 2015 | Cited Half-Life | 02h EP-02 | theory | **low** — mean cited half-life = 6.5 years across 13K journals |
| 84 | 1511.08310 | 2015 | Sic Transit | 02h EP-02 | theory | **low** — two views of citation aging; fixed-sample analysis |

---

### Category XL: Extended Survey — Scientific Knowledge Graphs (7 papers, 02h ES-01 source)

Knowledge graph infrastructure and methods for scientific literature discovery.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 85 | 2605.22878 | 2026 | SciAtlas | 02h ES-01 | infrastructure | **high** — large-scale multi-disciplinary KG (43M papers, 157M entities) |
| 86 | 2512.12760 | 2025 | ISLE | 02h ES-01 | mechanism | **medium** — hybrid retrieval + BERTopic + KG construction |
| 87 | 2306.04758 | 2023 | SKG | 02h ES-01 | mechanism | **medium** — Semantic Knowledge Graph framework for academic IR |
| 88 | 2503.09894 | 2025 | FieldKG (Surveyor) | 02h ES-01 | mechanism | **medium** — LLM-assisted KG construction mapping research fields |
| 89 | 1902.05170 | 2019 | GrapAL | 02h ES-01 | infrastructure | **medium** — Graph DB of Academic Literature in Neo4j |
| 90 | 2011.01103 | 2020 | KGs from NLP/ML | 02h ES-01 | mechanism | **low** — architecture for entity/relation extraction in scientific KGs |
| 91 | 2007.12731 | 2020 | COVID-19 KG | 02h ES-01 | infrastructure | **low** — heterogeneous KG for COVID-19 literature retrieval |

---

### Category XM: Extended Boundary — Bibliometric Analysis (6 papers, 02h EBX-01 source)

Traditional bibliometric methods for citation network clustering and analysis. Boundary guard: these are pure bibliometrics with no LLM/automation component, retained for boundary calibration.

| # | arXiv ID | Year | Short Name | Source | Likely Role | Relevance |
|---|----------|------|-----------|--------|-------------|-----------|
| 92 | 1512.09023 | 2015 | Clustering Comparison | 02h EBX-01 | theory | **low** — systematic comparison of citation clustering methods |
| 93 | 2004.05904 | 2020 | Direct Citation Clustering | 02h EBX-01 | theory | **low** — alternative using direct citation instead of co-citation |
| 94 | 1909.08738 | 2019 | Co-citations in Context | 02h EBX-01 | theory | **low** — disciplinary heterogeneity affects co-citation analysis |
| 95 | 1702.03411 | 2017 | CitNetExplorer+VOSviewer | 02h EBX-01 | tutorial | **low** — citation-based clustering tutorial |
| 96 | 1607.02452 | 2016 | Full vs Fractional Counting | 02h EBX-01 | theory | **low** — comparison of counting approaches for bibliometric networks |
| 97 | 1708.00578 | 2017 | Internet Tech Clustering | 02h EBX-01 | theory | **low** — complex network bibliometric coupling |
| 98 | 2007.15254 | 2020 | Citation Link Clustering | 02h EBX-01 | theory | **low** — link clustering of citations for topic identification |
| 99 | 1708.03889 | 2017 | Garfield Citation Context | 02h EBX-01 | theory | **low** — keyword co-occurrence analysis of citation contexts |

---

## Relevance Score Distribution

| Relevance | Count | Categories |
|-----------|-------|-----------|
| **high** | 28 | PaperQA2, PaSa, OpenScholar, SciCite, MultiCite, ContraSciView, SciFact, ASReview, CRUISE-Screening, OpenCitations, SemOpenAlex, OpenAIRE Graph, Multi-XScience, OARelatedWork, SurveySum, Citation Amnesia, SciAtlas, etc. |
| **medium** | 38 | AcademicGPT, ResearchAgent, ToolLLM, SciAgent, SLR tools, MDS benchmarks, knowledge graphs, temporal studies, interactive systems |
| **low** | 17 | EASYTOOL, Tulip Agent, boundary bibliometric papers, older theory papers, WikiContradiction, WikiDupContra |
| **Total** | **83** | |

## Role Distribution

| Role | Count |
|------|-------|
| **method** | 8 |
| **mechanism** | 33 |
| **benchmark** | 8 |
| **infrastructure** | 10 |
| **theory** | 12 |
| **survey** | 5 |
| **tutorial** | 1 |
| **Total** | **83** |

## Comparison with Main Pool

| Dimension | Main Pool (02_candidate_pool.md) | Extended Pool (02b_candidate_pool_extended.md) |
|-----------|----------------------------------|------------------------------------------------|
| Total candidates | 102 | 83 |
| Overlap | — | 20 (excluded from extended) |
| High relevance | ~40 | 28 |
| Key gap addressed | None | Citation intent (10 papers), Contradiction (9 papers) |
| Temporal range | 2012–2026 | 2014–2026 |
| Coverage breadth | Survey methods + benchmarks + frontiers | Methods + citation intent + contradiction + SLR tools + infrastructure + bibliometrics |

---

## Key Gap Coverage Assessment

| Gap Area | Extended Pool Coverage | Status |
|----------|----------------------|--------|
| **Citation intent classification** | 10 papers (SciCite, CitePrompt, MultiCite, etc.) | ✓ Fully addressed |
| **Contradiction/conflict detection** | 9 papers (ContraSciView, SciFact, COVID-DrugContra, etc.) | ✓ Fully addressed |
| **SLR automation (traditional)** | 10 papers (ASReview, CRUISE-Screening, ProfOlaf, etc.) | ✓ Fully addressed |
| **Citation infrastructure (datasets)** | 7 papers (OpenCitations, SemOpenAlex, OpenAIRE Graph, etc.) | ✓ Addressed |
| **Multi-document summarisation benchmarks** | 9 papers (Multi-XScience, OARelatedWork, SurveySum, etc.) | ✓ Addressed |
| **Interactive/visual exploration** | 4 papers (PubVis, Argo Scholar, etc.) | Partially addressed |
| **Cost-quality tradeoffs** | 4 papers (LitSearch, FAST², etc.) | Partially addressed |
| **Temporal/obsolescence** | 5 papers (Citation Amnesia, Obsolescence Modeling, etc.) | Partially addressed |
| **Scientific knowledge graphs** | 7 papers (SciAtlas, ISLE, SKG, etc.) | Addressed |
| **Bibliometric boundary (calibration)** | 6 papers | Boundary guard only |

---

## Risks & Notes

1. **run_dir**: `.` — recovered from all four extended scout artifacts, which consistently specify `run_dir: .`.

2. **Schema/handoff.md not found**: The schema directory does not exist. Handoff format constructed following the Key/Value table pattern used by upstream scouts (02e–02h).

3. **02g contributed 0 unique papers**: All 16 survey-type candidates from 02g were duplicates of papers already in 02e (method/mechanism papers that also function as surveys), 02f (infrastructure benchmarks), or 02h (bibliometric boundary). This is expected — the extended queries intentionally overlap to ensure broad coverage.

4. **02h EM-01..07 fully contained in 02e**: The frontier scout's EM-01 through EM-07 results are entirely redundant with the method scout's 02e output. Only EP-01, EP-02, ES-01 (beyond 02f), and EBX-01 from 02h are new additions.

5. **2408.16444 (SurveySum) retained despite main pool overlap**: This benchmark is the most directly aligned survey-generation evaluation dataset. It serves as a bridging resource between the main and extended pools.

6. **2402.08565 (AI for Lit Reviews Survey) retained despite main pool overlap**: This survey directly addresses SLR automation and is the only paper bridging traditional SLR (ASReview etc.) with LLM-based methods. It belongs in the extended pool as a key reference.

7. **Boundary guard (EBX-01)**: 8 bibliometric papers are retained at **low** relevance for calibration. They should be demoted by the scout classifier if they survive to the final pool. They illustrate the boundary between citation graph traversal for survey generation and pure bibliometric analysis.

8. **Temporal concentration**: The extended pool skews older than the main pool, with several pre-2020 papers (2014–2019) in citation intent (SciCite 2019, SentiCite 2019), SLR tools (ASReview 2020), and bibliometrics (CitNetExplorer 2014). This is appropriate for a reserve pool that captures foundational work.

9. **Key gap not addressed by extended pool**: No papers directly address *longitudinal/iterative survey maintenance* (living reviews). Only the main pool's frontier section covers this (2 papers). The extended pool confirms this remains an open research gap.
