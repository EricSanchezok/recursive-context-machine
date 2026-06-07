# Extended Survey Candidates — Surveys, Reviews, Tutorials, and Taxonomies from Extended Query Plan

**run_dir**: `.`
**source**: `02_candidate_pool.md`, `01b_query_plan_extended.md`
**generated**: 2026-06-07T10:25:26+08:00
**scout**: ExtendedSurveyScout

---

## Overview

This artifact records survey, review, tutorial, taxonomy, and comprehensive overview papers identified from executing the 13 extended queries (EM-01 through EBX-01) on arXiv. Each paper is classified into one or more role categories.

**Total extended queries executed**: 13  
**Total papers retrieved**: ~134 (across all queries; some duplicates)  
**Survey/Review/Tutorial/Taxonomy candidates identified**: 16  

---

## Survey Candidates

### S-ES-01: Survey of LLM-based Scientific Agents
| Field | Value |
|-------|-------|
| **arXiv ID** | 2503.24047 |
| **Title** | Towards Scientific Intelligence: A Survey of LLM-based Scientific Agents |
| **Authors** | Shuo Ren, Pu Jian, Zhenjiang Ren, Chunlin Leng, Can Xie, Jiajun Zhang |
| **Category** | cs.AI, cs.MA |
| **Published** | 2025-03-31 |
| **Query source** | EM-03 |
| **Role** | **survey** |
| **Relevance** | Focused review of architectures, design, benchmarks, applications of LLM-based scientific agents — directly adjacent to survey agent architectures |

---

### S-ES-02: Review of LLM-Based Agent Paradigms
| Field | Value |
|-------|-------|
| **arXiv ID** | 2406.05804 |
| **Title** | A Review of Prominent Paradigms for LLM-Based Agents: Tool Use (Including RAG), Planning, and Feedback Learning |
| **Authors** | Xinzhe Li |
| **Category** | cs.AI, cs.CL, cs.SE |
| **Published** | 2024-06-09 |
| **Query source** | EM-03 |
| **Role** | **review** |
| **Relevance** | Unified taxonomy of LLM agent paradigms (tool use, planning, feedback); directly relevant to how survey agents orchestrate tools |

---

### S-ES-03: Survey of Agentic Large Language Models
| Field | Value |
|-------|-------|
| **arXiv ID** | 2503.23037 |
| **Title** | Agentic Large Language Models, a survey |
| **Authors** | Aske Plaat, Max van Duijn, Niki van Stein, Mike Preuss, Peter van der Putten, Kees Joost Batenberg |
| **Category** | cs.AI, cs.CL, cs.LG |
| **Published** | 2025-03-29 |
| **Query source** | EM-03 |
| **Role** | **survey** |
| **Relevance** | Covers reasoning, acting, interaction in LLM agents — foundational reading for building autonomous survey agents |

---

### S-ES-04: Review of AI for Systematic Literature Reviews
| Field | Value |
|-------|-------|
| **arXiv ID** | 2402.08565 |
| **Title** | Artificial Intelligence for Literature Reviews: Opportunities and Challenges |
| **Authors** | Francisco Bolanos, Angelo Salatino, Francesco Osborne, Enrico Motta |
| **Category** | cs.AI, cs.HC, cs.IR |
| **Published** | 2024-02-13 |
| **Query source** | EM-06 |
| **Role** | **review** |
| **Relevance** | Comprehensive review of 21 SLR tools + 11 recent LLM-based tools; directly targets the intersection of AI x systematic literature review |

---

### S-ES-05: OpenCitations Infrastructure Overview
| Field | Value |
|-------|-------|
| **arXiv ID** | 1906.11964 |
| **Title** | OpenCitations, an infrastructure organization for open scholarship |
| **Authors** | Silvio Peroni, David Shotton |
| **Category** | cs.DL |
| **Published** | 2019-06-27 |
| **Query source** | EB-01 |
| **Role** | **survey/overview** |
| **Relevance** | Comprehensive overview of OpenCitations dataset, data model, services, and ontologies — foundational citation infrastructure |

---

## Review / Analysis / Study Candidates

