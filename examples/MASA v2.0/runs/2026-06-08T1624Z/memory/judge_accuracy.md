## Accuracy Judge — Round 2

### Factual Consistency Score: 4

**Numerical Claim Verification (mandatory)**:

I scanned the entire survey and identified every numerical / quantitative claim. Below is the complete ledger:

| # | Location | Claim | Citation(s) | Status |
|---|----------|-------|-------------|--------|
| 1 | §1 L9 | "a single controlled comparison [arXiv:2509.18661, arXiv:2406.10252]" | ✓ cited | Accurate |
| 2 | §2.2 L23 | "4.77/10 on a custom quality scale [arXiv:2406.10252]" | ✓ cited | Accurate |
| 3 | §2.2 L25 | "4,200+ human-written surveys" (SurveyGen) | ✓ cited [2508.17647] | Accurate |
| 4 | §2.3 L31 | "8.18/10 quality score vs. AutoSurvey's 4.77/10" | ✓ cited [2509.18661] | Accurate |
| 5 | §2.3 L31 | "+32% Citation F1 improvement" (SciSage) | ✓ cited [2506.12689] | Accurate |
| 6 | §2.3 L31 | "+27.2% quality improvement" (InsightAgent) | ✓ cited [2504.14822] | Accurate |
| 7 | §2.3 L31 | "reducing timeline from months to 1.5 hours" | ✓ cited [2504.14822] | Accurate |
| 8 | §2.5 L45 | "11,343 arXiv topics and 4,947 human surveys" (SurveyBench) | ✓ cited [2510.03120] | Accurate |
| 9 | §2.5 L45 | "7 subjects" (SurveyEval) | ✓ cited [2512.02763] | Accurate |
| 10 | §2.5 L45 | "1M papers" (SurGE) | ✓ cited [2508.15658] | Accurate |
| 11 | §2.5 L45 | "1,000 human surveys across 10 disciplines" (SurveyLens) | ✓ cited [2602.11238] | Accurate |
| 12 | §3.2 L93 | **"71% improvement"** | **Computed from cited scores, not directly attributed** | Minor concern |
| 13 | §3.4 L137 | **"71%"** | **Same derived number** | Minor concern |
| 14 | §4.0 L151 | **"5 of the 35 core method papers surveyed"** | **NO citation — author's own count** | Unresolved |
| 15 | §5.3 table | Agent counts (4, 5, 6 agents) and metric numbers | ✓ cited in each row | Accurate |

**Three specific findings:**

1. **"71% improvement" (L93, L137)**: This number is derived from (8.18−4.77)/4.77 ≈ 71.5%, rounded to 71%. The rounding is mathematically sound. However, the draft presents this as a factual finding without noting it is the *author's calculation* from two reported scores — no paper in the literature states "71% improvement" explicitly. This is a minor attribution issue, not a hallucination.

2. **"5 of the 35 core method papers surveyed" (L151)**: This is the author's own tally, not cited to any source. The candidate pool lists 46 `core_method` entries; an independent count of unique ASG method papers actually discussed in the draft yields ~23–29. The number "35" is opaque — what qualifies as a "core method paper" for this count? This was flagged in Round 1 and remains unaddressed.

3. **Uncited "5 of 35" reappears as "35 core method papers" (L242)**: Line 242 states "Only 5 of the 35 core method papers surveyed use citation graph structure" — same uncited authorial count.

**No fabricated claims or invented findings detected.** Every paper citation points to a real arXiv ID in the candidate pool. No claims attribute results to papers that don't exist.

### Citation Balance Score: 3

**Strengths:**
- The 15 core ASG method papers (§2 and §3) are each cited proportionally to their role in the narrative — AutoSurvey and Agentic AutoSurvey justifiably receive the most citations as the central comparison pair.
- Pre-LLM foundations (Phase 0) are cited once each — appropriate for foundational pointers.
- Blind-spot tools (FActScore, VERISCORE, CiteGuard, CiteME) are cited in §5.4 where they belong.
- The GNN papers in §4.3 (Temporal GNN, H2CGL, Context-Aware Citation Rec) are each cited.

**Issues:**

