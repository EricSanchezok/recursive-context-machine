# RankedPool Contract

`RankedPool` uses categorical signals, not a numeric total score.

For each retained paper, include:

- `paper_id`
- `title`
- `role` — the usual in-domain roles, or `cross_domain` for a transfer candidate
  surfaced by cross-domain mining.
- `topic_fit`: strong, medium, weak, or unknown.
- `influence_signal`: strong, medium, weak, or unknown.
- `diversity_gain`: strong, medium, weak, or unknown.
- `benchmark_signal`: strong, medium, weak, or unknown.
- `scope_risk`: high, medium, low, or unknown.
- `evidence_availability`: full_text, abstract_only, reference_only, or unknown.
- `citation_graph_role`: seed, cited_by_seed, neighbor, bridge, or boundary.
- `transfer_potential`: strong, medium, weak, or unknown — for `cross_domain`
  candidates only; how plausibly the paper's idea transfers to our anchor.
- `keep_reason`

End with:

- `core_set`
- `supporting_set`
- `boundary_set`
- `transfer_set` — the `cross_domain` candidates, ranked by `transfer_potential`.
  These are intentionally out-of-field: low `topic_fit` is expected and is NOT a
  reason to drop them, and they must not be mixed into the in-domain sets above.
- `missing_evidence`
