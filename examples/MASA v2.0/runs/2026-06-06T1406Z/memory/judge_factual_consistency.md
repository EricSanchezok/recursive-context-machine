## Factual Consistency Evaluation — Round 3

### Score: 3

**Summary**: The draft remains at the same consistency level as Round 2. One critical factual contradiction persists (AutoSurvey2 characterization contradicts pool entry). All pool-verifiable claims (~40+) match their source entries precisely, with no fabrications or hallucinations among verifiable claims. However, ~85 out of ~130 citations reference papers outside the 95-paper candidate pool, making the majority of specific factual claims unverifiable from provided source materials. The one direct source contradiction prevents a score of 4.

---

### Evidence

#### ✅ Pool-Verifiable Claims — All Match Source Entries

Every claim that can be checked against the candidate pool is accurate:

| # | Draft Claim (Section, Line) | Pool Entry | Match |
|---|----------------------------|------------|-------|
| 1 | AutoSurvey: "three-stage pipeline: outline generation, section drafting, and integration" (§2.1, line 27) | Pool #52: "outline → section drafting → integration pipeline" | ✅ |
| 2 | SurveyX: "two-phase Preparation+Generation pipeline with AttributeTree... +1.76 improvement" (§2.1, line 29) | Pool #4: "Two-phase generation (Preparation + Generation) with AttributeTree... +1.76 over baselines" | ✅ |
| 3 | SurveyForge: "outline heuristics learned from human-written surveys... scholar navigation agent" (§2.1, line 29) | Pool #6: "Learns outline structure from human-written surveys; scholar navigation agent" | ✅ |
| 4 | LitLLM: "web search, keyword extraction, paper re-ranking, and related work generation" (§2.1, line 31) | Pool #13: "web search → keyword extraction → paper re-ranking → related work generation" | ✅ |
| 5 | SurveyGen-I: "coarse-to-fine retrieval with adaptive planning and memory-guided writing" (§2.1, line 31) | Pool #5: "Coarse-to-fine retrieval with adaptive planning and memory-guided generation" | ✅ |
| 6 | Agentic AutoSurvey: "four specialist agents... 75–443 papers... 8.18/10... 4.77/10" (§2.2, line 37) | Pool #9: "Four specialized agents... 75–443 papers... 8.18/10... 4.77/10" | ✅ |
| 7 | SurveyG: "three hierarchical layers — Foundation, Development, Frontier" (§2.2, line 39) | Pool #2: "three layers (Foundation/Development/Frontier)" | ✅ |
| 8 | LiRA: "outlining, subsection writing, editing, and reviewing agents" (§2.2, line 41) | Pool #10: "outlining, subsection writing, editing, reviewing agents" | ✅ |
| 9 | ResearchPilot: "retrieving from Semantic Scholar and arXiv with structured findings extraction" (§2.2, line 41) | Pool #11: "retrieves from Semantic Scholar/arXiv; structured findings extraction" | ✅ |
| 10 | IterSurvey: "paper cards provide faithful grounding... review-and-refine loop... Survey-Arena" (§2.3, line 49) | Pool #8: "paper cards for faithful grounding... review-and-refine loop... Survey-Arena" | ✅ |
| 11 | InteractiveSurvey: "customize reference categorization, outline structure, and content" (§2.3, line 51) | Pool #7: "customization of reference categorization, outline, and content" | ✅ |
| 12 | SciSage: "+1.73 coherence improvement and +32% citation F1... SurveyScope — 46 papers across 11 CS domains" (§2.3, line 51) | Pool #14: "+1.73 coherence, +32% citation F1... SurveyScope (46 papers, 11 CS domains)" | ✅ |
| 13 | Cocitation comparison: "cocitation achieves the highest precision... combination yields best recall" (§3.1, line 85) | Pool #51: "shows advantage for co-citation, best when combining all three" | ✅ |
| 14 | Interleaved Snowballing: "LitBall desktop application" (§3.1, line 87) | Pool #28: "LitBall desktop app" | ✅ |
| 15 | Oignon: "dual-path ranking system that weights recency alongside relevance" (§3.1, line 89) | Pool #27: "dual-path ranking system with recency weighting" | ✅ |
| 16 | LitFM: "28.1% improvement in retrieval precision... three academic fields" (§3.2, line 93) | Pool #20: "28.1% retrieval precision improvement... 3 academic fields" | ✅ |
| 17 | CG-RAG: "lexical-semantic graph retrieval (LeSeGR)" (§3.2, line 95) | Pool #21: "lexical-semantic graph retrieval (LeSeGR)" | ✅ |
| 18 | PaSa: "+37.78% recall over the Google+GPT-4o baseline" (§3.4, line 109) | Pool #19: "+37.78% recall over Google+GPT-4o" | ✅ |
| 19 | PaSaMaster: "zero source hallucination, separating planning from retrieval" (§3.4, line 111) | Pool #29: "zero source hallucination; separates planning from retrieval" | ✅ |
| 20 | SPAR: "RefChain-based approach... up to +56% F1 improvement" (§3.4, line 113) | Pool #18: "RefChain-based query decomposition... up to +56% F1" | ✅ |
| 21 | OpenScholar: "45-million paper datastore... citation accuracy on par with human experts... outperforms GPT-4o by 5%" (§5.4, line 183) | Pool #12: "45M open-access paper datastore... citation accuracy on par with human experts; outperforms GPT-4o by 5%" | ✅ |
| 22 | FActScore: "<2% error rate in its automated version" (§6.4, line 215) | Pool #46: "automated version with <2% error rate" | ✅ |
| 23 | SurveyBench: "11,343 arXiv topics and 4,947 high-quality surveys... multi-faceted metric hierarchy... dual-mode protocol" (§7.1, line 231) | Pool #34: "11,343 arXiv topics and 4,947 high-quality surveys... multi-faceted metric hierarchy... dual-mode protocol" | ✅ |
| 24 | SurGE: "1M+ papers... 4-dimension evaluation (information coverage, referencing accuracy, structural organization, content quality)" (§7.1, line 231) | Pool #35: "1M+ papers... 4-dimension evaluation (information coverage, referencing accuracy, structural organization, content quality)" | ✅ |
| 25 | SurveyEval: "three dimensions (overall quality, outline coherence, reference accuracy) over 7 subjects" (§7.1, line 233) | Pool #39: "3 dimensions (overall quality, outline coherence, reference accuracy) over 7 subjects" | ✅ |
| 26 | DeepSurvey-Bench: "three 'academic value' dimensions — informational value, scholarly communication value, and research guidance value" (§7.1, line 233) | Pool #38: "3 dimensions (informational value, scholarly communication value, research guidance value)" | ✅ |
| 27 | SurveyLens: "1,000 human-written surveys across 10 disciplines... dual-lens evaluation" (§7.1, line 233) | Pool #37: "1,000 human-written surveys across 10 disciplines... dual-lens evaluation" | ✅ |
| 28 | HierCat: "7,600 hierarchical catalogues with 389,000 reference papers" (§7.3, line 253) | Pool #60: "7.6k literature review catalogues and 389k reference papers" | ✅ |
| 29 | SGSimEval: "combines LLM-based scoring with quantitative metrics and human preference data" (§7.1, line 233) | Pool #36: "Multi-faceted benchmark... LLM-based scoring... human preference metrics" | ✅ |
| 30 | vitaLITy 2: "66,692-paper corpus" (§8.2, line 285) | Pool #72: "66,692 paper corpus" | ✅ |
| 31 | PUREsuggest: "keyword-steerable rankings... visual exploration" (§3.3, line 103) | Pool #71: "keyword-steerable rankings and visual exploration" | ✅ |
| 32 | LLM Agent Survey [2503.21460]: "provides architectural context for the agent-based systems" (§1, line 13) | Pool #85A: "Comprehensive survey of LLM agents organized around architecture" | ✅ |
| 33 | Deep Search Agents Survey [2508.05668]: "catalogs architectures... comparing strategies" (§8.4, line 301) | Pool #85B: "First systematic survey of deep search agents... architecture, optimization, evaluation" | ✅ |
| 34 | AutoSurvey [2406.10252]: "operates without explicit citation graph traversal" (§2.1, line 27) | Pool #52: foundational paper, early system (no graph traversal mentioned) | ✅ |
| 35 | SurveyX [2502.14776]: "AttributeTree organizes papers hierarchically by attributes" (§2.1, line 29) | Pool #4: "AttributeTree preprocessing" | ✅ |

