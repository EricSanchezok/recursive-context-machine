# Outline History

## Version 1 — Round 1 — 2026-06-06T12:54+08:00

### Summary
Initial outline for the survey topic "Automated literature survey generation using large language models: agent architectures, retrieval-augmented pipelines, and evaluation methodologies".

### Structure (9 Sections)
1. Introduction and Scope
2. Architectural Taxonomy of Automated Survey Generation (4 subsections)
3. Planning and Outline Generation Strategies (4 subsections)
4. Retrieval-Augmented Pipelines for Evidence Collection (4 subsections)
5. Citation Attribution and Factuality Mechanisms (4 subsections)
6. Evaluation Methodologies and Benchmarks (5 subsections)
7. Emerging Frontiers (4 subsections)
8. Open Challenges and Future Directions (6 subsections)
9. Conclusion

### Key Design Decisions
- **Section 2** follows an architectural split (single-agent → multi-agent → hybrid → comparative) to meet the quality bar of describing 3+ architectural approaches.
- **Section 5** separates citation attribution (mechanisms) from evaluation (benchmarks), keeping the "how to cite" distinct from "how to evaluate citation quality".
- **Section 6** is the largest section (5 subsections), reflecting the abundance of benchmarks, datasets, and metrics found in the candidate pool (11 benchmark + 10 evaluation papers).
- **Section 7** on frontiers was carved out as a separate section rather than folded into open challenges, because interactive/living/coordinated surveys represent operational paradigms rather than just limitations.
- Papers from the extended pool (STORM 2402.14207, OpenScholar 2411.14199, DimInd 2504.18496, ChatCite 2403.02574, Synergi 2308.07517, etc.) were integrated where they fill gaps in the main pool.

### Total Reference Papers Referenced
~130 unique arXiv IDs across all sections (drawing from both main pool and extended pool).

---

## Version 2 — Round 2 — 2026-06-06T13:05+08:00

### Summary
Updated outline incorporating feedback from Supervisor (Round 1) and LLM Judge (Round 2). Key changes address citation relevance, section balance, and refinement guideline precision.

### Changes from Version 1

**Section 3.1 — Replaced tangentially related citations:**
- Removed [1905.10039] (Plan-and-Write, story generation) and [1911.08836] (Template NLG) — both flagged by Judge as only tangentially connected to survey outline generation.
- Added [2104.08668] (Generating Related Work — direct precursor to survey section generation) and [2408.16444] (SurveySum — multi-document summarization into survey sections) as domain-relevant alternatives.

**Section 3.4 — Expanded refinement guideline:**
- Updated to target ~200 words (was ~95 words), with explicit guidance on discussing the SurveyBench correlation finding, per-dimension correlations, and human rating protocol.

**Section 4.3 — Added caveat for adaptive retrieval transfer:**
- Added note that adaptive retrieval methods have been validated primarily on QA/summarization tasks; transfer to multi-section survey generation remains unproven.

**Section 5.1 — Softened MIRAGE mechanism description:**
- Changed from "saliency maps from attention layers" to "saliency-based attribution methods" to avoid over-attributing the specific mechanism.

**Section 5.4 — Added method-grouping guidance:**
- Added transitional phrases to group methods (decomposition-based, search-augmented, claim-verification, entity-grounded, fine-tuning-based) to reduce list-like presentation.

**Section 6.4 — Added method-grouping guidance and corrected HALoGEN labels:**
- Added grouping guidance (decomposition-based, benchmark-based, sampling-based, meta-evaluation, NLI-based, domain-specific).
- Added explicit note to verify HALoGEN error labels (Type A=incorrect recollection, Type B=incorrect knowledge, Type C=fabrication).

**Section 6.5 — Specified κ statistic:**
- Added note to specify "Cohen's κ" (or whichever κ statistic is reported in the source) when reporting inter-annotator agreement.

**Section 7.1 — Cross-reference directive:**
- Added guidance to use a cross-reference to Section 2.3 for InteractiveSurvey re-description rather than repeating architectural detail.

**Section 7.3 — Added LiRA cross-reference:**
- Added directive to cross-reference LiRA (2510.05138) to improve citation balance.

**Section 7.4 — Redundancy fix:**
- Added note to keep "blur the boundary" or "convergence" claim to a single occurrence.

**Section 8 — Expanded guidelines for all 6 subsections:**
- Added target word count (~1,100–1,300 words total; each subsection ~180–250 words vs ~108 words previously).
- 8.1: Added guidance to avoid FActScore repetition from Sections 5.4/6.4 via cross-references.
- 8.2: Replaced MIMIC reference with GLUE; standardized "coverage breadth" terminology.
- 8.3: Added specific temporal reasoning gaps to discuss.
- 8.4: Added API cost discussion.
- 8.5: Added caution about unverified "non-textual richness" dimension in SurveyBench.
- 8.6: Added guidance to compare interaction patterns across systems.

