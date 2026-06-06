# Handoff: ExtendedDiscoveryMerger → CandidateScorer

| Field | Value |
|-------|-------|
| **run_dir** | `.` |
| **artifact** | `02b_candidate_pool_extended.md` |
| **status** | ok |
| **Total extended candidates** | **107** (merged across 4 extended scout artifacts) |
| **Existing pool** | `02_candidate_pool.md` (95 candidates) — all extended candidates are new additions not present in the main pool |
| **Source artifacts** | `02e_extended_method_candidates.md`, `02f_extended_benchmark_candidates.md`, `02g_extended_survey_candidates.md`, `02h_extended_frontier_candidates.md` |

---

## Candidate Counts by Relevance Score

| Relevance Score | Count | Description |
|-----------------|-------|-------------|
| **high** | 23 | Directly relevant to survey generation, citation graph traversal, or evaluation |
| **medium** | 62 | Indirectly relevant — component, method, adjacent domain, or transferable technique |
| **low** | 22 | Boundary — useful for discrimination tests, comparative analysis, or historical context |
| **Total** | **107** | |

## Candidate Counts by Role

| Role | Count | Examples |
|------|-------|---------|
| core_method | 4 | PaperQA2, MATC, CKMAs, AgentCo-op |
| mechanism | 29 | Paperfetcher, Cascading Citation Expansion, PaperSearchQA, Attribute First |
| benchmark | 16 | ALCE, CiteME, DRACO, REASONS, SciFact, LitSearch, ReFACT |
| metric | 16 | SAFE/LongFact, VeriFact, D-FActScore, LongDocFACTScore, VERISCORE |
| survey_reference | 11 | Deep Search Agents Survey, LLM Agent Survey, Related Work Meta-Study |
| citation_seed | 5 | Refcat, Paper Evolution Graph, Direction Aware Citation Analysis |
| related_system | 14 | LLMSurver, AISysRev, AiReview, WisPaper, CRUISE-Screening |
| boundary | 12 | Hallucination audits, UKP-ATHENA, Multi-Agent Sampling |

---

## Key Additions vs Main Pool

### Gaps from `01b_query_plan_extended_analysis.md` that are now filled:

| Gap | Filled By | Confidence |
|-----|-----------|------------|
| Pre-2020 snowballing/cocitation methodology | Cascading Citation Expansion (2018), Paperfetcher (2021), Interleaved Snowballing (2024) | **High** |
| Deep Research paradigm | Deep Search Agents Survey (2508.05668v3), PaperQA2 (2409.13740v2), DRACO (2602.11685) | **High** |
| Knowledge-graph-driven survey methods | CKMAs (2411.06159v3), KG-Based SLR (2208.02334v1), KG-EmpiRE (2405.08351v1) | **High** |
| RL-based citation traversal | PaperSearchQA (2601.18207), Sparse RL (2509.05874v1), Multi-hop RL (2205.15281v1) | **Medium** |
| Broader citation hallucination evidence | Hallucination audit (2605.07723), Cross-Model Audit (2603.03299), Attribution Crisis (2508.00838v1) | **High** |
| Non-branded evaluation benchmarks | ALCE (2305.14627v2), Auto-survey Challenge (2310.04480v2) | **High** |
| Citation attribution accuracy | CiteME (2407.12861v2), REASONS (2405.02228v3) | **High** |
| Human-in-the-loop SLR tools | LLMSurver, AISysRev, AiReview, AIDE, GPTscreenR | **Medium** |
| Long-form factuality for surveys | SAFE/LongFact (2403.18802v4), VeriFact (2505.09701v1), LongDocFACTScore (2309.12455v2) | **High** |
| Entity-ambiguity in citation evaluation | D-FActScore (2402.05629v4) | **High** |
| Verifiable vs unverifiable claims | VERISCORE (2406.19276v1) | **High** |
| Query decomposition strategies | Iterated Decomposition (2301.01751v2), Entity-Centric Refinement (2204.00743v2) | **Medium** |
| Citation graph infrastructure | Refcat (2110.06595v2) — 1.3B citation graph dataset | **Medium** |

### Remaining open gaps (not filled by extended search):

