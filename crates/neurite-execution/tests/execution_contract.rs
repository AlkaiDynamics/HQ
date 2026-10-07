use neurite_core::{EntityId, IdNamespace};
use neurite_execution::{
    ContextBundle, ExecutionEngine, ExecutionError, ReceiptStore, RunSpec, SourceRef,
};
use neurite_model_gateway::{
    ChatMessage, Gateway, ModelSpec, ProviderAdapter, ProviderError, ProviderErrorKind,
};
use neurite_protocol::ProtocolId;
use neurite_workspace::Workspace;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const NS: IdNamespace = IdNamespace::new(34);
fn eid(n: u64) -> EntityId {
    EntityId::scoped(NS, n)
}
fn model() -> ModelSpec {
    ModelSpec::new(ProtocolId::new("local").unwrap(), "test").unwrap()
}
fn run_spec(id: &str, refs: Vec<SourceRef>) -> RunSpec {
    RunSpec::new(id, "Answer from my selected context.", refs, vec![model()])
}
fn temp_root() -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("hq-run-{stamp}-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    root
}

#[derive(Clone)]
struct Spy {
    calls: Arc<Mutex<Vec<Vec<ChatMessage>>>>,
    response: Result<String, ProviderError>,
}
impl ProviderAdapter for Spy {
    fn discover(&self) -> Result<Vec<String>, ProviderError> {
        Ok(vec!["test".into()])
    }
    fn generate(&self, _: &str, messages: &[ChatMessage]) -> Result<String, ProviderError> {
        self.calls.lock().unwrap().push(messages.to_vec());
        self.response.clone()
    }
}
fn gateway(response: Result<&str, ProviderError>) -> (Gateway, Arc<Mutex<Vec<Vec<ChatMessage>>>>) {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut gateway = Gateway::new();
    gateway.register(
        ProtocolId::new("local").unwrap(),
        Spy {
            calls: calls.clone(),
            response: response.map(str::to_owned),
        },
    );
    (gateway, calls)
}

