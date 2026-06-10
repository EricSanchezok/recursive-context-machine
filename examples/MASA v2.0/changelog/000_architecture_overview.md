# MASA Architecture Overview & Change Log

Date: 2026-06-06
Author: Agent (Session 1)
TODO: Append new entries at the top of the file for each new session.

## Current Architecture

### Pipeline Structure

```
Phase 0 (Paper Discovery) ───→ Core Loop (Iterative Writing) ───→ Finish (Polishing)

Phase 0 nodes:
  Anchor → QueryPlan → Discovery → Expansion → ExtendedQueryPlan → ExtendedDiscovery
    → PaperFetcher → PaperProfiler → TaxonomyBuilder → EvolutionMapper
    → [output to Core]

Core Loop nodes (per round):
  Researcher → Generator → Supervisor → JudgeContent + JudgeAccuracy + JudgeDepth
    → JudgeSynthesizer → verdict (STOP/CONTINUE)

Finish nodes:
  Polisher → Synthesizer → output
```

### Phase 0: Paper Discovery (10 nodes)

| Node | Prompt File | RCM File | Function |
|------|------------|---------|----------|
| Anchor | `prompts/phase0/anchor.txt` | `rcm/phase0/anchor.rcm` | Set research anchor |
| QueryPlan | `prompts/phase0/query_plan.txt` | `rcm/phase0/query_plan.rcm` | Initial query plan |
| Discovery | various scouts | `rcm/phase0/discovery.rcm` | Multi-scout discovery |
| Expansion | various scouts | `rcm/phase0/expansion.rcm` | Expand query pool |
| ExtendedQueryPlan | `prompts/phase0/extended_query_plan.txt` | `rcm/phase0/extended_query_plan.rcm` | Extended query plan |
| ExtendedDiscovery | various scouts | `rcm/phase0/extended_discovery.rcm` | Extended discovery |
| PaperFetcher | `prompts/phase0/paper_fetcher.txt` | `rcm/phase0/paper_fetcher.rcm` | Download PDFs for core/benchmark papers |
| PaperProfiler | `prompts/phase0/paper_profiler.txt` | `rcm/phase0/paper_profiler.rcm` | Read PDFs → extract structured profiles |
| TaxonomyBuilder | `prompts/phase0/taxonomy_builder.txt` | `rcm/phase0/taxonomy_builder.rcm` | Build classification dimensions + categories |
| EvolutionMapper | `prompts/phase0/evolution_mapper.txt` | `rcm/phase0/evolution_mapper.rcm` | Construct critical evolution narrative |

### Core Loop: Iterative Writing (7 nodes)

| Node | Prompt File | RCM File | Lines | Function |
|------|------------|---------|:-----:|----------|
| Researcher | `prompts/core/researcher.txt` | `rcm/core/researcher.rcm` | 121 | Read taxonomy+profiles+evolution → write outline + section_plan |
| Generator | `prompts/core/generator.txt` | `rcm/core/generator.rcm` | 146 | Subsection-by-subsection writing + PDF deep-read + fact verification + evidence mining |
| Supervisor | `prompts/core/supervisor.txt` | `rcm/core/supervisor.rcm` | 120 | Review → diagnose knowledge gaps → search arXiv → download PDFs → write profiles → update pool |
| JudgeContent | `prompts/core/judge_content.txt` | `rcm/core/judge_content.rcm` | - | Coverage + Citation Relevance + Section Balance |
| JudgeAccuracy | `prompts/core/judge_accuracy.txt` | `rcm/core/judge_accuracy.rcm` | - | Factual Consistency + Citation Balance + Redundancy |
| JudgeDepth | `prompts/core/judge_depth.txt` | `rcm/core/judge_depth.rcm` | - | Analysis Depth |
| JudgeSynthesizer | `prompts/core/judge_synthesizer.txt` | `rcm/core/judge_synthesizer.rcm` | 128 | Weighted score + verdict + iteration evidence audit |

### Key Data Flows

**Phase 0 → Core**:
- `phase0/paper_taxonomy.md` — classification framework
- `phase0/paper_profiles/*.md` — per-paper structured analysis
- `phase0/evolution_narrative.md` — critical evolution narrative
- `phase0/02_candidate_pool.md` — paper metadata

**Within Core (R1)**:
```
Researcher (outline + section_plan)
  → Generator (5 subsections + assemble draft)
  → Supervisor (review + focus_sections + evidence retrieval)
  → 3 Judges (score)
  → JudgeSynthesizer (verdict)
```

**Within Core (R2+)**:
```
Generator:
  ① Critical Assessment Verification (PDF fact-check claims)
  ② Iteration Evidence Mining (mine patterns from history)
  ③ Focused rewriting (2-3 focus sections)
  → Supervisor: diagnose gaps → arxiv_search → download PDF → write profile → update pool
  → 3 Judges → JudgeSynthesizer (includes WASTED_ITERATION check)
```

### Tools per Node

