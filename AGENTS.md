# AGENTS.md

RCM (Recursive Context Machine) is a Rust workspace for building and executing
composable context-machine pipelines for LLM agents.

## Repository governance

This repository's governance layer is managed by repo-seed; product code remains
repository-owned. `.repo-seed/manifest.json` records managed files and capability
state.

- Use `repo-review` for change and pull-request review.
- Use `repo-decisions` for durable choices with meaningful alternatives.
- Use `repo-governance` when complexity, ownership, security, release, or incident
  signals change.
- Use the global `repo-seed` skill for governance seeding and upgrades.

## Repository layout

See [docs/architecture.md](docs/architecture.md) for the module map and seams.
Rust crates live in `crates/`; the Python SDK lives in `sdks/python/`; runnable
RCM projects live in `examples/`; the protobuf contract is `proto/rcm.proto`.

## Commands

- Test: `cargo nextest run --workspace --locked`
- Documentation tests: `cargo test --workspace --doc --locked`
- Lint: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --tests --locked -- -D warnings`
- Gates: `node scripts/run-gates.mjs`

Run the relevant tests and gates before every commit. `protoc` and the stable Rust
toolchain are required for the full workspace checks.

## Development rules

- Develop on `dev` or a short-lived feature branch based on `dev`.
- `main` is protected and only receives release promotion pull requests.
- Comments explain why and provenance, not what the next line already says.
- Use descriptive names; single-letter variables and generic parameters are not
  allowed except loop indices `i`/`j` and the documented closure exception.
- Use `tracing` for operational logging; failed outcomes log at `warn!` with
  structured error fields.
- Keep `reactor` internal to `machine`; provider presets do not belong in `machine`.
- Use module-level timeout constants for every LLM and tool execution.
- `Environment::new` is an honest host snapshot; use `Environment::empty` for
  explicitly sandboxed scenarios.

## Change contracts

- Risk-boundary changes start with an Approved spec in [docs/specs/](docs/specs/).
- Durable alternatives belong in [docs/decisions/](docs/decisions/), not changelogs.
- A subtle or systemic escaped failure earns a postmortem linked to a permanent
  guardrail in [docs/postmortems/](docs/postmortems/).
- Changes to `proto/rcm.proto` regenerate the Python SDK and update server tests in
  the same change.

## Security and scope

- Never read `.env` files or commit credentials.
- Never modify files outside the requested scope.
- Never force-push or directly push `main`.
- Never commit or push unless the user explicitly asks.

## Documentation and skills

Follow [docs/AGENTS.md](docs/AGENTS.md) for document placement and hygiene, and
[docs/testing.md](docs/testing.md) for risk-adjusted evidence. Seeded governance
files are upgraded only by rerunning the repo-seed skill.

- [repo-review](.agents/skills/repo-review/SKILL.md) — semantic review policy.
- [repo-decisions](.agents/skills/repo-decisions/SKILL.md) — MADR decision records.
- [repo-governance](.agents/skills/repo-governance/SKILL.md) — capability assessment.

Enabled optional capabilities include CI, release policy, community health files,
CODEOWNERS, monorepo subtree instructions, and authorized local hooks.
