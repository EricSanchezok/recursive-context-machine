# Seed Papers for Citation Graph Expansion — CitationSeedSelector

**run_dir**: `.`
**generated**: 2026-06-07T10:18:07+08:00
**source**: `02_candidate_pool.md`
**seed count**: 10

---

## Selection Strategy

Seeds are chosen for maximal downstream expansion coverage across four roles:

| Role | Count | Purpose in Expansion |
|------|-------|---------------------|
| **Survey** | 3 | Backward expansion — surveys already cite dozens of relevant papers in their bibliography |
| **Method** | 4 | Bi-directional expansion — anchors for both backward (prior work they cite) and forward (later work that cites them) |
| **Benchmark** | 1 | Forward-citation magnet — papers using this benchmark will be discovered |
| **Mechanism** | 1 | Targeted expansion — citation-context mechanism papers fill a specific gap |
| **Dataset** | 1 | Forward-citation magnet — large dataset used by many follow-up papers |

### Selection Criteria (in order)

1. **Must have arXiv ID** for reliable download and forward/backward resolution.
2. **Surveys of the field** (Category B) are preferred — they provide the highest backward-expansion yield.
3. **Cross-source consensus** — papers appearing in ≥2 scout outputs are preferred (validated from multiple angles).
4. **Bridge value** — papers connecting citation analysis with LLM survey generation are preferred.
5. **Temporal recency** — preference for 2024–2026 papers to maximize expansion into the active frontier.

---

## Seed Papers

### Seed 1: 2402.08565 — AI for Literature Reviews: Opportunities and Challenges
| Field | Value |
|-------|-------|
| **Short Title** | AI for Literature Reviews (survey) |
| **Year** | 2024 |
| **Category** | B (Existing Surveys) |
| **Source Agent(s)** | survey-scout |
| **Likely Role** | **survey** |
| **Expansion Strategy** | **Backward** — cites most major automated survey systems up to 2024 |
| **Justification** | Best single seed for backward expansion. An actual survey of the survey-generation field, its bibliography is a near-complete reference list of pre-2024 method papers. |

### Seed 2: 2501.04306 — LLM4SR: LLMs for Scientific Research
| Field | Value |
|-------|-------|
| **Short Title** | LLM4SR |
| **Year** | 2025 |
| **Category** | B (Existing Surveys) |
| **Source Agent(s)** | survey-scout |
| **Likely Role** | **survey** |
| **Expansion Strategy** | **Backward** — wider lens (LLMs for all scientific research, including survey writing) |
| **Justification** | Complementary to Seed 1. Covers LLM-specific innovations that the 2024 survey may have missed. Broader scientific context captures adjacent work. |

### Seed 3: 2309.09727 — When LLMs Meet Citation: A Survey
| Field | Value |
|-------|-------|
| **Short Title** | When LLMs Meet Citation |
| **Year** | 2023 |
| **Category** | K (Citation Seeds) |
| **Source Agent(s)** | frontier-scout (BX-02) |
| **Likely Role** | **citation_seed** |
| **Expansion Strategy** | **Backward + bridge** — direct survey of the intersection of LLMs and citation practices |
| **Justification** | Bridge paper between the two core concepts of the survey spec (LLM agents + citation graph expansion). Its bibliography contains both citation-analysis methods and LLM integration papers. |

### Seed 4: 2508.17647 — SurveyGen
| Field | Value |
|-------|-------|
| **Short Title** | SurveyGen |
| **Year** | 2025 |
| **Category** | A (Core Method) |
| **Source Agent(s)** | method-scout, benchmark-scout, survey-scout, **frontier-scout** |
| **Likely Role** | **method** |
| **Expansion Strategy** | **Bi-directional** — appears in all 4 scouts |
| **Justification** | Highest-consensus paper in the pool (4/4 scouts). Central method paper for forward/backward citation expansion. Its references and citations will map the core of the field. |

### Seed 5: 2503.04629 — SurveyForge
| Field | Value |
|-------|-------|
| **Short Title** | SurveyForge |
| **Year** | 2025 |
| **Category** | A (Core Method) |
| **Source Agent(s)** | method-scout, benchmark-scout, **survey-scout** |
| **Likely Role** | **method** |
| **Expansion Strategy** | **Bi-directional** — appears in 3 scouts |
| **Justification** | Second-highest consensus. Method + benchmark + survey coverage. Complements SurveyGen as a different architectural approach. |

### Seed 6: 2510.21900 — IterSurvey / Survey-Arena
| Field | Value |
|-------|-------|
| **Short Title** | IterSurvey / Survey-Arena |
| **Year** | 2025 |
| **Category** | A (Core Method) |
| **Source Agent(s)** | method-scout, benchmark-scout, **survey-scout** |
| **Likely Role** | **method** |
| **Expansion Strategy** | **Bi-directional** — appears in 3 scouts |
| **Justification** | Bridges method and evaluation (Survey-Arena is an evaluation framework). Good for discovering evaluation-related citations. |

