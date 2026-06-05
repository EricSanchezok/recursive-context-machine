use machine::event;
use machine::{Content, Fragment, MachineStatus};

use crate::manager::Run;
use crate::rcm;

pub fn build_state(run: &Run) -> rcm::State {
    let state = &run.state;
    let run_state = &state.run;
    let counts = run_state
        .telemetry
        .action_counts
        .iter()
        .map(|(action, count)| (action.to_string(), *count))
        .collect();
    rcm::State {
        purpose: run_state.purpose.text.clone(),
        machine_id: run.machine.id.to_string(),
        fragments: run_state
            .context
            .fragments()
            .iter()
            .map(fragment_to_proto)
            .collect(),
        workdir: run_state.environment.cwd.to_string_lossy().into_owned(),
        env_vars: run_state.environment.vars.clone(),
        active_model: run_state.resources.active_model.clone(),
        active_tools: run_state.resources.active_tools.iter().cloned().collect(),
        available_models: run_state.resources.model_order.clone(),
        available_tools: run_state
            .resources
            .tool_definitions
            .keys()
            .cloned()
            .collect(),
        done: state.frame.status == MachineStatus::Done,
        inbox_pending: state.frame.inbox.peek().is_some(),
        inbox_peek: state
            .frame
            .inbox
            .peek()
            .map(|item| fragment_to_proto(&item.fragment)),
        counts,
        usages: run_state
            .telemetry
            .completions
            .iter()
            .map(completion_to_proto)
            .collect(),
        tool_profiles: build_tool_profiles(run),
        model_profiles: build_model_profiles(run),
        platform: run_state.environment.platform.clone(),
        root: run_state
            .environment
            .root
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned()),
    }
}

fn build_tool_profiles(run: &Run) -> Vec<rcm::ToolProfile> {
    let resources = &run.state.run.resources;
    resources
        .tool_definitions
        .iter()
        .map(|(name, tool)| rcm::ToolProfile {
            name: name.clone(),
            description: tool.description.clone(),
            active: resources.active_tools.contains(name),
        })
        .collect()
}

fn build_model_profiles(run: &Run) -> Vec<rcm::ModelProfile> {
    let resources = &run.state.run.resources;
    resources
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
            active: resources.active_model == *name,
        })
        .collect()
}

fn completion_to_proto(record: &machine::CompletionRecord) -> rcm::Usage {
    rcm::Usage {
        input_tokens: record.tokens.input_tokens,
        output_tokens: record.tokens.output_tokens,
        total_tokens: record.tokens.total_tokens,
        cached_input_tokens: record.tokens.cached_input_tokens,
        cache_creation_input_tokens: record.tokens.cache_creation_input_tokens,
        fragment_ids: record.output_fragment_ids.clone(),
    }
}

fn media_source_from_content(content: &machine::Content) -> Option<rcm::MediaSource> {
    use machine::fragment::DataSource;
    let source = match content {
        Content::Image(image) => &image.source,
        Content::Audio(audio) => &audio.source,
        Content::Video(video) => &video.source,
        Content::Document(document) => &document.source,
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
        Content::Text(text) => text.text.clone(),
        Content::Hitch { message, .. } => message.clone(),
        Content::ToolCall(tool_call) => format!("tool_call: {}", tool_call.name),
        Content::ToolResult(result) => {
            format!("tool_result: {}", result.title.as_deref().unwrap_or(""))
        }
        _ => String::new(),
    };
    if full.len() <= 200 {
        full
    } else {
        let mut clipped = full.chars().take(200).collect::<String>();
        clipped.push_str("...");
        clipped
    }
}
