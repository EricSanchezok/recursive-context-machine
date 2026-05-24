use machine::Usage;
use machine::event;
use machine::{Content, Fragment};

use crate::manager::Run;
use crate::rcm;

pub fn build_state(run: &Run) -> rcm::State {
    let counts = run
        .machine
        .counts
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect();
    rcm::State {
        purpose: run.purpose.clone(),
        machine_id: run.machine.id.to_string(),
        fragments: run.ctx.fragments().iter().map(fragment_to_proto).collect(),
        workdir: run.env.cwd.to_string_lossy().into_owned(),
        env_vars: run.env.vars.clone(),
        active_model: run.resources.active_model.clone(),
        active_tools: run.resources.active_tools.iter().cloned().collect(),
        available_models: run.resources.model_order.clone(),
        available_tools: run.resources.tools.keys().cloned().collect(),
        done: run.done,
        inbox_pending: run.inbox.peek().is_some(),
        inbox_peek: run.inbox.peek().map(fragment_to_proto),
        counts,
        usages: run.machine.usages.iter().map(usage_to_proto).collect(),
    }
}

fn usage_to_proto(usage: &Usage) -> rcm::Usage {
    rcm::Usage {
        input_tokens: usage.input_tokens,
        output_tokens: usage.output_tokens,
        total_tokens: usage.total_tokens,
        cached_input_tokens: usage.cached_input_tokens,
        cache_creation_input_tokens: usage.cache_creation_input_tokens,
        fragment_ids: usage.fragment_ids.clone(),
    }
}

fn fragment_to_proto(fragment: &Fragment) -> rcm::Fragment {
    let kind = match &fragment.content {
        Content::Text(_) => "text",
        Content::ToolCall(_) => "tool_call",
        Content::ToolResult(_) => "tool_result",
        Content::Hitch { .. } => "hitch",
        Content::Image(_) => "image",
        Content::Audio(_) => "audio",
        Content::Video(_) => "video",
        Content::Document(_) => "document",
    };
    rcm::Fragment {
        id: fragment.id(),
        role: event::role_name(fragment.role).into(),
        kind: kind.into(),
        text_preview: clip(fragment),
        tag: Some(fragment.tag.clone()),
    }
}

fn clip(fragment: &Fragment) -> String {
    let full = match &fragment.content {
        Content::Text(t) => t.text.clone(),
        Content::Hitch { message, .. } => message.clone(),
        Content::ToolCall(tc) => format!("tool_call: {}", tc.name),
        Content::ToolResult(tr) => format!("tool_result: {}", tr.title.as_deref().unwrap_or("")),
        _ => String::new(),
    };
    if full.len() <= 200 {
        full
    } else {
        let mut clipped: String = full.chars().take(200).collect();
        clipped.push_str("...");
        clipped
    }
}
