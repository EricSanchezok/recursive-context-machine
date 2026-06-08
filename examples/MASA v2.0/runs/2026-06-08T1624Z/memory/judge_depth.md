# Analysis Depth Judge — Round 2

## Overview

Evaluating the analytical depth of each core section (Sections 2–7) in `05_draft.md` against the 1–5 rubric. The draft remains unchanged from Round 1 (fixes not yet applied), so this evaluation reflects the current state.

---

## Per-Section Scores

| Section | Score (1–5) | Key Strength | Key Weakness |
|---------|-------------|--------------|--------------|
| Section 2: Evolution of ASG (Phases 0–4) | 4.0 | Cross-phase metric dispersion pattern identified; each phase has a critical transition note showing the limitation-to-advancement arc; Phase 2's uncontrolled complexity and Phase 4's irony are genuine analytical contributions | Phase 0 (2.1) is below 150 words and reads as annotated bibliography — one sentence per paper with no grouping or comparison; no cross-phase quantitative comparison table as the outline specified |
| Section 3: Core Architectures (3.1–3.4) | 4.5 | Comparison tables across multiple dimensions in all three paradigm subsections (5×7, 6×7, 2×6); mechanism-level detail on coordination patterns, error handling, and iteration strategies; Controlled Comparison Gap (3.4) is a model systematic audit — 4-point "what this cannot tell us" analysis with external corroboration [arXiv:2510.04311, arXiv:2505.18286] | No cross-cutting synthesis across the three paradigms (are they converging? diverging?); cost opacity identified as a gap but not quantified or analyzed with the same depth as other dimensions |
| Section 4: Graph Awareness (4.1–4.3) | 4.0 | Strong bolt-on vs. backbone spectrum framing; deep mechanism analysis of SurveyG's three-layer hierarchy, dual traversal modes, and graph-to-outline mapping (the only algorithmic-level system explanation in the survey); critical assessment of missing ablation studies; missed opportunity table (4.3) identifying a genuine cross-community gap | Section 4.3 ends speculatively ("practical concerns or disciplinary isolation") without specific technical barrier analysis; no isolated graph evaluation exists for any system — correctly noted as a field-level problem, but the section could recommend what an ablation study would look like |
| Section 5: Critical Assessment (5.1–5.4) | 4.5 | Claim-vs-evidence audit (5.1) with specific "what each claim would need" methodology column — this is Level 5 synthesis; five cross-cutting methodological weaknesses (5.2) with concrete paper examples; comprehensive benchmark dispersion table (5.3) showing all 9 systems × 9 metrics with "Comparable To" column; five blind spots (5.4) cataloged with specific tool references and consequences | Lacks PRISMA/systematic review methodology connection (identified in supervisor_notes.md); cost ballpark not yet added to blind spot #4; some redundancy between 5.2 and 5.3 (e.g., custom evaluation point appears in both) |
| Section 6: Future Directions (6.1–6.3) | 3.5 | Each direction specifies what/why/how-to-evaluate; grounded in specific gaps from earlier sections (6.1 → Section 4, 6.2 → Section 3.3, 6.3 → Section 5.3/5.4); avoids vague calls for "better evaluation" | Directions are brief/compressed (~200 words each); Section 6.4 is merged into 6.3, diluting both the ablation-culture and hallucination-auditing proposals; no trade-off analysis across directions; no comparison tables; no prioritization framework |
| Section 7: Conclusion | 3.0 | Succinctly restates four research questions and threads; effective closing call to action; aligns with narrative arc | Functions as a summary rather than a synthesis — restates findings without prioritizing which gap or direction is most consequential or most actionable; no cross-thread insight beyond earlier sections |

---

## Overall Depth Score: **3.9** (average across 6 core sections)

**Change from Round 1:** +0.7 (3.2 → 3.9). This increase does not reflect changes to the draft (none have been applied yet) but rather a more granular scoring that recognizes genuine analytical depth in Sections 3 and 5 that was underweighted in Round 1. The draft was already deeper than the Round 1 score of 3.2 suggested.

---

## Depth Issues

