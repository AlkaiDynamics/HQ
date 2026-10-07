//! Live, opt-in smoke probe (not a mock). Credentials are read from the
//! environment and never printed. Supply the prompt on stdin.
//!
//! Ollama:
//!   HQ_PROVIDER=ollama HQ_MODEL=qwen3:8b cargo run -p neurite-model-gateway --example infer
//! OpenAI-compatible:
//!   HQ_PROVIDER=openai HQ_MODEL=<model> HQ_BASE_URL=https://api.openai.com/v1 \
//!     HQ_API_KEY=<secret> cargo run -p neurite-model-gateway --example infer

use neurite_model_gateway::{
    ChatMessage, Gateway, InferenceRequest, ModelSpec, OllamaAdapter, OpenAiCompatibleAdapter,
};
use neurite_protocol::ProtocolId;
use std::env;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider_kind = env::var("HQ_PROVIDER").unwrap_or_else(|_| "ollama".into());
    let model = env::var("HQ_MODEL").map_err(|_| "HQ_MODEL must name an installed/accessible model")?;
    let provider = ProtocolId::new(format!("hq.model.provider.{provider_kind}"))
        .map_err(|_| "invalid HQ_PROVIDER identifier")?;

    let mut prompt = String::new();
    io::stdin().read_to_string(&mut prompt)?;
    if prompt.trim().is_empty() {
        return Err("write a non-empty prompt to stdin".into());
    }

    let mut gateway = Gateway::new();
    match provider_kind.as_str() {
        "ollama" => {
            let root = env::var("HQ_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:11434".into());
            gateway.register(provider.clone(), OllamaAdapter::new(&root).map_err(|error| error.detail)?);
        }
        "openai" => {
            let root = env::var("HQ_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".into());
            gateway.register(
                provider.clone(),
                OpenAiCompatibleAdapter::new(&root, env::var("HQ_API_KEY").ok())
                    .map_err(|error| error.detail)?,
            );
        }
        _ => return Err("HQ_PROVIDER must be ollama or openai".into()),
    }

    let result = gateway
        .infer(&InferenceRequest::new(
            vec![ChatMessage::user(prompt)],
            vec![ModelSpec::new(provider, model).map_err(|_| "invalid HQ_MODEL")?],
        ))
        .map_err(|error| format!("model gateway failure: {error:?}"))?;
    println!("{}", result.text);
    eprintln!("provider={} model={}", result.selected.provider.as_str(), result.selected.model);
    Ok(())
}
