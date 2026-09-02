use machine::Action;
use machine::Fragment;
use tonic::Status;

use crate::rcm::{ActionCommand, FragmentContent, ModelSpec};

pub fn build_fragment(content: &FragmentContent) -> Fragment {
    let kind = content.kind.as_str();
    let mut fragment = match kind {
        "tool_call" => Fragment::tool_call(
            content.tag.as_deref().unwrap_or(""),
            &content.text,
            serde_json::json!({}),
        ),
        "tool_result" => {
            Fragment::tool_result(content.tag.as_deref().unwrap_or(""), &content.text, None)
        }
        "hitch" => Fragment::hitch(&content.text, None, machine::Role::System, None::<&str>),
        _ => match content.role.as_str() {
            "user" => Fragment::user(&content.text),
            "assistant" => Fragment::assistant(&content.text),
            _ => Fragment::system(&content.text),
        },
    };
    if let Some(tag) = &content.tag {
        fragment.tag = tag.clone();
    }
    if let Some(media_source) = &content.media_source
        && let Some(source) = &media_source.source
    {
        let data_source = match source {
            crate::rcm::media_source::Source::Url(url) => {
                machine::fragment::DataSource::Url(url.clone())
            }
            crate::rcm::media_source::Source::Base64(data) => {
                machine::fragment::DataSource::Base64(data.clone())
            }
        };
        match content.kind.as_str() {
            "image" => {
                fragment = machine::Fragment::image(data_source, media_source.media_type.clone())
            }
            "audio" => {
                fragment = machine::Fragment::audio(data_source, media_source.media_type.clone())
            }
            "video" => {
                fragment = machine::Fragment::video(data_source, media_source.media_type.clone())
            }
            "document" => {
                fragment = machine::Fragment::document(data_source, media_source.media_type.clone())
            }
            _ => {}
        }
    }
    fragment
}

pub fn decode_command(command: &ActionCommand) -> Result<Action, Status> {
    match command.verb.as_str() {
        "Halt" => Ok(Action::Halt),
        "Done" => Ok(Action::Done),
        "Take" => Ok(Action::Take),
        "Append" => {
            let content = command
                .fragment
                .as_ref()
                .ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Append(build_fragment(content)))
        }
        "Remove" => {
            let id = command
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            Ok(Action::Remove(id))
        }
        "Swap" => {
            let id1 = command
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            let id2 = command
                .fragment_id2
                .ok_or(Status::invalid_argument("fragment_id2 required"))?;
            Ok(Action::Swap(id1, id2))
        }
        "Insert" => {
            let after = command
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            let content = command
                .fragment
                .as_ref()
                .ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Insert {
                after,
                fragment: build_fragment(content),
            })
        }
        "Replace" => {
            let id = command
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            let content = command
                .fragment
                .as_ref()
                .ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Replace {
                id,
                fragment: build_fragment(content),
            })
        }
        "Model" => {
            let name = command
                .name
                .as_ref()
                .ok_or(Status::invalid_argument("name required"))?;
            Ok(Action::Model(name.clone()))
        }
        "Activate" => {
            let name = command
                .name
                .as_ref()
                .ok_or(Status::invalid_argument("name required"))?;
            Ok(Action::Activate(name.clone()))
        }
        "Deactivate" => {
            let name = command
                .name
                .as_ref()
                .ok_or(Status::invalid_argument("name required"))?;
            Ok(Action::Deactivate(name.clone()))
        }
        other => Err(Status::invalid_argument(format!("unknown verb: {}", other))),
    }
}

pub fn build_model(spec: &ModelSpec) -> Result<machine::Model, Status> {
    use machine::Protocol;
    let protocol = match spec.protocol.as_str() {
        "openai" => Protocol::OpenAI,
        "anthropic" => Protocol::Anthropic,
        "gemini" => Protocol::Gemini,
        other => {
            return Err(Status::invalid_argument(format!(
                "unknown protocol: {}",
                other
            )));
        }
    };
    let credentials = spec.credentials.as_ref().and_then(|c| match &c.source {
        Some(crate::rcm::credential_spec::Source::Env(var)) => std::env::var(var).ok(),
        Some(crate::rcm::credential_spec::Source::Literal(key)) => Some(key.clone()),
        None => None,
    });
    let limit = spec.limit.as_ref().map(|l| machine::Limit {
        context: l.context,
        input: l.input,
        output: l.output,
    });
    let modalities = if !spec.modalities_input.is_empty() || !spec.modalities_output.is_empty() {
        let input = spec
            .modalities_input
            .iter()
            .map(|m| parse_modality(m))
            .collect::<Result<Vec<_>, _>>()?;
        let output = spec
            .modalities_output
            .iter()
            .map(|m| parse_modality(m))
            .collect::<Result<Vec<_>, _>>()?;
        Some(machine::Modalities { input, output })
    } else {
        None
    };
    Ok(machine::Model {
        name: spec.name.clone(),
        protocol,
        endpoint: spec.endpoint.clone(),
        credentials,
        limit,
        modalities,
        timeout: spec.timeout.unwrap_or(machine::DEFAULT_MODEL_TIMEOUT_SECS),
        temperature: spec.temperature,
        thinking: spec.thinking,
        cost: spec.cost.as_ref().map(|c| machine::Cost {
            input: c.input,
            output: c.output,
            cache_read: c.cache_read,
            cache_write: c.cache_write,
        }),
        headers: if spec.headers.is_empty() {
            None
        } else {
            Some(spec.headers.clone())
        },
        ..Default::default()
    })
}

fn parse_modality(value: &str) -> Result<machine::Modality, Status> {
    match value {
        "text" => Ok(machine::Modality::Text),
        "audio" => Ok(machine::Modality::Audio),
        "image" => Ok(machine::Modality::Image),
        "video" => Ok(machine::Modality::Video),
        "pdf" => Ok(machine::Modality::Pdf),
        other => Err(Status::invalid_argument(format!(
            "unknown modality: {}",
            other
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::build_model;
    use crate::rcm::ModelSpec;

    #[test]
    fn decoded_model_default_allows_thirty_minute_requests() {
        let model = build_model(&ModelSpec {
            name: "test".to_string(),
            protocol: "openai".to_string(),
            ..Default::default()
        })
        .expect("model should decode");

        assert_eq!(model.timeout, 1_800);
    }
}