#[test]
fn preview_is_exact_context_sent_and_excludes_unselected_notes_and_files() {
    let mut workspace = Workspace::create("research");
    workspace.set_note(eid(1), "Selected note A");
    workspace.set_note(eid(2), "Unselected private note");
    workspace.set_file(eid(3), "quoted.txt", "Selected document B").unwrap();
    workspace.set_file(eid(4), "private.txt", "Unselected private document").unwrap();
    let spec = run_spec("run-one", vec![SourceRef::File(eid(3)), SourceRef::Note(eid(1))]);
    let (gateway, calls) = gateway(Ok("Answer A"));
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    let engine = ExecutionEngine::new(&gateway, &store);

    let preview: ContextBundle = engine.preview(&workspace, &spec).unwrap();
    assert_eq!(calls.lock().unwrap().len(), 0);
    assert_eq!(preview.source_refs, spec.sources);
    assert!(preview.messages[0].content.contains("Selected document B"));
    assert!(preview.messages[0].content.contains("Selected note A"));
    assert!(!preview.messages[0].content.contains("Unselected private"));
    assert!(preview.messages[0].content.find("Selected document B").unwrap()
        < preview.messages[0].content.find("Selected note A").unwrap());

    engine.run(&workspace, &spec).unwrap();
    assert_eq!(calls.lock().unwrap()[0], preview.messages);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn missing_source_fails_before_inference_or_receipt_creation() {
    let workspace = Workspace::create("empty");
    let (gateway, calls) = gateway(Ok("must not execute"));
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    let engine = ExecutionEngine::new(&gateway, &store);
    let result = engine.run(&workspace, &run_spec("missing", vec![SourceRef::Note(eid(5))]));

    assert!(matches!(result, Err(ExecutionError::MissingSource(_))));
    assert!(calls.lock().unwrap().is_empty());
    assert!(store.read("missing").unwrap().is_none());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn receipt_survives_store_reopening_and_captures_selection_and_output() {
    let mut workspace = Workspace::create("durable");
    workspace.set_note(eid(7), "Provenance note");
    let root = temp_root();
    let (gateway, _) = gateway(Ok("persisted answer"));
    let store = ReceiptStore::new(root.clone());
    let spec = run_spec("durable-1", vec![SourceRef::Note(eid(7))]);
    let receipt = ExecutionEngine::new(&gateway, &store).run(&workspace, &spec).unwrap();

    assert_eq!(receipt["status"], "succeeded");
    assert_eq!(receipt["selected"]["provider"], "local");
    assert_eq!(receipt["selected"]["model"], "test");
    assert_eq!(receipt["output"], "persisted answer");
    let reopened = ReceiptStore::new(root.clone());
    assert_eq!(reopened.read("durable-1").unwrap(), Some(receipt));
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn duplicate_run_id_fails_closed_without_repeating_model_execution() {
    let workspace = Workspace::create("duplicate");
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    let (gateway, calls) = gateway(Ok("done"));
    let engine = ExecutionEngine::new(&gateway, &store);
    let spec = run_spec("same-id", vec![]);
    engine.run(&workspace, &spec).unwrap();
    let result = engine.run(&workspace, &spec);

    assert!(matches!(result, Err(ExecutionError::AlreadyExists)));
    assert_eq!(calls.lock().unwrap().len(), 1);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn failed_inference_is_persisted_as_failure_without_losing_workspace() {
    let mut workspace = Workspace::create("failure");
    workspace.set_note(eid(8), "still here");
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    let (gateway, calls) = gateway(Err(ProviderError::new(
        ProviderErrorKind::Authentication, "bad credentials must not be exposed",
    )));
    let engine = ExecutionEngine::new(&gateway, &store);
    let result = engine.run(&workspace, &run_spec("failed", vec![SourceRef::Note(eid(8))]));

    assert!(matches!(result, Err(ExecutionError::Inference(_))));
    let saved = store.read("failed").unwrap().unwrap();
    assert_eq!(saved["status"], "failed");
    let dumped = saved.to_string();
    assert!(!dumped.contains("bad credentials"));
    assert_eq!(calls.lock().unwrap().len(), 1);
    assert_eq!(workspace.notes[&eid(8)], "still here");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn oversized_context_and_invalid_receipt_id_fail_closed() {
    let mut workspace = Workspace::create("limits");
    workspace.set_note(eid(9), "x".repeat(100_001));
    let (gateway, calls) = gateway(Ok("should never run"));
    let root = temp_root();
    let store = ReceiptStore::new(root.clone());
    let engine = ExecutionEngine::new(&gateway, &store);
    let spec = run_spec("too-large", vec![SourceRef::Note(eid(9))]);
    assert!(matches!(engine.run(&workspace, &spec), Err(ExecutionError::ContextTooLarge)));
    assert!(matches!(
        engine.run(&workspace, &run_spec("../escape", vec![])),
        Err(ExecutionError::InvalidRunId)
    ));
    assert!(calls.lock().unwrap().is_empty());
    assert!(!root.parent().unwrap().join("escape.json").exists());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn importing_real_utf8_file_snapshots_content_and_rejects_nontext() {
    let root = temp_root();
    let source = root.join("report.txt");
    std::fs::write(&source, "Imported evidence").unwrap();
    let mut workspace = Workspace::create("import");
    workspace.import_text_file(eid(10), &source).unwrap();
    assert_eq!(workspace.files[&eid(10)].name, "report.txt");
    assert_eq!(workspace.files[&eid(10)].content, "Imported evidence");
    std::fs::write(&source, [0xff, 0xfe]).unwrap();
    assert!(workspace.import_text_file(eid(11), &source).is_err());
    assert_eq!(workspace.files[&eid(10)].content, "Imported evidence");
    std::fs::remove_dir_all(root).unwrap();
}
