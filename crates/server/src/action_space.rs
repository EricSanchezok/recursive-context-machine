use crate::manager::Run;
use crate::rcm::{ActionCommand, ActionItem, ActionSpace, FragmentContent};

pub fn build_action_space(run: &Run) -> ActionSpace {
    let mut actions = Vec::new();

    if run.inbox.is_empty() {
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Halt".into(),
                ..Default::default()
            }),
            label: "Halt".into(),
            sink: None,
        });

        for (name, text) in &run.resources.prompts {
            let fc = FragmentContent {
                role: "system".into(),
                text: text.clone(),
                ..Default::default()
            };
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Append".into(),
                    fragment: Some(fc.clone()),
                    ..Default::default()
                }),
                label: format!("Append {}", name),
                sink: Some(sink_clip(fc)),
            });
        }

        for fragment in run.ctx.fragments().iter() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Remove".into(),
                    fragment_id: Some(fragment.id()),
                    ..Default::default()
                }),
                label: format!("Remove #{}", fragment.id()),
                sink: None,
            });
        }

        for fragment in run.ctx.fragments().iter() {
            for (name, text) in &run.resources.prompts {
                let fc = FragmentContent {
                    role: "system".into(),
                    text: text.clone(),
                    ..Default::default()
                };
                actions.push(ActionItem {
                    command: Some(ActionCommand {
                        verb: "Replace".into(),
                        fragment_id: Some(fragment.id()),
                        fragment: Some(fc.clone()),
                        ..Default::default()
                    }),
                    label: format!("Replace #{} with {}", fragment.id(), name),
                    sink: Some(sink_clip(fc)),
                });
            }
        }

        for fragment in run.ctx.fragments().iter() {
            for (name, text) in &run.resources.prompts {
                let fc = FragmentContent {
                    role: "system".into(),
                    text: text.clone(),
                    ..Default::default()
                };
                actions.push(ActionItem {
                    command: Some(ActionCommand {
                        verb: "Insert".into(),
                        fragment_id: Some(fragment.id()),
                        fragment: Some(fc.clone()),
                        ..Default::default()
                    }),
                    label: format!("Insert after #{} with {}", fragment.id(), name),
                    sink: Some(sink_clip(fc)),
                });
            }
        }

        for i in 0..run.ctx.fragments().len().saturating_sub(1) {
            let id1 = run.ctx.fragments()[i].id();
            let id2 = run.ctx.fragments()[i + 1].id();
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Swap".into(),
                    fragment_id: Some(id1),
                    fragment_id2: Some(id2),
                    ..Default::default()
                }),
                label: format!("Swap #{} ↔ #{}", id1, id2),
                sink: None,
            });
        }

        for model_name in &run.resources.model_order {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Model".into(),
                    name: Some(model_name.clone()),
                    ..Default::default()
                }),
                label: format!("Model {}", model_name),
                sink: None,
            });
        }
        for tool_name in run.resources.tool_definitions.keys() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Activate".into(),
                    name: Some(tool_name.clone()),
                    ..Default::default()
                }),
                label: format!("Activate {}", tool_name),
                sink: None,
            });
        }
        for tool_name in &run.resources.active_tools {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Deactivate".into(),
                    name: Some(tool_name.clone()),
                    ..Default::default()
                }),
                label: format!("Deactivate {}", tool_name),
                sink: None,
            });
        }
    } else {
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Take".into(),
                ..Default::default()
            }),
            label: "Take".into(),
            sink: None,
        });
        for fragment in run.ctx.fragments().iter() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Remove".into(),
                    fragment_id: Some(fragment.id()),
                    ..Default::default()
                }),
                label: format!("Remove #{}", fragment.id()),
                sink: None,
            });
        }
    }

    if !run.done {
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Done".into(),
                ..Default::default()
            }),
            label: "Done".into(),
            sink: None,
        });
    }

    ActionSpace { actions }
}

fn sink_clip(content: FragmentContent) -> FragmentContent {
    if content.text.len() <= 200 {
        content
    } else {
        let mut clipped: String = content.text.chars().take(200).collect();
        clipped.push_str("...");
        FragmentContent {
            text: clipped,
            ..content
        }
    }
}
