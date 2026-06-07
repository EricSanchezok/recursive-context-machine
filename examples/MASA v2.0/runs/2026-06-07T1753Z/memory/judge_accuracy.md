## Accuracy Judge — Round 2

### Factual Consistency Score: 3

**Numerical Claim Verification (mandatory)**:

Scan of the entire survey (689 lines) identified the following numerical/quantitative claims:

| # | Claim | Location | Cited? | Citation Format | Status |
|---|-------|----------|--------|----------------|--------|
| 1 | "over 2 million papers published annually" | §1, line 5 | Yes | [STM Global Brief 2023; UNESCO Science Report 2021] | **UNSOURCED** — non-arXiv sources cited without page/section; number cannot be verified |
| 2 | "135+ papers across six architectural paradigms" | §1, line 13 | Self-report | — | Acceptable — survey's own taxonomy scope |
| 3 | "280M+ papers, authors, and venues" (Semantic Scholar) | §2.1, line 25 | Yes | [arXiv:1805.02262] | Paper-level — original 2018 paper may state a smaller number; number appears to describe current infrastructure scale rather than the 2018 paper's claim |
| 4 | "+28% MAP improvement" (Context-Aware Citation Rec) | §2.1, line 33 | Yes | [arXiv:1903.06464] | OK |
| 5 | "+28.1% precision" (LitFM) | §2.1, line 37 | Yes | [arXiv:2409.12177] | OK |
| 6 | "SOTA on the LitQA benchmark" (PaperQA) | §2.2, line 65 | Yes | [arXiv:2312.07559] | OK |
| 7 | "superhuman performance on LitQA2" (PaperQA2) | §2.2, line 67 | Yes | [arXiv:2409.13740] | OK — survey later critiques this claim |
| 8 | "8B beats GPT-4o by 5%" (OpenScholar) | §2.2, line 69 | Yes | [arXiv:2411.14199] | OK |
| 9 | "45M paper datastore" (OpenScholar) | §2.2, line 69 | Yes | [arXiv:2411.14199] | OK |
| 10 | "+32% citation F1" (SciSage) | §2.3, line 93 | Yes | [arXiv:2506.12689] | OK |
| 11 | "4.77/10 to 8.18/10" (Agentic AutoSurvey) | §2.3, line 95 | Yes | [arXiv:2509.18661] | OK |
| 12 | "+27.2% quality improvement" (InsightAgent) | §2.3, line 99 | Yes | [arXiv:2504.14822] | OK |
| 13 | "+37.78% recall@20" (PaSa) | §2.4, line 122 | Yes | [arXiv:2501.10120] | OK |
| 14 | "80–95% effort reduction" (ASReview) | §4.1, line 322 | Yes | [arXiv:2006.12166] | OK |
| 15 | "53h → 3h" (FAST²) | §4.1, line 333 | Yes | [1705.05420] | OK |
| 16 | "62% of citations ... last five years" | §5.4, line 526 | Yes | [arXiv:2305.18554, arXiv:2402.12046] | OK — two supporting citations |
| 17 | "65.4% on CiteME (vs. human 69.7%)" | §5.2, line 467; §5.4, line 518 | Yes | [arXiv:2510.17853] | OK |
| 18 | "11 distinct benchmarks" | §5.3, line 451 | Self-enumeration | — | Acceptable — survey's own classification |
| 19 | "1,000 survey topics across 10 disciplines" (SurveyLens) | §6.4, line 665 | Yes | [arXiv:2602.11238] | OK |
| 20 | "4,200+ human-written surveys" (SurveyGen) | §4.2, line 349 | Yes | [arXiv:2508.17647] | OK |

