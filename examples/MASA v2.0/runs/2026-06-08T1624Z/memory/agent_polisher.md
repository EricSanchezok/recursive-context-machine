# Agent: Polisher — Round 1

## Round 2 — Skipped (No Work Needed)

**Date**: 2026-06-08
**Status**: Skipped — `07_survey.md` is not stale relative to `05_draft.md`.

- `05_draft.md` modified: 2026-06-08 09:08:47 UTC
- `07_survey.md` modified: 2026-06-08 09:25:18 UTC
- `06_review.md` modified: 2026-06-08 09:32:33 UTC

The current `07_survey.md` is newer than the source draft, already contains the same body content (with citation format differences) plus 4 Mermaid figure blocks and a References section. No re-polishing or figure re-generation is needed.

The supervisor review (`06_review.md`) identifies content-level issues (missing subsection 6.4, PRISMA gap, scoping universal claims, deepening Section 4.3) that fall under the **Generator's** responsibility per the MASA workflow. The Polisher role is limited to language polish and figure generation, and both are already complete on a non-stale `07_survey.md`.

## Language Polishing Actions

**Date**: 2026-06-08
**Draft**: 05_draft.md → 07_survey.md
**Review consulted**: 06_review.md (for awareness only; no structural changes applied as per Polisher role)

### Summary of Changes

Below is a summary of the language polish applied. No content, citations, taxonomy, structure, or technical claims were altered.

**Section 1 (Introduction)**:
- Changed "the most important claim" → "the central claim" (more precise wording)
- Changed "insight/novelty evaluation" → "insight and novelty evaluation" (parallel structure)

**Section 2 (Evolution)**:
- Section 2 intro: Changed "different evaluation metrics on different datasets with different human rubrics" → "different metrics, datasets, and human rubrics" (parallel structure)
- Section 2 intro: "making cross-phase comparison of quality impossible" → "rendering cross-phase comparison of quality impossible" (stronger verb)
- Section 2.1: "a pre-existing infrastructure ... had already been developed" → "an infrastructure ... was already in place" (removed redundancy)
- Section 2.1: Changed "forward/backward citation expansion" → "forward and backward citation expansion" (clarity)
- Section 2.2: "A single LLM handles all cognitive stages — planning the survey outline... and refining post-hoc" → "every cognitive stage — planning the outline... and refining the result post-hoc" (minor clarity)
- Section 2.2: Standardized "forward/backward" → "forward and backward" and "bfs" → "BFS" throughout
- Section 2.3: Changed "4 specialized agents" → "four specialized agents" and "5 specialized" → "five specialized" (consistency with spelled-out numbers)
- Section 2.3: "8.18/10.4.77 comparison" → "8.18 vs. 4.77 comparison" (clarity)
- Section 2.4: Standardized "Foundation/Development/Frontier" → "Foundation, Development, Frontier" (readability)
- Section 2.5: "7 subjects" → "seven subjects", "1,000 human surveys across 10 disciplines" → consistent around spelled numbers

**Section 3 (Core Architectures)**:
- Section 3 intro: "most important claim" → "most consequential claim" (stronger, more specific)
- Section 3.1 table: Normalized "bfs" → "BFS" in table cells
- Section 3.1: "lack the depth and accuracy of human-written reviews" → "human-written surveys" (terminology consistency)
- Section 3.1: "before retrieving and generation" → "before generation begins" (minor)
- Section 3.3: "sub-topics" → "subtopics" (spelling)
- Section 3.4: "But ... carries an outsized evidentiary burden that it cannot sustain alone" → "Yet ... carries an evidentiary burden that no single data point can sustain" (stronger, more precise)
- Section 3.4: "or whether there is a saturation point" → "or whether a saturation point exists" (more active)

**Section 4 (Graph Awareness)**:
- Section 4.1: Standardized all "forward/backward" → "forward and backward" and "bfs" → "BFS"
- Section 4.1: Split long SciSage sentence into two for readability
- Section 4.1: "finding papers that cite a known relevant paper" — rephrased to avoid repetition
- Section 4.2: "highly-cited" → "highly cited" (removed unnecessary hyphen in predicative position)
- Section 4.2: "sub-topics" → "subtopics"

**Section 5 (Critical Assessment)**:
- Section 5 intro: Added section reference formatting for clarity
- Section 5.1 table: Standardized formatting
- Section 5.1: "What each claim would need:" → "What each claim would require:" (more formal)
- Section 5.3: Standardized formatting, split long paragraph for readability
- Section 5.3: "10 topics" → "10 topics" (consistent)
- Section 5.4: "NLP/ML" → "NLP and ML" (readability in prose)

**Section 6 (Future Directions)**:
- Section 6.1: "bfs" → "BFS"
- Section 6.3: No significant changes (well-written)

**Section 7 (Conclusion)**:
- Minor: "multi-agent outperforms" → clearer phrasing in context
- No substantive changes

**Global / formatting changes**:
- Standardized all instances of "BFS" (upper case) for breadth-first search
- Standardized "forward and backward" (and) over "forward/backward" (slash) in prose
- Applied consistent em-dash spacing (space en-space space)
- Made minor parallel-structure improvements in lists
- Verified consistent hyphenation of compound modifiers

## Generated Figures

Four Mermaid diagrams were generated and appended to `07_survey.md`:

1. **Figure 1 — Timeline (Gantt chart)**: Five-phase evolution of ASG from 2012–2026, showing overlapping phases and key systems per phase.
2. **Figure 2 — Taxonomy (directed graph)**: Architecture spectrum from single-agent pipelines through multi-agent, graph-aware, and iterative refinement, with specific systems under each paradigm.
3. **Figure 3 — Benchmark Landscape (quadrant chart)**: Evaluation benchmarks mapped by comprehensiveness (narrow→broad) vs. adoption (low→high), illustrating the empty top-right quadrant.
4. **Figure 4 — Graph Awareness Spectrum (horizontal flowchart)**: Continuum from no graph awareness → BFS chaining as retrieval bolt-on → hierarchical graph as structural backbone.

All diagram content is grounded in the actual survey content. No invented categories or systems.
