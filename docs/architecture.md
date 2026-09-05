# Architecture

This document is the ordered map of the repository: composition, modules, seams, and extension points. Read it before changing structure. Per-module detail lives in the owning module's own documentation; durable decision rationale lives in [docs/decisions/](../docs/decisions/).

The Rust workspace is composed of six crates: machine owns context, model, policy, reactor, tools, and execution; accelerator composes graphs, policies, provider/model resolution, tools, LSP, MCP, and trajectory handling; storage persists WAL-backed sessions; cli exposes the accelerate binary and RCM parser; server exposes the gRPC API; utils provides shared identifiers and naming. The Python SDK under sdks/python mirrors the gRPC contract.

Cross-crate seams are the machine API, accelerator graph and tool registries, storage snapshots, the CLI parser/compiler boundary, the protobuf service in proto/rcm.proto, and the SDK generated bindings.

Model protocol selection is explicit at the machine boundary. OpenAI-compatible gateways use the generic `openai` transport, while DeepSeek uses the provider-native `deepseek` transport because its thinking/tool-history assistant schema requires string content; endpoint and model names never select a protocol implicitly.

## Conventions

- New behavior goes on documented extension points; a decision record is required when a core-flow change chooses among meaningful alternatives.
- Explicit > implicit at boundaries: defaulting is an explicit step in the owning implementation, never a hidden fallback.
- Registrations are effects: every contribution goes through the owning registry and returns a disposer.
- Opaque cross-boundary identifiers are branded types, never bare strings.

## Change procedure

1. Trace the current owners of the flow you change.
2. For a risk-boundary change, obtain an Approved spec in [docs/specs/](../docs/specs/).
3. Record durable design choices in [docs/decisions/](../docs/decisions/) when genuine alternatives exist.
4. Update this document and the owning module docs in the same change.
5. Run the gates; add the smallest sufficient evidence for the behavior and risk, not tests for line count.
