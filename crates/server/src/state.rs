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
        available_tools: run.resources.tool_definitions.keys().cloned().collect(),
        done: run.done,
        inbox_pending: run.inbox.peek().is_some(),
        inbox_peek: run.inbox.peek().map(fragment_to_proto),
        counts,
        usages: run.machine.usages.iter().map(usage_to_proto).collect(),
        tool_profiles: build_tool_profiles(run),
        model_profiles: build_model_profiles(run),
        platform: run.env.platform.clone(),
        root: run
            .env
            .root
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned()),
    }
}

fn build_tool_profiles(run: &Run) -> Vec<rcm::ToolProfile> {
    run.resources
        .tool_definitions
        .iter()
        .map(|(name, tool)| rcm::ToolProfile {
            name: name.clone(),
            description: tool.description.clone(),
            active: run.resources.active_tools.contains(name),
        })
        .collect()
}

fn build_model_profiles(run: &Run) -> Vec<rcm::ModelProfile> {
    run.resources
        .models
        .iter()
        .map(|(name, model)| rcm::ModelProfile {
            name: name.clone(),
            protocol: format!("{:?}", model.protocol),
            limit: model.limit.as_ref().map(|limit| rcm::LimitSpec {
                context: limit.context,
                input: limit.input,
                output: limit.output,
            }),
            active: run.resources.active_model == *name,
        })
        .collect()
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

fn media_source_from_content(content: &machine::Content) -> Option<rcm::MediaSource> {
    use machine::fragment::DataSource;
    let source = match content {
        Content::Image(img) => &img.source,
        Content::Audio(a) => &a.source,
        Content::Video(v) => &v.source,
        Content::Document(d) => &d.source,
        _ => return None,
    };
    let source = match source {
        DataSource::Url(url) => rcm::media_source::Source::Url(url.clone()),
        DataSource::Base64(data) => rcm::media_source::Source::Base64(data.clone()),
        _ => return None,
    };
    Some(rcm::MediaSource {
        source: Some(source),
        media_type: None,
        alt_text: None,
    })
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
        content_text: Some(fragment.content_as_text()),
        media_source: media_source_from_content(&fragment.content),
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
