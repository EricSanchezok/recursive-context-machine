# Provider normalization order dropped reasoning

Artifact-Version: 1

## Executive summary

RCM v0.2.24 selected DeepSeek's provider-native transport and produced the correct replay request shape, but the pinned adapter normalized response tool calls before their shared reasoning block. RCM only attached reasoning already seen while decoding, so it lost the original block and replayed a placeholder that DeepSeek rejected. The durable correction treats every completion as one assistant turn and associates its reasoning with all tool calls regardless of normalized content order.

## Summary

A downstream production release canary still received HTTP 400 on the second request of a fixed multi-turn tool workflow. Content-free diagnostics proved that assistant content was a string, every tool call had one result, and reasoning was present in the rejected request. The remaining mismatch was semantic: the value was RCM's legacy placeholder rather than the reasoning returned by DeepSeek.

## Timeline

- RCM v0.2.24 replaced the generic OpenAI transport with Rig 0.36's native DeepSeek adapter.
- Local wire tests proved the outgoing request shape when reasoning was already stored on a tool-call fragment.
- Scholight adopted v0.2.24 and its hermetic provider contract passed.
- The live production canary returned HTTP 400 before stack mutation with a balanced thinking/tool-history diagnostic.
- Inspection of the pinned adapter showed that it emits normalized response content in text, tool-call, reasoning order.
- A regression reproduced the loss by decoding a tool call followed by reasoning.

## Root cause

RCM assumed normalized assistant content preserved the provider's logical reasoning-before-tool order. Its decoder accumulated reasoning while iterating and attached only the blocks seen before each tool call. Rig's native DeepSeek adapter returned the same assistant turn in a different internal order, so the tool-call fragment retained no reasoning. The next request satisfied the structural checks because the legacy thinking path synthesized a placeholder, but DeepSeek requires the original reasoning content to be replayed unchanged.

The exact-wire regression did not catch this because it began with a manually constructed tool-call fragment whose reasoning was already attached. The hermetic downstream model returned reasoning in the order the regression expected instead of the order produced by the pinned native adapter.

## Guardrails

- [Order-independent decode regression](../../crates/machine/tests/completion.rs) covers the pinned adapter's tool-before-reasoning order as well as reasoning-before-tool and parallel calls.
- [Completion decoder](../../crates/machine/src/completion.rs) collects reasoning across the complete assistant turn before creating tool-call fragments.
- [DeepSeek native transport specification](../specs/0004-deepseek-native-transport.md) now makes response-order independence an explicit compatibility contract.
- The downstream live multi-turn provider canary remains a release gate before any production stack mutation.
