# Supervisor Review Report — Round 2

**Review Date**: 2026-06-08  
**Run Directory**: `D:\RCM\examples\MASA v2.0\runs\2026-06-08T1624Z`  
**Round**: 2  
**Previous score**: 3.70 (Round 1, BELOW_THRESHOLD → CONTINUE)  

---

## Summary

The draft is structurally sound with strong analytical framing. The judge-identified issues from Round 1 remain unaddressed. This review adds new findings and upgrades several judge-identified issues to **CRITICAL** (Section 6 missing subsection, cross-domain coverage gap, unscoped universal claims, speculative Section 4.3). Evidence retrieval was performed for three focus sections.

---

## Issues

### 馃敶 CRITICAL (Must fix before STOP)

1. **Section 6 is under-developed (missing subsection 6.4)**. Outline specifies 4 subsections; draft has only 3 (6.4 merged into 6.3). Ablation culture and citation hallucination auditing each deserve dedicated focus. Restore Section 6.4 as a separate subsection.

2. **Cross-domain coverage gap — No PRISMA/systematic review methodology connection**. The draft critiques evaluation methodology without connecting to PRISMA guidelines, dual screening, risk-of-bias assessment, or evidence synthesis standards. Add 3–5 sentences in Section 5.2 or 5.3.

3. **Universal claim at L241 not scoped**: `"Every method paper in Phases 1–3 evaluates on custom topics with custom metrics and custom human rubrics."` — Scope should be explicitly "of the papers surveyed in this work."

4. **Section 4.3 ends speculatively**. The "practical concerns or disciplinary isolation" dichotomy is vague. Replace with three specific technical barriers: (a) GNN training requires large labeled citation graphs unavailable for niche topics; (b) learned representations are task-specific — retraining per survey topic is computationally prohibitive vs. zero-shot embedding search; (c) existing GNN methods are designed for recommendation/ranking, not generative organization. New paper [2605.14790] (GoR) provides evidence that learned graph representations for generation are possible but require domain-specific fine-tuning.

### 馃煛 MODERATE (Should fix)

5. **Redundancy of controlled-comparison finding (4× → 2×)**. "Single data point" claim appears at L49–51, L128, L137, L221 with near-identical phrasing. Section 3 intro should reference §3.4 rather than restating. Eliminate verbatim Phase 4 irony repetition at L47 and L273.

6. **"35 core method papers" count is unsourced** (L151, L242). Taxonomy lists ~46 core_method entries. Add scope qualifier or adjust count.

7. **Section 2 missing cross-phase comparison table**. Outline specifies a quantitative trend table. Data exists in text; a table would make metric-dispersion pattern concrete.

8. **Universal claim at L285**: `"No ASG paper reports inference cost, token usage, API calls, or runtime"` — Scope to "the papers surveyed in this work."

9. **"71% improvement" is derived but presented as reported** (L93). Rewrite as "a ~71% improvement over AutoSurvey's 4.77/10 baseline (computed from the two reported scores)."

### 馃數 NICE-TO-HAVE

10. **Add cost ballpark to Section 5.4 blind spot #4**. New paper [2604.22750] shows agentic tasks consume 1000× more tokens than non-agentic tasks — directly usable as a frame of reference.

11. **Add prioritization sentence to Section 7**. Identify most actionable blind spot (e.g., citation hallucination auditing via CiteGuard/VERISCORE).

12. **Add "Key Findings" callout box early in survey**. List key numbers (8.18/4.77, +32%, +27.2%, benchmark sizes) for reference throughout.

---

## Modification Instructions for Generator

**Priority order**:
1. Restore Section 6.4: Split current 6.3 into "6.3 Standardized Evaluation and Cost-Quality Reporting" and "6.4 Ablation Studies and Citation Hallucination Auditing"
2. Deepen Section 4.3: Replace speculative ending with 3 specific technical barriers (GNN data requirements, task-specific retraining cost, architecture mismatch). Cite [2605.14790] as evidence that learned graph representations for generation are possible but require domain-specific adaptation.
3. Add PRISMA paragraph (3–5 sentences) to Section 5.2 (after "custom evaluation is universal" paragraph) or 5.3
4. Scope universal claims at L241 and L285 to "the papers surveyed in this work"
5. Reduce redundancy — controlled-comparison finding 4× → 2×; eliminate Phase 4 irony repetition (L47 and L273)
6. Source/adjust "35 core method papers" — add scope qualifier or adjust to "the core method papers surveyed in this work"
7. Add cross-phase comparison table to Section 2 showing metric, benchmark, and evaluation type per phase
8. Attribute "71% improvement" as computed from the two reported scores
9. Add cost ballpark to Section 5.4 blind spot #4 using [2604.22750] evidence
10. Add prioritization sentence to Section 7
11. Add Key Findings callout box early in survey

**Do NOT**: Change narrative threads, add new sections beyond restoring 6.4, remove existing content structure, or alter citation format (arXiv IDs).
