# Extended Survey Candidates — Surveys, Reviews, Tutorials, Taxonomy, and Related Overviews

**Generated:** 2026-06-07T18:04Z  
**Run dir:** `.`  
**Phase:** ExtendedSurveyScout (arXiv search for extended meta-survey papers)

---

## Methodology

Queried arXiv with each of the 12 extended queries (E1–E12) from `01b_query_plan_extended.md`, topK=10 each. Each result was tagged by type: **survey**, **review**, **tutorial**, **taxonomy**, **guidelines**, **benchmark**, or **related overview**. Deduplicated against the existing 69-entry pool from `03_expansion.md`.

---

## Candidate Inventory (22 unique entries)

### A. Taxonomy Papers (3)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 1 | 2410.03761 | 2024 | Taxonomy Tree Generation from Citation Graph (HiGTL) | E1 | **taxonomy** | End-to-end hierarchical taxonomy learning from citation graphs; recursive clustering + LLM verbalization |
| 2 | 2402.04854 | 2024 | Hierarchical Tree-structured Knowledge Graph For Academic Insight Survey | E1 | **taxonomy** / **survey** | KG for beginner researchers; hierarchy reflects inheritance and relevance insight |
| 3 | 2504.13834 | 2025 | Science Hierarchography: Hierarchical Organization of Science Literature | E1 | **taxonomy** | Embedding-based clustering + LLM prompting for multi-level science hierarchy |

### B. Survey / Review Papers (6)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 4 | 2002.06961 | 2020 | Citation Recommendation: Approaches and Datasets | E2, E6 | **survey** | Comprehensive survey of citation recommendation; covers approaches, datasets, evaluation — only dedicated survey on this topic |
| 5 | 2402.12046 | 2024 | Citation Amnesia: On The Recency Bias of NLP and Other Academic Fields | E6 | **survey** / **analysis** | Quantitative study of recency bias across 20 fields over 43 years; 240M papers dataset |
| 6 | 2305.18554 | 2023 | Forgotten Knowledge: Examining the Citational Amnesia in NLP | E6 | **survey** / **analysis** | 71.5K NLP papers analysis; shows 62% citations from immediate 5 years, median age declining since 2014 |
| 7 | 2501.06322 | 2025 | Multi-Agent Collaboration Mechanisms: A Survey of LLMs | E8 | **survey** | Comprehensive survey of LLM-based multi-agent systems; taxonomy of collaboration mechanisms |
| 8 | 2103.04044 | 2021 | Putting Humans in the Natural Language Processing Loop: A Survey | E10 | **survey** | HITL NLP survey covering feedback types, human interaction models, learning methods |
| 9 | 2412.15249 | 2024 | LitLLMs, LLMs for Literature Review: Are we there yet? | E4 | **survey** / **review** | Zero-shot assessment of LLMs for literature review; two-step retrieval+generation framework evaluation |

### C. Tutorial / Book (1)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 10 | 2504.12501 | 2025 | Reinforcement Learning from Human Feedback (Nathan Lambert) | E9 | **tutorial** / **book** | Comprehensive book-length tutorial on RLHF covering origins, math, reward modeling, RL, DPO, open questions |

### D. Guidelines / Protocols (1)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 11 | 2301.13298 | 2023 | LongEval: Guidelines for Human Evaluation of Faithfulness in Long-form Summarization | E5, E11 | **guidelines** | Human evaluation guidelines for faithfulness; survey of 162 papers; clause-level annotation reduces inter-annotator variance |

### E. Systematic Review Methodology Papers (3)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 12 | 2006.12166 | 2020 | ASReview: Open Source Software for Efficient and Transparent Reviews | E12 | **systematic review methodology** | Active learning pipeline for title/abstract screening; simulation studies show major efficiency gains |
| 13 | 2409.04600 | 2024 | The emergence of LLM as a tool in literature reviews: an LLM automated systematic review | E12 | **systematic review** | LLM-assisted SR of 3,788 articles on LLM use in review automation; 73.2% GPT-based; 15.1% actual reviews |
| 14 | 2407.18657 | 2024 | SWARM-SLR: Streamlined Workflow Automation for Machine-actionable SLR | E12 | **systematic review workflow** | Composes 65 requirements from guidelines; maps 11 tools to SLR lifecycle stages |

