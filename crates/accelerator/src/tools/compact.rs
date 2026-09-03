//! Context self-management: the compact tool (C3).
//!
//! `context.compact` compresses a document range into the `@summary` slot
//! through one metered assistant completion, then returns an edits payload
//! the fire loop's drain channel applies. Full style writes the summary and
//! deletes the compacted range (write-first, delete-after — the payload
//! order guarantees the document never loses both the summary and its
//! sources at once); rolling style only refreshes the summary slot.

use std::future::Future;
use std::pin::Pin;

use machine::edit::{ContentSpec, EditOp, Selector};
use machine::{AssistantRequest, Environment, Role, Tool, ToolResult};
use serde_json::{Value, json};

const SUMMARY_ANCHOR: &str = "@summary";
const PREVIEW_CHARS: usize = 200;

const FULL_INSTRUCTION: &str = "Summarize the following material for a long-running agent. \
Preserve: decisions made, open threads, key facts, and any identifiers needed later. \
Be complete but concise.";
const ROLLING_INSTRUCTION: &str = "Update the rolling summary with the following material. \
Merge new information into the existing summary narrative, keep decisions, open threads, \
key facts, and identifiers; drop what is superseded.";

pub struct ContextCompactTool;

impl Tool for ContextCompactTool {
    fn name(&self) -> &str {
        "context.compact"
    }

    fn description(&self) -> &str {
        "Compress a context range into the @summary slot via one completion. \
Args: range (selector over the document), style ('full' also deletes the \
compacted range; 'rolling' only refreshes @summary). Returns an edits \
payload applied by the harness drain channel."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "range": {
                    "type": "object",
                    "description": "Selector for the source cells. Forms: {\"Anchor\": \"@x\"}, {\"Id\": n}, {\"Range\": {\"from\": pos, \"to\": pos}}, {\"Where\": {role?, tag?, kind?, skip_newest?, bytes_gt?}}. Position: {\"Anchor\": \"@x\"} | {\"Id\": n} | \"End\"."
                },
                "style": {
                    "type": "string",
                    "enum": ["full", "rolling"],
                    "description": "full = summarize then delete the range; rolling = refresh @summary only."
                }
            },
            "required": ["range"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let args = args.clone();
        Box::pin(async move {
            let assistant = env.assistant.clone().ok_or(
                "context.compact requires a completion assistant; it is available inside accelerator runs",
            )?;
            let range = args
                .get("range")
                .cloned()
                .ok_or("missing required parameter 'range'")?;
            let selector: Selector = serde_json::from_value(range)
                .map_err(|error| format!("'range' is not a valid selector: {error}"))?;
            let style = args.get("style").and_then(Value::as_str).unwrap_or("full");
            let instruction = match style {
                "full" => FULL_INSTRUCTION,
                "rolling" => ROLLING_INSTRUCTION,
                other => {
                    return Err(format!(
                        "unknown style '{other}' (expected 'full' or 'rolling')"
                    ));
                }
            };

            // One metered completion per tool call; the gateway enforces
            // the cap and returns model failures as errors.
            assistant.begin_tool_call();
            let (summary, usage) = assistant
                .complete(AssistantRequest {
                    instruction: instruction.to_string(),
                    source: selector.clone(),
                })
                .await?;
            if summary.trim().is_empty() {
                return Err("assistant returned an empty summary".into());
            }

            let summary_chars = summary.chars().count();
            let summary_preview: String = summary.chars().take(PREVIEW_CHARS).collect();
            let set_op = EditOp::Set {
                anchor: SUMMARY_ANCHOR.to_string(),
                content: ContentSpec::Literal {
                    text: summary,
                    role: Role::System,
                    tag: Some("summary".to_string()),
                },
            };
            let edits = match style {
                "rolling" => vec![set_op],
                _ => vec![
                    set_op,
                    EditOp::Delete {
                        selector: selector.clone(),
                    },
                ],
            };

            let content = json!({
                "tool": "context.compact",
                "style": style,
                "source": selector,
                "summary_chars": summary_chars,
                "summary_preview": summary_preview,
                "edits": edits,
                "usage": usage,
            })
            .to_string();
            Ok(ToolResult {
                call_id: String::new(),
                content,
                title: Some(format!("context.compact {style}")),
            })
        })
    }
}