### Seed 7: 2406.10252 — AutoSurvey
| Field | Value |
|-------|-------|
| **Short Title** | AutoSurvey |
| **Year** | 2024 |
| **Category** | A (Core Method) |
| **Source Agent(s)** | method-scout, survey-scout |
| **Likely Role** | **method** |
| **Expansion Strategy** | **Backward** — earliest major method paper (2024), cites foundational pre-2024 work |
| **Justification** | Oldest method seed. Its bibliography captures the pre-2024 foundation that later methods (SurveyGen, SurveyForge) build on. Essential for recovering historical context. |

### Seed 8: 2510.03120 — SurveyBench
| Field | Value |
|-------|-------|
| **Short Title** | SurveyBench |
| **Year** | 2025 |
| **Category** | F (Benchmarks) |
| **Source Agent(s)** | benchmark-scout, survey-scout |
| **Likely Role** | **benchmark** |
| **Expansion Strategy** | **Forward** — papers that use SurveyBench for evaluation will cite it |
| **Justification** | Key evaluation benchmark for survey generation. Forward-citation expansion from this seed will recover papers that evaluate their methods against it. |

### Seed 9: 2302.07302 — CiteSee (Persistent Citation Context)
| Field | Value |
|-------|-------|
| **Short Title** | CiteSee |
| **Year** | 2023 |
| **Category** | E (Citation Graph Mechanisms) |
| **Source Agent(s)** | method-scout, frontier-scout |
| **Likely Role** | **mechanism** |
| **Expansion Strategy** | **Bi-directional** — directly targets the citation graph expansion theme |
| **Justification** | Appears in 2 scouts. Provides citation context mechanisms that are core to how survey agents retrieve and contextualize citations. Bridges mechanism and method. |

### Seed 10: 2305.15186 — SciReviewGen
| Field | Value |
|-------|-------|
| **Short Title** | SciReviewGen |
| **Year** | 2023 |
| **Category** | G (Datasets) |
| **Source Agent(s)** | benchmark-scout |
| **Likely Role** | **dataset** |
| **Expansion Strategy** | **Forward** — large dataset (10K reviews, 690K cited papers) used by many evaluation papers |
| **Justification** | Massive dataset for survey evaluation. Forward-citation expansion from this seed will recover papers that use it for training or evaluation, including many 2024–2025 method papers. |

---

## Expansion Coverage Map

| Seed | Backward | Forward | Bridge |
|------|----------|---------|--------|
| 2402.08565 (AI4LitReview) | ★★★★★ | ★★★ | — |
| 2501.04306 (LLM4SR) | ★★★★ | ★★★ | — |
| 2309.09727 (LLM+Citation) | ★★★★ | ★★★ | ★★★★★ |
| 2508.17647 (SurveyGen) | ★★★★ | ★★★★★ | — |
| 2503.04629 (SurveyForge) | ★★★★ | ★★★★ | — |
| 2510.21900 (IterSurvey) | ★★★ | ★★★★ | ★★★ |
| 2406.10252 (AutoSurvey) | ★★★★★ | ★★★★ | — |
| 2510.03120 (SurveyBench) | ★★ | ★★★★★ | — |
| 2302.07302 (CiteSee) | ★★★ | ★★★ | ★★★★ |
| 2305.15186 (SciReviewGen) | ★★★ | ★★★★★ | — |

**Legend**: ★ = low, ★★★★★ = high. Backward = papers this seed cites. Forward = papers citing this seed. Bridge = connects method and citation graph themes.

---

## Risks

1. **No `schema/expansion.md` exists** — Selection strategy follows workflow instructions and the query plan. Expansion parameters (k-hop depth, max seed citations, forward/backward ratio) should be configured by the downstream ExpansionAgent.
2. **Seed 3 (2309.09727)** is from the BX-02 boundary guard, which may contain content bordering on pure citation analysis without LLM integration. Included for bridge value; the downstream expansion classifier should verify relevance.
3. **Three surveys as seeds** — The backward expansion yield from surveys is high but may include papers that are too general (e.g., LLM4SR covers all of scientific research, not just surveys). The ExpansionAgent should apply relevance filtering.
4. **Seed temporal range**: 2023–2025. No pre-2023 seeds included (older theory papers were evaluated but excluded in favor of higher-value surveys). Pre-2023 work will be recovered through backward expansion from the 2023–2025 seeds.
5. **No arXiv ID without PDF** concern — All 10 seeds have arXiv IDs and are expected to have downloadable PDFs for full-text processing.