### Critical (score 1–2)
None — every core section scores at least 3.0. This is a strong analytical draft.

### Moderate (score 3)
- **Section 7 (Conclusion): Score 3.0** — The conclusion summarizes findings without synthesizing them. It tells the reader what each section found but does not weigh which finding is most consequential, which blind spot is most tractable, or which direction should be prioritized. A Level 4 conclusion would identify the single most actionable finding — citation hallucination auditing has the most concrete available tools (CiteGuard, VERISCORE) and the clearest path to community adoption — and explain why it should be prioritized over other gaps.

- **Section 6 (Future Directions): Score 3.5** — The three existing subsections (6.1–6.3) are well-grounded but compressed. Missing Section 6.4 reduces both coverage and analytical emphasis on ablation science. No prioritization or trade-off reasoning across directions. The section reads as three separate proposals rather than an integrated research agenda. Approaches Level 4 with: (a) restoration of 6.4, (b) one sentence per direction comparing feasibility vs. impact, (c) explicit cross-referencing (e.g., "6.1 directly addresses the gap identified in 4.2's missing ablation study").

### Notable (score 4.5, near ceiling)
- **Section 3** and **Section 5** are the strongest analytical sections in the draft. Both approach Level 5 (Comprehensive Synthesis) but fall short on specific missing elements.

---

## Recommendations for Depth Improvement

### 1. Section 7: Conclusion → Target 4.0
Replace the final paragraph's closing sentence with a prioritization sentence that identifies which blind spot or direction has the most concrete path to resolution. The citation hallucination blind spot (Section 5.4, #1) is uniquely tractable because CiteGuard [arXiv:2510.17853] and VERISCORE [arXiv:2406.19276] already exist and could be applied to existing ASG outputs with zero architectural changes. Example: *"Among these gaps, citation hallucination auditing stands out as the most immediately actionable: CiteGuard and VERISCORE are available today, and applying them to existing ASG outputs would provide the field's first systematic baseline for a failure mode that renders surveys useless regardless of their coherence."*

### 2. Section 6: Future Directions → Target 4.0+
- **Restore Section 6.4** as a standalone subsection ("Ablation Studies and Citation Hallucination Auditing") as the outline intended. This gives the ablation-culture proposal and the hallucination-auditing proposal each their own success criteria. Use the new paper [arXiv:2605.14790] (Graphs of Research) as evidence for how graph-aware organization could be ablated against prompt-based organization.
- **Add feasibility-vs.-impact comparison**: One sentence per direction ranking it on these two axes. Example: *"Among these directions, 6.3 (standardized evaluation) is the most immediately actionable because it requires no new architecture — only community agreement on reporting conventions. Direction 6.1 (graph integration) is the most architecturally ambitious but faces the highest barrier to entry: labeled citation graphs are unavailable for niche topics, and learned representations are task-specific."*

### 3. Section 4.3: Missed Opportunity → Target 4.5
Replace the speculative closing sentence ("practical concerns or disciplinary isolation") with three specific technical barriers:
- **Data requirement**: GNN training needs large labeled citation graphs (thousands of papers with known relevance judgments), which are unavailable for most survey topics. Zero-shot embedding search works on any topic with no training data.
- **Computational cost**: Learned representations are task-specific — retraining for each new survey topic is computationally prohibitive compared to the single forward pass of embedding similarity.
- **Representation alignment**: Existing GNN methods (Temporal GNN, H2CGL) are designed for recommendation and impact prediction, not for the generative organization task that ASG requires. The output of a GNN (paper ranking) does not directly map to a survey outline.
- **Bridge**: Reference Graphs of Research [arXiv:2605.14790] as a potential middle ground — a citation evolution DAG that provides structural awareness without full GNN training.

### 4. Section 2: Evolution → Target 4.5
Add a cross-phase comparison table as the outline specified. The data is already in the text; a table would make the metric-dispersion pattern visually concrete.

