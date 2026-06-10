# CandidatePool Contract

`CandidatePool` records discovery results without pretending to be final ranking.

Each paper row should include:

- `paper_id`: arXiv ID when available.
- `title`
- `year`
- `source_query`
- `source_agent`
- `likely_role`: foundation, method, benchmark, survey, application, critique, or boundary.
- `inclusion_reason`
- `matched_terms`
- `known_links`: PDF URL, arXiv URL, code URL, or unknown.

Deduplication should be title-first, then arXiv ID.

Ranking is deferred to `RankedPool`; this file should preserve discovery provenance.