| Node | Tools |
|------|-------|
| Researcher | arxiv_download, arxiv_search, fs, shell |
| Generator | arxiv_download, fs |
| Supervisor | arxiv_download, arxiv_search, fs |
| PaperFetcher | arxiv_download, fs |
| PaperProfiler | arxiv_download, fs |
| TaxonomyBuilder | fs |
| EvolutionMapper | fs |

### Iteration Termination Criteria

1. MAX_ROUNDS (>=5) → STOP
2. CONVERGED (improvement < 0.15 for 2 rounds) + no WASTED_ITERATION → STOP
3. WASTED_ITERATION + no improvement → CONTINUE (must utilize available evidence)
4. CRITICAL_ISSUE (any dimension < 3.0) → CONTINUE
5. BELOW_THRESHOLD (total < 4.3) → CONTINUE
6. Quality OK (total >= 4.3 + no critical issues + no wasted iteration) → STOP

## Change History

### Changes from Previous Architecture (Original → Current)

All changes are listed from most recent to oldest.

---

#### Change 5: Prompt Cleanup — Remove Redundancy, Rebalance Agent Roles

**Files Changed**:
- `prompts/core/generator.txt` (242 → 146 lines, -40%)
- `prompts/core/supervisor.txt` (77 → 120 lines, +56%)
- `prompts/core/researcher.txt` (190 → 121 lines, -36%)
- `rcm/core/supervisor.rcm` (added arxiv_download + arxiv_search tools)

**Changes**:
1. **Generator**: Removed 5c Adaptive Retrieval (~70 lines). Generator no longer needs to search arXiv, download PDFs, write profiles, or update taxonomy. Focuses purely on writing + fact-checking + evidence mining.
2. **Supervisor**: Added "Evidence Retrieval (Rounds 2+)" section (~43 lines). Supervisor now owns the search → download → read PDF → write profile → update pool workflow. This is a natural fit since Supervisor already diagnoses knowledge gaps and writes search_keywords.
3. **Researcher**: Removed Paper Supplement Protocol (~30 lines). Paper supplementation is now fully handled by Supervisor's Evidence Retrieval in R2+, which is more rigorous (downloads PDFs, writes profiles, not just reads abstracts).

**Rationale**: Generator's Adaptive Retrieval (from Change 4) overloaded Generator with 3 distinct responsibilities (writing + evidence mining + search engine). Moving retrieval to Supervisor gives each Agent a single clear responsibility. Researcher's Paper Supplement was redundant with Supervisor's more thorough Evidence Retrieval.

---

#### Change 4: Adaptive Retrieval — Feedback-Driven Evidence Search

**Files Changed**:
- `prompts/core/supervisor.txt` (focus_sections format extended)
- `prompts/core/generator.txt` (5c Adaptive Retrieval added)
- `rcm/core/generator.rcm` (added arxiv_download tool)

**Changes**:
1. **Supervisor**: focus_sections.md output format extended from simple section IDs to include `knowledge_gap` + `search_keywords` fields. Supervisor now diagnoses not just WHICH section is shallow, but WHAT SPECIFIC knowledge is missing.
2. **Generator**: Added 5c Adaptive Retrieval step in Rounds 2+. Generator searches arXiv, downloads PDFs, writes paper profiles, and updates the paper pool based on Supervisor's search_keywords.

**Note**: Adaptive Retrieval was later moved from Generator to Supervisor in Change 5.

---

#### Change 3: Evolution Narrative — Critical Analysis in the Survey

**Files Added**:
- `prompts/phase0/evolution_mapper.txt` (new)
- `rcm/phase0/evolution_mapper.rcm` (new)

**Files Changed**:
- `rcm/masa_phase0.rcm` (inserted EvolutionMapper after TaxonomyBuilder)
- `prompts/core/researcher.txt` (added evolution_narrative reading + narrative-driven outline structure)
- `prompts/core/generator.txt` (added evolution_narrative reading)
- `prompts/core/generator.txt` (added Critical Assessment Verification + Iteration Evidence Mining)
- `prompts/core/judge_synthesizer.txt` (added Iteration Evidence Assessment + WASTED_ITERATION logic)

**Changes**:
1. **Phase 0 new node**: EvolutionMapper constructs a critical evolution narrative from paper profiles + taxonomy. Outputs `phase0/evolution_narrative.md` with phase structure, performance trends, claim-vs-evidence gap analysis, and critical assessment.
2. **Researcher**: Outline restructured from "taxonomy-driven listing" to "narrative-driven arc": evolution overview → deep dive → critical assessment (Section 5).
3. **Generator**: Added two new Rounds 2+ steps: (a) Critical Assessment Verification — fact-check each claim in Section 5 against paper profiles/PDFs; (b) Iteration Evidence Mining — extract cross-paper patterns from judge reports and logs.
4. **JudgeSynthesizer**: Added Iteration Evidence Assessment block that checks whether Generator utilized PDF deep-read findings and mined patterns. Added WASTED_ITERATION verdict override — if evidence exists but Generator didn't use it, force CONTINUE even if scores are high.

