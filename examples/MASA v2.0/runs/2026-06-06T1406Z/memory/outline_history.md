# Outline History

## Version 1 — 2026-06-06T14:29+08:00

**Round**: 1
**Run**: 2026-06-06T1406Z
**Topic**: Automated Literature Survey Agents with Citation Graph Expansion

### Changes from previous version
- Initial outline for new run.

### Structure
| Section | Title | Subsections |
|---------|-------|-------------|
| 1 | Introduction and Scope | — |
| 2 | Architectural Taxonomy | 4 (Single-Agent, Multi-Agent, Hybrid, Comparison) |
| 3 | Citation Graph Expansion Strategies | 4 (Classical, Graph-Based Retrieval, Hierarchical, Agent-Driven RL) |
| 4 | Planning and Outline Generation | 4 (Hierarchical, Adaptive, Refinement, Quality) |
| 5 | Retrieval-Augmented Pipelines | 4 (Query Formulation, Evidence Extraction, Adaptive, Multi-Source) |
| 6 | Citation Attribution and Factuality | 4 (Sentence-Level, Capacity, Graph Expansion, Factuality) |
| 7 | Evaluation Methodologies | 6 (Benchmarks, Citation-Specific, Datasets, Metrics, Hallucination, Human Eval) |
| 8 | Emerging Frontiers | 4 (Interactive, Living, Coordination, Deep Research) |
| 9 | Open Challenges | 6 (Hallucination, Eval Standardization, Freshness, Graph Trade-offs, Domain, Multi-modal) |
| 10 | Conclusion | — |

**Total reference papers**: ~120 unique across all sections
**Total sections**: 10
**Total subsections**: 36

---

## Version 2 — 2026-06-06T14:42+08:00

**Round**: 2
**Run**: 2026-06-06T1406Z
**Topic**: Automated Literature Survey Agents with Citation Graph Expansion

### Changes from Version 1

**Motivation**: Address supervisor (C1, C2, M1–M4, N1–N3) and judge (items 1–16) feedback from Round 1.

### Section-level changes:
1. **Section 1**: Added arXiv:2605.07723 (citation hallucination audit) as problem-context data point. Added arXiv:2503.21460 (LLM Agent Survey) as broader agent-context framing. Explicitly prohibited citing [arXiv:2002.06961] and [arXiv:2306.14905] in exclusion sentences. Added survey spec quality bar to Writing Requirements.

2. **Section 2.1**: Explicitly stated AutoSurvey2 belongs in Section 2.3 only (fixes C1).

3. **Section 2.3**: Added 2–3 sentence automation-vs-user-control spectrum analysis (InteractiveSurvey → IterSurvey → STORM). Added SurveyScope cross-reference to Section 7.1.

4. **Section 2.4**: Added "evaluation scores" column to the comparison table axes.

5. **Section 4.4**: Added community detection (Louvain, spectral clustering) for unsupervised section heading generation. Added SurveyBench quantitative finding about outline-quality correlation.

6. **Section 6.2**: Strengthened with Generate-then-Refine empirical analysis details.

7. **Section 6.3**: Added concrete attribution example (method X SOTA claim with corroborating papers B, C).

8. **Section 7.1**: Added SurveyScope benchmark (from SciSage).

9. **Section 7.3**: Added cross-reference to Section 4.1 for HierCat.

10. **Section 8**: Added minimum 150-word requirement per subsection. Expanded 8.3 with coordination-graph-partitioning mechanisms (AgensFlow, KABB, Federation of Agents). Expanded 8.4 with PaperQA2 convergence paragraph.

11. **Section 9.1**: Added arXiv:2605.07723 citation hallucination audit reference.

12. **Section 9.2**: Added explicit "coverage breadth" definition.

13. **Section 9.4**: Added PaSa cross-reference from Section 3.4.

14. **Section 9.6**: Added PaperArena and Deep Search Agents Survey citations. Added reference to multi-modal parsing capabilities.

15. **Cross-references added**: HierCat (§4.1↔§7.3), LiRA (§2.2↔§8.3), PaSa (§3.4↔§9.4), AutoSurvey2 (removed from §2.1, kept in §2.3).

### Paper supplement:
- Added arXiv:2503.21460 (LLM Agent Survey) to main pool from extended pool
- Added arXiv:2508.05668 (Deep Search Agents Survey) to main pool from extended pool

### Structure
Same 10-section / 36-subsection structure (no sections added or removed).

**Total reference papers**: ~130 unique across all sections
