# ResearchMap Contract

`ResearchMap` is the main intermediate product.

Required sections:

- `taxonomy`: method families with definitions and representative papers.
- `paper_cards`: compact PaperCard entries for core papers.
- `benchmark_matrix`: datasets, metrics, settings, and comparable papers.
- `relation_graph`: method extends, contrasts, evaluates_on, assumes, and reports_limitation edges.
- `comparison_readiness`: ready and not-ready clusters with reasons.
- `gap_evidence`: limitations, contradictions, benchmark failures, and missing evaluations.
- `citation_roles`: foundation, method, benchmark, critique, and bridge papers.
- `cross_domain_transfer`: the transfer candidates (from the ranked pool's
  `transfer_set` and the expansion `patterns`). For each, the pattern it shares
  with our field and a one-line transfer hypothesis. Kept separate from the
  in-domain taxonomy — this is the field-adjacent inspiration, not core method.

Avoid final prose. This file should be structured enough for downstream judges.