### F. Benchmark / Evaluation Frameworks (5)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 15 | 2510.03120 | 2025 | SurveyBench: How Well Can LLM(-Agents) Write Academic Surveys? | E11 | **benchmark** | 11,343 arXiv papers + 4,947 surveys; quiz-driven evaluation; 21% gap human vs. LLM |
| 16 | 2512.02763 | 2025 | SurveyEval: Comprehensive Evaluation of LLM-Generated Academic Surveys | E5, E11 | **benchmark** | Three dimensions (quality, outline, reference); 7 subjects; human-reference augmentation |
| 17 | 2601.15307 | 2026 | DeepSurvey-Bench: Evaluating Academic Value of Automatically Generated Surveys | E5 | **benchmark** | Three academic value dimensions: informational, scholarly communication, research guidance |
| 18 | 2508.15658 | 2025 | SurGE: Benchmarking Computer Science Survey Generation | E5, E7, E11 | **benchmark** | 1M+ paper corpus; 4 evaluation dimensions; CS domain-specific |
| 19 | 2602.11238 | 2026 | SurveyLens: A Research Discipline-Aware Benchmark for ASG | E7 | **benchmark** | First cross-discipline ASG benchmark; 1,000 surveys across 10 disciplines; dual-lens evaluation |

### G. Notable Related Overviews / Tools (3)

| # | arXiv ID | Year | Title | Query Source | Type | Notes |
|---|----------|------|-------|-------------|------|-------|
| 20 | 2401.03545 | 2024 | Is there really a Citation Age Bias in NLP? | E6 | **analysis** | Counter-argument to Citation Amnesia; ~300k papers across 15 fields; attributes trend to field dynamics |
| 21 | 2604.02507 | 2026 | RLHF: A Statistical Perspective | E9 | **survey** (statistical) | Statistical perspective on RLHF; BTL model, active learning, uncertainty quantification |
| 22 | 2411.11761 | 2024 | Mapping out the Space of Human Feedback for RL: A Conceptual Framework | E9 | **conceptual framework** / **taxonomy** | 9-dimension taxonomy of feedback types; 7 quality metrics of human feedback |

---

## Gap Analysis vs. 03_expansion.md Pool

| Gap Area | Previously Covered (main pool) | New Candidates | Net Gain |
|----------|-------------------------------|----------------|----------|
| Graph/tree-based literature analysis | ~3 papers (SurveyG, LitFM) | 3 taxonomy papers (2410.03761, 2402.04854, 2504.13834) + 1 survey (2002.06961) | **+4** |
| RAG for literature survey | ~5 systems (AutoSurvey, OpenScholar, SurveyX) | 0 new (all systems already captured) | **0** |
| Citation quality / noise | 1 paper (2508.12735) | 1 guideline (2301.13298) + 3 benchmarks (2512.02763, 2601.15307, 2508.15658) | **+4** |
| Temporal bias | 0 | 3 survey/analysis papers (2402.12046, 2305.18554, 2401.03545) | **+3** |
| Cross-domain transfer | 0 | 1 benchmark (2602.11238) | **+1** |
| Multi-agent architectures | 2 papers (Agentic AutoSurvey, SciSage) | 1 survey (2501.06322) + 1 conceptual (2411.11761) | **+2** |
| Post-training / RL | 1 paper (PaSa) | 1 tutorial (2504.12501) + 1 statistical survey (2604.02507) | **+2** |
| Human-in-the-loop | 1 paper (InteractiveSurvey) | 1 survey (2103.04044) | **+1** |
| Human evaluation protocols | ~2 benchmarks | 4 benchmarks (2510.03120, 2512.02763, 2601.15307, 2508.15658) | **+4** |
| Systematic review alignment | 0 | 3 papers (2006.12166, 2409.04600, 2407.18657) + tools | **+3** |

---

## Deduplication Notes

All 22 candidates are **new** — none appear in the 69-entry main pool (`03_expansion.md`). However, some survey generation *systems* returned by searches (e.g., AutoSurvey2, SurveyGen-I, Agentic AutoSurvey, InteractiveSurvey, SurveyForge, OpenScholar, LitLLM) are excluded from this list because they are already represented in the main pool or are primary systems rather than surveys/reviews about the domain.

---

## Key Recommendations for Synthesizer

1. **Citation Age / Recency Bias** is the most actionable new dimension — 3 papers (2402.12046, 2305.18554, 2401.03545) directly challenge the temporal validity of citation graph expansion, a known high-severity risk.
2. **Citation Recommendation survey** (2002.06961) is the only dedicated survey on this sub-topic; essential as a grounding reference for citation graph methods.
3. **LongEval** (2301.13298) provides immediately usable human evaluation guidelines for survey faithfulness — relevant for both the benchmark and evaluation dimensions of the survey brief.
4. **SurveyLens** (2602.11238) is the first cross-discipline ASG benchmark; critical for the cross-domain transfer gap area.
5. **RLHF tutorial/book** (2504.12501) and the **statistical RLHF survey** (2604.02507) together provide comprehensive coverage of the post-training optimization gap.