### R-ES-01: Citation Amnesia — Recency Bias Study
| Field | Value |
|-------|-------|
| **arXiv ID** | 2402.12046 |
| **Title** | Citation Amnesia: On The Recency Bias of NLP and Other Academic Fields |
| **Authors** | Jan Philip Wahle, Terry Ruas, Mohamed Abdalla, Bela Gipp, Saif M. Mohammad |
| **Category** | cs.DL, cs.CL |
| **Published** | 2024-02-19 |
| **Query source** | EP-02 |
| **Role** | **large-scale analysis / study** |
| **Relevance** | 43-year analysis of citation age recession across 20 fields; directly relevant to temporal dynamics and literature obsolescence |

---

### R-ES-02: In-text Citation Characterization
| Field | Value |
|-------|-------|
| **arXiv ID** | 1710.03094 |
| **Title** | Characterizing in-text citations in scientific articles: A large-scale analysis |
| **Authors** | Kevin W. Boyack, Nees Jan van Eck, Giovanni Colavizza, Ludo Waltman |
| **Category** | cs.DL |
| **Published** | 2017-10-09 |
| **Query source** | EM-05 |
| **Role** | **large-scale analysis / characterization** |
| **Relevance** | 5M+ full-text articles analyzed for citation position, interval, counts by field — essential empirical context for citation-aware survey agents |

---

### R-ES-03: Machine Understanding of Scientific Language (Thesis)
| Field | Value |
|-------|-------|
| **arXiv ID** | 2506.23990 |
| **Title** | Machine Understanding of Scientific Language |
| **Authors** | Dustin Wright |
| **Category** | cs.CL, cs.LG |
| **Published** | 2025-06-30 |
| **Query source** | EM-07 |
| **Role** | **comprehensive thesis / review** |
| **Relevance** | Covers fact-checking, claim verification, contradiction detection, scientific text processing — relevant to how survey agents handle conflicting evidence |

---

### R-ES-04: Seed-based Retrieval Review
| Field | Value |
|-------|-------|
| **arXiv ID** | 2403.09295 |
| **Title** | Seed-based information retrieval in networks of research publications |
| **Authors** | Peter Sjögårde, Per Ahlgren |
| **Category** | cs.IR |
| **Published** | 2024-03-14 |
| **Query source** | EM-04 |
| **Role** | **method paper with comprehensive review** |
| **Relevance** | "Fairly comprehensive review of earlier research" on citation relations for IR; evaluation of direct citation, bibliographic coupling, co-citation |

---

### R-ES-05: LitLLMs — Are we there yet?
| Field | Value |
|-------|-------|
| **arXiv ID** | 2412.15249 |
| **Title** | LitLLMs, LLMs for Literature Review: Are we there yet? |
| **Authors** | Shubham Agarwal, Gaurav Sahu, Abhay Puri et al. |
| **Category** | cs.CL, cs.AI, cs.DL, cs.LG |
| **Published** | 2024-12-15 |
| **Query source** | EP-01 |
| **Role** | **evaluation study** |
| **Relevance** | Zero-shot abilities of LLMs for literature review; two-step search strategy + writing evaluation; cost-quality perspective |

---

## Tutorial / Tool-Guide Candidates

### T-ES-01: CitNetExplorer Tutorial
| Field | Value |
|-------|-------|
| **arXiv ID** | 1404.5322 |
| **Title** | CitNetExplorer: A new software tool for analyzing and visualizing citation networks |
| **Authors** | Nees Jan van Eck, Ludo Waltman |
| **Category** | cs.DL, cs.SI |
| **Published** | 2014-04-21 |
| **Query source** | EM-04 |
| **Role** | **software tutorial / tool guide** |
| **Relevance** | Demonstrates citation network analysis and clustering; useful as tutorial for citation graph methods |

---

### T-ES-02: CitNetExplorer + VOSviewer Clustering Tutorial
| Field | Value |
|-------|-------|
| **arXiv ID** | 1702.03411 |
| **Title** | Citation-based clustering of publications using CitNetExplorer and VOSviewer |
| **Authors** | Nees Jan van Eck, Ludo Waltman |
| **Category** | cs.DL |
| **Published** | 2017-02-11 |
| **Query source** | EBX-01 |
| **Role** | **tutorial / demonstration** |
| **Relevance** | Step-by-step demonstration of citation clustering; foundational methodology for anyone building citation-based survey agents |

