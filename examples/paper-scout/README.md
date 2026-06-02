# Paper Scout

A single-agent RCM that turns a one-line research topic into a compact reading route. It uses built-in arXiv search first, then AnySearch and Fetch MCP when available for broader web and primary-source grounding.

## Run

```sh
accelerate run /path/to/RCM/examples/paper-scout/paper_scout.rcm \
  --purpose "我想快速了解 KV cache compression，帮我找值得读的论文"
```

When running from this repository without installing:

```sh
cargo run --manifest-path /path/to/RCM/Cargo.toml --bin accelerate -- \
  run /path/to/RCM/examples/paper-scout/paper_scout.rcm \
  --purpose "我想快速了解 KV cache compression，帮我找值得读的论文"
```

## MCP Setup

This example uses:

- AnySearch MCP for web search: `https://api.anysearch.com/mcp`
- Fetch MCP for reading web pages: `uvx mcp-server-fetch`

AnySearch supports anonymous access with lower quota. If you have an API key, configure the MCP header in the `.rcm` file or through a future environment profile.

Fetch MCP requires `uvx`:

```sh
uvx mcp-server-fetch --help
```

## What It Collects

- arXiv papers through built-in `arxiv_search`
- Web search results through AnySearch MCP
- Important source pages through Fetch MCP

## Output

- `outputs/paper-scout.md`

## Safety

This example should not claim citation counts, venues, benchmarks, or URLs unless they are found in sources. If MCP tools are unavailable, it should clearly label the result as arXiv-limited.
