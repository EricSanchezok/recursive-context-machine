## Citation Balance Evaluation

### Score: 5

### Citation Distribution

Total unique papers cited: **~83**
Total citation instances: **~185**
Citation range: **1–5** per paper

| Citation Count | # of Papers | % of Total | Assessment |
|---------------|-------------|-----------|------------|
| **5** | 2 | 2.4% | **Appropriate** — papers spanning 3+ sections naturally |
| **4** | 4 | 4.8% | **Appropriate** — high-impact systems/benchmarks with cross-sectional relevance |
| **3** | 18 | 21.7% | **Appropriate** — core systems with mentions across multiple analytical dimensions |
| **2** | 22 | 26.5% | **Appropriate** — typical for cross-referenced supporting or background papers |
| **1** | ~37 | 44.6% | **Appropriate** — single-mention contributions, background, foundational or peripheral works |

**Papers cited 5 times (top tier):**

| Papers | Count | Sections | Assessment |
|--------|-------|----------|------------|
| LitLLM toolkit [arXiv:2402.01788] | 5 | 2.1 (intro), 4.1×2 (query formulation), 4.2×2 (evidence extraction) | **Appropriate** — appears in 3 different sections reflecting its cross-cutting relevance as both an architecture and a retrieval pipeline component |
| SurveyGen-I [arXiv:2508.14317] | 5 | 2.1 (intro), 3.2×2 (adaptive planning), 4.1×2 (coarse-to-fine retrieval) | **Appropriate** — appears in 3 sections as it contributes to architecture, planning, and retrieval |

**Papers cited 4 times:**

| Papers | Count | Sections | Assessment |
|--------|-------|----------|------------|
| SurveyForge [arXiv:2503.04629] | 4 | 2.1 (intro+summary), 3.1 (planning), 4.1 (retrieval) | **Appropriate** — spans architecture, planning, and retrieval sections |
| Agentic AutoSurvey [arXiv:2509.18661] | 4 | 2.2 (intro+summary), 2.4 (comparison), 7.1 (spectrum mention) | **Appropriate** — highest-performing system, justifies multiple cross-references |
| LitFM [arXiv:2409.12177] | 4 | 4.2×2 (evidence extraction), 5.3×2 (citation graph) | **Appropriate** — core to both retrieval and citation graph sections |
| SurveyBench [arXiv:2510.03120] | 4 | 6.1 (benchmark description), 6.5×2 (human eval), 8.2 (standardization) | **Appropriate** — capped at 4, uses cross-references in Sections 3.4 and 8.5 without arXiv IDs |

**Papers cited 3 times (representative sample — 18 total):**

| Papers | Count | Sections | Assessment |
|--------|-------|----------|------------|
| AutoSurvey [arXiv:2406.10252] | 3 | 2.1 (foundational), 3.1 (planning) | **Appropriate** — foundational system |
| SurveyX [arXiv:2502.14776] | 3 | 2.1, 3.1, 4.1 | **Appropriate** — cross-cutting methodology |
| InteractiveSurvey [arXiv:2504.08762] | 3 | 2.3×2, 2.4 | **Appropriate** — primarily in its architectural section; cross-references in 7.1 and 8.6 without arXiv IDs |
| STORM [arXiv:2402.14207] | 2 | 2.3, 2.4 | **Appropriate** — seminal influence, cited where described and in comparative analysis |
| Self-RAG [arXiv:2310.11511] | 2 | 4.3 (foundational), 4.3 (caveat) | **Appropriate** — foundational adaptive retrieval method |
| FActScore [arXiv:2305.14251] | 3 | 5.4, 6.4, 8.1 | **Appropriate** — key method across factuality, hallucination, and challenges |
| SurGE [arXiv:2508.15658] | 3 | 6.1, 6.5, 8.2 | **Appropriate** — benchmark referenced across evaluation and challenges |
| SurveyEval [arXiv:2512.02763] | 3 | 6.1, 6.5, 8.2 | **Appropriate** — benchmark referenced across evaluation and challenges |
| OpenScholar [arXiv:2411.14199] | 3 | 4.4, 7.4×2 | **Appropriate** — retrieval synthesis and frontiers sections |
| ResearchPilot [arXiv:2603.14629] | 3 | 2.2, 7.4, 8.4 | **Appropriate** — architecture, frontiers, and cost discussion |
| ChatCite [arXiv:2403.02574] | 3 | 7.1, 7.1, 8.6 | **Appropriate** — interactive survey method across frontiers and challenges |
| vitaLITy 2 [arXiv:2408.13450] | 3 | 7.2, 7.2, 8.3 | **Appropriate** — living survey method across frontiers and challenges |
| Evolving Lit Analysis [arXiv:2502.18791] | 3 | 7.2, 7.2, 8.3 | **Appropriate** — living survey method across frontiers and challenges |

