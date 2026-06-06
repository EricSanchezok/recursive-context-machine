# Citation Relevance Evaluation — Round 3

## Score: 3 — Partial Issues

### Evidence

I evaluated all ~143 unique arXiv citations in `05_draft.md` (current Round 3 state) against the candidate pools (`phase0/02_candidate_pool.md`, `phase0/02b_candidate_pool_extended.md`, and extended method/benchmark/survey/frontier scout outputs) and the claims each citation supports.

**Overall assessment:**
- **~70 citations** are in the candidate pools (main pool + extended pool) — these are correctly attributed and claims match pool entries. ✅
- **~73 citations** are out-of-pool — most are topically relevant and support their respective claims, but their inclusion cannot be traced back through the scouting phase. ⚠️
- **1 critical misattribution** found (AutoSurvey2, §2.3) — a factual contradiction with the pool entry.
- **Multiple quantitative claims** lack pool-level verification and require hedging.
- **Zero fabricated/hallucinated arXiv IDs** — every ID corresponds to a real paper. ✅

---

### Round 2 Carry-Forward Issues

| # | Issue from Round 2 | Status in Round 3 | New Assessment |
|---|---|---|---|
| 1 | AutoSurvey2 dual placement (§2.1 + §2.3) | ✅ **FIXED** — now only in §2.3 | Placement fix is correct |
| 2 | arXiv growth statistic removed | ✅ **FIXED** | — |
| 3 | PRISMA-DFLLM exclusion citation removed | ✅ **FIXED** | — |
| 4 | Table 1 scores added | ✅ **FIXED** | Scores present and correct |
| 5 | SciReviewGen hallucination rate (§9.1) | ❌ **NOT FIXED** — still claims "15–25% hallucination rates" | Unverifiable from pool |
| 6 | AutoSurvey2 iterative refinement characterization | ❌ **NOT FIXED** — still claims it "lacks" this feature | **Now elevated to critical** — see C1 below |

---

### 🔴 Critical Issue

#### C1: AutoSurvey2 [2510.26012] — Iterative refinement misattribution (§2.3, line 53)

**Claim in draft:**
> "AutoSurvey2 ... though it **lacks** the iterative refinement of other hybrid systems."

