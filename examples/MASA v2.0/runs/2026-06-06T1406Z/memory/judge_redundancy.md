# Redundancy Evaluation

## Score: 4 (Good)

Most content appears in its most logical location. Cross-references are used consistently throughout to avoid full re-descriptions. Systems that appear in multiple sections (SurveyG, LitFM, SciSage, LiRA, PaSa) are discussed from genuinely different analytical angles — architecture vs. traversal vs. attribution. The few instances of repeated phrasing are minor and do not significantly harm readability.

---

## Redundant Content

### Redundancy 1 — OpenScholar Evaluative Phrase (§5.4 / §8.4)
**§5.4** (line 183): "achieving citation accuracy on par with human experts and outperforming GPT-4o by 5%"
**§8.4** (line 301): "achieving citation accuracy on par with human experts"

The key evaluative claim appears nearly verbatim in two sections. Each serves a different analytical role (multi-source synthesis vs. deep research convergence), but the §8.4 occurrence could reference §5.4 instead.

**Suggested fix:** Replace §8.4's "achieving citation accuracy on par with human experts" with: "(as detailed in §5.4, achieving citation accuracy on par with human experts)."

### Redundancy 2 — GLUE Standardization Analogy (§9.2 / §10)
**§9.2** (line 319): "The history of NLP suggests that a standardized evaluation framework akin to GLUE (General Language Understanding Evaluation) could catalyze progress"
**§10** (line 357): "establishing a standardized evaluation framework for citation-aware survey systems, analogous to GLUE (General Language Understanding Evaluation) in NLP"

The GLUE analogy is introduced in §9.2 with full context, then reiterated in §10 as a research agenda item. The §10 mention is expected in a conclusion that recaps recommendations, but the analogy explanation is duplicated.

**Suggested fix:** Replace §10's GLUE parenthetical with: "(as argued in §9.2)."

### Redundancy 3 — Coverage Breadth Definition (§2 / §9)
**§2** (line 23): "coverage breadth to mean the fraction of relevant papers in a topic that a survey cites"
**§9** (line 319): "Coverage breadth — the fraction of relevant papers in a topic that a survey cites"

The definition is formally introduced in §2 and then re-stated verbatim in §9. A simple cross-reference would suffice.

**Suggested fix:** Replace §9's definition with: "Coverage breadth (defined in §2) should be standardized as a key evaluation dimension."

### Redundancy 4 — SuperWriter Across Adjacent Subsections (§4.2 / §4.3)
**§4.2** (line 139): "SuperWriter [arXiv:2506.04180] extends this with reflection-driven planning, hierarchical DPO, and Monte Carlo Tree Search (MCTS) for exploring alternative outline structures."
**§4.3** (line 145): "SuperWriter's structured thinking-through process [arXiv:2506.04180] combines iterative refinement with multi-level feedback"

SuperWriter is introduced in §4.2 (adaptive planning) and then re-introduced in §4.3 (iterative refinement). These are adjacent subsections, and the two descriptions overlap in mentioning the system's planning capability.

**Suggested fix:** In §4.3, replace the SuperWriter sentence with: "SuperWriter (cross-referenced from §4.2) adds multi-level feedback to the iterative refinement loop — outline quality, section coherence, and citation correctness are evaluated at different granularities."

### Redundancy 5 — HierCat Dataset Details (§4.1 / §7.3)
**§4.1** (line 131): "HierCat [arXiv:2304.03512] provides a large-scale dataset of 7,600 hierarchical catalogues and 389,000 reference papers"
**§7.3** (line 253): "HierCat [arXiv:2304.03512] contains 7.6K hierarchical catalogues with 389K references"

Both sections state the same statistics (7,600/7.6K catalogues, 389K references) with circular cross-references to each other. The full description should live in one location.

