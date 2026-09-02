# Rust crates

The crates in this directory form one Cargo workspace. Keep package versions,
path dependencies, and public vocabulary consistent with the workspace root.

## Boundaries

- `machine` owns context, model protocol types, policies, the reactor, tools, and
  execution state.
- `accelerator` owns graph composition, Flux routing, resource catalogs, built-in
  tools, MCP, LSP, and trajectory integration.
- `storage` owns WAL, indexes, snapshots, and persistence errors.
- `server` owns the gRPC transport and maps protobuf messages to domain actions.
- `cli` owns the `accelerate` binary, RCM parser/compiler, output, and dispatch.
- `utils` owns shared identifiers and naming constraints.

Respect these seams when adding code; a cross-crate move needs an architecture
update and a decision record when alternatives are meaningful.

## Verification

Use the workspace commands in the root [AGENTS.md](../AGENTS.md). Unit tests stay
with their owning module; integration tests stay under each crate's `tests/`.
Use `cargo nextest run -p <crate> --locked` for focused evidence and the complete
workspace command before a release.