**Section 9 — Added full GLUE name:**
- Added note to write "GLUE (General Language Understanding Evaluation)."

**Candidate Pool Updates (5 papers added):**
- Added OpenScholar (2411.14199), DimInd (2504.18496), ResearchAgent (2404.07738), DAHL (2411.09255), and ChatCite (2403.02574) from the extended pool to address Judge items #4 and #13 and Supervisor citation notes.

### Trigger Sources for Changes
- Judge Round 2 suggestions: Items #1 (HALoGEN labels), #4 (provenance), #5 (FActScore repetition), #6 (InteractiveSurvey cross-ref), #7 (expand Section 8), #8 (expand Section 3.4), #10 (replace tangential citations), #11 (LiRA cross-ref), #12 (soften MIRAGE), #13 (add to pool), #14 (condense Section 7.4).
- Supervisor Round 1 notes: Items #1 (anchor questions), #2 (MIMIC→GLUE), #3 (surveys verb), #4 (caveat), #5 (LitLLM disambiguation), #6 (κ statistic), #7 (GLUE full name), #8 (sentence split), #9 (transitional phrases), #10 (redundancy), #11 (terminology standardization), citation notes (DAHL, ResearchAgent).

### Structure
9 sections preserved (no structural changes).

### Total Reference Papers Referenced
~130 unique arXiv IDs across all sections (including new pool entries).

---

## Version 3 — Round 3 — 2026-06-06T13:19+08:00

### Summary
Updated outline addressing Judge Round 3 feedback (total 3.95 < 4.3 threshold) and Supervisor Round 3 remaining issues (M1-M3). Since this is Round 3+, section structure is preserved; only refinement guidelines and reference paper lists were updated.

### Changes from Version 2

**Paper Supplement (1 paper added to candidate pool):**
- Added STORM (2402.14207) from extended pool — resolves Supervisor M3 (STORM not in candidate pool)

**SurveyBench citation consolidation (Judge Critical #1, Supervisor M2):**
- Section 3.4: Removed arXiv:2510.03120 from reference list; added cross-reference to Section 6.1
- Section 8.2: Removed arXiv:2510.03120 from reference list; replaced SurveyBench comparison with SurGE/SurveyEval/SurveyLens + cross-reference to Section 6.1
- Section 6.1: Preservation of canonical description with consolidated citation instruction

**Section 8 expansion (Judge Critical #2):**
- 8.3 (~95w→~200w): Added concrete temporal-ordering failure example and temporal reasoning benchmark guidance
- 8.4 (~125w→~200w): Added social-science discourse structure expansion guidance with specific literature references
- 8.5 (~85w→~200w): Added multi-modal LLM references (GPT-4V, LLaVA); concrete content-type examples; clarified SurveyBench non-textual dimension handling
- 8.6 (~108w→~200w): Updated target word count

**LitLLM citation reduction (Judge Important #3):**
- Section 6.3: Removed arXiv ID (2402.01788) from LitLLM toolkit disambiguation; use name-only reference since Section 2.1 introduced it

**InteractiveSurvey citation reduction (Judge Important #4):**
- Section 7.1: Replaced explicit citation with cross-reference to Section 2.3; removed arXiv:2504.08762 from reference list
- Section 8.6: Replaced explicit citation with cross-reference; removed arXiv:2504.08762 from Section 8 reference list

**Section 4.2/4.4 expansion (Judge Important #5):**
- 4.2: Added comparative sentence guidance (LitFM graph-based vs LitLLM cross-encoder)
- 4.4: Added comparative sentence guidance (OpenScholar datastore-centric vs DimInd facet-based)

**Cross-reference additions:**
- Section 3.3→Section 5.2: Connect Self-Refine/EIPE-text to Generate-then-Refine (Judge Important #6, Supervisor M1)
- Section 2.4→STORM: Reflect seminal influence on hybrid systems (Judge Nice-to-have #8, Supervisor M3)
- Section 7.4→Section 7.3: Connect OpenScholar/ResearchAgent to coordination patterns (Judge Nice-to-have #7)

**MATC paradigm verification (Judge Nice-to-have #9):**
- Section 2.2: Added verification note for MATC's collaboration paradigm labels

### Structure
9 sections preserved (no structural changes).

### Total Reference Papers Referenced
~130 unique arXiv IDs across all sections (including new pool entry for STORM).
