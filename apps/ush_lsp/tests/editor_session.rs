//! End-to-end LSP wire test for the new
//! `textDocument/definition`, `textDocument/references`, and
//! `textDocument/rename` handlers.

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};

fn ush_lsp() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush_lsp"))
}

fn send(stdin: &mut ChildStdin, body: &str) {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    stdin.write_all(header.as_bytes()).expect("write header");
    stdin.write_all(body.as_bytes()).expect("write body");
    stdin.flush().expect("flush");
}

fn recv_with_id(
    reader: &Receiver<serde_json::Value>,
    deadline: Instant,
    wanted_id: u64,
) -> serde_json::Value {
    loop {
        let response = recv(reader, deadline);
        if response.get("id").and_then(|v| v.as_u64()) == Some(wanted_id) {
            return response;
        }
    }
}

fn recv(reader: &Receiver<serde_json::Value>, deadline: Instant) -> serde_json::Value {
    reader
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .expect("timed out or LSP output ended")
}

fn reader(stdout: ChildStdout) -> Receiver<serde_json::Value> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            let mut length = None;
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 {
                    return;
                }
                if line == "\r\n" {
                    break;
                }
                if let Some(rest) = line.strip_prefix("Content-Length:") {
                    length = rest.trim().parse::<usize>().ok();
                }
            }
            let Some(length) = length else {
                return;
            };
            let mut body = vec![0; length];
            if reader.read_exact(&mut body).is_err() {
                return;
            }
            let Ok(body) = serde_json::from_slice(&body) else {
                return;
            };
            if tx.send(body).is_err() {
                return;
            }
        }
    });
    rx
}

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn shake_hands(stdin: &mut ChildStdin, reader: &Receiver<serde_json::Value>, deadline: Instant) {
    send(
        stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#,
    );
    let init = recv_with_id(reader, deadline, 1);
    assert!(init.get("result").is_some(), "initialize: {init}");
    send(
        stdin,
        r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#,
    );
}

fn open_document(stdin: &mut ChildStdin, uri: &str, text: &str) {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "ush",
                "version": 1,
                "text": text,
            }
        }
    });
    send(stdin, &payload.to_string());
}

#[test]
fn unicode_incremental_changes_close_and_invalid_requests_do_not_break_the_session() {
    let mut child = ChildGuard(
        ush_lsp()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let mut stdin = child.0.stdin.take().unwrap();
    let reader = reader(child.0.stdout.take().unwrap());
    let deadline = Instant::now() + Duration::from_secs(20);
    shake_hands(&mut stdin, &reader, deadline);
    let uri = "file:///tmp/session.ush";
    open_document(&mut stdin, uri, "let value = 1\nprint \"😀\" + value\n");
    let diagnostic = recv(&reader, deadline);
    assert_eq!(diagnostic["params"]["version"], 1);
    let req = |id, method: &str, params: serde_json::Value| {
        serde_json::json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}).to_string()
    };
    send(
        &mut stdin,
        &req(
            2,
            "textDocument/definition",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":1,"character":13}}),
        ),
    );
    assert_eq!(
        recv_with_id(&reader, deadline, 2)["result"]["range"]["start"]["character"],
        4
    );
    send(
        &mut stdin,
        &req(
            3,
            "textDocument/rename",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":1,"character":13},"newName":"answer"}),
        ),
    );
    let renamed = recv_with_id(&reader, deadline, 3);
    assert_eq!(
        renamed["result"]["changes"][uri][1]["range"]["start"]["character"],
        13
    );
    send(&mut stdin,&serde_json::json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri,"version":2},"contentChanges":[{"range":{"start":{"line":0,"character":12},"end":{"line":0,"character":13}},"text":"2"}]}}).to_string());
    assert_eq!(recv(&reader, deadline)["params"]["version"], 2);
    send(
        &mut stdin,
        &req(4, "textDocument/hover", serde_json::json!({})),
    );
    assert_eq!(recv_with_id(&reader, deadline, 4)["error"]["code"], -32602);
    send(&mut stdin,&serde_json::json!({"jsonrpc":"2.0","method":"textDocument/didClose","params":{"textDocument":{"uri":uri}}}).to_string());
    assert_eq!(
        recv(&reader, deadline)["params"]["diagnostics"],
        serde_json::json!([])
    );
    send(
        &mut stdin,
        &req(
            5,
            "textDocument/hover",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":0,"character":5}}),
        ),
    );
    assert!(recv_with_id(&reader, deadline, 5).get("error").is_some());
    send(&mut stdin, &req(6, "shutdown", serde_json::Value::Null));
    assert!(recv_with_id(&reader, deadline, 6).get("result").is_some());
    send(&mut stdin, r#"{"jsonrpc":"2.0","method":"exit"}"#);
    drop(stdin);
    assert!(child.0.wait().unwrap().success());
}