**Unsourced Numerical Claims Tally**: **1** (claim #1: "over 2 million papers" — STM Global Brief and UNESCO Science Report are not arXiv papers, no page/section provided).

**Additional factual issues found**:

1. **SummEval claim without citation** (§1, line 11): The survey states: "Multi-dimensional summarization evaluation, exemplified by SummEval's four-dimension rubric (coherence, consistency, fluency, relevance), provides validated protocols for assessing generation quality that survey systems could adapt." This describes SummEval's specific rubric dimensions without any citation. The candidate pool contains SummEval [arXiv:2007.12626] but it is not cited in this passage. A factual claim about a specific framework's evaluation dimensions requires supporting evidence.

2. **Unsupported analytical claim** (§2.3, line 95): "Agentic AutoSurvey uses a more capable base LLM, better retrieval, and a different evaluation rubric than the original AutoSurvey" — this is a critical claim about confounding variables. It lacks a supporting citation for the assertion that the base LLM or evaluation method differs. While analytical claims are legitimate in a critical survey, this reads as a factual statement rather than marked analysis. The same issue applies to "the comparison is confounded" (same line).

3. **Imprecise comparison framing** (§4.1, line 320): "InsightAgent achieves a 27.2% quality improvement over manual systematic reviews" — the original paper [2504.14822] compares against unassisted human reviews. The word "systematic" imports PRISMA-style methodological rigor that may not accurately describe the baseline condition. Minor overstatement.

4. **Baseline absolute values missing for some claims** (noted by outline refinement guideline #5 for §3.4): The bottleneck transfer table (§3.4, lines 289-296) includes improvement metrics but the "Baseline Absolute Value" column for LitFM says "reported in [2409.12177]" and for SciSage says "reported in [2506.12689]" without restating the numbers in the table. This makes the table less self-contained than intended.

### Citation Balance Score: 3

**Distribution analysis**:

The draft covers 60+ cited papers. The most frequently cited papers are:

| Paper | Mentions | Role |
|-------|----------|------|
| LitFM [2409.12177] | 7 | Phase 1/transitional (retrieval) |
| SciSage [2506.12689] | 6 | Phase 3 (multi-agent) |
| PaSa [2501.10120] | 7 | Phase 4 (RL) |
| SurveyG [2510.07733] | 5 | Phase 5 (graph re-integration) |
| Agentic AutoSurvey [2509.18661] | 5 | Phase 3 (multi-agent) |
| OpenScholar [2411.14199] | 4 | Phase 2 (scale) |
| InsightAgent [2504.14822] | 4 | Cross-cutting (HITL) |
| AutoSurvey [2406.10252] | 4 | Phase 2 (canonical pipeline) |
| DeepSurvey-Bench [2601.15307] | 4 | Benchmark/evaluation |
| SurGE [2508.15658] | 4 | Evaluation framework |
| CiteGuard [2510.17853] | 4 | Citation validation |
| PaperQA2 [2409.13740] | 3 | Phase 2 (contradiction detection) |
| SPECTER [2004.07180] | 3 | Phase 1 (foundational embedding) |
| IterSurvey [2510.21900] | 3 | Phase 4 (procedural) |
| SurveyLens [2602.11238] | 3 | Evaluation framework |

**Identified imbalances**:

1. **Missing DeepSurvey [arXiv:2605.29522] from §2.5** — CRITICAL. The supervisor handoff explicitly retrieved this May 2026 paper to fill the §2.5 gap. DeepSurvey combines citation-graph expansion with multi-granularity agentic refinement, achieving 8.644/10 content score and 83.3% expert preference over human-written surveys. Its absence means §2.5 covers only 4 papers (SurveyG, Graphs of Research, Science Hierarchography, LitFM) and misses the strongest quantitative evidence for deep graph-LLM integration in the current frontier.

2. **Missing arXiv:2512.20854 from §3.4** — MODERATE. The supervisor handoff retrieved this paper to fill the §3.4 empirical gap. It directly measures the correlation between retrieval quality metrics and LLM-judged response quality, supporting the bottleneck transfer argument with empirical evidence rather than theoretical reasoning alone. Its absence leaves §3.4's core argument as purely analytical.

3. **Missing SummEval citation [arXiv:2007.12626] from §1** — MODERATE. The survey describes SummEval's rubric but provides no citation. The candidate pool contains this canonical paper.

4. **Foundation papers under-represented relative to pool**: The taxonomy assigns ~35 papers to Phase 1 (Category 1 in taxonomy), but the draft names only ~5 in detail (Semantic Scholar Graph, SPECTER, Context-Aware Citation Rec, LitFM, HiGTL). Many taxonomy papers (e.g., 1205.1143 direction-aware citation analysis, 1511.05078 taxonomy generation comparison, 1905.00075 arXiv citation graph, 2106.05633 knowledge graph recommendation, 2403.09295 seed-based IR comparison) are in the pool but not cited. While not every pool paper must appear, this 35→5 ratio means Phase 1 is underrepresented relative to its taxonomy scope.

5. **Recent paper dominance is appropriate but risks temporal blind spot**: ~70% of citations are 2024–2026. For a fast-moving field this is defensible, but foundational citation analysis methods (co-citation analysis, bibliographic coupling from the 1970s–1990s) are entirely absent, reinforcing the recency-bias blind spot the survey itself identifies in §5.4.

### Redundancy Score: 2

**Severe redundancy and duplication identified**:

1. **Nearly identical comparison tables in §2 and §3**:
   - **§2.2 table (lines 73-79)** and **§3.2 table (lines 240-246)**: Both present the same systems (AutoSurvey, STORM, PaperQA/PaperQA2, OpenScholar) with the same columns (Pipeline Stages, Retrieval, Graph Awareness, Iteration, Claimed Metric, Benchmark, Scale). The §3.2 version adds SurveyX and drops PaperQA. ~80% content overlap.
   - **§2.3 table (lines 103-108)** and **§3.3 table (lines 270-275)**: Both present SciSage, Agentic AutoSurvey, MATC, InsightAgent with the same metrics and similar column structure. Column ordering differs slightly; data is nearly identical. ~85% content overlap.

2. **Core metrics restated 5–7 times each** (systematic violation of the outline's consolidation rule):
   - SciSage's +32% citation F1: **6 appearances** — §2.3 (line 93), §2.5 cross-phase table (line 180), §3.3 (line 258), §3.4 (line 283), §5.1 (line 436), §6.1 (line 589)
   - LitFM's +28.1% precision: **7 appearances** — §2.1 (line 37), §2.5 (line 163), §2.5 cross-phase table (line 180), §3.1 (line 199), §3.4 (line 283), §5.1 (line 437), §6.1 (line 585)
   - PaSa's +37.78% recall@20: **7 appearances** — §2.4 (line 122), §2.5 cross-phase table (line 180), §3.4 (line 283), §4.3 (line 369), §5.1 (line 438), §6.1 (line 587), §6.2 (line 603)
   - Agentic AutoSurvey 8.18/10 vs 4.77/10: **5 appearances** — §2.3 (line 95), §2.5 cross-phase table (line 180), §3.3 (line 262), §3.4 (line 294), §5.1 (line 436)
   - InsightAgent +27.2%: **4 appearances** — §2.3 (line 99), §4.1 (line 320), §4.4 (line 400), §4.4 (line 409)
   - OpenScholar "8B beats GPT-4o by 5%": **4 appearances** — §2.2 (line 69), §3.2 (line 234), §5.1 (line 435), §2.5 cross-phase table (line 180)

   The outline (§2 refinement guideline #4) explicitly instructs: *"First mention of each core metric... goes here in §2.3–2.5. Later sections (3, 4, 5) should cross-reference these values rather than re-stating them in full. Exception: §3.4's bottleneck transfer table may synthesize metrics from multiple sources."* This instruction was systematically not followed — every later section restates metrics in full.

3. **Nearly verbatim sentences**:
   - **§2.2 line 59 vs §3.2 line 226**: "The key architectural insight is the structured outline as a planning skeleton that guides section-level retrieval" / "The key architectural insight is that the outline serves as a planning skeleton that constrains retrieval to section-relevant papers" — same concept, trivially reworded.
   - **§2.3 line 114 vs §3.3 line 279**: "multi-agent coordination amplifies rather than solves retrieval gaps: if the Researcher agent returns an incomplete or biased set of papers, the Writer works on an impoverished knowledge base and the Reviewer can detect but not fix the problem" / "multi-agent coordination amplifies rather than solves retrieval gaps: if the Researcher agent returns an incomplete set of papers, the Writer drafts from an impoverished knowledge base, the Reviewer detects incompleteness but cannot fix it without a more complete retrieval, and the Refiner makes the same error loop" — the same argument, with §3.3's being a slightly extended version.

4. **§5.1 claim-evidence table (lines 431-439) fully restates all metrics**: Every one of the 7 claims has its metric value restated in full (e.g., "+32% citation F1 on SurveyScope" for SciSage), even though these exact values were introduced in §2.3–2.5 and appear again in §3.4. The table could use cross-references like "§2.3" instead of 30+ lines of redundant content.

5. **Cross-phase table (§2.5, lines 174-183) and bottleneck transfer table (§3.4, lines 289-296) both reproduce the same metric inventory**: The cross-phase table lists every system's metric; the bottleneck transfer table lists most of them again. Combined, these two tables account for a significant portion of the metric restatement problem.

### Accuracy Score: 2.7

(3 + 3 + 2) / 3 = 8/3 = 2.67 → rounded to **2.7**

### Evidence

- **Factual Consistency**: No fabricated papers or invented claims detected — the draft's critical self-awareness is a strength. However, one numerical claim ("over 2 million papers") has an insufficiently specific citation (non-arXiv sources without page/section). SummEval's four-dimension rubric is described without any citation (§1, line 11). The analytical claim about Agentic AutoSurvey using "a more capable base LLM" is presented as fact without supporting evidence. The word "systematic" in InsightAgent's comparison condition may slightly overstate the original paper's claim. These are minor but cumulative issues that lower the score from the Round 1 baseline.

- **Unsourced Numerical Claims**: 
  1. **"over 2 million papers published annually across disciplines"** (§1, line 5) — Cited to [STM Global Brief 2023; UNESCO Science Report 2021], which are non-arXiv sources with no section/page specificity provided. The 2M figure is widely reported in STM industry reports, but the citation is not verifiable at the granularity required for academic rigor.
  2. **SummEval's "four-dimension rubric (coherence, consistency, fluency, relevance)"** (§1, line 11) — A qualitative factual claim (describing the specific dimensions of a known evaluation framework) made without any citation. SummEval [arXiv:2007.12626] exists in the candidate pool but is not cited here.

- **Citation Balance**: Good distribution across core systems — the most-cited papers are the architecturally central ones (LitFM, SciSage, PaSa, SurveyG). However, three citation gaps are significant: (1) DeepSurvey [2605.29522] — explicitly retrieved for §2.5 by the supervisor — is absent despite providing the strongest quantitative evidence for deep graph-LLM integration; (2) arXiv:2512.20854 — retrieved for §3.4 — is absent, leaving the bottleneck transfer argument without direct empirical support; (3) SummEval [2007.12626] is described but not cited in §1. Additionally, ~30 Phase 1 taxonomy papers from the pool are never mentioned, though not all must be.

- **Redundancy**: The most severe dimension. Found 2 pairs of nearly identical comparison tables (§2.2 ≈ §3.2; §2.3 ≈ §3.3), 2 pairs of nearly verbatim sentences (planning skeleton, amplification of gaps), and 6 key metrics each appearing 4–7 times across the draft. The outline explicitly instructed metric cross-referencing after §2, but every later section restates metrics in full rather than using parenthetical cross-references. The §5.1 claim-evidence table reproduces 30+ lines of metric data already present in §2 and §3.4. This makes the draft feel substantially padded.

### Potential Hallucinations / Balance Issues / Redundant Content

1. **SummEval rubric uncited** (§1, line 11): Describes SummEval's four evaluation dimensions without any citation. This is a factual assertion about a specific framework that requires support.

2. **DeepSurvey [2605.29522] absent from §2.5**: The supervisor handoff explicitly retrieved this paper for this section. It provides 8.644/10 content score and 83.3% expert preference over human-written surveys — the strongest quantitative evidence for deep graph-LLM integration. Its absence weakens §2.5's empirical grounding.

3. **arXiv:2512.20854 absent from §3.4**: This paper directly measures the correlation between retrieval quality metrics and LLM-judged response quality. The current §3.4 argues purely theoretically that retrieval gains may not transfer to survey quality; this paper could provide the empirical evidence the argument needs.

4. **§2.2 table ≈ §3.2 table**: Same 5 systems (~80% overlap). Merge into one or remove one with a cross-reference.

5. **§2.3 table ≈ §3.3 table**: Same 4 systems (~85% overlap). Merge into one or remove one with a cross-reference.

6. **Metrics restated 5–7 times**: +32%, +28.1%, +37.78%, 8.18/10, +27.2% each appear in §2 (origin), §2.5 cross-phase table, §3.3 or §3.4, §5.1, and §6.1 — 4–7 locations each. Replace all re-statements in §3, §4, §5, and §6 with cross-references like "(see §2.3 table)".

7. **"Amplification of gaps" paragraph nearly verbatim in §2.3 (line 114) and §3.3 (line 279)**: Keep §3.3's more detailed version; trim §2.3's version to a brief mention with a forward reference to §3.3.

8. **"Planning skeleton" sentence nearly verbatim in §2.2 (line 59) and §3.2 (line 226)**: Same concept trivially reworded. Keep the first occurrence; trim the second.

### Suggestions

1. **Integrate DeepSurvey [2605.29522] into §2.5**: Add a paragraph after the Science Hierarchography discussion (after line 155). DeepSurvey's agentic citation-graph expansion with multi-granularity refinement achieves 8.644/10 content score (83.3% expert preference over human-written). This directly addresses the "open problems" analysis by showing a first attempt at deeper integration.

2. **Integrate arXiv:2512.20854 into §3.4**: Add a paragraph after line 310 noting that this paper empirically tests the correlation between retrieval quality metrics and LLM-judged response quality, finding the relationship is metric-dependent — directly supporting the bottleneck transfer argument with empirical evidence.

3. **Add SummEval citation [arXiv:2007.12626]** to §1, line 11, after the description of its four-dimension rubric.

4. **Eliminate redundant tables**: 
   - Remove the §2.2 comparison table (lines 73-79); keep the §3.2 version (lines 240-246) which adds SurveyX.
   - Remove the §2.3 comparison table (lines 103-108); keep the §3.3 version (lines 270-275).
   - Add cross-reference sentences: "See Table X in §3.2 for a full architectural comparison" and "See Table X in §3.3 for a full multi-agent comparison."

5. **Cross-reference metrics instead of restating**: Throughout §3, §4, §5, and §6, replace full metric re-statements with parenthetical cross-references:
   - "SciSage achieves +32% citation F1 on SurveyScope (§2.3)" — already does this, keep as-is.
   - In §5.1's claim-evidence table, replace the "Supporting Evidence" column's re-statement of each metric with referencing the §2 subsection where it's fully described. E.g., change "SciSage: +32% citation F1 on SurveyScope; Agentic AutoSurvey: 8.18/10 vs 4.77/10 on custom rubric" to "See §2.3 for primary metric values and evaluation context."

6. **Trim duplicate "amplification of gaps" paragraph**: Keep §3.3's more detailed version (line 279). Replace §2.3's version (line 114) with a single sentence: "This amplification effect is analyzed in detail in §3.3."

7. **Trim duplicate "planning skeleton" sentence**: Keep §2.2's version (line 59). Replace §3.2's version (line 226) with a brief reference: "As described in §2.2, the outline serves as a planning skeleton."

8. **Mark analytical claims clearly**: In §2.3 (line 95), prefix the confound analysis with "Analysis: " or "Our assessment: " to distinguish the survey author's critical evaluation from cited facts.