**Suggested fix:** Keep the primary description in §7.3 (datasets section). Replace §4.1's mention with: "The HierCat dataset [arXiv:2304.03512] (detailed in §7.3) provides data-driven outline patterns derived from hierarchical catalogues."

---

## Evidence

### What the Draft Does Well

| Practice | Example | Location |
|---|---|---|
| Explicit cross-references | "(cross-referenced from Section 2.2)" for LiRA | §8.3 |
| Explicit cross-references | "(cross-referenced from Section 6.4)" for FActScore/SAFE/SelfCheckGPT | §9.1 |
| Explicit cross-references | "(cross-referenced from Section 3.4)" for PaSa | §9.4 |
| Explicit cross-references | "(cross-referenced from Section 4.1)" for HierCat | §7.3 |
| Explicit cross-references | "(cross-referenced from Section 7.1)" for SurveyScope | §2.3 |
| Differentiated multi-section systems | SurveyG in §2.2 (architecture) vs. §3.3 (traversal) vs. §6.3 (attribution) | All distinct analytical angles |
| Differentiated multi-section systems | LitFM in §3.2 (graph retrieval) vs. §5.2 (re-ranking comparison) | Different contexts |
| Differentiated multi-section systems | SciSage in §2.3 (architecture) vs. §7.1 (benchmark release) | Different roles |
| Concise cross-reference pattern | "The Deep Search Agents Survey [arXiv:2508.05668] (cross-referenced from Section 8.4)" | Appears only once |

### What Is NOT Redundant (Exemplary Differentiation)

| System | Sections | Why Different |
|---|---|---|
| SurveyG | §2.2 (architecture), §3.3 (traversal method), §6.3 (attribution support), §10 (recommendation) | Architecture vs. traversal mechanism vs. attribution role vs. practitioner advice |
| LitFM | §3.2 (graph retriever mechanics), §5.2 (re-ranking comparison), §6.3 (non-obvious citation surfacing) | Retrieval model vs. re-ranking evaluation vs. attribution enhancement |
| PaSa | §3.4 (RL traversal description), §9.4 (trade-off cross-reference) | Primary description vs. targeted cross-reference |
| FActScore/SAFE/SelfCheckGPT | §6.4 (method descriptions), §9.1 (mitigation context) | Full method detail vs. application in pipeline discussion |
| PaperQA2 | §8.4 (primary description) | Appears only once |
| Self-RAG | §5.3 (primary description) | Appears only once |

---

## Summary of Redundancy

| # | Instance | Sections | Type | Severity |
|---|---|---|---|---|
| 1 | OpenScholar evaluative phrase | §5.4 / §8.4 | Near-verbatim claim | Minor |
| 2 | GLUE analogy | §9.2 / §10 | Same analogy with explanation | Minor |
| 3 | Coverage breadth definition | §2 / §9 | Verbatim definition | Minimal |
| 4 | SuperWriter introduction | §4.2 / §4.3 | Overlapping adjacent description | Minimal |
| 5 | HierCat dataset statistics | §4.1 / §7.3 | Near-identical statistics | Minor |

**No claims or findings appear three or more times.** All five instances are at the phrase or sentence level — no paragraph-level or section-level duplication. The draft uses ~9,600 words across 10 sections covering ~130 papers; the redundant content accounts for less than 1% of total text.

---

## Suggestions

1. **§8.4 (OpenScholar):** Replace "achieving citation accuracy on par with human experts" with a concise cross-reference to §5.4.
2. **§10 (GLUE):** Replace "analogous to GLUE (General Language Understanding Evaluation)" with "(see §9.2)."
3. **§9 (coverage breadth):** Replace the full definition with "(defined in §2)."
4. **§4.3 (SuperWriter):** Introduce as "SuperWriter (cross-referenced from §4.2)" instead of re-describing the system.
5. **§4.1 (HierCat):** Remove the dataset statistics and reference §7.3 for the full description.

---

## Weighted Contribution

**Score 4 × 15% = 0.60**
