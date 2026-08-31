use crate::manager::Run;
use crate::rcm::{ActionCommand, ActionItem, ActionSpace, FragmentContent};

/// v2 menu: the seven verbs, per-cell structural items expressed as
/// single-op Edit commands, and one Tool item per registered tool. Ops are
/// JSON-encoded in the command (serde is the single source of truth).
pub fn build_action_space(run: &Run) -> ActionSpace {
    let state = &run.state;
    let context = &state.run.context;
    let resources = &state.run.resources;
    let mut actions = Vec::new();

    let push = |actions: &mut Vec<ActionItem>, command: ActionCommand, label: String| {
        actions.push(ActionItem {
            command: Some(command),
            label,
            sink: None,
        });
    };

    // Pacing.
    if !state.frame.status.is_done() {
        push(
            &mut actions,
            ActionCommand {
                verb: "Done".into(),
                ..Default::default()
            },
            "Done".into(),
        );
    }
    if state.frame.inbox.is_empty() {
        push(
            &mut actions,
            ActionCommand {
                verb: "Halt".into(),
                ..Default::default()
            },
            "Halt".into(),
        );
    }

    // Consume: one Edit(Inbox) per pending item (call_id addressed for
    // tool results, FIFO otherwise).
    for item in state.frame.inbox.items() {
        let call_id = match &item.fragment.content {
            machine::Content::ToolResult(result) => Some(result.call_id.clone()),
            _ => None,
        };
        let ops = vec![machine::edit::EditOp::Insert {
            position: machine::edit::Position::End,
            content: machine::edit::ContentSpec::Inbox {
                call_id: call_id.clone(),
            },
            anchor: None,
        }];
        let label = match &call_id {
            Some(call_id) => format!("Consume {call_id}"),
            None => "Consume oldest".to_string(),
        };
        let edit_ops_json = serde_json::to_string(&ops).unwrap_or_default();
        push(
            &mut actions,
            ActionCommand {
                verb: "Edit".into(),
                edit_ops_json: Some(edit_ops_json),
                ..Default::default()
            },
            label,
        );
    }

    // Structural: delete per non-protected cell; set per known slot.
    for cell in context.fragments() {
        let protected = cell
            .anchor
            .as_deref()
            .is_some_and(|anchor| machine::PROTECTED_ANCHORS.contains(&anchor));
        if protected {
            continue;
        }
        let ops = vec![machine::edit::EditOp::Delete {
            selector: machine::edit::Selector::Id(cell.id()),
        }];
        let edit_ops_json = serde_json::to_string(&ops).unwrap_or_default();
        let label = match &cell.anchor {
            Some(anchor) => format!("Delete {anchor}"),
            None => format!("Delete #{}", cell.id()),
        };
        push(
            &mut actions,
            ActionCommand {
                verb: "Edit".into(),
                edit_ops_json: Some(edit_ops_json),
                ..Default::default()
            },
            label,
        );
    }

    for slot in machine::SLOT_ORDER {
        if context.find_anchor(slot).is_none() {
            let ops = vec![machine::edit::EditOp::Set {
                anchor: slot.to_string(),
                content: machine::edit::ContentSpec::Literal {
                    text: String::new(),
                    role: machine::Role::System,
                    tag: None,
                },
            }];
            let edit_ops_json = serde_json::to_string(&ops).unwrap_or_default();
            let sink = FragmentContent {
                role: "system".into(),
                text: String::new(),
                ..Default::default()
            };
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Edit".into(),
                    edit_ops_json: Some(edit_ops_json),
                    ..Default::default()
                }),
                label: format!("Set {slot}"),
                sink: Some(sink_clip(sink)),
            });
        }
    }

    // Prompt slots materialize as tail inserts with the prompt body.
    for (name, text) in &resources.prompts {
        let ops = vec![machine::edit::EditOp::Insert {
            position: machine::edit::Position::End,
            content: machine::edit::ContentSpec::Literal {
                text: text.clone(),
                role: machine::Role::System,
                tag: Some(name.clone()),
            },
            anchor: None,
        }];
        let edit_ops_json = serde_json::to_string(&ops).unwrap_or_default();
        let sink = FragmentContent {
            role: "system".into(),
            text: text.clone(),
            ..Default::default()
        };
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Edit".into(),
                edit_ops_json: Some(edit_ops_json),
                ..Default::default()
            }),
            label: format!("Append {name}"),
            sink: Some(sink_clip(sink)),
        });
    }

    // Resources: model switch, tool visibility, and policy-initiated tools.
    for model_name in &resources.model_order {
        push(
            &mut actions,
            ActionCommand {
                verb: "Model".into(),
                name: Some(model_name.clone()),
                ..Default::default()
            },
            format!("Model {model_name}"),
        );
    }
    for tool_name in resources.tool_definitions.keys() {
        push(
            &mut actions,
            ActionCommand {
                verb: "Activate".into(),
                name: Some(tool_name.clone()),
                ..Default::default()
            },
            format!("Activate {tool_name}"),
        );
        push(
            &mut actions,
            ActionCommand {
                verb: "Tool".into(),
                name: Some(tool_name.clone()),
                args_json: Some("{}".into()),
                ..Default::default()
            },
            format!("Tool {tool_name}"),
        );
    }
    for tool_name in &resources.active_tools {
        push(
            &mut actions,
            ActionCommand {
                verb: "Deactivate".into(),
                name: Some(tool_name.clone()),
                ..Default::default()
            },
            format!("Deactivate {tool_name}"),
        );
    }

    // Document outline: labels for the first ENVELOPE_DIRECTORY_ROWS cells
    // (anchor when present, else #id) plus the exact total, so external
    // controllers can address cells without fetching full state.
    let cells = context.fragments();
    let document_outline: Vec<String> = cells
        .iter()
        .take(machine::obs::ENVELOPE_DIRECTORY_ROWS)
        .map(|cell| match &cell.anchor {
            Some(anchor) => anchor.clone(),
            None => format!("#{}", cell.id()),
        })
        .collect();
    ActionSpace {
        actions,
        document_outline,
        document_cells: cells.len() as u64,
    }
}

fn sink_clip(content: FragmentContent) -> FragmentContent {
    if content.text.len() <= 200 {
        content
    } else {
        let mut clipped = content.text.chars().take(200).collect::<String>();
        clipped.push_str("...");
        FragmentContent {
            text: clipped,
            ..content
        }
    }
}
