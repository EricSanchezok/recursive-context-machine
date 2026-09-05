# DeepSeek native transport

Artifact-Version: 1
Status: Implemented

## Intent

DeepSeek workflows must use a provider-native transport whose serialized request follows DeepSeek's own assistant-message schema instead of relying on the broader OpenAI-compatible content-parts representation. The change must preserve existing generic OpenAI behavior, explicit thinking control, request diagnostics, custom endpoints, and model limits without selecting a provider from a hostname.

## Contract

- `deepseek` is an accepted RCM model protocol in the CLI, server adapter, serialized model configuration, and built-in provider registry.
- The release artifact carrying this contract is versioned `0.2.24`; downstream pins must use its published archive checksum.
- A `deepseek` model uses Rig's DeepSeek client and sends requests to its configured endpoint or the provider default.
- A replayed pure assistant tool-call turn serializes `content` as the empty JSON string, preserves the original `reasoning_content` unchanged, and preserves its tool calls and following tool results.
- Explicit `thinking = "true"` or `thinking = "false"` produces the matching DeepSeek `thinking.type` request object; omission leaves the parameter absent.
- A configured output-token limit remains present as `max_tokens` on the DeepSeek wire request even though the pinned Rig adapter does not project the generic field itself.
- Existing `openai`, `anthropic`, and `gemini` declarations retain their public behavior. RCM does not infer DeepSeek from an endpoint or model name.
- Existing programmatic OpenAI thinking-mode methods remain source compatible while additive provider-neutral aliases support the DeepSeek path.

## Plan

Add the protocol variant at the model boundary, route it through Rig's DeepSeek client, project explicit thinking and output-token limits into DeepSeek additional parameters, update compiler/server/provider mappings, and replace the loose OpenAI conversion assertion with a local wire-level DeepSeek transport regression.

## Verification

- A local HTTP fixture captures the actual DeepSeek adapter request and asserts the exact `content`, `reasoning_content`, `tool_calls`, `thinking`, and `max_tokens` JSON shape.
- Compiler, server, provider-registry, serde, and legacy OpenAI tests cover the additive protocol and N-1-compatible behavior.
- Machine, CLI, server, and accelerator tests pass together with workspace nextest, doc tests, formatting, Clippy, and repository governance gates.
- A downstream live DeepSeek multi-turn tool canary succeeds before production adoption.

## Evidence

- [Exact DeepSeek wire regression and request-parameter tests](../../crates/machine/tests/completion.rs)
- [Protocol model and serde contract](../../crates/machine/src/model.rs)
- [CLI protocol compiler](../../crates/cli/src/rcm/compile.rs)
- [Server protocol adapter](../../crates/server/src/decode.rs)
- [Built-in provider registry](../../crates/accelerator/src/provider.rs)
- [Provider-native transport decision](../decisions/0007-use-provider-native-deepseek-transport.md)
- [Escaped request-shape postmortem](../postmortems/0003-content-presence-did-not-prove-deepseek-shape.md)
