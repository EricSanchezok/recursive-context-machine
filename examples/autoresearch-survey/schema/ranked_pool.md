# RankedPool Contract

`RankedPool` uses categorical signals, not a numeric total score.

For each retained paper, include:

- `paper_id`
- `title`
- `role`
- `topic_fit`: strong, medium, weak, or unknown.
- `influence_signal`: strong, medium, weak, or unknown.
- `diversity_gain`: strong, medium, weak, or unknown.
- `benchmark_signal`: strong, medium, weak, or unknown.
- `scope_risk`: high, medium, low, or unknown.
- `evidence_availability`: full_text, abstract_only, reference_only, or unknown.
- `citation_graph_role`: seed, cited_by_seed, neighbor, bridge, or boundary.
- `keep_reason`

End with:

- `core_set`
- `supporting_set`
- `boundary_set`
- `missing_evidence`
