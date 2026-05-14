use serde::Deserialize;
use serde_json::Value;

use machine::Model;

/// Station configuration response.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub model: Model,
    pub system_prompt: String,
    pub tools: Vec<ToolDef>,
}

/// Tool definition from Station.
#[derive(Debug, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub executor: String,
    pub config: Value,
}

/// Tool execution response from Station.
#[derive(Debug, Deserialize)]
pub struct Execution {
    pub command: String,
}

/// Station HTTP client.
pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Fetch full configuration.
    pub async fn config(&self) -> Result<Config, String> {
        let url = format!("{}/v1/config", self.base_url);
        reqwest::get(&url)
            .await
            .map_err(|e| e.to_string())?
            .json::<Config>()
            .await
            .map_err(|e| e.to_string())
    }

    /// Execute a tool and receive the shell command.
    pub async fn execute_tool(&self, name: &str, args: &Value) -> Result<Execution, String> {
        let url = format!("{}/v1/tools/{}/execute", self.base_url, name);
        reqwest::Client::new()
            .post(&url)
            .json(args)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<Execution>()
            .await
            .map_err(|e| e.to_string())
    }
}