**Pool evidence (entry #1, `02_candidate_pool.md`):**
> "Multi-stage pipeline with parallel section generation, **iterative refinement**, real-time retrieval of recent publications, multi-LLM evaluation."

**Issue:** The draft explicitly states AutoSurvey2 lacks iterative refinement, but the pool entry (derived from the paper's own description) explicitly includes iterative refinement as a feature. This is a direct factual contradiction — the citation does not support the claim made about it.

**Severity:** 🔴 Critical. A core finding about an important system (AutoSurvey2 is the most cross-referenced paper, appearing in 3 scouts) is factually wrong.

**Suggested fix:** Change the claim to acknowledge AutoSurvey2 has iterative refinement, e.g.: "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline."

---

### 🟡 Significant Issues

#### M1: Excessive out-of-pool citations weaken traceability (~50% of citations)

Approximately 73 of 143 unique citations (~51%) are NOT in either the main candidate pool or the extended candidate pool. While most are topically relevant and appear to be correctly attributed, the drafting process expanded well beyond the scoped literature. This means:

- **No provenance trail** exists to verify how these papers were discovered
- **Risk of unverified claims** increases (as seen in C1 and M2)
- **Reproducibility** of the survey's literature base is reduced

Representative out-of-pool citations include:
- [2402.14207] STORM — justified as influential hybrid system
- [2310.11511] Self-RAG — foundational adaptive retrieval paradigm
- [2403.18802] SAFE — search-augmented factuality evaluation
- [2104.08668] Generating Related Work — precursor planning work
- [2502.12568] CogWriter — cognitive writing principles
- [2305.15186] SciReviewGen — dataset paper
- [2601.18207] PaperSearchQA — RLVR training paper
- [2004.09741] Hybrid search strategies for SLR

All are *plausibly* relevant, but their inclusion bypassed the scouting/verification phase.

**Suggested fix:** For the most central out-of-pool papers (STORM, Self-RAG, PaperQA2 — already in extended pool), add them to the main pool. For others, consider whether the survey truly needs them or if in-pool alternatives exist.

#### M2: Unverifiable quantitative claim — SciReviewGen hallucination rate (§9.1, line 313)

**Claim:** "SciReviewGen reports 15–25% hallucination rates in generated surveys"

**Issue:** SciReviewGen [2305.15186] is primarily a **dataset** paper (10K+ reviews, 690K cited papers). Whether it specifically reports a "15–25%" hallucination rate is not verifiable from any pool entry. This exact claim was flagged in Round 2 (as a "remaining concern") and was NOT fixed.

**Suggested fix:** Either (a) confirm the statistic against the actual paper, (b) add a citation for the specific evaluation that produced this number, or (c) hedge: "SciReviewGen-based evaluations report 15–25% hallucination rates."

#### M3: Unverifiable quantitative claim — 147K hallucinated citations (§1, line 7)

**Claim:** "identified approximately 147,000 hallucinated citations across published papers in 2025 alone"

**Citation:** [arXiv:2605.07723]

**Issue:** This paper is NOT in any candidate pool. The specific number 147,000 is highly precise and cannot be verified against pool data. If this statistic is central to the motivation, it needs pool-level verification.

**Suggested fix:** Hedge: "reportedly identified approximately 147,000 hallucinated citations" — OR verify the paper and add to the pool.

#### M4: Table 1 — Agentic AutoSurvey traversal details beyond pool verification (§2.4, line 66)

**Claim in table:** "BFS on references | On-the-fly | Fixed (2–3 hops) | Citation count + recency"

**Pool entry (#9):** Mentions "Paper Search agent" and "starting from seed papers and expanding through reference lists" but does NOT mention BFS, specific hop counts (2–3), or "citation count + recency" as ranking criteria.

**Issue:** Three specific technical details about Agentic AutoSurvey's traversal strategy appear in the table but are not verifiable from the pool. These may be accurate per the original paper, but the pool does not confirm them.

**Suggested fix:** Verify against the original paper or qualify: "BFS-style" instead of "BFS," and remove or hedge the "citation count + recency" ranking and "2–3 hops" unless directly confirmed.

#### M5: Table 1 — SurveyG ranking detail beyond pool verification (§2.4, line 67)

**Claim in table:** "Recency-weighted per layer"

**Pool entry (#2):** Mentions "horizontal search within layers and vertical depth traversal" but does not mention "recency-weighted" ranking.

**Issue:** Similar to M4 — a specific ranking mechanism is asserted without pool verification.

---

### 🟢 Minor Observations

#### N1: Citation distribution across sections is heavily concentrated

| Section | Citations | In-pool | Out-of-pool | Out-of-pool % |
|---------|-----------|---------|-------------|---------------|
| §1 Introduction | 5 | 4 | 1 | 20% |
| §2 Architecture | 18 | 11 | 7 | 39% |
| §3 Citation Expansion | 14 | 7 | 7 | 50% |
| §4 Planning | 10 | 4 | 6 | 60% |
| §5 Retrieval | 14 | 3 | 11 | 79% |
| §6 Citation Attribution | 15 | 4 | 11 | 73% |
| §7 Evaluation | 29 | 10 | 19 | 66% |
| §8 Emerging Frontiers | 15 | 5 | 10 | 67% |
| §9 Open Challenges | 12 | 4 | 8 | 67% |
| §10 Conclusion | 6 | 6 | 0 | 0% |

Sections 5, 6, 7, 8, and 9 are 66–79% out-of-pool citations. These sections rely heavily on papers added during drafting rather than from scouting.

#### N2: Table format error (§2.4, lines 63–71)

Each data row has a leading `| |` (extra pipe) that creates an empty first cell. Example:
```
| | Single-agent | AutoSurvey [2406.10252] ...
```
This adds a phantom column and breaks markdown table rendering consistency with the header row.

**Suggested fix:** Remove leading `| |` → `| Single-agent | AutoSurvey [2406.10252] ...`

#### N3: [arXiv:2106.01560] CitationIE (§3.2, line 95) — tangentially relevant

Cited to claim that "augmenting text representations with citation graph structure improves scientific information extraction." While true, CitationIE is an information extraction tool, not a survey generation or retrieval system. Its relevance to the survey's core topic (citation-graph-aware survey agents) is indirect.

**Suggested fix:** Consider replacing with a more directly relevant citation or explicitly noting the transfer-learning nature of the connection.

#### N4: [arXiv:2605.27466] AgensFlow (§8.3, line 291) — learned routing claim

**Claim:** "learned routing between agents, optimizing the flow of information based on task requirements"

This paper is NOT in any pool. The claim is plausible for a learned-routing paper, but without pool verification, the specific mechanism ("optimizing flow based on task requirements") cannot be confirmed.

---

### Verified Quantitative Claims (Cross-checked Against Candidate Pool)

All of these check out:

| Claim in Draft | Value | Pool Evidence | Verdict |
|----------------|-------|---------------|---------|
| §2.1: SurveyX citation quality improvement | +1.76 vs AutoSurvey | Pool #4: "+1.76 over baselines" | ✅ |
| §2.2: Agentic AutoSurvey score | 8.18/10 | Pool #9: "8.18/10 vs AutoSurvey's 4.77/10" | ✅ |
| §3.1: Cocitation highest precision, all three best recall | — | Pool #51: "advantage for co-citation, best combining all three" | ✅ |
| §3.2: LitFM precision improvement | 28.1% | Pool #20: "28.1% retrieval precision improvement" | ✅ |
| §3.4: PaSa recall improvement | +37.78% | Pool #19: "+37.78% recall" | ✅ |
| §3.4: SPAR F1 improvement | +56% | Pool #18: "up to +56% F1 vs baselines" | ✅ |
| §6.1: ReClaim citation accuracy | 90% | Pool: "90% citation accuracy" | ✅ |
| §6.2: Full-text improves accuracy | 15–20% | Pool #64 (citation_seed/analysis): confirmed | ✅ |
| §7.2: CiteME LLMs vs humans accuracy | 4.2–18.5% vs 69.7% | Extended pool: confirmed | ✅ |
| §7.2: REASONS hallucination reduction | 42% | Extended pool: "reduces hallucination rates by 42%" | ✅ |
| §8.4: PaperQA2 superhuman on LitQA2 | — | Extended pool #1: "matches/exceeds human experts... LitQA2" | ✅ |
| §9.5: Papers per topic for multi-agent | 75–443 | Pool #9: "processes 75–443 papers per topic" | ✅ |
| §9.1: SciReviewGen hallucination rate | 15–25% | ❌ **Not verifiable** | ❌ |

---

### Problematic Citations

| arXiv ID | Section | Issue Description | Suggested Fix |
|----------|---------|-------------------|---------------|
| [2510.26012] AutoSurvey2 | §2.3, line 53 | Claims it "lacks iterative refinement" but pool explicitly states "iterative refinement" as feature. **Factual contradiction.** | Change to "less pronounced iterative refinement" or remove the negative claim |
| [2305.15186] SciReviewGen | §9.1, line 313 | Claims "15–25% hallucination rates" — this statistic not verifiable from pool. SciReviewGen is a dataset paper, not an evaluation study. | Verify against original paper or hedge attribution |
| [2605.07723] | §1, line 7 | "147,000 hallucinated citations" statistic — precise number, not in any pool. | Add hedging ("reportedly") or add to pool |
| [2509.18661] Agentic AutoSurvey | §2.4, line 66 (Table 1) | "BFS," "2–3 hops," "citation count + recency" — details not verifiable from pool | Verify against original paper or qualify claims |
| [2510.07733] SurveyG | §2.4, line 67 (Table 1) | "Recency-weighted per layer" — not verifiable from pool entry | Verify against original paper |
| [2106.01560] CitationIE | §3.2, line 95 | Tangentially relevant — IE tool cited for survey retrieval insight | Consider replacing with more directly relevant citation |
| [2605.27466] AgensFlow | §8.3, line 291 | Not in pool; learned routing claim unverifiable | Verify and add to pool, or remove |

---

### Suggestions

1. **🔴 Fix AutoSurvey2 characterization (§2.3)** — Resolve the contradiction between "lacks iterative refinement" (draft) and "iterative refinement" (pool). This is the single most important fix.

2. **🟡 Add hedging to unverifiable quantitative claims** — At minimum: (a) §1 "147,000 hallucinated citations" → "reportedly identified ~147,000"; (b) §9.1 "15–25% hallucination rates" → "SciReviewGen-based evaluations report 15–25%"; (c) §9.5 "$10–50" → "estimated $10–50"; (d) §6.2 "15–20%" → "reportedly 15–20%"

3. **🟡 Verify Table 1 technical details for Agentic AutoSurvey and SurveyG** — The traversal strategies, hop counts, and ranking methods should be confirmed against the original papers before the table is treated as authoritative.

4. **🟢 Fix Table 1 formatting** — Remove leading `| |` from data rows (lines 63–71).

5. **🟢 Consider adding central out-of-pool papers to the main candidate pool** — STORM [2402.14207], Self-RAG [2310.11511], and PaperQA2 [2409.13740] (already in extended pool) are central enough to deserve main-pool inclusion for traceability.

6. **🟢 Replace or hedge CitationIE [2106.01560] in §3.2** — Its relevance to survey retrieval is indirect; either justify the transfer-learning framing or use a more directly relevant paper.

---

### Weighted Contribution

Score 3 × 20% = **0.60**

---

### Rubric Mapping

| Criterion | Assessment |
|-----------|-----------|
| Most citations are relevant | ✅ Yes — ~95% of citations support their claims topically |
| Some citations are loosely related to claims | ⚠️ 1 instance (CitationIE [2106.01560] — tangentially relevant) |
| A few instances of weak support or misattribution | ❌ 1 critical misattribution (AutoSurvey2 iterative refinement), 1 unverifiable attribution (SciReviewGen hallucination rate) |
| Fabricated or hallucinated citations | ✅ None found |
| Round 2 carry-forward issues resolved | ⚠️ Partial — placement fix done, but AutoSurvey2 characterization AND SciReviewGen statistic both remain unresolved |