#### ❌ Critical Issue — One Direct Source Contradiction

**C1: AutoSurvey2 characterization contradicts pool entry (§2.3, line 53)**

- **Draft claim**: "AutoSurvey2 [arXiv:2510.26012] also falls in this category [hybrid] due to its parallel section generation with real-time retrieval, **though it lacks the iterative refinement** of other hybrid systems."
- **Pool entry (#1, line 32)**: "Multi-stage pipeline with parallel section generation, **iterative refinement**, real-time retrieval of recent publications, multi-LLM evaluation."
- **Issue**: The draft explicitly states AutoSurvey2 "lacks iterative refinement," but the pool entry explicitly includes "iterative refinement" as a feature. This is a direct factual contradiction — the draft claims the absence of a feature that the source confirms is present.
- **Severity**: **🔴 Critical** — One of the few claims in the draft that can be definitively checked against a pool entry, and it is wrong. This single error prevents a score of 4.

#### ⚠️ Unverifiable Details

**M1: Agentic AutoSurvey "BFS-based" traversal (§2.2, line 37)**
- **Draft claim**: "though the specific traversal strategy is BFS-based rather than learned."
- **Pool entry (#9)**: Mentions "four specialist agents" and "citation-aware retrieval by starting from seed papers and expanding through reference lists" but does not mention "BFS" or any specific graph traversal algorithm name.
- **Severity**: 🟡 Moderate — While "reference-list expansion from seed papers" is plausibly BFS-like, the draft asserts "BFS-based" as a specific algorithmic claim not verified by the pool.

#### ❌ Out-of-Pool Quantitative Claims

The following claims cite papers absent from the 95-paper candidate pool, making their specific numerical values unverifiable:

| # | Claim (Section) | Citation | Pool Status | Specific Figures |
|---|----------------|----------|-------------|------------------|
| 1 | "identified approximately 147,000 hallucinated citations" (§1, line 7) | [2605.07723] | ❌ Not in pool | "147,000" |
| 2 | "4–6 weeks per systematic review in manual screening alone" (§1, line 7) | [2409.04600] | ❌ Not in pool | "4–6 weeks" |
| 3 | "ReClaim... achieving 90% citation accuracy" (§6.1, line 197) | [2407.01796] | ❌ Not in pool | "90%" |
| 4 | "improves citation accuracy (by 15–20% over abstract-only access)... 70B+ parameter models" (§6.2, line 205) | [2410.11217] | ❌ Not in pool | "15–20%", "70B+" |
| 5 | "LLMs achieve 4.2–18.5% accuracy versus humans at 69.7%" (§7.2, line 249) | [2407.12861] | ❌ Not in pool | "4.2–18.5%", "69.7%" |
| 6 | "REASONS... 42% reduction in hallucination when training with citation-aware objectives" (§7.2, line 249) | [2405.02228] | ❌ Not in pool | "42%" |
| 7 | "SciReviewGen reports 15–25% hallucination rates in generated surveys" (§9.1, line 313) | [2305.15186] | ❌ Not in pool | "15–25%" |
| 8 | "costs $10–50 in API fees for current multi-agent systems... [2509.18661]" (§9.5, line 337) | [2509.18661] | ❌ Pool #9 entry lacks cost data | "$10–50" |

None of these claims appear fabricated — they are attributed to specific, well-known papers. However, because those papers are not in the candidate pool, the numerical values cannot be verified against provided source materials. This is a documentation gap rather than necessarily a factual error, but it reduces the evaluation's confidence.

#### 🔵 Table 1 Formatting Issue (§2.4, lines 63–71)

All content in Table 1 (system names, traversal types, evaluation scores) matches pool entries. However, each data row has a leading `| |` (double pipe) that creates an extra empty column before the architecture label. This is a presentation issue, not a factual error.

---

### Citation Coverage Assessment

| Category | Count | Notes |
|----------|-------|-------|
| Total unique arXiv IDs cited in draft | ~130 | Across all 10 sections |
| Present in candidate pool | ~45 | ✅ Fully verifiable from pool entries |
| NOT in candidate pool | ~85 | ❌ Cannot verify claim-source alignment |
| Direct contradictions (pool-verifiable) | 1 | AutoSurvey2 "lacks iterative refinement" |
| Out-of-pool claims with specific numbers | 8 | Precise figures unverifiable (see table above) |

---

### Potential Hallucinations

| # | Claim | Issue | Suggestion |
|---|-------|-------|------------|
| 1 | AutoSurvey2 "lacks the iterative refinement of other hybrid systems" (§2.3) | Pool entry (#1) explicitly says "iterative refinement." Draft directly contradicts its cited source. | Change to "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline" (as specified in supervisor notes) |
| 2 | Agentic AutoSurvey "BFS-based" traversal (§2.2) | Pool entry (#9) does not mention BFS; unverifiable detail | Change to "reference-list expansion" or "breadth-first style expansion" |
| 3 | SciReviewGen "15–25% hallucination rates" (§9.1) | Paper [2305.15186] not in pool; specific figure unverifiable | Verify against full paper or hedge ("reportedly finds 15–25% hallucination rates") |
| 4 | "$10–50 in API fees" (§9.5) | Pool entry (#9) for [2509.18661] lacks cost data | Verify against full paper or hedge ("estimated $10–50") |
| 5 | "4.2–18.5% vs 69.7%" CiteME accuracy (§7.2) | Paper [2407.12861] not in pool | Verify against full paper or hedge |
| 6 | "147,000 hallucinated citations" (§1) | Paper [2605.07723] not in pool | Verify against full paper |
| 7 | "15–20% improvement, 70B+ parameter models" (§6.2) | Paper [2410.11217] not in pool | Verify against full paper |
| 8 | "4–6 weeks per systematic review" (§1) | Paper [2409.04600] (Pool #67) does not mention 4–6 weeks in its entry | Verify against full paper |
| 9 | ReClaim "90% citation accuracy" (§6.1) | Paper [2407.01796] not in pool | Verify against full paper |
| 10 | REASONS "42% reduction in hallucination" (§7.2) | Paper [2405.02228] not in pool | Verify against full paper |

---

### Comparison with Round 2 Evaluation

| Dimension | Round 2 | Round 3 (Current) | Change |
|-----------|---------|-------------------|--------|
| Pool-verifiable claims | ~45 claims, all match | ~45 claims, all match | ✅ Same |
| Direct contradictions | 1 (AutoSurvey2) | 1 (AutoSurvey2) | ❌ Not fixed |
| Out-of-pool quantitative claims | ~6–7 identified | ~8 identified (including 2 newly noted: ReClaim 90%, REASONS 42%) | ⚠️ Same |
| Score | 3 | 3 | Same |

**The draft has not been updated since Round 2.** The AutoSurvey2 contradiction identified in the Round 2 judge evaluation and reiterated in the Round 3 supervisor notes remains unfixed. All other scoring factors are unchanged.

---

### Suggestions

**Critical (must fix before score can reach 4):**

1. **Fix AutoSurvey2 characterization (§2.3, line 53)**: Change "though it lacks the iterative refinement of other hybrid systems" to "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline." The pool entry explicitly attributes "iterative refinement" to AutoSurvey2 — denying it entirely is a factual error.

**Important (should address):**

2. **Audit out-of-pool quantitative claims**. The draft contains ~8 specific numerical claims that cite papers outside the candidate pool. For each, either (a) add the cited paper to the pool, (b) verify the number against the full paper, or (c) hedge with "reportedly," "estimated," or similar qualifiers. The most impactful to fix:
   - §7.2: CiteME accuracy [2407.12861]
   - §9.1: SciReviewGen hallucination rates [2305.15186]  
   - §6.2: Citation Capacity figures [2410.11217]
   - §9.5: Cost claim [2509.18661]

3. **Qualify Agentic AutoSurvey BFS claim (§2.2, line 37)**: Change "BFS-based" to "reference-list expansion" which is directly supported by the pool's description of "starting from seed papers and expanding through reference lists."

**Nice-to-have:**

4. **Add hedging language** to unverifiable numerical claims even when attributed to a specific paper outside the pool. Example: "reportedly achieves 4.2–18.5% accuracy" rather than "achieves 4.2–18.5% accuracy."

5. **Fix Table 1 formatting** (§2.4, lines 63–71): Remove the leading `| |` (double pipe) from data rows to correct the extra empty column.

---

### Weighted Contribution

Score 3 × 20% = **0.60**

*Rationale for Score 3 (Partial Consistency):*
- **Cannot be Score 4** because one pool-verifiable claim (AutoSurvey2) directly contradicts its source. The standard for Score 4 is "nearly all claims are accurately supported by cited sources" — one confirmed contradiction disqualifies this.
- **Cannot be Score 2** because (a) all critical Round 1 fabrications were fixed in Round 2, (b) every pool-verifiable claim except one matches its source precisely, (c) no fabricated statistics or source misrepresentations exist, and (d) the out-of-pool claims, while unverifiable, are attributed to legitimate papers and are not prima facie false.
- **Score 3 is appropriate** because there is one clear factual error in an otherwise well-sourced draft where all ~45 other pool-verifiable claims are accurate.
