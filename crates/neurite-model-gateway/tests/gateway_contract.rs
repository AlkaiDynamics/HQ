use neurite_model_gateway::{
    AttemptFailure, ChatMessage, Gateway, GatewayError, InferenceRequest, ModelSpec,
    OllamaAdapter, OpenAiCompatibleAdapter, ProviderAdapter, ProviderError,
    ProviderErrorKind,
};
use neurite_protocol::{CapabilityId, ProtocolId};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn id(value: &str) -> ProtocolId {
    ProtocolId::new(value).unwrap()
}

fn request(candidates: Vec<ModelSpec>) -> InferenceRequest {
    InferenceRequest::new(vec![ChatMessage::user("Explain this node")], candidates)
}

fn target(provider: &str, model: &str) -> ModelSpec {
    ModelSpec::new(id(provider), model).unwrap()
}

struct FakeAdapter {
    models: Vec<String>,
    outcome: Result<String, ProviderError>,
    calls: Arc<Mutex<Vec<String>>>,
}

impl ProviderAdapter for FakeAdapter {
    fn discover(&self) -> Result<Vec<String>, ProviderError> {
        Ok(self.models.clone())
    }

    fn generate(&self, model: &str, _messages: &[ChatMessage]) -> Result<String, ProviderError> {
        self.calls.lock().unwrap().push(model.to_owned());
        self.outcome.clone()
    }
}

fn fake(outcome: Result<&str, ProviderError>) -> (FakeAdapter, Arc<Mutex<Vec<String>>>) {
    let calls = Arc::new(Mutex::new(Vec::new()));
    (
        FakeAdapter {
            models: vec!["test-model".into()],
            outcome: outcome.map(str::to_owned),
            calls: calls.clone(),
        },
        calls,
    )
}

#[test]
fn discovery_does_not_invent_model_capabilities() {
    let (adapter, _) = fake(Ok("ready"));
    let mut gateway = Gateway::new();
    gateway.register(id("local"), adapter);
    let models = gateway.discover(&id("local")).unwrap();

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].model, "test-model");
    assert!(models[0].capabilities.is_empty());
}

#[test]
fn transient_failure_switches_provider_and_preserves_candidate_provenance() {
    let mut gateway = Gateway::new();
    let (failed, failed_calls) = fake(Err(ProviderError::new(
        ProviderErrorKind::Transport,
        "connection refused",
    )));
    let (working, working_calls) = fake(Ok("gateway result"));
    gateway.register(id("local"), failed);
    gateway.register(id("cloud"), working);

    let result = gateway
        .infer(&request(vec![
            target("local", "alpha"),
            target("cloud", "beta"),
        ]))
        .unwrap();

    assert_eq!(result.text, "gateway result");
    assert_eq!(result.selected.provider, id("cloud"));
    assert_eq!(result.selected.model, "beta");
    assert_eq!(result.failed_attempts.len(), 1);
    assert_eq!(failed_calls.lock().unwrap().as_slice(), ["alpha"]);
    assert_eq!(working_calls.lock().unwrap().as_slice(), ["beta"]);
}

#[test]
fn authentication_error_stops_instead_of_silently_trying_another_provider() {
    let mut gateway = Gateway::new();
    let (failed, _) = fake(Err(ProviderError::new(
        ProviderErrorKind::Authentication,
        "not authorized",
    )));
    let (other, other_calls) = fake(Ok("would succeed"));
    gateway.register(id("broken"), failed);
    gateway.register(id("other"), other);

    let result = gateway.infer(&request(vec![
        target("broken", "m1"),
        target("other", "m2"),
    ]));

    assert!(matches!(result, Err(GatewayError::Stopped(_))));
    assert!(other_calls.lock().unwrap().is_empty());
}

#[test]
fn capability_requirements_never_fall_through_to_unknown_models() {
    let mut gateway = Gateway::new();
    let (unknown, unknown_calls) = fake(Ok("not safe for vision"));
    let (vision, vision_calls) = fake(Ok("vision enabled"));
    gateway.register(id("unknown"), unknown);
    gateway.register(id("vision"), vision);
    let requirement = CapabilityId::new("model.vision.input").unwrap();

    let result = gateway
        .infer(
            &request(vec![
                target("unknown", "unknown-model"),
                target("vision", "vision-model").with_capability(requirement.clone()),
            ])
            .require(requirement),
        )
        .unwrap();

    assert_eq!(result.selected.provider, id("vision"));
    assert!(matches!(
        result.failed_attempts[0].failure,
        AttemptFailure::UnsupportedCapabilities(_)
    ));
    assert!(unknown_calls.lock().unwrap().is_empty());
    assert_eq!(vision_calls.lock().unwrap().as_slice(), ["vision-model"]);
}

#[test]
fn no_matching_capability_returns_exhausted_without_inference() {
    let mut gateway = Gateway::new();
    let (adapter, calls) = fake(Ok("not allowed"));
    gateway.register(id("local"), adapter);
    let result = gateway.infer(
        &request(vec![target("local", "generic")])
            .require(CapabilityId::new("model.audio.input").unwrap()),
    );
    assert!(matches!(result, Err(GatewayError::Exhausted(_))));
    assert!(calls.lock().unwrap().is_empty());
}

#[test]
fn invalid_model_name_is_rejected_at_registration_boundary() {
    assert!(ModelSpec::new(id("local"), " ").is_err());
    assert!(ModelSpec::new(id("local"), "model\nspoof").is_err());
}

fn mock_http_once(body: &'static str) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let worker = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut bytes = [0_u8; 8192];
        let size = stream.read(&mut bytes).unwrap();
        assert!(size > 0);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).unwrap();
    });
    (url, worker)
}

#[test]
fn ollama_adapter_parses_real_http_chat_response() {
    let (url, worker) = mock_http_once(r#"{"message":{"role":"assistant","content":"hello from ollama"}}"#);
    let adapter = OllamaAdapter::new(&url).unwrap();
    let text = adapter.generate("qwen3:8b", &[ChatMessage::user("hello")]).unwrap();
    worker.join().unwrap();
    assert_eq!(text, "hello from ollama");
}

#[test]
fn openai_compatible_adapter_parses_real_http_completion_response() {
    let (url, worker) = mock_http_once(r#"{"choices":[{"message":{"role":"assistant","content":"hello from openai"}}]}"#);
    let adapter = OpenAiCompatibleAdapter::new(&url, None).unwrap();
    let text = adapter.generate("gpt-test", &[ChatMessage::user("hello")]).unwrap();
    worker.join().unwrap();
    assert_eq!(text, "hello from openai");
}
