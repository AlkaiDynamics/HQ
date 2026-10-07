#![forbid(unsafe_code)]

//! Transport-independent inference routing. Adapters own network specifics; the
//! gateway owns capability admission, provider selection, and failure provenance.
//! Discovered model names do not imply verified capabilities.

use neurite_protocol::{CapabilityId, ProtocolId};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl MessageRole {
    fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpec {
    pub provider: ProtocolId,
    pub model: String,
    /// Explicitly certified by trusted configuration. Discovery alone sets none.
    pub capabilities: BTreeSet<CapabilityId>,
}

impl ModelSpec {
    pub fn new(provider: ProtocolId, model: impl Into<String>) -> Result<Self, GatewayError> {
        let model = model.into();
        if model.trim().is_empty() || model.len() > 256 || model.chars().any(char::is_control) {
            return Err(GatewayError::InvalidRequest("invalid model identifier"));
        }
        Ok(Self {
            provider,
            model,
            capabilities: BTreeSet::new(),
        })
    }

    pub fn with_capability(mut self, capability: CapabilityId) -> Self {
        self.capabilities.insert(capability);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceRequest {
    pub messages: Vec<ChatMessage>,
    /// Ordered candidates, potentially supplied by a future Jev policy adapter.
    pub candidates: Vec<ModelSpec>,
    pub required_capabilities: BTreeSet<CapabilityId>,
}

impl InferenceRequest {
    pub fn new(messages: Vec<ChatMessage>, candidates: Vec<ModelSpec>) -> Self {
        Self {
            messages,
            candidates,
            required_capabilities: BTreeSet::new(),
        }
    }

    pub fn require(mut self, capability: CapabilityId) -> Self {
        self.required_capabilities.insert(capability);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderErrorKind {
    Transport,
    RateLimited,
    Server,
    Authentication,
    BadRequest,
    UnsupportedResponse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderError {
    pub kind: ProviderErrorKind,
    pub detail: String,
}

impl ProviderError {
    pub fn new(kind: ProviderErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn retryable(&self) -> bool {
        matches!(
            self.kind,
            ProviderErrorKind::Transport
                | ProviderErrorKind::RateLimited
                | ProviderErrorKind::Server
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttemptFailure {
    UnknownProvider,
    UnsupportedCapabilities(Vec<CapabilityId>),
    Provider(ProviderError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedAttempt {
    pub candidate: ModelSpec,
    pub failure: AttemptFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayError {
    InvalidRequest(&'static str),
    UnknownProvider(ProtocolId),
    Discovery(ProviderError),
    Stopped(FailedAttempt),
    Exhausted(Vec<FailedAttempt>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceResult {
    pub text: String,
    pub selected: ModelSpec,
    pub failed_attempts: Vec<FailedAttempt>,
}

pub trait ProviderAdapter: Send + Sync {
    fn discover(&self) -> Result<Vec<String>, ProviderError>;
    fn generate(&self, model: &str, messages: &[ChatMessage]) -> Result<String, ProviderError>;
}

#[derive(Default)]
pub struct Gateway {
    providers: BTreeMap<ProtocolId, Box<dyn ProviderAdapter>>,
}

impl Gateway {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, provider: ProtocolId, adapter: impl ProviderAdapter + 'static) {
        self.providers.insert(provider, Box::new(adapter));
    }

    /// Returns discovered names with no fabricated modality/tool capabilities.
    pub fn discover(&self, provider: &ProtocolId) -> Result<Vec<ModelSpec>, GatewayError> {
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| GatewayError::UnknownProvider(provider.clone()))?;
        adapter
            .discover()
            .map_err(GatewayError::Discovery)?
            .into_iter()
            .map(|model| ModelSpec::new(provider.clone(), model))
            .collect()
    }

    /// Synchronous text-only inference. Retry only transport, rate-limit and
    /// server failures; never replay a potentially successful tool invocation.
    pub fn infer(&self, request: &InferenceRequest) -> Result<InferenceResult, GatewayError> {
        if request.candidates.is_empty() {
            return Err(GatewayError::InvalidRequest(
                "at least one candidate is required",
            ));
        }
        if request.messages.is_empty()
            || !request
                .messages
                .iter()
                .any(|message| message.role == MessageRole::User)
            || request
                .messages
                .iter()
                .any(|message| message.content.trim().is_empty())
        {
            return Err(GatewayError::InvalidRequest(
                "non-empty messages and at least one user message required",
            ));
        }

        let mut failed_attempts = Vec::new();
        for candidate in &request.candidates {
            let missing: Vec<_> = request
                .required_capabilities
                .difference(&candidate.capabilities)
                .cloned()
                .collect();
            if !missing.is_empty() {
                failed_attempts.push(FailedAttempt {
                    candidate: candidate.clone(),
                    failure: AttemptFailure::UnsupportedCapabilities(missing),
                });
                continue;
            }
            let Some(adapter) = self.providers.get(&candidate.provider) else {
                failed_attempts.push(FailedAttempt {
                    candidate: candidate.clone(),
                    failure: AttemptFailure::UnknownProvider,
                });
                continue;
            };

            match adapter.generate(&candidate.model, &request.messages) {
                Ok(text) if !text.trim().is_empty() => {
                    return Ok(InferenceResult {
                        text,
                        selected: candidate.clone(),
                        failed_attempts,
                    });
                }
                Ok(_) => {
                    return Err(GatewayError::Stopped(FailedAttempt {
                        candidate: candidate.clone(),
                        failure: AttemptFailure::Provider(ProviderError::new(
                            ProviderErrorKind::UnsupportedResponse,
                            "provider returned empty text",
                        )),
                    }));
                }
                Err(error) => {
                    let retryable = error.retryable();
                    let attempt = FailedAttempt {
                        candidate: candidate.clone(),
                        failure: AttemptFailure::Provider(error),
                    };
                    if !retryable {
                        return Err(GatewayError::Stopped(attempt));
                    }
                    failed_attempts.push(attempt);
                }
            }
        }
        Err(GatewayError::Exhausted(failed_attempts))
    }
}

/// HTTP details are confined to provider adapters; the router depends only on
/// ProviderAdapter and can be reused by desktop, service or Android clients.
struct HttpJson {
    root: String,
    key: Option<String>,
    agent: ureq::Agent,
}

impl HttpJson {
    fn new(root: &str, key: Option<String>) -> Result<Self, ProviderError> {
        let root = root.trim_end_matches('/');
        if !(root.starts_with("http://") || root.starts_with("https://"))
            || root.contains(char::is_whitespace)
            || root.contains('@')
        {
            return Err(ProviderError::new(
                ProviderErrorKind::BadRequest,
                "invalid provider base URL",
            ));
        }
        Ok(Self {
            root: root.to_owned(),
            key,
            agent: ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(60))
                .build(),
        })
    }

    fn get(&self, path: &str) -> Result<Value, ProviderError> {
        let mut request = self.agent.get(&format!("{}{}", self.root, path));
        if let Some(key) = &self.key {
            request = request.set("Authorization", &format!("Bearer {key}"));
        }
        parse_http_response(request.call())
    }

    fn post(&self, path: &str, payload: Value) -> Result<Value, ProviderError> {
        let mut request = self
            .agent
            .post(&format!("{}{}", self.root, path))
            .set("Content-Type", "application/json");
        if let Some(key) = &self.key {
            request = request.set("Authorization", &format!("Bearer {key}"));
        }
        parse_http_response(request.send_string(&payload.to_string()))
    }
}

fn parse_http_response(
    response: Result<ureq::Response, ureq::Error>,
) -> Result<Value, ProviderError> {
    let response = response.map_err(|error| match error {
        ureq::Error::Status(code, _) => {
            let kind = match code {
                401 | 403 => ProviderErrorKind::Authentication,
                408 | 429 => ProviderErrorKind::RateLimited,
                500..=599 => ProviderErrorKind::Server,
                _ => ProviderErrorKind::BadRequest,
            };
            ProviderError::new(kind, format!("provider HTTP status {code}"))
        }
        ureq::Error::Transport(error) => {
            ProviderError::new(ProviderErrorKind::Transport, error.to_string())
        }
    })?;

    let mut body = String::new();
    response
        .into_reader()
        .take(4 * 1024 * 1024)
        .read_to_string(&mut body)
        .map_err(|error| ProviderError::new(ProviderErrorKind::Transport, error.to_string()))?;
    serde_json::from_str(&body).map_err(|_| {
        ProviderError::new(
            ProviderErrorKind::UnsupportedResponse,
            "provider returned invalid or oversized JSON",
        )
    })
}

fn message_json(messages: &[ChatMessage]) -> Vec<Value> {
    messages
        .iter()
        .map(|message| json!({"role": message.role.as_str(), "content": message.content}))
        .collect()
}

fn text_field<'a>(data: &'a Value, path: &[&str]) -> Option<&'a str> {
    path.iter()
        .try_fold(data, |value, field| value.get(*field))?
        .as_str()
}

pub struct OllamaAdapter {
    http: HttpJson,
}

impl OllamaAdapter {
    pub fn new(base_url: &str) -> Result<Self, ProviderError> {
        Ok(Self {
            http: HttpJson::new(base_url, None)?,
        })
    }
}

impl ProviderAdapter for OllamaAdapter {
    fn discover(&self) -> Result<Vec<String>, ProviderError> {
        let data = self.http.get("/api/tags")?;
        let models = data
            .get("models")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ProviderError::new(
                    ProviderErrorKind::UnsupportedResponse,
                    "missing Ollama models",
                )
            })?;
        models
            .iter()
            .map(|item| {
                item.get("name")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| {
                        ProviderError::new(
                            ProviderErrorKind::UnsupportedResponse,
                            "Ollama model missing name",
                        )
                    })
            })
            .collect()
    }

    fn generate(&self, model: &str, messages: &[ChatMessage]) -> Result<String, ProviderError> {
        let data = self.http.post(
            "/api/chat",
            json!({"model": model, "stream": false, "messages": message_json(messages)}),
        )?;
        text_field(&data, &["message", "content"])
            .map(str::to_owned)
            .ok_or_else(|| {
                ProviderError::new(
                    ProviderErrorKind::UnsupportedResponse,
                    "missing Ollama message.content",
                )
            })
    }
}

/// Covers OpenAI's Chat Completions wire format and compatible local servers.
/// It does not claim Anthropic, Gemini native APIs, Responses API or tool calls.
pub struct OpenAiCompatibleAdapter {
    http: HttpJson,
}

impl OpenAiCompatibleAdapter {
    /// base_url includes /v1 (for example https://api.openai.com/v1).
    pub fn new(base_url: &str, api_key: Option<String>) -> Result<Self, ProviderError> {
        Ok(Self {
            http: HttpJson::new(base_url, api_key)?,
        })
    }
}

impl ProviderAdapter for OpenAiCompatibleAdapter {
    fn discover(&self) -> Result<Vec<String>, ProviderError> {
        let data = self.http.get("/models")?;
        let models = data.get("data").and_then(Value::as_array).ok_or_else(|| {
            ProviderError::new(ProviderErrorKind::UnsupportedResponse, "missing model data")
        })?;
        models
            .iter()
            .map(|item| {
                item.get("id")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| {
                        ProviderError::new(
                            ProviderErrorKind::UnsupportedResponse,
                            "model missing id",
                        )
                    })
            })
            .collect()
    }

    fn generate(&self, model: &str, messages: &[ChatMessage]) -> Result<String, ProviderError> {
        let data = self.http.post(
            "/chat/completions",
            json!({"model": model, "stream": false, "messages": message_json(messages)}),
        )?;
        data.get("choices")
            .and_then(Value::as_array)
            .and_then(|choices| choices.first())
            .and_then(|choice| text_field(choice, &["message", "content"]))
            .map(str::to_owned)
            .ok_or_else(|| {
                ProviderError::new(
                    ProviderErrorKind::UnsupportedResponse,
                    "missing completion message.content",
                )
            })
    }
}


/// Jev is a decision model, not a text-generation backend. This policy calls
/// the authenticated loopback `/ask` surface from jev-codex-router and only
/// reorders candidates. All inference content still goes to the provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteError {
    InvalidEndpoint,
    NoEligibleModels,
    InvalidDecision,
    Transport(ProviderError),
}

pub struct JevAskPolicy {
    http: HttpJson,
}

impl JevAskPolicy {
    /// The Jev local relay must be configured separately and authenticated.
    /// Use only the loopback HTTP server; do not send its private key to a remote URL.
    pub fn new(base_url: &str, credential: &str) -> Result<Self, RouteError> {
        let host_port = base_url
            .trim_end_matches('/')
            .strip_prefix("http://")
            .ok_or(RouteError::InvalidEndpoint)?;
        let (host, port) = host_port.rsplit_once(':').ok_or(RouteError::InvalidEndpoint)?;
        if !matches!(host, "localhost" | "127.0.0.1")
            || port.parse::<u16>().ok().filter(|port| *port > 0).is_none()
            || credential.trim().is_empty()
            || credential.chars().any(char::is_control)
        {
            return Err(RouteError::InvalidEndpoint);
        }
        let http = HttpJson::new(base_url, Some(credential.to_owned()))
            .map_err(RouteError::Transport)?;
        Ok(Self { http })
    }

    /// Submits only a bounded task summary + eligible inventory to Jev.
    /// Never shares notes, files, conversation contents or credentials as state.
    /// Invalid decisions and unavailable Jev fail closed rather than silently
    /// taking an unapproved route. Static routing remains independently usable.
    pub fn rank(
        &self,
        request: &InferenceRequest,
        task_summary: &str,
    ) -> Result<InferenceRequest, RouteError> {
        if request.candidates.len() > 24 || task_summary.trim().is_empty() {
            return Err(RouteError::InvalidDecision);
        }
        let eligible: Vec<(usize, &ModelSpec)> = request
            .candidates
            .iter()
            .enumerate()
            .filter(|(_, model)| {
                request
                    .required_capabilities
                    .is_subset(&model.capabilities)
            })
            .collect();
        if eligible.is_empty() {
            return Err(RouteError::NoEligibleModels);
        }
        let mut criteria = serde_json::Map::new();
        for (index, (_, model)) in eligible.iter().enumerate() {
            criteria.insert(
                format!("m{index}"),
                Value::String(format!(
                    "provider={} model={} explicitly certified capabilities={}",
                    model.provider.as_str(),
                    model.model,
                    model
                        .capabilities
                        .iter()
                        .map(CapabilityId::as_str)
                        .collect::<Vec<_>>()
                        .join(",")
                )),
            );
        }
        let summary: String = task_summary.chars().take(1000).collect();
        let payload = json!({
            "state": {"task": summary, "purpose": "HQ model selection"},
            "questions": {
                "model": {
                    "type": "choice",
                    "instructions": "Choose the most appropriate available model for the current task, considering sufficient capability and cost. Choose exactly one supplied identifier. Do not invent candidates.",
                    "criteria": criteria
                }
            }
        });
        let data = self.http.post("/ask", payload).map_err(RouteError::Transport)?;
        let key = data
            .pointer("/answers/model/choice")
            .and_then(Value::as_str)
            .ok_or(RouteError::InvalidDecision)?;
        let index = key
            .strip_prefix('m')
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or(RouteError::InvalidDecision)?;
        // Reject unrecognized forms such as m01, even if the integer is in range.
        if key != format!("m{index}") {
            return Err(RouteError::InvalidDecision);
        }
        let candidate_index = eligible
            .get(index)
            .map(|(index, _)| *index)
            .ok_or(RouteError::InvalidDecision)?;
        let mut routed = request.clone();
        let selected = routed.candidates.remove(candidate_index);
        routed.candidates.insert(0, selected);
        Ok(routed)
    }
}