---

### T-ES-03: ASReview Open Source Tool Guide
| Field | Value |
|-------|-------|
| **arXiv ID** | 2006.12166 |
| **Title** | Open Source Software for Efficient and Transparent Reviews |
| **Authors** | Rens van de Schoot et al. |
| **Category** | cs.IR, cs.LG |
| **Published** | 2020-06-22 |
| **Query source** | EM-06 |
| **Role** | **software tool / tutorial** |
| **Relevance** | Active learning + ML for systematic review screening; influential tool (ASReview) with tutorial-style description |

---

## Dataset / Benchmark Candidates (with survey-adjacent framing)

### D-ES-01: SurveySum — Survey Section Summarization Dataset
| Field | Value |
|-------|-------|
| **arXiv ID** | 2408.16444 |
| **Title** | SurveySum: A Dataset for Summarizing Multiple Scientific Articles into a Survey Section |
| **Authors** | Leandro Carísio Fernandes et al. |
| **Category** | cs.CL |
| **Published** | 2024-08-29 |
| **Query source** | EB-02 |
| **Role** | **benchmark / dataset** |
| **Relevance** | Directly targets summarization of multiple articles into a survey section — most aligned dataset for survey generation evaluation |

---

### D-ES-02: Open Research Knowledge Graph (ORKG) Infrastructure
| Field | Value |
|-------|-------|
| **arXiv ID** | 1901.10816 |
| **Title** | Open Research Knowledge Graph: Next Generation Infrastructure for Semantic Scholarly Knowledge |
| **Authors** | Mohamad Yaser Jaradeh et al. |
| **Category** | cs.DL, cs.IR |
| **Published** | 2019-01-30 |
| **Query source** | ES-01 |
| **Role** | **position / infrastructure overview** |
| **Relevance** | First steps towards KG-based scholarly infrastructure; adjacent to citation graph traversal for survey purposes |

---

### D-ES-03: Clustering Methods Comparison
| Field | Value |
|-------|-------|
| **arXiv ID** | 1512.09023 |
| **Title** | Clustering scientific publications based on citation relations: A systematic comparison of different methods |
| **Authors** | Lovro Šubelj, Nees Jan van Eck, Ludo Waltman |
| **Category** | cs.DL, cs.SI |
| **Published** | 2015-12-30 |
| **Query source** | EBX-01 |
| **Role** | **systematic comparison / methodology survey** |
| **Relevance** | Systematic comparison of clustering methods for citation networks; expert-based assessment; key methodology reference |

---

## Summary Statistics

| Role Category | Count | Candidate IDs |
|--------------|-------|-------|
| **survey** | 3 | S-ES-01, S-ES-03, S-ES-05 |
| **review** | 2 | S-ES-02, S-ES-04 |
| **large-scale analysis / study** | 5 | R-ES-01, R-ES-02, R-ES-03, R-ES-04, R-ES-05 |
| **tutorial / tool guide** | 3 | T-ES-01, T-ES-02, T-ES-03 |
| **benchmark / dataset** | 3 | D-ES-01, D-ES-02, D-ES-03 |
| **Total** | **16** | |

---

## Notes

1. The extended queries are intentionally **broader** than the main query plan; therefore many retrieved papers are method/tool/system papers rather than surveys/reviews themselves. Only papers with explicit survey/review/tutorial/overview framing are listed above.
2. Papers from the `extended_problem` (EP-01, EP-02) and `extended_boundary` (EBX-01) queries yielded mostly empirical analyses and traditional bibliometric methods, not surveys.
3. The `extended_survey` query (ES-01: Knowledge Graphs for Scientific Literature) yielded infrastructure descriptions and method papers rather than surveys — the ORKG paper (1901.10816) is the closest to a survey/overview.
4. Several candidates (e.g., 2403.09295, 1512.09023) contain significant review/overview components embedded within method papers; they are marked accordingly.
5. **Previously unclassified papers** (not in the main pool) are marked with asterisks (\*) in the classification column.
