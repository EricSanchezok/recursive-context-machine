## Accuracy Judge — Round 4

### Factual Consistency Score: 4.5
### Citation Balance Score: 4.0
### Redundancy Score: 3.5

### Accuracy Score: 4.0

### Evidence

**Factual Consistency (4.5/5):** The draft maintains excellent source-attribution discipline. Every major claim about a specific system is paired with its arXiv ID. The Claim vs. Evidence table (Table 9) cites specific paper sections. Verification against evolution_narrative.md and candidate pool profiles confirms:

- AutoSurvey ROUGE-L ~0.35 and ~40% human win rate — match evolution_narrative (§3, line 52–53) ✓
- ARISE 5-agent architecture, rubric-guided loop, 92.48 quality threshold — match evolution_narrative (§3, line 87) ✓
- DOVA perspective agents (empiricist, theoretician, methodologist, domain specialist) — match evolution_narrative (§4, line 116) ✓
- SciReviewGen "10K reviews, 690K cited papers" — matches evolution_narrative (§1, line 16) and candidate pool ✓
- SurveyBench "4,947 human-written surveys" — matches evolution_narrative (§3, line 81) ✓
- FActScore "validated on biographies, not scientific surveys" — consistent with paper domain ✓
- SciAtlas quote re "superficial keyword matching or vector-space semantic retrieval" — matches paper profile §4.2 context ✓
- ProfOlaf SLR snowballing protocol description — consistent with paper profile ✓
- Reproducible Pipeline "F1>0.85, near-linear scalability to 1,000 papers" — consistent with paper profile ✓
- RAG marginalization formula `P(y|x) ≈ Σ P(y|x, d_i) · P(d_i|x)` — structurally consistent with Lewis et al. formulation ✓
- Field-wide evaluation statistics (mean ~20 topics, median ~15 topics, max ~100 across 12 systems) — hedged with "approximately" and "roughly," appropriate for aggregated data ✓
- "Most surveyed papers" (softened from previously reported "77") — fix confirmed ✓

No hallucinated papers or fabricated claims detected. The three critical items from Round 3 (multi-hop barrier analysis with SciAtlas in §4.2.3, quality control paradigm comparison in §5.3, reproducibility crisis and cost blindness in §6.2) are all properly addressed in the current draft.

One remaining concern: the ReClaim cost estimate (~$50–100 per survey, lines 257–258) and the ResearchPilot quantization performance claim ("60–75% of the benchmark performance," line 348) are architectural/general extrapolations, not direct paper claims. Both are appropriately hedged with "~" and "typically," but a brief methodological note would strengthen defensibility for each.

**Citation Balance (4.0/5):** Distribution is reasonable across all four phases with notable improvements from Round 3:

- Phase 1: RAG + 6 datasets + FActScore — all cited with arXiv IDs ✓
- Phase 2: AutoSurvey, SurveyX, LitLLM, OpenScholar, ScholarCopilot, AcademicGPT, PaperQA2, DimInd — comprehensive ✓
- Phase 3 Table 4 (Multi-Agent): 8 systems (ARISE, SciSage, Agentic AutoSurvey, AutoSurvey2, MARCO, Federation of Agents, OrchMAS, DOVA) — comprehensive ✓
- Phase 3 Table 5 (Graph): 5 systems (SurveyForge, SurveyG, SurveyGen, ProfOlaf, GEAR-Up) — comprehensive ✓
- Phase 3 Table 6 (HITL/Iterative): 5 systems (InteractiveSurvey, PROMPTHEUS, CRUISE-Screening, ReClaim, IterSurvey) — comprehensive ✓
- Phase 3 Table 7 (Benchmarks): 9 benchmarks — all listed ✓
- Phase 4 (Frontier): DOVA, OrchMAS, ResearchPilot — all covered ✓
- SciAtlas (2605.22878) — now cited in §4.2.3 (Round 3 gap resolved) ✓
- Reproducible Pipeline (2508.04612) — now cited in §6.2 (Round 3 gap resolved) ✓
- AutoSurvey2 (2510.26012) — now in Table 4 and §4.1.1 (Round 2 gap resolved) ✓

Remaining balance gaps (important items from Round 3 not yet addressed):
- **SurveyGen-I (2508.14317)** — in the candidate pool (4 scouts), listed in the outline's §4.3 reference list, but still absent from Table 6. Not cited anywhere in the draft. This iterative refinement system bridges ReClaim's sentence-level verification and IterSurvey's draft-level iteration — a gap in the §4.3 landscape.
- **Early evaluation prototypes** — Auto-survey Challenge (2310.04480) and Wikipedia-style Survey Eval (2308.10410) are in the candidate pool and outline reference list but not acknowledged in §4.4. A 1-sentence parenthetical would contextualize the 2025 benchmark explosion.
- Some peripheral systems (MARCO, Federation of Agents, GEAR-Up) still receive only table-level mentions with minimal analysis — acceptable given space constraints and the draft's emphasis on depth over exhaustive listing.

**Redundancy (3.5/5):** The most significant issue from Round 3 — near-identical content between §4.4.2 and §6.3 — remains unresolved. Both sections present the same four fragmentation problems (metric fragmentation, reference survey fragmentation, human evaluation inconsistency, benchmark proliferation without consolidation) using nearly identical examples:

> **§4.4.2 (lines 298–305):** "Metric fragmentation: ROUGE-L, BLEU, FActScore, Citation F1, rubric-based quality scores (varying dimension counts: 5 in ARISE, 12 in Agentic AutoSurvey), win-rate, quiz answerability — no two systems report the same set. Reference survey fragmentation: AutoSurvey uses its own 10-topic set; SurveyForge uses SurveyBench (100 surveys); SciSage uses SurveyScope (46 papers) — no system evaluates against another's reference set."

