# Expansion Contract

`ExpansionReport` records two expansion routes:

- citation graph expansion from seed PDFs and references.
- semantic neighbor expansion using arXiv embedding search.

Required sections:

- `seed_papers`: selected papers and why they were selected.
- `citation_edges`: source paper to referenced paper or title.
- `resolved_references`: references resolved back to arXiv candidates.
- `semantic_neighbors`: arXiv embedding-search neighbors by query.
- `new_candidates`: deduplicated additions.
- `drift_risks`: papers or clusters likely outside scope.
- `next_expansion_queries`: targeted follow-up queries.

Citation links may be partial. Mark unknowns explicitly instead of fabricating IDs.
