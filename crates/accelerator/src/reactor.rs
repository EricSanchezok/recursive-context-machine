use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Fragment, Inbox, Reactor, Resources};
use rig::OneOrMany;
use rig::client::CompletionClient;
use rig::completion::{AssistantContent, CompletionModel, Message, ToolDefinition};

/// LLMReactor — the default Reactor implementation.
pub struct LLMReactor;

impl Default for LLMReactor {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMReactor {
    pub fn new() -> Self {
        Self
    }
}

impl Reactor for LLMReactor {
    fn react<'a>(
        &'a self,
        ctx: &'a Context,
        _env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a mut Inbox,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let model = match resources.active_model() {
                Some(m) => m,
                None => return,
            };

            let client = build_client(model);
            let completion_model = client.completion_model(&model.name);

            let messages: Vec<Message> =
                ctx.fragments().iter().filter_map(to_rig_message).collect();

            let active_tools = resources.active_tools();
            let tool_defs: Vec<ToolDefinition> = active_tools
                .iter()
                .map(|t| ToolDefinition {
                    name: t.name().to_string(),
                    description: t.description().to_string(),
                    parameters: t.parameters(),
                })
                .collect();

            let mut request = completion_model
                .completion_request(Message::user(""))
                .messages(messages)
                .tools(tool_defs);

            if let Some(temp) = model.temperature {
                request = request.temperature(temp);
            }
            if let Some(max_tok) = model.max_tokens {
                request = request.max_tokens(max_tok);
            }

            let response = match request.send().await {
                Ok(r) => r,
                Err(e) => {
                    inbox.push(Fragment::assistant(format!("Error: {}", e)));
                    return;
                }
            };

            for content in response.choice.iter() {
                match content {
                    AssistantContent::Text(text) => {
                        inbox.push(Fragment::assistant(&text.text));
                    }
                    AssistantContent::ToolCall(tc) => {
                        let call_id = tc.id.clone();
                        let tool_name = tc.function.name.clone();
                        let args = tc.function.arguments.clone();

                        inbox.push(Fragment::tool_call(&call_id, &tool_name, args.clone()));

                        let result = execute_tool(&active_tools, &tool_name, args).await;
                        inbox.push(Fragment::tool_result(&call_id, &result));
                    }
                    _ => {}
                }
            }
        })
    }
}

fn build_client(model: &machine::Model) -> rig::providers::openai::CompletionsClient {
    let base_url = model
        .endpoint
        .as_deref()
        .unwrap_or("https://api.openai.com/v1");
    let api_key = model.credentials.as_deref().unwrap_or("");

    rig::providers::openai::CompletionsClient::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()
        .expect("failed to build rig client")
}

fn to_rig_message(frag: &Fragment) -> Option<Message> {
    match frag.role {
        machine::Role::System => frag.as_text().map(Message::system),
        machine::Role::User => {
            if let machine::Content::ToolResult(tr) = &frag.content {
                Some(Message::tool_result(&tr.call_id, &tr.content))
            } else {
                frag.as_text().map(Message::user)
            }
        }
        machine::Role::Assistant => {
            if let machine::Content::ToolCall(tc) = &frag.content {
                Some(Message::Assistant {
                    id: None,
                    content: OneOrMany::one(AssistantContent::tool_call(
                        &tc.id,
                        &tc.name,
                        tc.arguments.clone(),
                    )),
                })
            } else {
                frag.as_text().map(Message::assistant)
            }
        }
    }
}

async fn execute_tool(tools: &[&dyn machine::Tool], name: &str, args: serde_json::Value) -> String {
    let tool = match tools.iter().find(|t| t.name() == name) {
        Some(t) => t,
        None => return format!("Tool '{}' not found", name),
    };

    match tool.execute(args).await {
        Ok(output) => output.content,
        Err(e) => format!("Tool error: {}", e),
    }
}
