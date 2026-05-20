use std::fs;
use std::path::PathBuf;

fn write_rcm(name: &str, source: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rica-{}-{}-{}.rcm",
        name,
        std::process::id(),
        nonce
    ));
    fs::write(&path, source).unwrap();
    path
}

fn compile_result(source: &str) -> Result<accelerator::Accelerator, String> {
    let path = write_rcm("compile", source);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = runtime.block_on(cli::rcm::compile::compile_file(&path));
    let _ = fs::remove_file(path);
    result
}

#[test]
fn compile_rejects_flux_slot_out_of_range() {
    let source = r#"
        name = "bad flux"
        model gpt {
            protocol = "openai"
            credentials = { key = "REDACTED" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        graph {
            accelerator source {
                purpose = "source"
                model = "gpt"
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
fn compile_rejects_graph_cycles() {
    let source = r#"
        name = "cycle"
        model gpt {
            protocol = "openai"
            credentials = { key = "REDACTED" }
            limit = { context = "1000", output = "100" }
            modalities = { input = ["text"], output = ["text"] }
        }
        graph {
            accelerator first {
                purpose = "first"
                model = "gpt"
            }
            accelerator second {
                purpose = "second"
                model = "gpt"
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
