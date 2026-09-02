# JudgePanel Contract

Judges should be independent and evidence-oriented.

Each judge returns:

- `verdict`: strong, acceptable, insufficient, or blocked.
- `evidence`: concrete observations from the current artifacts.
- `risks`: missing coverage, overclaim, benchmark mismatch, or speculative gaps.
- `required_fixes`: specific actions before final writing.
- `suggested_queries`: targeted retrieval queries.

The synthesizer returns:

- `overall_verdict`
- `what_is_ready`
- `what_must_be_caveated`
- `what_to_expand_next`
- `allowed_final_claims`
- `forbidden_overclaims`
