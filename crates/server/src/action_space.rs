use crate::manager::Run;
use crate::rcm::{ActionCommand, ActionItem, ActionSpace, FragmentContent};

pub fn build_action_space(run: &Run) -> ActionSpace {
    let state = &run.state;
    let context = &state.run.context;
    let resources = &state.run.resources;
    let mut actions = Vec::new();

    if state.frame.inbox.is_empty() {
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Halt".into(),
                ..Default::default()
            }),
            label: "Halt".into(),
            sink: None,
        });

        for (name, text) in &resources.prompts {
            let fragment = FragmentContent {
                role: "system".into(),
                text: text.clone(),
                ..Default::default()
            };
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Append".into(),
                    fragment: Some(fragment.clone()),
                    ..Default::default()
                }),
                label: format!("Append {name}"),
                sink: Some(sink_clip(fragment)),
            });
        }

        for fragment in context.fragments() {
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

        for fragment in context.fragments() {
            for (name, text) in &resources.prompts {
                let content = FragmentContent {
                    role: "system".into(),
                    text: text.clone(),
                    ..Default::default()
                };
                actions.push(ActionItem {
                    command: Some(ActionCommand {
                        verb: "Replace".into(),
                        fragment_id: Some(fragment.id()),
                        fragment: Some(content.clone()),
                        ..Default::default()
                    }),
                    label: format!("Replace #{} with {name}", fragment.id()),
                    sink: Some(sink_clip(content)),
                });
            }
        }

        for fragment in context.fragments() {
            for (name, text) in &resources.prompts {
                let content = FragmentContent {
                    role: "system".into(),
                    text: text.clone(),
                    ..Default::default()
                };
                actions.push(ActionItem {
                    command: Some(ActionCommand {
                        verb: "Insert".into(),
                        fragment_id: Some(fragment.id()),
                        fragment: Some(content.clone()),
                        ..Default::default()
                    }),
                    label: format!("Insert after #{} with {name}", fragment.id()),
                    sink: Some(sink_clip(content)),
                });
            }
        }

        for index in 0..context.fragments().len().saturating_sub(1) {
            let first_id = context.fragments()[index].id();
            let second_id = context.fragments()[index + 1].id();
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Swap".into(),
                    fragment_id: Some(first_id),
                    fragment_id2: Some(second_id),
                    ..Default::default()
                }),
                label: format!("Swap #{first_id} ↔ #{second_id}"),
                sink: None,
            });
        }

        for model_name in &resources.model_order {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Model".into(),
                    name: Some(model_name.clone()),
                    ..Default::default()
                }),
                label: format!("Model {model_name}"),
                sink: None,
            });
        }
        for tool_name in resources.tool_definitions.keys() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Activate".into(),
                    name: Some(tool_name.clone()),
                    ..Default::default()
                }),
                label: format!("Activate {tool_name}"),
                sink: None,
            });
        }
        for tool_name in &resources.active_tools {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Deactivate".into(),
                    name: Some(tool_name.clone()),
                    ..Default::default()
                }),
                label: format!("Deactivate {tool_name}"),
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
        for fragment in context.fragments() {
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

    if !state.frame.status.is_done() {
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
        let mut clipped = content.text.chars().take(200).collect::<String>();
        clipped.push_str("...");
        FragmentContent {
            text: clipped,
            ..content
        }
    }
}
