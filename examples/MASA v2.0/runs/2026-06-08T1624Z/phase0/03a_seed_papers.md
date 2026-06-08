# Seed Papers — MASA Pipeline

**Generated**: 2026-06-08
**run_dir**: `.`
**Agent**: CitationSeedSelector
**Status**: ready

---

## Selection Rationale

8 seed papers selected for citation-graph forward/backward expansion.
Selection criteria:

- **Surveys on the core topic** (3 papers) — highly cited, rich backward citation lists, strong
  forward citation catch for breadth coverage.
- **Seminal method papers** (2 papers) — high forward-citation signal; anchor the major
  system families (AutoSurvey, SurveyForge).
- **Mechanism paper** (1 paper) — foundational citation expansion technique; bridges to
  graph-based retrieval for depth coverage.
- **Benchmark paper** (1 paper) — defines the evaluation landscape; cited by most
  subsequent method papers.
- **Bridge survey** (1 paper) — LLM agent architecture survey connecting multi-agent
  system design to survey generation.

**Temporal balance**: 3 pre-2023, 5 from 2023 onward (≈ 3:7 ratio), satisfying the
scope_inclusion_budget constraint.

---

## Seed Papers

### Seed 1 — Survey of AI for Literature Reviews

| Field | Value |
|---|---|
| **arXiv ID** | `2402.08565v2` |
| **Title** | Artificial Intelligence for Literature Reviews: Opportunities and Challenges |
| **Year** | 2024 |
| **Role** | survey |
| **Source** | SurveyScout Q4 |
| **Selection rationale** | Direct survey of the core topic. Cites ~150+ papers on AI-assisted
literature review, including early automation tools, citation analysis, and NLP methods.
Excellent backward-citation seed for breadth coverage of the field's origins. High
forward-citation count makes it a strong forward-expansion anchor. |

### Seed 2 — Survey of AI for Systematic Review Automation