1. **Three supervisor-retrieved papers completely absent from the draft.** The candidate pool was updated with three papers specifically retrieved to address blind spots in Round 2:
   - **arXiv:2605.07723** (citation hallucination prevalence: ~147K hallucinated citations in 2025) — directly relevant to §5.4 blind spot #1. Not cited.
   - **arXiv:2604.22750** (token consumption: 1000× cost multiplier for agentic tasks) — directly relevant to §5.4 blind spot #4 (cost opacity). Not cited.
   - **arXiv:2605.14790** (Graphs of Research: citation evolution DAG for generation) — directly relevant to §4.3 (missed opportunity) and §6.1 (first-class graph integration). Not cited.
   
   Their absence represents a **significant imbalance** — the draft's own blind-spot analysis lacks the very evidence that was retrieved to support it.

2. **Some outline-specified reference papers omitted from the draft:**
   - arXiv:1904.07579 (Go Wide, Go Deep) — specified in outline §4 reference papers, not cited in draft.
   - arXiv:2402.08339 (Interleaved snowballing) — specified in outline §4, not cited.
   - arXiv:2104.02562 (Structured Citation Trend Prediction) — specified in outline §4, not cited.

3. **Phase 2 papers (KMCA, AutoSurvey2)** could use more specific citations in the comparison tables.

### Redundancy Score: 3

**Specific findings:**

1. **The "single controlled comparison" message appears 6 times** with near-identical phrasing:
   - §1 L9: "rests on a single controlled comparison"
   - §2.3 L31: "the field's only controlled comparison"
   - §3 intro L51: "rests on a single controlled comparison"
   - §3.4 L128: "rests on a single data point"
   - §5.1 table L221: "The only controlled comparison in the literature"
   - §7 L309: "rests on a single controlled comparison"

2. **The "Phase 4 irony" is repeated verbatim.**
   - §2.5 L47 vs. §5.3 L273: Nearly identical sentence structures.

3. **Section 3.4 and Section 5.3 overlap substantially.** Both present system-benchmark tables and discuss the inability to compare systems.

4. **Section 4.3 (missed opportunity) and Section 6.1 (future direction) overlap** on GNN method enumeration (Temporal GNN, LitFM references appear in both).

5. **"+32% Citation F1" appears 4 times** — each in a different context, but could be cross-referenced.

### Accuracy Score: 3.3

Computed as: (4 × 0.429) + (3 × 0.286) + (3 × 0.286) = 1.716 + 0.858 + 0.858 = 3.432 → rounded to **3.3**

### Potential Hallucinations / Balance Issues / Redundant Content

- **"71% improvement" (L93, L137)**: Not a hallucination — calculation is correct — but presented without noting it is the author's derivation. Minor fix: "a ~71% improvement (computed from the two reported scores)".
- **"5 of the 35 core method papers" (L151, L242)**: Unclear what qualifies as a "core method paper." Candidate pool lists 46 core_method entries. Could be read as a factual claim without supporting methodology.
- **3 supervisor-retrieved papers missing from draft**: arXiv:2605.07723 (citation hallucination evidence) would directly strengthen §5.4 blind spot #1; arXiv:2604.22750 (token cost analysis) would strengthen §5.4 blind spot #4; arXiv:2605.14790 (Graphs of Research) would strengthen §4.3.
- **Verbatim Phase 4 irony** in §2.5 and §5.3: same rhetorical structure, same sentiment. One should be eliminated or turned into a cross-reference.
- **Section 3.4 and 5.3 overlap**: Both contain system-benchmark tables and discuss the evaluation crisis.

### Suggestions

1. **Incorporate the 3 supervisor-retrieved papers** (highest priority for next round).
2. **Reduce redundancy**: Trim "single controlled comparison" from 6 occurrences to 3.
3. **Source or adjust "35 core method papers"**: Either add a methodology note or replace with "among the ASG systems surveyed in this work."
4. **Attribute "71% improvement" as a derived number**: Change to "a ~71% improvement over AutoSurvey's 4.77/10 baseline (computed from the two reported scores)."
5. **Add missing outline-specified references**: 1904.07579, 2402.08339, 2104.02562.
