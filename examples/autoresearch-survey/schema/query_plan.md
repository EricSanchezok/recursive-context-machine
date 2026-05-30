# QueryPlan Contract

`QueryPlan` turns `SurveySpec` into a typed retrieval program.

Each query must include:

- `query_id`
- `query_type`: core_method, mechanism, problem, benchmark, survey, citation_seed, boundary, frontier, or cross_domain.
- `query`
- `target_dimension`
- `expected_gain`
- `negative_filter`
- `top_k`

The plan must include at least:

- 3 core or mechanism queries.
- 2 benchmark or evaluation queries.
- 1 survey or review query.
- 1 boundary query.
- 1 frontier query.

Queries should be short enough for arXiv search and broad enough for embedding retrieval.