1. **Pre-2000 cocitation theory lineage** — Webster & Watson (2002), Wohlin (2014), Garfield (1970s) — unavailable on arXiv; would require Google Scholar/DBLP retrieval. Partially addressed by Cascading Citation Expansion (2018) which traces back to Garfield.
2. **Closed-source Deep Research evaluations** — OpenAI/Gemini/Perplexity evaluations are not academic publications. DRACO (Perplexity's own) partially addresses this.
3. **Human-evaluation protocol papers** — no standalone human evaluation protocol paper for survey generation exists. Auto-survey Challenge (2310.04480v2) provides a competition-based human peer-review paradigm.
4. **Cross-annotator agreement studies** — no paper systematically studies inter-annotator agreement for survey quality dimensions.
5. **Cost/throughput evaluation** — no benchmark measures computational or monetary cost of survey generation as part of evaluation.

---

## Representative High-Value Candidates

### Core survey generation methods:
1. **PaperQA2** (2409.13740v2) — Superhuman literature synthesis; key open-source competitor to closed Deep Research
2. **MATC** (2508.04306v1) — Multi-agent with compounding error correction; SOTA benchmarks
3. **CKMAs** (2411.06159v3) — Knowledge-minigraph-driven survey generation (KG gap filler)
4. **AgentCo-op** (2605.20425) — Retrieval-based workflow composition; 2026 frontier signal

### Retrieval & citation graph mechanisms:
5. **Paperfetcher** (2110.12490v3) — Bidirectional snowballing tool; directly applicable
6. **Cascading Citation Expansion** (1806.00089v1) — Foundational iterative expansion
7. **PaperSearchQA** (2601.18207) — RLVR-trained paper search; fills RL traversal gap
8. **Attribute First** (2403.17104v3) — Select-then-generate paradigm; mirrors ideal survey workflow

### Evaluation benchmarks:
9. **ALCE** (2305.14627v2) — First citation evaluation benchmark
10. **CiteME** (2407.12861v2) — Citation attribution accuracy evaluation
11. **DRACO** (2602.11685) — Cross-domain deep research evaluation
12. **REASONS** (2405.02228v3) — Sentence-level citation attribution dataset
13. **ReFACT** (2509.25868) — Scientific confabulation detection benchmark

### Evaluation metrics:
14. **SAFE/LongFact** (2403.18802v4) — Search-augmented F1 factuality evaluation
15. **D-FActScore** (2402.05629v4) — Entity-ambiguity-aware citation factuality
16. **VERISCORE** (2406.19276v1) — Verifiable vs unverifiable claim distinction
17. **LongDocFACTScore** (2309.12455v2) — Scientific long-document factuality

### Problem context (citation hallucination):
18. **LLM Hallucinations in the Wild** (2605.07723) — 111M reference audit, 147K hallucinated citations
19. **Cross-Model Citation Audit** (2603.03299) — 69K citation audit, multi-model consensus filter

---

## Risks

1. **Internal deduplication conservatism** — The merged count of 107 is an estimate across four independently-created scout artifacts. Some papers may still be duplicates where arXiv IDs differ but titles are near-identical, or where papers were captured under different titles. Title-based fuzzy matching was not performed; only exact arXiv ID and exact title matches were deduplicated.

2. **run_dir recovery from cwd** — The run_dir is `.` (current working directory). This was used consistently across all scout artifacts, but verify this is correct before downstream stages consume paths.

3. **Heavy overlap across extended scouts** — Papers like Paperfetcher, Cascading Citation Expansion, CiteME, DRACO, ChatCite, and PaperSearchQA were captured by 3+ scouts each. Provenance was concatenated, but downstream scoring should avoid over-weighting multi-capture as consensus signal (same queries are run by different scouts).

4. **General long-form factuality metrics ≠ survey-specific** — SAFE, VeriFact, FaStFACT, FACTORY, D-FActScore, VERISCORE, and LongDocFACTScore are not survey-generation-specific. Their F1 formulation, entity-ambiguity handling, and verifiability distinction are methodologically transferable but would require adaptation. Flag for CandidateScorer to assign lower confidence in their direct applicability.

5. **Many 2026 frontier entries are preprints with low citation counts** — Papers like AgentCo-op (2605.20425), PaperSearchQA (2601.18207), DRACO (2602.11685), and LLM Hallucinations (2605.07723) are very recent. Their impact and reproducibility are not yet established.

6. **DRACO may be proprietary-derived** — Uses Perplexity Deep Research usage data. Availability of tasks and rubrics is public, but the methodology may not be fully reproducible outside Perplexity.

7. **No formal ontology/schema papers** — Despite KG-focused queries (emc-02), no dedicated contribution ontology or survey schema paper was found beyond the ORKG approach (which is already in the main pool).