> **§6.3 (lines 398–400):** "Metric fragmentation: ROUGE-L, BLEU, FActScore, Citation F1, rubric scores (5–12 dimensions), win-rate, quiz answerability — no two systems report the same set. Reference survey fragmentation: AutoSurvey uses its own 10-topic set; SurveyForge uses SurveyBench (100 surveys); SciSage uses SurveyScope (46 papers) — no system evaluates against another's reference set."

The §4.4.2 version provides complete analysis; §6.3 is a condensed excerpt that adds no new information. This is the most actionable redundancy.

Other instances are structurally justified or minor:
- DOVA deliberation quality critique in §5.1 (mechanism description) and §5.3 (critical assessment) — now properly cross-referenced as recommended in Round 3 ✓
- Narrative thread repetitions — by design as unifying devices ✓
- "No multi-hop reasoning" claim in §4.2.3, §5.3, and §6.4 — each serves a different analytical purpose (barrier decomposition, frontier critique, blind spot exposure) ✓

### Potential Hallucinations / Balance Issues / Redundant Content

1. **§4.4.2 ↔ §6.3 near-duplicate fragmentation analysis** (Reported Rounds 2–3, still unresolved): The four fragmentation problems are described with near-identical examples and language in both sections. §6.3 adds no new analysis; it acts as a recap. This is the most actionable redundancy in the draft and has been flagged for 3 consecutive rounds.

2. **SurveyGen-I (2508.14317) missing from §4.3** (Reported Round 3 important item, unresolved): This iterative refinement system bridges ReClaim's sentence-level verification and IterSurvey's draft-level iteration. It appears in 4 scout outputs and the outline's §4.3 reference list. Its absence leaves a gap in the iterative refinement landscape.

3. **Early evaluation prototypes not acknowledged in §4.4** (Reported Round 3 important item, unresolved): Auto-survey Challenge (2310.04480) and Wikipedia-style Survey Eval (2308.10410) predate the 2025 benchmark explosion. A 1-sentence acknowledgment would properly contextualize the field's evaluation history.

4. **ReClaim cost estimate lacks source caveat** (Reported Round 3 important item, unresolved): The ~$50–100 per survey figure (line 257) and "~1,000 API calls per survey" (line 257) are architectural extrapolations derived from the system description, not direct cost reports from the paper. The draft uses "~" and "approximately" which is appropriate hedging, but a brief methodological note such as "These are architectural estimates based on the per-sentence verification loop described in the paper, not direct cost reports from the authors" would improve traceability.

5. **ResearchPilot quantization performance claim is unsourced** (New finding): Line 348 states "Quantized 7B–13B models typically achieve 60–75% of the benchmark performance of the unquantized 70B+ cloud models they distill from." This is a general claim about quantized models, not a specific ResearchPilot result. While plausible and used in context of discussing trade-offs, this figure would benefit from a citation to a quantization benchmark paper (e.g., from the LLM compression literature).

6. **DimInd (2504.18496) and LitLLM (2402.01788) receive shallow treatment**: Both appear only in Table 2 with 2-line descriptions. DimInd's multi-level compression and LitLLM's modular RAG approach are interesting architectural variations that could benefit from 1–2 additional sentences of mechanism description. Minor issue given 12+ systems to cover.

### Suggestions

1. **Resolve §4.4.2 / §6.3 duplication — CHOOSE ONE approach** (Repeat of Round 1–3 suggestion):
   - **Option A** (recommended, as in Round 3): Remove the fragmentation analysis from §4.4.2 entirely. Let §4.4 simply describe the benchmark landscape (Table 7 + chronological overview). All critical analysis consolidates in §6.3, where it belongs thematically (Section 6 is the critical assessment).
   - **Option B**: Keep §4.4.2 as-is but replace the §6.3 paragraph with a single cross-reference sentence: "As detailed in §4.4.2, the evaluation comparability crisis manifests in four dimensions (metric, reference, human evaluation, and benchmark fragmentation) — no progress on any dimension has been made since." Then add genuinely new analysis (e.g., a concrete table showing how one system's scores differ across SurveyBench vs. SurveyScope vs. SurGE).

2. **Add SurveyGen-I (2508.14317) to Table 6 (§4.3)**: Insert one row: `| SurveyGen-I [2508.14317] | Quality-driven (iterative refinement with quality feedback) | Per-section (retrieval → draft → evaluate → refine) | Full survey (iterative refinement) | None |`. This is a 30-second table edit that fills the iterative refinement landscape gap.

3. **Acknowledge early evaluation prototypes in §4.4** (line 277 or 279): Add: "Two earlier evaluation prototypes — the Auto-survey Challenge (2310.04480) and Wikipedia-style Survey Evaluation (2308.10410) — predate this explosion and established the basic evaluation template for the field, though neither achieved widespread adoption." This is a 1-sentence addition.

4. **Add methodological note to ReClaim cost estimate** (§4.3, line 257): Insert after "$50–100 per survey in API fees": "— these are architectural estimates derived from the per-sentence verification loop described in the paper (~10 calls per sentence × 100 sentences × current GPT-4 API pricing), not direct cost reports from the authors."

5. **Add citation for ResearchPilot quantization claim** (§5.1, line 348): Either source the "60–75%" figure to a quantization benchmark paper, or soften to "generally achieve lower benchmark performance than unquantized cloud models" to avoid an unsourced specific range.

6. **Add 1–2 sentences of mechanism detail for DimInd and LitLLM** (§3.1.2): Expand DimInd from "multi-level compression" to briefly explain the dimension→indicator→facet cascade; expand LitLLM to note its pluggable retriever design. Minor analysis depth improvement.
