use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

use machine::{Tool, ToolOutput};

use crate::station;

/// Shell-based tool — execution command fetched from Station.
pub struct ShellTool {
    name: String,
    description: String,
    parameters: Value,
    station: String,
}

impl ShellTool {
    pub fn new(def: &station::ToolDef, station_url: impl Into<String>) -> Self {
        Self {
            name: def.name.clone(),
            description: def.description.clone(),
            parameters: def.parameters.clone(),
            station: station_url.into(),
        }
    }
}

impl Tool for ShellTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Value {
        self.parameters.clone()
    }

    fn execute<'a>(
        &'a self,
        args: Value,
    ) -> Pin<Box<dyn Future<Output = Result<ToolOutput, String>> + Send + 'a>> {
        let station = self.station.clone();
        let name = self.name.clone();
        Box::pin(async move {
            let client = station::Client::new(station);
            let exec = client.execute_tool(&name, &args).await?;

            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&exec.command)
                .output()
                .map_err(|e| e.to_string())?;

            let content = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if !stderr.is_empty() && content.is_empty() {
                return Err(stderr);
            }

            Ok(ToolOutput {
                content,
                title: Some(name),
            })
        })
    }
}