| System | Phase | Graph Awareness | Evaluation Metric | Reported Score | Controlled Comparison? |
|--------|-------|----------------|-------------------|----------------|----------------------|
| AutoSurvey | 1 | None | Custom quality (1–10) | 4.77/10 | N/A (baseline) |
| SurveyX | 1 | None | Not directly reported | — | No |
| SurveyGen | 1 | None | Quality est. accuracy | — | No |
| SurveyForge | 1 | bfs chaining | Improved coverage | — | No |
| Agentic AutoSurvey | 2 | None | Custom quality (1–10) | 8.18/10 | Yes (vs. AutoSurvey) |
| SciSage | 2 | bfs chaining | SurveyScope Citation F1 | +32% | No |
| SurveyG | 3 | Hierarchical 3-layer | Custom qualitative | — | No |
| SurveyGen-I | 3 | None | Custom quality estimation | — | No |

This table visually demonstrates that every reported score is on a different metric, making the comparison crisis immediately obvious.

### 5. Section 5: Critical Assessment → Target 5.0
Two additions would push this to Level 5:
- **PRISMA connection (5.2 or 5.3)**: Add 3–4 sentences connecting ASG evaluation quality criteria to established systematic review methodology: PRISMA guidelines, dual screening, risk-of-bias assessment, PICOS framework. This grounds the field's evaluation crisis in a mature scholarly tradition and strengthens the cross-domain positioning. Example: *"The evaluation crisis documented here echoes challenges that systematic review methodology solved decades ago. PRISMA guidelines [Page et al., 2021] mandate dual screening with inter-screener agreement, explicit inclusion/exclusion criteria, and risk-of-bias assessment — practices that no ASG evaluation currently adopts. Adapting these practices to automated survey evaluation would immediately address the unreproducible human evaluation weakness (5.2 point #3) and provide a framework for comparing systems with different architectural assumptions."*
- **Cost ballpark (5.4, blind spot #4)**: Add one sentence quantifying the cost multiplier using [arXiv:2604.22750]. Example: *"A preliminary estimate using [arXiv:2604.22750]'s token-cost analysis suggests a 4-agent system with 2 iteration rounds consumes 8–12× more API tokens than a single-agent pipeline of comparable output length, yet no ASG paper reports whether this cost translates to proportional quality gains."*

### 6. Section 3: Architectures → Target 5.0
Add a brief cross-cutting paragraph at the end of Section 3 (before 3.4 or integrated into the section intro) that identifies the common thread across all three architectural paradigms. Example: *"Across all three paradigms — single-agent, multi-agent, and iterative — a common limitation persists: none of them uses the citation graph structure to determine both what to retrieve and how to organize the output. The richest multi-agent coordination pattern and the most sophisticated iteration loop both operate on a paper pool assembled by embedding similarity, with outline structure determined by an LLM prompt rather than by the research landscape's intrinsic topology. This shared blind spot provides the organizing contrast for Section 4's analysis of graph awareness."* This ties Section 3's architectural analysis directly to Section 4's graph-awareness spectrum, creating the cross-cutting synthesis that distinguishes Level 5 from Level 4.

---

## Summary: Depth Improvement Path

| Section | Current Score | Target | Key Action |
|---------|-------------|--------|------------|
| Section 2: Evolution | 4.0 | 4.5 | Add cross-phase comparison table |
| Section 3: Architectures | 4.5 | 5.0 | Add cross-paradigm synthesis paragraph tying to Section 4 |
| Section 4: Graph Awareness | 4.0 | 4.5 | Replace speculative 4.3 ending with 3 specific technical barriers |
| Section 5: Critical Assessment | 4.5 | 5.0 | Add PRISMA paragraph + cost ballpark |
| Section 6: Future Directions | 3.5 | 4.0–4.5 | Restore 6.4 + add feasibility-vs.-impact comparison |
| Section 7: Conclusion | 3.0 | 4.0 | Add prioritization of most actionable blind spot |
| **Overall** | **3.9** | **4.4–4.5** | — |

The draft's analytical foundation is already strong — no section falls below 3.0, and Sections 3 and 5 approach the ceiling. The remaining depth gap is concentrated in Sections 6 and 7 (under-developed) and in the absence of cross-cutting synthesis that would connect analytical threads across sections rather than keeping them in silos.
