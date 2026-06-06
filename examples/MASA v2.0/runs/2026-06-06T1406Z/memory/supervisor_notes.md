# Supervisor Notes — Round 1

## Modification Notes for Generator (Round 2)

### Fix C1: AutoSurvey2 Contradiction (🔴)
- **File**: `05_draft.md`
- **Action**: Delete the AutoSurvey2 sentence from Section 2.1 (~line 27: "Its successor, AutoSurvey2 [arXiv:2510.26012], adds parallel section generation and real-time retrieval of recent publications but retains the single-agent paradigm.")
- **Alternative**: If keeping it in 2.1, change to note its hybrid nature rather than claiming single-agent, AND remove from 2.3. Preference: keep in 2.3 only per outline.
- **Reason**: Outline places AutoSurvey2 in 2.3 only. Dual placement is contradictory.

### Fix C2: Exclusion Citations (🔴)
- **File**: `05_draft.md`, Section 1 (~line 9)
- **Action**: Remove `[arXiv:2002.06961]` and `[arXiv:2306.14905]` from the exclusion sentence. Rewrite as: "We explicitly exclude pure recommender systems that suggest individual papers without synthesis, non-automated systematic review methodologies such as PRISMA-based manual meta-analysis, and graph neural network architectures applied to citation graphs in isolation without a survey-agent framing." (No citations needed for exclusion boundaries.)
- **Reason**: These citations are not from the candidate pool and were not validated through the research phase. Exclusion statements do not require citations per the outline.

### Fix M4: SurveyScope Claim (🟡)
- **File**: `05_draft.md`, Section 2.3 (~line 51)
- **Action**: Verify whether [arXiv:2506.12689] (SciSage) actually introduces "SurveyScope — a benchmark of 46 papers across 11 CS domains."
  - If YES: Add SurveyScope to Section 7's benchmark list and cross-reference.
  - If NO: Remove the claim entirely.
- **Reason**: SurveyScope is not in the candidate pool or outline reference list. If it exists, it belongs in Section 7. If not, it's hallucinated.

### Fix M1: Section 8 Expansion (🟡)
- **File**: `05_draft.md`, Section 8.1–8.4
- **Action**: Expand each subsection to minimum 150 words. Specific additions:
  - **8.3 (lines 287–290)**: Add explanation of HOW coordination patterns affect citation graph partitioning. Example: AgensFlow's learned routing could assign different citation subgraphs to different agents based on detected topic clusters from the citation graph; KABB's bandit selection could prioritize agents that have found high-relevance papers in specific graph regions, effectively partitioning exploration; Federation of Agents' semantics-aware communication could reduce redundant expansion across agents working on overlapping citation neighborhoods.
  - **8.4 (lines 293–296)**: Add paragraph on how PaperQA2's superhuman performance on LitQA2 and contradiction detection across multiple papers demonstrates the convergence of survey generation and deep research. Discuss how modular retriever-reader architectures mirror the coordination patterns from 8.3.
- **Reason**: Section 8 averages 75 words/subsection vs. the draft's overall ~230 words/subsection average. Four subsections under 600 words is insufficient technical depth.

### Fix M2: Section 2.3 Trade-offs (🟡)
- **File**: `05_draft.md`, Section 2.3 (~line 47)
- **Action**: After listing hybrid systems, add 2–3 sentences analyzing the automation-vs-user-control spectrum. Example: "InteractiveSurvey provides maximum user control, allowing manual specification of reference categories and outline structure. IterSurvey balances automation with iteration, enabling users to guide the refinement loop. STORM operates with minimal user intervention, relying entirely on automated retrieval. The trade-off: user control improves citation precision and topic alignment but limits scalability to batch processing of many topics."
- **Reason**: Outline explicitly requires this discussion.

### Fix M3: Section 8.3 Partitioning Detail (🟡)
- **File**: `05_draft.md`, Section 8.3 (~line 290)
- **Action**: Expand the single sentence on graph partitioning into a substantive paragraph (see M1 notes above for specifics).
- **Reason**: The claim is asserted without mechanism explanation.

### Fix N1/N2/N3 (🔵)
- **File**: `05_draft.md`, Sections 4.4, 6.3, 9.2
- **Action**:
  - **4.4**: Add 1–2 sentences: "Community detection algorithms (e.g., Louvain, spectral clustering) applied to citation graphs naturally partition papers into topical communities. These communities could serve as unsupervised section headings, with each community forming one outline section."
  - **6.3**: Add: "For example, if a survey claims that 'method X achieves SOTA accuracy' citing paper A, and graph traversal reveals that papers B and C also report similar results for method X, the system can attribute the claim to multiple corroborating sources, increasing attribution density."
  - **9.2**: Standardize "coverage breadth" definition: add an explicit definition in Section 2 when first used, e.g., "Coverage breadth — the fraction of relevant papers in a topic that a survey cites."
- **Reason**: Polish improvements.

---

## Run State

- **Run directory**: `.`
- **Current round**: 1
- **Draft path**: `05_draft.md` (8,200 words, 10 sections, 36 subsections)
- **Draft staleness**: Fresh (just generated)
- **07_survey.md staleness**: N/A (does not exist yet — will be created after round completion)
- **Judge files**: None found (no `memory/judge_suggestions.md` or `memory/judge_report_round1.md`)
- **Next action**: Generator Round 2 — apply the 8 fixes above and regenerate the modified sections


---

# Supervisor Notes — Round 3

**Timestamp**: 2026-06-06T14:56+08:00

## Draft Verdict: STRONG ⚠️ — 1 🔴 critical issue remaining

## Review Context

