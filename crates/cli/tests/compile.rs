use std::fs;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use accelerator::{Accelerator, Catalog};
use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

static FILE_SEQ: AtomicU64 = AtomicU64::new(0);

struct ExternalTool;

impl Tool for ExternalTool {
    fn name(&self) -> &str {
        "external_tool"
    }

    fn description(&self) -> &str {
        "External test tool"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({ "type": "object", "properties": {} })
    }

    fn execute<'a>(
        &'a self,
        _args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async {
            Ok(ToolResult {
                call_id: String::new(),
                content: "ok".to_string(),
                title: None,
            })
        })
    }
}

fn unique_path(name: &str, extension: &str) -> PathBuf {
    let process_id = std::process::id();
    let sequence = FILE_SEQ.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("rcm-{name}-{process_id}-{sequence}.{extension}"))
}

fn write_rcm(name: &str, source: &str) -> PathBuf {
    let path = unique_path(name, "rcm");
    fs::write(&path, source).unwrap();
    path
}

fn write_rcm_near(name: &str, source: &str, prompt_name: &str, prompt: &str) -> PathBuf {
    let dir = unique_path(name, "dir");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join(prompt_name), prompt).unwrap();
    let path = dir.join("main.rcm");
    fs::write(&path, source).unwrap();
    path
}

fn remove_compile_path(path: &Path) {
    if let Some(parent) = path.parent()
        && parent
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("rcm-") && name.ends_with(".dir"))
    {
        let _ = fs::remove_dir_all(parent);
        return;
    }
    let _ = fs::remove_file(path);
}

fn compile_path(path: &Path) -> Result<Accelerator, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = runtime.block_on(cli::rcm::compile::compile_file(path));
    remove_compile_path(path);
    result
}

fn compile_path_with_catalog(path: &Path, catalog: Catalog) -> Result<Accelerator, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = runtime.block_on(cli::rcm::compile::compile_file_with_catalog(path, catalog));
    remove_compile_path(path);
    result
}

fn compile_result(source: &str) -> Result<Accelerator, String> {
    let path = write_rcm("compile", source);
    compile_path(&path)
}

fn primitive_state(accelerator: &Accelerator) -> &accelerator::State {
    accelerator
        .internal_state()
        .expect("expected primitive accelerator")
}

#[test]
fn compile_selects_resource_pools_without_initial_activation() {
    let source = r#"
        name = "resources"
        model fast {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        model careful {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "2000", output = "200" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
            models = ["fast", "careful"]
            prompts = { captain = "Custom captain" }
            tools = ["fs", "shell"]
        }
    "#;

    let accelerator = compile_result(source).unwrap();
    let state = primitive_state(&accelerator);

    assert_eq!(state.res.models.len(), 2);
    assert!(state.res.models.contains_key("fast"));
    assert!(state.res.models.contains_key("careful"));
    assert!(state.res.active_model.is_empty());
    assert!(state.res.prompts.contains_key("captain"));
    assert_eq!(
        state.res.prompts.get("captain").map(String::as_str),
        Some("Custom captain")
    );
    assert_eq!(state.res.tools.len(), 2);
    assert!(state.res.tools.contains_key("fs"));
    assert!(state.res.tools.contains_key("shell"));
    assert!(state.res.active_tools.is_empty());
}

#[test]
fn compile_keeps_no_tools_when_only_prompts_are_supplied() {
    let source = r#"
        name = "prompt only"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
            models = ["gpt"]
            prompts = { captain = "Custom captain" }
        }
    "#;

    let accelerator = compile_result(source).unwrap();
    let state = primitive_state(&accelerator);

    assert!(state.res.prompts.contains_key("captain"));
    assert!(state.res.tools.is_empty());
    assert!(state.res.active_tools.is_empty());
}

#[test]
fn compile_reads_prompt_files_relative_to_rcm_file() {
    let source = r#"
        name = "file prompt"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
            models = ["gpt"]
            prompts = { captain = file "captain.txt" }
        }
    "#;
    let path = write_rcm_near("prompt", source, "captain.txt", "File captain");

    let accelerator = compile_path(&path).unwrap();
    let state = primitive_state(&accelerator);

    assert_eq!(
        state.res.prompts.get("captain").map(String::as_str),
        Some("File captain")
    );
}

#[test]
fn compile_accepts_external_catalog_tools() {
    let source = r#"
        name = "external tool"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
            models = ["gpt"]
            tools = ["external_tool"]
        }
    "#;
    let path = write_rcm("external-tool", source);
    let mut catalog = Catalog::new();
    catalog.register_tool(Arc::new(ExternalTool)).unwrap();

    let accelerator = compile_path_with_catalog(&path, catalog).unwrap();
    let state = primitive_state(&accelerator);

    assert!(state.res.tools.contains_key("external_tool"));
    assert_eq!(state.res.tools.len(), 1);
}

#[test]
fn compile_requires_at_least_one_accelerator_model() {
    let source = r#"
        name = "missing models"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
        }
    "#;

    let error = match compile_result(source) {
        Ok(_) => panic!("expected compile error"),
        Err(error) => error,
    };
    assert!(error.contains("accelerator requires at least one model"));
}

#[test]
fn compile_rejects_flux_slot_out_of_range() {
    let source = r#"
        name = "bad flux"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        graph {
            accelerator source {
                purpose = "source"
                models = ["gpt"]
            }
            flux joined {
                channel = context
                mode = append
                arity = 1
            }
            source.context -> joined.slot(2)
        }
    "#;

    let error = match compile_result(source) {
        Ok(_) => panic!("expected compile error"),
        Err(error) => error,
    };
    assert!(error.contains("out of range"));
}

#[test]
fn compile_does_not_start_unselected_mcp_servers() {
    let source = r#"
        name = "unused mcp"
        mcp docs {
            transport = stdio
            command = "definitely-not-a-real-mcp-command"
        }
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
            models = ["gpt"]
            tools = ["fs"]
        }
    "#;

    let accelerator = compile_result(source).unwrap();
    let state = primitive_state(&accelerator);
    assert!(state.res.tools.contains_key("fs"));
    assert!(!state.res.tools.keys().any(|name| name.starts_with("docs.")));
}

#[test]
fn compile_rejects_unknown_mcp_selection() {
    let source = r#"
        name = "unknown mcp"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "inspect"
            models = ["gpt"]
            mcps = ["docs"]
        }
    "#;

    let error = match compile_result(source) {
        Ok(_) => panic!("expected compile error"),
        Err(error) => error,
    };
    assert!(error.contains("unknown mcp server: docs"));
}

#[test]
fn compile_rejects_graph_cycles() {
    let source = r#"
        name = "cycle"
        model gpt {
            protocol = "openai"
            credentials = { key = "test" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        graph {
            accelerator first {
                purpose = "first"
                models = ["gpt"]
            }
            accelerator second {
                purpose = "second"
                models = ["gpt"]
            }
            first.done -> second.trigger
            second.done -> first.trigger
        }
    "#;

    let error = match compile_result(source) {
        Ok(_) => panic!("expected compile error"),
        Err(error) => error,
    };
    assert!(error.contains("cycle"));
}
