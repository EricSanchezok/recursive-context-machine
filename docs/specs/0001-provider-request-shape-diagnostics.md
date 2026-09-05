# Provider request-shape diagnostics

Artifact-Version: 1
Status: Implemented

## Intent

Completion failures from OpenAI-compatible providers must expose enough content-free metadata to distinguish oversized requests, invalid thinking/tool history, and unknown client errors without copying prompts, tool results, provider response messages, model output, or credentials into operational telemetry.

This contract is additive to the completion stream and does not change provider selection, model-visible history, or retry policy.

## Contract

- The release artifact carrying this contract is versioned `0.2.21`; downstream pins must not adopt it before its archive checksum is published and the provider canary passes.
- Every completion end event reports the serialized generic request-envelope size, estimated input tokens, message count, tool-definition count, tool-call count, tool-result count, thinking flag, reasoning-content presence and byte length, unmatched tool-call count, and duplicate tool-call count when a request was constructed.
- Provider failures may report only a sanitized provider code, sanitized provider type, sanitized request identifier, and one stable request class: `request_size`, `thinking_tool_history`, or `unknown_request`.
- Provider error bodies are inspected only in memory for classification and allowlisted identifiers; arbitrary response fields and messages never enter completion telemetry or the sanitized operational failure log.
- HTTP provider hitches retain the status code but replace the response body with a stable content-free summary before entering the machine tape.
- HTTP 400 remains non-retryable at the RCM transport boundary so a host workflow can select a bounded request-shrink or non-thinking fallback instead of replaying the same invalid request.
- Existing stream consumers remain compatible because every field is additive.

## Plan

Derive shape counters from the normalized `CompletionRequest`, parse provider error envelopes into allowlisted identifiers, carry sanitized diagnostics alongside the completion result, and project them into tracing hooks and JSON-line output.

## Verification

- Machine tests cover request counters, tool-history anomalies, provider classification, and diagnostic redaction.
- CLI stream tests prove the additive JSON contract and legacy-consumer compatibility.
- Workspace tests, formatting, Clippy, and repository gates pass.

## Evidence

- [Request-shape and redaction tests](../../crates/machine/tests/completion.rs)
- [Completion telemetry classification tests](../../crates/machine/tests/completion_telemetry.rs)
- [Additive CLI stream contract tests](../../crates/cli/tests/run_stream.rs)