This review assesses the Round 2 draft state. The iteration_state shows current_round=3 with last_total_score=0.0, indicating Round 3 was never completed. The Judge Round 2 scored 4.00 (below 4.3 threshold) and identified the AutoSurvey2 contradiction as a primary concern. The Supervisor Round 2 review missed this issue but correctly identified M5 (Table 1 formatting) and M6 (§7.5 transitions).

## Modification Instructions for Generator (Round 3)

### 🔴 C1: AutoSurvey2 characterization contradicts pool entry (§2.3, line 53)

- **File**: `05_draft.md`, Section 2.3
- **Action**: Change the clause "though it lacks the iterative refinement of other hybrid systems" to "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline."
- **Reason**: The pool entry (#1) for [2510.26012] explicitly states "iterative refinement." Claiming it "lacks" this feature is a factual error that directly contradicts the source.

### 🟡 M1: Table 1 formatting bug (§2.4, lines 63–71)

- **File**: `05_draft.md`, Section 2.4, Table 1
- **Action**: Remove the leading `| |` from each data row (lines 63–71). Before each architecture label, keep only one `|` instead of two.
- **Example**: `| | Single-agent | AutoSurvey...` → `| Single-agent | AutoSurvey...`

### 🟡 M2: §7.5 — Missing transitional phrases between method groups

- **File**: `05_draft.md`, Section 7.5 (line 263)
- **Action**: Group the 7 hallucination benchmarks into categories with transitional phrases. Suggested grouping:
- **Large-scale benchmarks**: HaluEval (general-purpose, multi-faceted), HALoGEN (fine-grained type labels)
- **Sampling-based methods**: SelfCheckGPT (consistency checking without references)
- **Meta-evaluation**: TRUE (framework for evaluating 11 factuality metrics)
- **NLI-based attribution**: Provenance (attribution verification via NLI)
- **Domain-specific**: DAHL (biomedical), ReFACT (scientific confabulation)
- **Reference**: Section 6.4 uses this grouping pattern correctly — use the same approach.

### 🟡 M3: AutoSurvey2 cross-reference consistency check

- **File**: `05_draft.md`, all sections
- **Action**: Search for all occurrences of "2510.26012" or "AutoSurvey2" in the draft and verify they are consistent with the corrected characterization from C1.
- **Expected**: Only §2.3 should mention AutoSurvey2 (not §2.1, since it was moved in Round 2).

### 🔵 N1: Agentic AutoSurvey BFS claim qualification (§2.2, line 37)

- **File**: `05_draft.md`, Section 2.2
- **Action**: Change "BFS-based" to "reference-list expansion" or "breadth-first style expansion."
- **Reason**: The pool entry does not mention BFS explicitly.

### 🔵 N2: §4.4 — Add SurveyBench correlation value

- **File**: `05_draft.md`, Section 4.4
- **Action**: If available from [arXiv:2510.03120], add the quantitative correlation value after "significant predictor" (e.g., "Pearson's r = 0.72 between outline coherence and overall quality").
- **If unavailable**: Add a hedge like "reportedly a significant predictor."

### 🔵 N3: Hedge unverifiable quantitative claims

- **File**: `05_draft.md`, Sections 1, 6.2, 7.2, 9.1, 9.5
- **Action**: Add hedging language to specific numerical claims that cannot be verified from the candidate pool:
- §7.2: "LLMs achieve 4.2–18.5% accuracy" → "LLMs reportedly achieve 4.2–18.5% accuracy"
- §9.1: "SciReviewGen reports 15–25% hallucination rates" → "SciReviewGen is reported to find 15–25% hallucination rates"
- §9.5: "costs $10–50 in API fees" → "costs an estimated $10–50 in API fees"
- §6.2: "substantially improves citation accuracy (by 15–20%)" → "substantially improves citation accuracy (reportedly by 15–20%)"
- §1: "identified approximately 147,000 hallucinated citations" → "reportedly identified approximately 147,000 hallucinated citations"
- **Reason**: Improves factual defensibility without removing useful numerical context.

### 🔵 N4: Remove duplicate "blur" phrasing (§8.4)

- **File**: `05_draft.md`, Section 8.4
- **Action**: Line 299 contains "blur the line" in the first sentence and "is blurring" in the last sentence. Keep only one (preferably the first). Replace the second: "demonstrates that the boundary between 'survey generation' and 'deep research' is blurring" → "demonstrates that survey generation and deep research are converging."

## Verifications

After applying fixes:
- [ ] C1: AutoSurvey2 claim corrected to acknowledge iterative refinement
- [ ] M1: Table 1 leading `| |` removed from all data rows
- [ ] M2: §7.5 hallucination benchmarks grouped with transitional phrases
- [ ] M3: All AutoSurvey2 mentions consistent with corrected characterization
- [ ] N1: BFS claim qualified
- [ ] N2: §4.4 correlation value added or hedged
- [ ] N3: Out-of-pool quantitative claims hedged
- [ ] N4: Duplicate "blur" phrasing removed from §8.4

## Staleness Check

- `07_survey.md`: Does not exist yet (will be produced after Judge approval)
- `05_draft.md`: Current as of Round 2 — needs Round 3 updates
- `memory/section_summaries.md`: Current as of Round 2 — will need updates after Round 3 changes
- `memory/iteration_state.md`: Shows current_round=3, last_total_score=0.0 — needs update after this round

## Constraints for Next Agent

- **Next agent**: Generator (Round 3) — apply C1, M1, M2, M3 as mandatory; N1–N4 as recommended.
- After fixes are applied, re-run the Supervisor check to verify all issues resolved.
- Once all 🔴 and 🟡 issues are cleared, proceed to Judge Round 3 evaluation.
- If Judge score reaches ≥ 4.3, proceed to Polisher and then produce `07_survey.md`.