**Phase 0 pipeline**: `... → TaxonomyBuilder → EvolutionMapper → [output]`

---

#### Change 2: Paper Profile Pipeline — Phase 0 Expansion

**Files Added**:
- `prompts/phase0/paper_fetcher.txt` (new)
- `prompts/phase0/paper_profiler.txt` (new)
- `prompts/phase0/taxonomy_builder.txt` (new)
- `rcm/phase0/paper_fetcher.rcm` (new)
- `rcm/phase0/paper_profiler.rcm` (new)
- `rcm/phase0/taxonomy_builder.rcm` (new)

**Files Changed**:
- `rcm/masa_phase0.rcm` (inserted PaperFetcher → PaperProfiler → TaxonomyBuilder after ExtendedDiscovery)
- `prompts/core/researcher.txt` (added taxonomy+profiles reading + taxonomy writing + arxiv_download tool)
- `prompts/core/generator.txt` (added taxonomy+profiles reading + arxiv_download tool + PDF deep-read)
- `rcm/core/researcher.rcm` (added arxiv_download + arxiv_search tools)
- `rcm/core/generator.rcm` (added arxiv_download tool)

**Changes**:
1. **Three new Phase 0 nodes**: (a) PaperFetcher downloads PDFs for core_method + benchmark papers; (b) PaperProfiler reads each PDF and extracts structured profiles (Method Description, Architecture, Key Innovations, Benchmark Results, etc.); (c) TaxonomyBuilder analyzes all profiles and builds a classification framework with dimensions and categories.
2. **Researcher + Generator**: Both can now read `paper_taxonomy.md` and `paper_profiles/` for detailed paper understanding. Both have `arxiv_download` for on-demand PDF deep-read.
3. **Phase 0 pipeline**: `... → ExtendedDiscovery → PaperFetcher → PaperProfiler → TaxonomyBuilder → [output]`

---

#### Change 1: Core Optimization — Judge Merge + SectionPlanner Removal + Focused Iteration

**Files Added**:
- `prompts/core/judge_content.txt` (new)
- `prompts/core/judge_accuracy.txt` (new)
- `rcm/core/judge_content.rcm` (new)
- `rcm/core/judge_accuracy.rcm` (new)

**Files Changed**:
- `rcm/masa_core.rcm` (7→3 judges, removed SectionPlanner edges, arity 7→3)
- `prompts/core/researcher.txt` (added section_plan.md output, Section Planner format, Context Discipline)
- `prompts/core/generator.txt` (split Memory Access into R1/R2+, focused reading rules, Context Discipline)
- `prompts/core/supervisor.txt` (Context Discipline)
- `prompts/core/judge_synthesizer.txt` (3-judge input, same weights)

**Files Deleted**:
- `rcm/core/judge_coverage.rcm`, `judge_relevance.rcm`, `judge_factual.rcm`, `judge_redundancy.rcm`, `judge_balance.rcm`, `judge_section_balance.rcm`
- `prompts/core/section_planner.txt`
- `prompts/core/judge_coverage.txt`, `judge_relevance.txt`, `judge_factual.txt`, `judge_redundancy.txt`, `judge_balance.txt`, `judge_section_balance.txt`

**Changes**:
1. **Judge Panel**: 7 individual judges merged into 3 composite judges (Content: Coverage+Relevance+Balance; Accuracy: Factual+Citation Balance+Redundancy; Depth: Analysis Depth). Same 7 dimensions, same weights, fewer LLM calls.
2. **SectionPlanner removed**: Functionality absorbed into Researcher. Researcher now outputs both `memory/outline.md` and `memory/section_plan.md`.
3. **Generator focused reading**: Rounds 2+ only reads focus section files + first/last 30 lines of draft. No full-draft re-reads.
4. **Context Discipline**: All agents told to not explore filesystem. Exact paths are provided in prompts.

---

#### Change 0: File System Reorganization

**Changes**: Moved all rcm and prompt files from root directories into phase0/core/finish subdirectories. Updated all internal paths. Updated README.md paths.

## File Inventory

### RCM Files (15)
```
rcm/masa_phase0.rcm          — Phase 0 orchestrator (10 nodes)
rcm/masa_core.rcm            — Core loop orchestrator (7 nodes)

rcm/phase0/ (10 files)
  anchor.rcm, query_plan.rcm, discovery.rcm, expansion.rcm,
  extended_query_plan.rcm, extended_discovery.rcm,
  paper_fetcher.rcm, paper_profiler.rcm,
  taxonomy_builder.rcm, evolution_mapper.rcm

rcm/core/ (7 files)
  researcher.rcm, generator.rcm, supervisor.rcm,
  judge_content.rcm, judge_accuracy.rcm, judge_depth.rcm,
  judge_synthesizer.rcm

rcm/finish/ (N files)
  polisher.rcm, ...
```

### Prompt Files (27 total)
```
prompts/phase0/ (20 files)
prompts/core/ (7 files — researcher, generator, supervisor, judge_content,
               judge_accuracy, judge_depth, judge_synthesizer)
```
