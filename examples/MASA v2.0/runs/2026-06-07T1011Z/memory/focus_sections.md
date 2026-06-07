# Focus Sections — Round 3

## Selection Criteria Applied
These sections were identified as the weakest in terms of analytical depth after the Round 2 fixes. Each currently states a limitation but does not substantiate it with technical analysis, comparative evidence, or concrete examples.

---

## Focus Section 1: §4.2 — Graph-Enhanced Retrieval

**Issue**: The critique that "all graph traversal is single-hop BFS — no multi-hop reasoning" is stated but not analyzed. Missing: what specific barriers prevent multi-hop reasoning? The field needs to understand whether this is (a) a computational cost problem (multi-hop traversal is expensive at scale), (b) a relevance degradation problem (each hop introduces noise), (c) a data availability problem (no claim-level citation graph infrastructure exists), or (d) an evaluation problem (no benchmark measures multi-hop citation accuracy).

**Knowledge Gap**: Why is multi-hop citation reasoning hard to implement in current survey generation systems? What specific technical, infrastructural, and evaluation barriers exist?

**Search Keywords**: ["multi-hop citation reasoning challenges", "citation graph traversal limitations", "topological reasoning academic retrieval"]

**Retrieved Papers**:
  - arxiv_id: "2605.22878"
    title: "SciAtlas: A Large-Scale Knowledge Graph for Automated Scientific Research"
    relevance: "Explicitly identifies that current retrieval tools 'predominantly rely on superficial keyword matching or vector-space semantic retrieval, which lack the topological reasoning capabilities required to navigate complex logical connections.' Provides the KG infrastructure argument: multi-hop reasoning requires graph infrastructure that most ASG systems lack. Also introduces neuro-symbolic retrieval (tri-path collaborative recall + graph reranking) as an alternative paradigm."

**Generator Instructions**:
1. In §4.2.3 (Critical Assessment), replace the current single sentence "all graph traversal is single-hop BFS" with a structured analysis of why multi-hop reasoning is hard:
   - **Engineering barrier**: Multi-hop traversal at citation-graph scale (millions of papers) requires efficient path-finding algorithms; current systems use one-shot BFS for latency reasons
   - **Relevance degradation**: Each citation hop introduces noise — papers that cite relevant papers may be about different topics. Filtering relevant papers per hop is a research problem
   - **Infrastructural gap**: No claim-level citation graph exists — current citation graphs are paper-to-paper, not claim-to-claim. Multi-hop reasoning about claims requires claim-level provenance data
   - **Evaluation gap**: No benchmark measures multi-hop citation accuracy — current evaluation (citation F1, ROUGE) only measures surface citation attribution
   - Cite SciAtlas (2605.22878) for the "superficial keyword matching lacks topological reasoning" argument

---

## Focus Section 2: §6 — Critical Assessment

**Issue**: The critical assessment is strong on evaluation comparability and citation shallowness, but misses two systemic methodological weaknesses: (a) the **reproducibility gap** — none of the 12+ compared systems provide publicly available, reproducible code that generates surveys in a repeatable manner; (b) the **computational cost blindness** — no system reports standardized compute costs (GPU-hours, API calls per survey), making practical feasibility assessment impossible.

**Knowledge Gap**: Evidence that reproducibility is a systemic problem in automated survey generation, and evidence that reproducible literature synthesis is achievable in principle (making the gap in ASG more salient).

**Search Keywords**: ["reproducible automated literature survey pipeline", "computational cost survey generation", "open source code survey generation system"]

**Retrieved Papers**:
  - arxiv_id: "2508.04612"
    title: "A Reproducible, Scalable Pipeline for Synthesizing Autoregressive Model Literature"
    relevance: "First fully open-source, reproducible pipeline for automated literature synthesis. Achieves F1>0.85 for relevance classification and near-linear scalability (1000 papers, 8 CPU workers). Demonstrates that reproducibility is achievable — making the gap in ASG systems (none of which provide comparable infrastructure) a genuine methodological weakness rather than a field-wide inevitability."

**Generator Instructions**:
1. In §6.2 (Methodological Weaknesses), add a seventh weakness after "No measure of analytical depth":
   - **Reproducibility crisis**: None of the 12+ representative systems (AutoSurvey, SurveyX, ARISE, SciSage, Agentic AutoSurvey, SurveyForge, SurveyG, DOVA, OrchMAS, ResearchPilot, IterSurvey, InteractiveSurvey) provide publicly available code that generates reproducible surveys. Most systems evaluate on self-created datasets with unreleased code. The Reproducible Pipeline (2508.04612) demonstrates that reproducibility is achievable in literature synthesis (F1>0.85, near-linear scalability) — the absence of comparable infrastructure in ASG systems is therefore a choice, not a technical inevitability
   - **Computational cost blindness**: No system reports standardized compute costs (GPU-hours, API calls per survey, total inference time). This makes practical feasibility assessment impossible — a system that achieves marginally higher quality at 10x the cost may be worse in deployment terms

---

## Focus Section 3: §5 — Current Frontier (Quality Control Paradigm Comparison)

**Issue**: Three fundamentally different quality control paradigms exist across the surveyed phases: (a) **deliberation-first** (DOVA, §5) — agents discuss before retrieval to guide information gathering; (b) **rubric-guided iteration** (ARISE, §4.1) — evaluation scores against rubric dimensions, revision improves scores; (c) **reflect-when-you-write** (SciSage, §4.1) — real-time reflection during drafting. These are described in separate sections but never directly compared. A comparative analysis would deepen §5's critical assessment.

**Knowledge Gap**: Comparative analysis of quality control paradigms in automated survey generation — what are the trade-offs between deliberation-first, rubric-guided iterative, and reflect-when-you-write approaches?

**Search Keywords**: ["quality control automated survey generation", "deliberation rubric reflection comparison survey generation"]

**Retrieved Papers**: None — this gap is purely analytical and can be addressed by synthesizing existing content from §4.1 and §5.

**Generator Instructions**:
1. In §5.3 (Critical Assessment), add a paragraph after the DOVA/OrchMAS/ResearchPilot individual critiques:
   - **Comparing quality control paradigms**: The field has produced three distinct approaches to quality control. **Deliberation-first (DOVA)** prevents wasted retrieval by identifying information needs before search — but deliberation quality is unmeasured, and useful deliberation may be indistinguishable from surface-level conversation. **Rubric-guided iteration (ARISE)** provides a measurable quality signal (rubric scores) and guarantees convergence to quality targets — but the rubric is system-defined and the scores lack human calibration. **Reflect-when-you-write (SciSage)** prevents error accumulation by catching mistakes during drafting — but the feedback is generated by the same model doing the drafting, risking self-confirmation. None addresses the fundamental question: does quality control in the generation loop actually improve the final survey, measured against human expert judgment? This question is unfalsifiable without a unified evaluation protocol (see §7, Direction 1).

## Criteria
These three sections were selected because they each make a valid critical point but fail to substantiate it with enough technical depth:
- §4.2: States a limitation ("single-hop BFS") without analyzing its causes
- §6: Misses the reproducibility gap entirely, and computational cost blindness
- §5: Describes three quality control paradigms without comparing their trade-offs
