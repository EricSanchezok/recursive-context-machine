# Use provider-native DeepSeek transport

## Status
Accepted
Class: bug-fix

## Context and Problem Statement

DeepSeek accepts an OpenAI-like Chat Completions API, but its assistant `content` field is a string or null. RCM's generic OpenAI path through Rig 0.36 represents assistant content as parts; the v0.2.23 compatibility item made a missing field present as a text-parts array rather than an empty string. A release canary exposed the mismatch when the model produced a pure tool-call turn, while earlier canaries happened to include visible assistant text and therefore did not exercise the invalid shape.

## Decision Drivers

- The serialized request must follow DeepSeek's documented thinking/tool-history contract exactly.
- The permanent regression must inspect the actual provider wire request, not only provider-neutral messages.
- DeepSeek-specific behavior must be explicit and must not depend on endpoint or model-name inspection.
- Existing generic OpenAI-compatible callers must remain unchanged.
- Explicit thinking selection and configured output limits must survive transport conversion.
- The solution must stay within the pinned Rig release rather than combining the incident fix with a broad dependency upgrade.

## Considered Options

- Add a first-class DeepSeek protocol backed by Rig's native DeepSeek adapter.
- Keep the generic OpenAI adapter and change the compatibility value again.
- Detect DeepSeek endpoints and selectively rewrite the OpenAI JSON body.
- Upgrade Rig and depend on a changed generic OpenAI conversion.

## Decision Outcome

RCM adds an explicit `deepseek` protocol and routes it through Rig's provider-native DeepSeek adapter. That adapter converts provider-neutral assistant content to the documented string field and retains reasoning and tool calls. RCM supplies `thinking` and `max_tokens` through additional parameters because those are owned configuration values and the pinned adapter does not project the generic output limit.

This beats another generic OpenAI workaround because the provider adapter now owns the provider schema and the regression observes its real HTTP body. It beats endpoint detection because provider identity remains a declared contract. It gives up treating DeepSeek as interchangeable with every OpenAI-compatible gateway and adds one protocol branch that must remain covered.

## Pros and Cons of the Options

### Provider-native DeepSeek protocol

- Good: serializes assistant content using DeepSeek's string schema.
- Good: makes provider selection explicit in source configuration and inventory.
- Good: supports custom DeepSeek-compatible endpoints without hostname logic.
- Trade-off: adds one protocol variant and transport branch.
- Trade-off: RCM must preserve `max_tokens` through additional parameters until the pinned adapter does so.

### Another generic OpenAI workaround

- Good: minimizes protocol-surface changes.
- Bad: continues translating a provider-specific string contract through a content-parts schema.
- Bad: a fix for DeepSeek could alter unrelated OpenAI-compatible providers.

### Endpoint-based JSON rewriting

- Good: could produce the required field without a public protocol value.
- Bad: alternate gateways, proxies, and self-hosted endpoints make identity inference incomplete.
- Bad: handwritten body mutation duplicates provider-adapter responsibility.

### Upgrade Rig

- Good: may contain broader provider improvements.
- Bad: expands the release and regression surface during a production compatibility incident.
- Bad: does not remove the need for an explicit exact-wire test or prove generic OpenAI is the right DeepSeek schema.

## Links

- [DeepSeek thinking-mode tool-call contract](https://api-docs.deepseek.com/guides/thinking_mode/)
- [DeepSeek Chat Completions assistant message schema](https://api-docs.deepseek.com/api/create-chat-completion/)
- [DeepSeek native transport specification](../specs/0004-deepseek-native-transport.md)
- [Escaped request-shape postmortem](../postmortems/0003-content-presence-did-not-prove-deepseek-shape.md)
