# Provider 400 observability gap

Artifact-Version: 1

## Executive summary

A production research workflow repeatedly received HTTP 400 during reference expansion, but RCM exposed only the status and an unsafe raw error string, so operators could not distinguish request size from malformed thinking/tool history without inspecting user-bearing data; the permanent correction is content-free request-shape telemetry with stable failure classes and redaction tests.

## Summary

The affected workflow grouped several bibliography-heavy papers into one model unit. When its OpenAI-compatible provider rejected a request, the execution produced a generic invalid-request outcome and downstream processing continued without the expected artifact. The incident did not establish one unique provider-side cause because the available evidence lacked request-shape counters and a stable provider error classification.

## Timeline

- A production Survey reference-expansion unit began returning HTTP 400.
- Identical transport retries did not add diagnostic evidence or create the required expansion artifact.
- Investigation identified an observability gap spanning request size, reasoning replay, and tool-call history validation.
- The host workflow was redesigned around per-seed shards and RCM received a content-free provider diagnostic contract.

## Root cause

RCM converted provider failures into a hitch containing the raw display string and an optional HTTP status. Completion telemetry classified all client errors as `invalid_request`, while the operational warning formatted the full hitch. No contract measured the serialized request, counted reasoning and tool-history structure, extracted allowlisted provider identifiers, or prevented arbitrary provider content from entering diagnostics.

The test suite covered reasoning replay regressions but did not assert the observability behavior needed when a provider still rejects a structurally complex request.

## Guardrails

- [Machine provider diagnostic tests](../../crates/machine/tests/completion_telemetry.rs) assert stable request classes and content-free values.
- [Machine request-shape tests](../../crates/machine/tests/completion.rs) assert size and tool-history counters without retaining message content.
- [CLI stream tests](../../crates/cli/tests/run_stream.rs) assert additive sanitized output and legacy-consumer compatibility.
- [Provider request-shape specification](../specs/0001-provider-request-shape-diagnostics.md) owns the diagnostic boundary.
