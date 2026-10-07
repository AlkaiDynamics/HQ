use neurite_model_gateway::{ChatMessage, InferenceRequest, JevAskPolicy, ModelSpec, RouteError};
use neurite_protocol::{CapabilityId, ProtocolId};
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn model(provider: &str, name: &str) -> ModelSpec {
    ModelSpec::new(ProtocolId::new(provider).unwrap(), name).unwrap()
}
fn fixture(body: &'static str) -> (String, thread::JoinHandle<String>) {
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", socket.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let (mut conn, _) = socket.accept().unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 8192];
        loop {
            let n = conn.read(&mut buffer).unwrap();
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..n]);
            if let Some(offset) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                let header = String::from_utf8_lossy(&request[..offset]);
                let length = header
                    .lines()
                    .find_map(|line| {
                        line.to_lowercase()
                            .strip_prefix("content-length: ")
                            .and_then(|x| x.trim().parse::<usize>().ok())
                    })
                    .unwrap_or(0);
                if request.len() >= offset + 4 + length {
                    break;
                }
            }
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body
        );
        conn.write_all(response.as_bytes()).unwrap();
        String::from_utf8(request).unwrap()
    });
    (url, handle)
}

#[test]
fn jev_ask_reorders_only_eligible_models_and_omits_workspace_content() {
    let (url, handle) = fixture(r#"{"answers":{"model":{"choice":"m1","confidence":0.88}}}"#);
    let policy = JevAskPolicy::new(&url, "local-secret").unwrap();
    let available = vec![
        model("generic", "no-vision"),
        model("local", "vision-a")
            .with_capability(CapabilityId::new("model.vision.input").unwrap()),
        model("cloud", "vision-b")
            .with_capability(CapabilityId::new("model.vision.input").unwrap()),
    ];
    let req = InferenceRequest::new(
        vec![
            ChatMessage::system("PRIVATE DOCUMENT NEVER SENT TO JEV"),
            ChatMessage::user("Analyze image"),
        ],
        available,
    )
    .require(CapabilityId::new("model.vision.input").unwrap());

    let chosen = policy.rank(&req, "Pick a capable model").unwrap();
    assert_eq!(chosen.candidates[0].provider.as_str(), "cloud");
    assert_eq!(chosen.candidates[1].provider.as_str(), "generic");
    let wire = handle.join().unwrap();
    assert!(wire.starts_with("POST /ask HTTP/1.1"));
    assert!(wire
        .to_lowercase()
        .contains("authorization: bearer local-secret"));
    assert!(!wire.contains("PRIVATE DOCUMENT"));
    let body: Value = serde_json::from_str(wire.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(body["questions"]["model"]["type"], "choice");
    assert_eq!(
        body["questions"]["model"]["criteria"]
            .as_object()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn malicious_or_unknown_jev_decision_is_rejected() {
    let (url, handle) = fixture(r#"{"answers":{"model":{"choice":"m999"}}}"#);
    let policy = JevAskPolicy::new(&url, "key").unwrap();
    let req = InferenceRequest::new(vec![ChatMessage::user("hi")], vec![model("local", "one")]);
    assert!(matches!(
        policy.rank(&req, "hi"),
        Err(RouteError::InvalidDecision)
    ));
    handle.join().unwrap();
}

#[test]
fn jev_endpoint_requires_loopback_and_authentication() {
    assert!(JevAskPolicy::new("https://example.com", "key").is_err());
    assert!(JevAskPolicy::new("http://127.0.0.1:4319", "").is_err());
}

#[test]
fn jev_policy_fails_closed_if_no_model_satisfies_required_capabilities() {
    let policy = JevAskPolicy::new("http://127.0.0.1:4319", "key").unwrap();
    let req = InferenceRequest::new(
        vec![ChatMessage::user("vision")],
        vec![model("local", "plain")],
    )
    .require(CapabilityId::new("model.vision.input").unwrap());
    assert!(matches!(
        policy.rank(&req, "vision"),
        Err(RouteError::NoEligibleModels)
    ));
}