**Papers cited 2 times (~22 papers):** All appropriate — typical for cross-referenced supporting contributions.

**Papers cited 1 time (~37 papers):** All appropriate — single-purpose contributions, background papers, and foundational works cited in their specific context.

**Papers cited 0 times (notable from outline, correctly cross-referenced without arXiv ID):** SurveyBench (Sections 3.4, 8.5), InteractiveSurvey (Sections 7.1, 8.6), LitLLM toolkit (Section 6.3). All follow the outline's instruction to use cross-references instead of redundant citations.

### Evidence

**Strengths:**

1. **Excellent gradient.** The distribution follows a textbook long-tail: most papers (44.6%) cited once, with a smooth gradient up to a maximum of 5. No single paper or group dominates — the top paper (5 citations) accounts for only ~2.7% of total citation instances.

2. **Every outline-referenced paper is cited at least once.** All ~83 papers from the outline's reference lists appear in the draft. No omissions of major contributions.

3. **Cross-references wisely replace redundant citations.** The draft uses section cross-references (e.g., "SurveyBench (described in Section 6.1)", "InteractiveSurvey (described in Section 2.3)") to avoid re-citing arXiv IDs when a paper is mentioned outside its primary section. This significantly improves balance compared to the Round 2 evaluation where SurveyBench had 9 raw citations.

4. **Section 9 correctly has zero new citations.** Per outline guidelines, the conclusion uses system names without arXiv IDs, drawing only from earlier sections.

5. **No evidence of citation bias.** The distribution does not cluster around a single author group, institution, or system family. Papers from diverse research groups receive proportional treatment.

6. **Cross-sectional papers cited proportionally.** Papers that genuinely span multiple topics (LitLLM: architecture + retrieval; SurveyGen-I: architecture + planning + retrieval; FActScore: factuality + evaluation + challenges) receive more citations — reflecting their actual scope rather than padding.

7. **Foundational papers cited with appropriate restraint.** Self-RAG (2 citations), Self-Refine (2), STORM (2), and the foundational RAG paper [arXiv:2005.11401] (1) appear where needed without over-citation.

8. **Significant improvement from Round 2.** Previous issues have been resolved:
   - SurveyBench: reduced from 9 to 4 (target was ≤4)
   - LitLLM: reduced from 6 to 5
   - InteractiveSurvey: reduced from 6 to 3 (cross-references in Sections 7.1 and 8.6)
   - STORM: increased from 1 to 2 (second mention added in Section 2.4 comparative analysis)

9. **No gaps in section-level distribution.** Each major section cites 5–23 unique papers proportional to the number of systems discussed. No section is over- or under-cited relative to its content density.

**Minor observations:**

1. LitLLM and SurveyGen-I at 5 citations each are slightly above the 4-citation tier but justifiably — each spans 3 different sections covering distinct contributions (architecture, planning/retrieval for SurveyGen-I; architecture, query formulation, evidence extraction for LitLLM). This is not an imbalance.

2. Section 4.3 clusters several general RAG methods (FoRAG, InstructRAG, RA-RAG) with single citations each. This is appropriate given the outline's caveat that these methods have not yet been validated on survey generation tasks — the single mentions accurately reflect their supporting role.

3. Section 8.5 (Multi-Modal Content) correctly has no arXiv citations — it discusses GPT-4V and LLaVA as generic model capability references rather than citing specific papers, consistent with the outline's guidance.

### Weighted Contribution

**Score: 5**

Weight: 15%

**Weighted contribution: 5 × 15% = 0.75**