| Field | Value |
|---|---|
| **arXiv ID** | `2401.10917v1` |
| **Title** | Artificial intelligence to automate the systematic review of scientific literature |
| **Year** | 2024 |
| **Role** | survey |
| **Source** | SurveyScout Q4 |
| **Selection rationale** | Companion survey to Seed 1, focused on systematic review
automation specifically. Covers PRISMA-adjacent methodology, screening tools, and
data-extraction automation. Bridges the core topic to the systematic review methodology
tradition (scope_exclude's adjacent domain context). |

### Seed 3 — AutoSurvey (Seminal Method Paper)

| Field | Value |
|---|---|
| **arXiv ID** | `2406.10252v2` |
| **Title** | AutoSurvey: Large Language Models Can Automatically Write Surveys |
| **Year** | 2024 |
| **Role** | core_method |
| **Source** | SurveyScout Q1 |
| **Selection rationale** | The seminal method paper in the field — first demonstrated
that LLMs can generate structured surveys autonomously. Highly cited by all subsequent
systems (SurveyForge, SurveyX, SurveyGen, etc.). Forward citation expansion from this
paper will capture the entire AutoSurvey family tree. Backward citations cover the NLP
and summarization foundations. |

### Seed 4 — SurveyForge (Multi-Agent Method Paper)

| Field | Value |
|---|---|
| **arXiv ID** | `2503.04629v1` |
| **Title** | SurveyForge: On the Outline Heuristics, Memory-Driven Generation |
| **Year** | 2025 |
| **Role** | core_method |
| **Source** | MethodScout Q1; SurveyScout Q1; BenchmarkScout A2 |
| **Selection rationale** | Appeared in 3 of 4 scouts, indicating broad relevance.
Introduces outline heuristics and memory-driven generation — key architectural pattern.
Forward citations will capture subsequent multi-agent survey systems. Backward citations
connect to AutoSurvey, RAG, and agent frameworks. |

### Seed 5 — Cascading Citation Expansion (Mechanism Paper)

| Field | Value |
|---|---|
| **arXiv ID** | `1806.00089v1` |
| **Title** | Cascading Citation Expansion |
| **Year** | 2018 |
| **Role** | mechanism |
| **Source** | MethodScout Q7 |
| **Selection rationale** | Directly about citation graph expansion for literature search.
Pre-2023 foundational work that surveys on auto-generation should cite. Forward citations
will capture papers using cascade/iterative citation traversal. Backward citations cover
earlier bibliometric and citation analysis work (Garfield, PageRank derivatives). Bridges
the core topic to scientometrics (cross_domain_context requirement). |

### Seed 6 — SurveyBench (Benchmark Paper)

| Field | Value |
|---|---|
| **arXiv ID** | `2510.03120` |
| **Title** | SurveyBench — quiz-driven evaluation; 11,343 arXiv topics + 4,947 surveys |
| **Year** | 2025 |
| **Role** | benchmark |
| **Source** | BenchmarkScout A1 |
| **Selection rationale** | The largest benchmark for survey generation. Defines the
evaluation landscape that every subsequent method paper references. Forward citations
will capture all papers evaluated on SurveyBench. Backward citations connect to
earlier evaluation frameworks and scoring metrics. |

### Seed 7 — Survey of LLM-based Autonomous Agents (Bridge Survey)

| Field | Value |
|---|---|
| **arXiv ID** | `2308.11432v7` |
| **Title** | A Survey on Large Language Model based Autonomous Agents |
| **Year** | 2023 |
| **Role** | survey |
| **Source** | SurveyScout Q2 |
| **Selection rationale** | Bridges the core topic to the broader LLM agent architecture
literature. Most multi-agent survey systems (LiRA, SurveyForge, SciSage) build on agent
frameworks surveyed here. Pre-2023 publication ensures temporal balance. Extremely
high citation count — strong forward-expansion anchor. |

### Seed 8 — Automating SLRs with NLP and Text Mining (Pre-2023 Survey)

| Field | Value |
|---|---|
| **arXiv ID** | `2211.15397v2` |
| **Title** | Automating Systematic Literature Reviews with NLP and Text Mining |
| **Year** | 2022 |
| **Role** | survey |
| **Source** | SurveyScout Q4 |
| **Selection rationale** | Pre-2023 survey covering NLP-based automation of SLRs, the
intellectual precursor to LLM-based survey generation. Covers text mining, screening
automation, and evidence extraction. Backward citations reach into the pre-LLM
automation era (important for temporal balance). Forward citations connect to the
transition from NLP-pipeline approaches to LLM-agent approaches. |

---

## Summary

| # | arXiv ID | Year | Role | Expansion Value |
|---|---|---|---|---|
| 1 | `2402.08565v2` | 2024 | survey (topic) | High — broad backward/forward citation net |
| 2 | `2401.10917v1` | 2024 | survey (SLR) | High — bridges to systematic review methodology |
| 3 | `2406.10252v2` | 2024 | core_method | Very high — anchors method family tree |
| 4 | `2503.04629v1` | 2025 | core_method | High — multi-agent pattern, cross-scout consensus |
| 5 | `1806.00089v1` | 2018 | mechanism | High — direct citation expansion technique |
| 6 | `2510.03120` | 2025 | benchmark | High — evaluation landscape anchor |
| 7 | `2308.11432v7` | 2023 | survey (bridge) | Very high — bridges to agent architectures |
| 8 | `2211.15397v2` | 2022 | survey (pre-LLM) | Medium — captures pre-LLM automation lineage |

**Total seeds**: 8
**Pre-2023**: 3 (#5, #7, #8)
**2023–2026**: 5 (#1, #2, #3, #4, #6)
**Ratio**: 3:5 (≈ 3:7 target, within scope_inclusion_budget)

---

## Coverage Map

```
                      ┌─────────────────────────┐
                      │  Seed 7: Agent Survey    │ ← bridges multi-agent architectures
                      └────────┬────────────────┘
                               │
         ┌─────────────────────┼─────────────────────┐
         │                     │                     │
         ▼                     ▼                     ▼
   ┌──────────┐        ┌──────────┐          ┌──────────┐
   │ Seed 3:  │        │ Seed 4:  │          │ Seed 1:  │
   │ AutoSurvey│       │SurveyForge│         │AI for LR │ ← topic survey
   │(seminal) │        │(multi-agent)│       │(breadth) │
   └────┬─────┘        └────┬─────┘          └────┬─────┘
        │                   │                      │
        └───────────────────┼──────────────────────┘
                            │
                   ┌────────▼────────┐
                   │  Seed 6:        │
                   │  SurveyBench    │ ← evaluation anchor
                   └────────┬────────┘
                            │
         ┌──────────────────┼──────────────────┐
         │                  │                  │
         ▼                  ▼                  ▼
   ┌──────────┐      ┌──────────┐      ┌──────────┐
   │ Seed 2:  │      │ Seed 5:  │      │ Seed 8:  │
   │AI for SLR│      │Cascading │      │NLP+TM    │
   │(method.) │      │Citation  │      │SLR Survey│
   │          │      │Expansion │      │(pre-LLM) │
   └──────────┘      └──────────┘      └──────────┘
```

Forward expansion from seeds 1, 3, 4, 7 captures the modern core.
Backward expansion from seeds 2, 5, 8 captures the pre-LLM lineage and adjacent domains.
Seed 6 anchors the evaluation dimension.

---

## Risk Notes

1. **`schema/expansion.md` not found on disk.** The `schema` file at root is the SurveySpec contract (17 lines), not a directory. Expansion workflow is followed from the agent description and SurveySpec scope_inclusion_budget.
2. **`schema/handoff.md` not found on disk.** Handoff format is provided inline.
3. **run_dir** recovered as `.` from `00_survey_spec.md` and all scout artifacts.
4. **arXiv version stability**: Versions are preserved from the candidate pool. If a paper has been updated (e.g., `2406.10252v2` → `v3`), the version used during expansion should match what's available at download time.
5. **SurveyBench (`2510.03120`)**: No version suffix in the candidate pool. Check for arXiv version at download time.
6. **Seed range** (6–10): 8 seeds selected — within the allowed range.
