# RCM — Composable AI Pipelines

RCM (Recursive Context Machine) is a tool for building composable AI pipelines
using `.rcm` files. Each `.rcm` file exports an accelerator — either a
primitive (single LLM call with tools) or a composite graph (multiple
accelerators wired together).

## Install

### macOS / Linux

```bash
# Homebrew
brew tap EricSanchezok/rcm-dist
brew install rcm

# or one-line install
curl -fsSL https://raw.githubusercontent.com/EricSanchezok/rcm-dist/main/install.sh | sh
```

### Windows

```powershell
irm https://raw.githubusercontent.com/EricSanchezok/rcm-dist/main/install.ps1 | iex
```

### Manual download

Pre-built binaries for all platforms are available on the
[rcm-dist releases page](https://github.com/EricSanchezok/rcm-dist/releases/latest).
Download the archive for your platform, extract it, and place `accelerate` on
your `PATH`.

Verify your installation:

```bash
accelerate --version
```

---
## Examples

```bash
# List all available resources in a project
accelerate inventory examples/research-assistant

# Parse a .rcm file and output its AST as JSON
accelerate parse examples/research-assistant/rcm/weather.rcm

# Run a single accelerator
accelerate run examples/research-assistant/rcm/captain.rcm

# Run a composite graph with cross-file import
accelerate run examples/research-assistant/rcm/arxiv_pipeline.rcm

# Run with streaming JSON output (for frontend integration)
accelerate run examples/research-assistant/rcm/weather.rcm --stream

# Run with tape animation speed control (ms per step)
accelerate run examples/research-assistant/rcm/captain.rcm --speed 300
```

Streaming `completion_end` events include additive, sanitized completion
telemetry when emitted by RCM v0.2.16 or later: `outcome`, `duration_ms`, and,
for failures, `http_status`, `failure_kind`, and `retryable`. Provider response
text, prompts, and credentials are never included. Consumers must continue to
accept older events where these fields are absent.

### gRPC + Python Demo

RCM exposes a gRPC server for programmatic control. The Python SDK lets you
drive the full agent lifecycle from Python — ideal for MoEH training pipelines.

```bash
# Terminal 1: start the gRPC server
DEEPSEEK_API_KEY="sk-xxx" cargo run --release -p server --bin rcm-server

# Terminal 2: run the Paper Digest demo
pip install grpcio protobuf grpcio-tools
cd sdks/python && bash generate.sh
DEEPSEEK_API_KEY="sk-xxx" python examples/research-assistant/grpc_demo.py
```

See `sdks/python/README.md` for SDK setup details.

---

## Desktop Frontend

The desktop app is in `desktop/`. It's an Electron + React app that opens `.rcm` files as interactive graph workspaces.

### Setup

```bash
cd desktop
npm install
```

### Development

```bash
# One command — builds the CLI, sets up desktop, and starts everything
./dev.sh
```

This script will:
1. Build the `accelerate` CLI binary (`cargo build -p cli --bin accelerate`)
2. Set `ACCELERATE_PATH` so Electron can find it
3. Install desktop dependencies (only on first run)
4. Start the Vite dev server and Electron window with hot reload

You can also start the desktop app manually:
```bash
cargo build -p cli --bin accelerate
ACCELERATE_PATH="$PWD/target/debug/accelerate" cd desktop && npm install && npm run dev
```

### Environment

The desktop app calls the `accelerate` CLI binary. Make sure `accelerate` is in your `PATH`:

```bash
# From the RCM project root
cargo build -p cli --bin accelerate
export ACCELERATE_PATH="$PWD/target/debug/accelerate"
```

### Building for distribution

```bash
cd desktop
npm run build        # build only
npm run dist         # package for current platform
npm run dist:mac     # package macOS .dmg
npm run dist:win     # package Windows installer
```

---

## CLI Reference

```text
accelerate
├── run <file.rcm>                    Run a .rcm file
│   ├── --speed <ms>                  Step delay for tape animation (default 50)
│   ├── --format <text|json>          Output format (default text)
│   ├── --context                     Show full context, not just final message
│   └── --stream                      Output hook events as JSON lines
├── parse <file.rcm>                  Parse to JSON AST
└── inventory [project-dir]           List policies, tools, prompts, models, MCPs
```

### Standard Project Layout

```text
project/
├── rcm.toml               Project metadata
├── rcm/                   .rcm files
│   ├── weather.rcm
│   └── pipeline.rcm
└── prompts/               External prompt files
    └── reviewer.txt
```

---

## Development

```bash
# Build the accelerate binary
cargo build -p cli --bin accelerate

# Run all Rust tests
cargo test

# Run frontend type check
cd desktop && npx tsc --noEmit
```

### Examples Project

`examples/research-assistant/` is a standardized demo project that showcases all core features:

| RCM file | Feature |
|----------|---------|
| `rcm/captain.rcm` | Primitive accelerator, model, policy |
| `rcm/weather.rcm` | Shell tool (curl wttr.in) |
| `rcm/arxiv_search.rcm` | Built-in arxiv search tool |
| `rcm/arxiv_pipeline.rcm` | Graph with `use` cross-file import + wire + output |
| `prompts/reviewer.txt` | External prompt loaded via `inventory` |
| `rcm.toml` | Project metadata |
| `grpc_demo.py` | gRPC full lifecycle demo (Python) |
| `sdks/python/src/rcm/` | Python SDK with Open/Step/Destroy |
| `sdks/python/examples/` | Python examples (arxiv, weather, system) |
