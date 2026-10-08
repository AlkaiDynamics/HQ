//! End-to-end wire and persistence regression tests. Mock HTTP servers are
//! used deliberately; these tests do not claim live provider/model inference.

use neurite_core::{EntityId, IdNamespace};
use neurite_execution::{
    ExecutionEngine, ExecutionError, ReceiptStatus, ReceiptStore, RunSpec, SourceRef,
};
use neurite_model_gateway::{Gateway, ModelSpec, OllamaAdapter, OpenAiCompatibleAdapter};
use neurite_protocol::ProtocolId;
use neurite_workspace::Workspace;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn temp_root() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("hq-http-{stamp}-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    root
}

fn serve_once(
    status: u16,
    expected_path: &'static str,
    body: &'static str,
) -> (String, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let worker = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(10)))
            .unwrap();
        let mut captured = Vec::new();
        let mut buffer = [0u8; 8192];
        loop {
            let size = stream.read(&mut buffer).unwrap();
            assert!(size > 0, "client closed before completing request");
            captured.extend_from_slice(&buffer[..size]);
            if let Some(end) = captured.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                let header = String::from_utf8_lossy(&captured[..end]);
                let length = header
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or(0);
                if captured.len() >= end + 4 + length {
                    break;
                }
            }
        }
        let request = String::from_utf8(captured).unwrap();
        assert!(request.starts_with(&format!("POST {expected_path} HTTP/1.1")));
        let reason = if status == 200 {
            "OK"
        } else {
            "Service Unavailable"
        };
        let response = format!(
            "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len(),
        );
        stream.write_all(response.as_bytes()).unwrap();
        request
    });
    (url, worker)
}

#[test]
fn real_http_failover_persists_selection_and_survives_receipt_reopen() {
    let (primary_url, primary) = serve_once(503, "/api/chat", "{}");
    let (secondary_url, secondary) = serve_once(
        200,
        "/v1/chat/completions",
        r#"{"choices":[{"message":{"content":"secondary confirmed"}}]}"#,
    );
    let primary_id = ProtocolId::new("hq.provider.ollama.primary").unwrap();
    let secondary_id = ProtocolId::new("hq.provider.openai.secondary").unwrap();
    let mut gateway = Gateway::new();
    gateway.register(
        primary_id.clone(),
        OllamaAdapter::new(&primary_url).unwrap(),
    );
    gateway.register(
        secondary_id.clone(),
        OpenAiCompatibleAdapter::new(&format!("{secondary_url}/v1"), None).unwrap(),
    );
    let note_id = EntityId::scoped(IdNamespace::new(34), 42);
    let mut workspace = Workspace::create("wire-test");
    workspace.set_note(note_id, "VISIBLE_TEST_CONTEXT");
    let spec = RunSpec::new(
        "fallback-http",
        "Answer about this note",
        vec![SourceRef::Note(note_id)],
        vec![
            ModelSpec::new(primary_id, "local-test").unwrap(),
            ModelSpec::new(secondary_id, "cloud-test").unwrap(),
        ],
    );
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    let engine = ExecutionEngine::new(&gateway, &store);
    let preview = engine.preview(&workspace, &spec).unwrap();
    assert!(preview.messages[0].content.contains("VISIBLE_TEST_CONTEXT"));
    let receipt = engine.run(&workspace, &spec).unwrap();
    assert_eq!(receipt["status"], "succeeded");
    assert_eq!(
        receipt["selected"]["provider"],
        "hq.provider.openai.secondary"
    );
    assert_eq!(receipt["selected"]["model"], "cloud-test");
    assert_eq!(receipt["attempts"][0]["failure"], "server");
    assert_eq!(receipt["output"], "secondary confirmed");
    assert_eq!(
        ReceiptStore::new(root.clone()).read(&spec.id).unwrap(),
        Some(receipt)
    );
    assert_eq!(store.status(&spec.id).unwrap(), ReceiptStatus::Complete);
    let first_wire = primary.join().unwrap();
    let second_wire = secondary.join().unwrap();
    assert!(first_wire.contains("VISIBLE_TEST_CONTEXT"));
    assert!(second_wire.contains("VISIBLE_TEST_CONTEXT"));
    assert!(second_wire.contains("Answer about this note"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_receipt_survives_reopening_and_is_not_silently_missing() {
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    assert_eq!(store.status("interrupted").unwrap(), ReceiptStatus::Missing);
    assert!(store.read("interrupted").unwrap().is_none());
    fs::write(root.join("interrupted.pending"), b"pending").unwrap();
    let restarted = ReceiptStore::new(root.clone());
    assert_eq!(
        restarted.status("interrupted").unwrap(),
        ReceiptStatus::Pending
    );
    assert!(matches!(
        restarted.read("interrupted"),
        Err(ExecutionError::PendingRun)
    ));
    assert!(matches!(
        restarted.status("../escape"),
        Err(ExecutionError::InvalidRunId)
    ));
    fs::remove_dir_all(root).unwrap();
}
