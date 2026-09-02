use std::fs;

use accelerator::lsp::LspClient;
use accelerator::lsp::ServerSpec;

const FAKE_SERVER: &str = r#"
import json, sys

def read_msg():
    length = None
    while True:
        line = sys.stdin.buffer.readline()
        if not line:
            return None
        if line in (b'\r\n', b'\n'):
            break
        name, value = line.decode().split(':', 1)
        if name.lower() == 'content-length':
            length = int(value.strip())
    return json.loads(sys.stdin.buffer.read(length).decode())

def write_msg(msg):
    body = json.dumps(msg, separators=(',', ':')).encode()
    sys.stdout.buffer.write(b'Content-Length: ' + str(len(body)).encode() + b'\r\n\r\n' + body)
    sys.stdout.buffer.flush()

while True:
    msg = read_msg()
    if msg is None:
        break
    if msg.get('method') == 'initialize':
        write_msg({'jsonrpc':'2.0','id':msg['id'],'result':{'capabilities':{'textDocumentSync':1}}})
    elif msg.get('method') == 'textDocument/didOpen':
        doc = msg['params']['textDocument']
        write_msg({'jsonrpc':'2.0','method':'textDocument/publishDiagnostics','params':{'uri':doc['uri'],'version':doc['version'],'diagnostics':[{'range':{'start':{'line':0,'character':0},'end':{'line':0,'character':1}},'severity':1,'message':'fake error'}]}})
    elif msg.get('method') == 'textDocument/didChange':
        doc = msg['params']['textDocument']
        write_msg({'jsonrpc':'2.0','method':'textDocument/publishDiagnostics','params':{'uri':doc['uri'],'version':doc['version'],'diagnostics':[{'range':{'start':{'line':1,'character':0},'end':{'line':1,'character':1}},'severity':1,'message':'changed error'}]}})
"#;

#[tokio::test]
async fn fake_lsp_server_reports_versioned_diagnostics() {
    if std::process::Command::new("python3")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipping fake LSP test: python3 not found");
        return;
    }
    let root = std::env::temp_dir().join(format!("rcm_fake_lsp_{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let path = root.join("lib.rs");
    fs::write(&path, "fn main() {}\n").unwrap();
    let server = ServerSpec {
        id: "fake-lsp",
        language_id: "rust",
        extensions: &["rs"],
        root_markers: &["Cargo.toml"],
        command: "python3",
        args: &[],
    };
    let client =
        LspClient::start_with_command(server, root.clone(), "python3", &["-u", "-c", FAKE_SERVER])
            .await
            .unwrap();
    let first = client
        .touch_file_with_text(&path, "fn main() {}\n", true)
        .await
        .unwrap();
    assert_eq!(first[0].message, "fake error");
    let second = client
        .touch_file_with_text(&path, "fn main() {}\nlet x = 1;\n", true)
        .await
        .unwrap();
    assert_eq!(second[0].message, "changed error");
    fs::remove_dir_all(root).ok();
}
