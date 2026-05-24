use machine::Action;
use machine::Fragment;
use tonic::Status;

use crate::rcm::{ActionCommand, FragmentContent, ModelSpec};

pub fn build_fragment(content: &FragmentContent) -> Fragment {
    let mut fragment = match content.role.as_str() {
        "user" => Fragment::user(&content.text),
        _ => Fragment::system(&content.text),
    };
    if let Some(tag) = &content.tag {
        fragment.tag = tag.clone();
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
    Ok(machine::Model {
        name: spec.name.clone(),
        protocol,
        endpoint: spec.endpoint.clone(),
        credentials,
        limit,
        timeout: spec.timeout.unwrap_or(180),
        headers: if spec.headers.is_empty() {
            None
        } else {
            Some(spec.headers.clone())
        },
        ..Default::default()
    })
}
