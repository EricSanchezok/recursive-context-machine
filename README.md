# RCM — Recursive Context Machine

RCM is a Rust runtime for composable, stateful AI workflows. Pipelines are
described with `.rcm` files and can combine model calls, tools, MCP servers,
LSP diagnostics, storage, and explicit state transitions.

The public source, release assets, installers, and Homebrew Formula live in
this repository:

<https://github.com/EricSanchezok/recursive-context-machine>

## Install

### Homebrew

The Formula is generated from each release and is available directly from this
repository:

```bash
brew install --formula \
  https://raw.githubusercontent.com/EricSanchezok/recursive-context-machine/main/Formula/rcm.rb
```

### Shell installer

```bash
curl -fsSL https://raw.githubusercontent.com/EricSanchezok/recursive-context-machine/main/install.sh | sh
```

On Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/EricSanchezok/recursive-context-machine/main/install.ps1 | iex
```

Manual downloads are published on the [Releases page](https://github.com/EricSanchezok/recursive-context-machine/releases/latest).

## Quick start

Build the CLI from source and inspect an example pipeline:

```bash
cargo run --release --bin accelerate -- inventory examples/general
cargo run --release --bin accelerate -- parse examples/general/general.rcm
cargo run --release --bin accelerate -- run examples/general/general.rcm --stream
```

Other complete examples include:

- `examples/city-halfday/city_halfday.rcm` — a compact multi-step workflow.
- `examples/cook-tonight/cook_tonight.rcm` — a tool-oriented workflow.
- `examples/paper-scout/paper_scout.rcm` — a research and synthesis workflow.
- `examples/project-maintainer/dispatch.toml` — event routing for the maintainer pipeline.

Each example directory contains its own README with setup and runtime notes.

## gRPC and Python SDK

Run the server locally:

```bash
cargo run --release -p server --bin rcm-server
```

The Python SDK is an independent package under `sdks/python`. Install it and
regenerate the protobuf stubs when `proto/rcm.proto` changes:

```bash
cd sdks/python
pip install -e . grpcio-tools
bash generate.sh
PYTHONPATH=src python examples/echo_agent.py "hello from Python"
```

The SDK examples also include a local MCP math server. See
[`sdks/python/README.md`](sdks/python/README.md) for the API shape and model
credential setup.

## Development

Normal development happens on `dev` or a feature branch. `main` is updated only
by the automated release-promotion pull request. Useful local checks are:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --tests --locked -- -D warnings
cargo nextest run --workspace --locked
cargo test --workspace --doc --locked
node scripts/run-gates.mjs
```

Read [`AGENTS.md`](AGENTS.md) for repository rules, and the focused guides in
[`docs/`](docs/) for architecture, development, testing, and release policy.

## License

RCM is released under the [MIT License](LICENSE).
