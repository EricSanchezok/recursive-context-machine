# Outline History

## Version 1 — 2026-06-03T1208Z

- **Status**: Initial outline
- **Agent**: Researcher
- **Run Dir**: examples/MASA/runs/2026-06-03T1208Z
- **Sections**: 7
- **Top 3 Section Titles**: (1) Introduction and Motivation, (2) Agent Architectures for Automated Survey Generation, (3) Retrieval-Augmented Pipelines for Scientific Survey Writing

### Changes from Previous
- Initial creation. No prior version.

### Risks
- Candidate pool and seed papers were simulated (no automated Phase 0 executed).
- arXiv ID accuracy relies on agent knowledge.
- Some sections may lack sufficient reference papers — expansion phase needed.

## Version 2 — 2026-06-05T0539Z

- **Status**: Updated outline (Round 1 refinement)
- **Agent**: Researcher
- **Run Dir**: examples/MASA/runs/2026-06-03T1208Z
- **Sections**: 7
- **Top 3 Section Titles**: (1) Introduction and Motivation, (2) Agent Architectures for Automated Survey Generation, (3) Retrieval-Augmented Pipelines for Scientific Survey Writing

### Changes from Previous
- Added ResearchAgent (2409.13737) to Section 2 reference papers and refinement guidelines (guideline 4).
- Added cross-reference from Section 6.2 (evaluation standardization) to Section 4.6 (evaluation challenges) per supervisor feedback.
- No structural changes (7 sections preserved).
- All 26 relevant candidate pool papers are now cited (6 adjacent surveys intentionally omitted).

### Risks
- Candidate pool and seed papers remain simulated (no automated Phase 0 executed).
- arXiv ID accuracy still relies on agent knowledge; no verification performed.
- No extended candidate pool exists for Round 2+ supplementation.

## Version 3 — 2026-06-05T0544Z

- **Status**: Updated outline (Round 1 supervisor feedback applied)
- **Agent**: Researcher
- **Run Dir**: examples/MASA/runs/2026-06-03T1208Z
- **Sections**: 7
- **Top 3 Section Titles**: (1) Introduction and Motivation, (2) Agent Architectures for Automated Survey Generation, (3) Retrieval-Augmented Pipelines for Scientific Survey Writing

### Changes from Previous
1. **Hybrid label** (Section 2 guideline 1): Added "hybrid" to architectural design space definition (single-agent vs. multi-agent vs. hybrid vs. hierarchical).
2. **STORM explicitly labeled as hybrid** (Section 2 guideline 2): Added explicit sentence — "explicitly label this simulated conversation as a **hybrid approach** (single-agent implementation that mimics multi-agent role specialization through prompt differentiation)."
3. **Forward reference Section 2→4** (Section 2 guideline 6): Added "(See Section 4 for how these trade-offs are evaluated.)" to trade-offs discussion.
4. **Forward reference Section 3.6→4.2** (Section 3 guideline 6): Added "(forward-reference Section 4.2 for evaluation metrics for citation faithfulness)."

### Risks
- Same as Version 2: simulated Phase 0, unverified arXiv IDs, no extended pool.
