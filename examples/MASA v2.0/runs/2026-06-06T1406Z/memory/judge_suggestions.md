## Critical (must fix)

1. **Fix AutoSurvey2 characterization (§2.3, line 53)**: Change "though it lacks the iterative refinement of other hybrid systems" to "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline." This is a direct factual contradiction with the pool entry and is flagged by three separate judge evaluations.

## Important (should fix)

2. **Add hedging to unverifiable quantitative claims**: 
   - §1: "identified approximately 147,000 hallucinated citations" → "reportedly identified ~147,000"
   - §9.1: "SciReviewGen reports 15–25% hallucination rates" → "SciReviewGen-based evaluations report 15–25%"
   - §9.5: "costs $10–50 in API fees" → "costs an estimated $10–50"
   - §6.2: "by 15–20% over abstract-only access" → "reportedly by 15–20%"
   - §7.2: "LLMs achieve 4.2–18.5% accuracy" → "LLMs reportedly achieve 4.2–18.5%"

3. **Add forward cross-reference from §3.4 to §9.4**: After the PaSa description, add "(see Section 9.4 for a discussion of traversal trade-offs)."

4. **Group §7.5 hallucination benchmarks** with transitional phrases (e.g., "Consistency-based methods... NLI-based benchmarks... Domain-specific benchmarks...").

5. **Add two missing columns to Table 1** (§2.4): "Number of Agents" and "Retrieval Integration" per outline specification.

## Nice-to-have

6. **Consolidate OpenScholar phrase** (§8.4): Replace "achieving citation accuracy on par with human experts" with cross-reference to §5.4.

7. **Trim GLUE analogy from §10**: Replace with "(as argued in §9.2)."

8. **Consolidate coverage breadth definition in §9**: Replace full definition with "(defined in §2)."

9. **Add SuperWriter cross-reference in §4.3**: Replace re-introduction with "(cross-referenced from §4.2)."

10. **Trim HierCat statistics in §4.1**: Replace with brief cross-reference to §7.3.

11. **Verify SciReviewGen 15–25% hallucination rate** against actual paper.

12. **Verify Table 1 technical details** for Agentic AutoSurvey (BFS, 2–3 hops, citation count + recency) and SurveyG (recency-weighted per layer) against original papers.

13. **Consider adding** STORM [2402.14207], Self-RAG [2310.11511], and PaperQA2 [2409.13740] to the main candidate pool.
