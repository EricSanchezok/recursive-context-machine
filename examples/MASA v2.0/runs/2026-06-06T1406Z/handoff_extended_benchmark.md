# Handoff: ExtendedBenchmarkScout → DiscoveryMerger

| Field | Value |
|-------|-------|
| **run_dir** | `.` |
| **artifact** | `02f_extended_benchmark_candidates.md` |
| **status** | ok |
| **candidate count** | 12 new candidates (from 2 extended queries × 2 auxiliary sweeps = 40 raw hits) |
| **Existing pool overlap** | 11 of 40 raw hits were already in `02b_benchmark_candidates.md` (e.g., SurveyBench, SurGE, SGSimEval, SurveyLens, DeepSurvey-Bench, SurveyEval, CiteEval, FActScore, Core) |

## Summary

Extended benchmark search successfully broadened coverage in three areas the main BenchmarkScout under-represented:

1. **Citation evaluation beyond binary NLI** — ALCE, CiteME, VERISCORE, D-FActScore all address citation quality from different angles (fluency+correctness+citation, correct paper identification, verifiable vs unverifiable distinction, entity-ambiguity-aware)
2. **Long-form factuality evaluation methodology** — SAFE/LongFact (search-augmented F1), FACTS Grounding (multi-judge leaderboard), FACTOR (corpus transformation), VeriFact (precision+recall), FaStFACT (efficiency)
3. **Commercial deep research evaluation** — DRACO (Perplexity's cross-domain benchmark with citation quality dimension)
4. **Competition-based evaluation protocol** — Auto-survey Challenge (human peer-review paradigm)

## Representative IDs

- 2310.04480v2 (Auto-survey Challenge)
- 2305.14627v2 (ALCE)
- 2407.12861v2 (CiteME)
- 2403.18802v4 (SAFE / LongFact)
- 2602.11685 (DRACO)

## Risks

1. **Semantic overlap with main pool candidate** — ALCE and CiteME represent a different evaluation paradigm from CiteEval (already in pool). The DiscoveryMerger should preserve both, as they evaluate complementary citation dimensions.
2. **General long-form factuality ≠ survey-specific** — SAFE, VeriFact, FaStFACT, FACTORY, D-FActScore, VERISCORE, and LongDocFACTScore are not survey-generation-specific. Their F1 formulation, entity-ambiguity handling, and verifiability distinction are methodologically transferable but would require adaptation. Flag for CandidateScorer to assign lower confidence in their direct applicability.
3. **DRACO may be proprietary-derived** — uses Perplexity Deep Research usage data. Availability of tasks and rubrics is public, but the methodology may not be fully reproducible outside Perplexity.
4. **No human-evaluation-protocol papers found** — despite the ebm-01 query, no standalone human evaluation protocol paper for survey generation was discovered. This remains an open gap.
