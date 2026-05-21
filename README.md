# RICA — Composable AI Pipelines

RICA (RCM Integrated Context Accelerator) is a tool for building composable AI pipelines using `.rcm` files. Each `.rcm` file exports an accelerator — either a primitive (single LLM call with tools) or a composite graph (multiple accelerators wired together).

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
cd desktop
npm run dev
```

This starts both the Vite dev server and an Electron window. Hot reload is enabled.

### Environment

The desktop app calls the `accelerate` CLI binary. Make sure `accelerate` is in your `PATH`:

```bash
# From the RICA project root
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
├── rica.toml              Project metadata
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
| `rica.toml` | Project metadata |
