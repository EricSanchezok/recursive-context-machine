# Multi-Benchmark Evaluation Summary

**Survey**: D:\RCM\examples\MASA v2.0\runs\2026-06-07T1011Z\07_survey.md
**Run directory**: D:\RCM\examples\MASA v2.0\runs\2026-06-07T1011Z
**Judge model**: Cross-Judge (gpt-5.5 via gmncode.com)
**Date**: 2026-06-07 13:38 UTC

## Score Summary

| Benchmark | Score | Time (min) | Report |
|-----------|:-----:|:----------:|--------|
| Cross-SurveyBench | 4.80 | 1 | [link](survey_bench_cross.md) |
| Cross-DeepSurvey-Bench | 4.20 | 0.9 | [link](deepsurvey_bench_cross.md) |
| Cross-SurveyLens | 4.00 | 0.9 | [link](survey_lens_cross.md) |
| Cross-SurveyEval | 3.70 | 0.8 | [link](survey_eval_cross.md) |
| Cross-SGSimEval | 4.00 | 0.6 | [link](sgsim_eval_cross.md) |
| Cross-SurveyScope | 3.50 | 0.9 | [link](survey_scope_cross.md) |

---

## Cross-Judge vs. Self-Judge Comparison

| Benchmark | Self-Judge (DeepSeek) | Cross-Judge | Difference |
|-----------|:--------------------:|:-----------:|:----------:|
| Cross-SurveyBench | 4.90 | 4.80 | -0.10 |
| Cross-DeepSurvey-Bench | 5.00 | 4.20 | -0.80 |
| Cross-SurveyLens | 5.00 | 4.00 | -1.00 |
| Cross-SurveyEval | 5.00 | 3.70 | -1.30 |
| Cross-SGSimEval | 5.00 | 4.00 | -1.00 |
| Cross-SurveyScope | 5.00 | 3.50 | -1.50 |

**Interpretation**: Positive difference = cross-judge rated higher than self-judge,
Negative = self-judge rated higher. Large gaps (>0.5) suggest self-enhancement bias.

---

### Notes

- Dims vary per benchmark (see individual reports)
- All scores out of 5.0