#![forbid(unsafe_code)]

use neurite_core::{EntityId, IdNamespace, SpatialState};
use neurite_execution::{ExecutionEngine, ReceiptStore, RunSpec, SourceRef};
use neurite_model_gateway::{
    ChatMessage, Gateway, InferenceRequest, JevAskPolicy, ModelSpec, OllamaAdapter,
    OpenAiCompatibleAdapter,
};
use neurite_protocol::ProtocolId;
use neurite_scene::SpatialEntity;
use neurite_workspace::Workspace;
use std::env;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("--agent-preview") => {
            if let Err(error) = agent_command(false) {
                eprintln!("HQ agent preview error: {error}");
                std::process::exit(1);
            }
        }
        Some("--agent-run") => {
            if let Err(error) = agent_command(true) {
                eprintln!("HQ agent run error: {error}");
                std::process::exit(1);
            }
        }
        Some(_) => {
            eprintln!("Usage: neurite-app [--agent-preview|--agent-run]");
            std::process::exit(2);
        }
        None => original_kernel_demo(),
    }
}

fn original_kernel_demo() {
    let mut workspace = Workspace::create("local-default");
    let entity = SpatialEntity::new(
        EntityId::scoped(IdNamespace::new(1), 1),
        Some(SpatialState::default()),
    );
    let entity_id = workspace
        .scene
        .add_entity(entity)
        .expect("new entity id must be unique");
    workspace.set_note(entity_id, "HQ native kernel is alive.");
    println!(
        "workspace={} entities={} dirty={}",
        workspace.id,
        workspace.scene.entities().count(),
        workspace.is_dirty()
    );
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("required setting {name} is missing"))
}

fn register_provider(
    gateway: &mut Gateway,
    slot: &str,
    kind: &str,
    base_url: &str,
    model: &str,
    key: Option<String>,
) -> Result<ModelSpec, String> {
    let provider = ProtocolId::new(format!("hq.provider.{kind}.{slot}"))
        .map_err(|_| "invalid provider kind")?;
    match kind {
        "ollama" => gateway.register(
            provider.clone(),
            OllamaAdapter::new(base_url).map_err(|error| error.detail)?,
        ),
        "openai" => gateway.register(
            provider.clone(),
            OpenAiCompatibleAdapter::new(base_url, key).map_err(|error| error.detail)?,
        ),
        _ => return Err("HQ_PROVIDER must be ollama or openai".into()),
    }
    ModelSpec::new(provider, model).map_err(|error| format!("invalid model: {error:?}"))
}

fn default_url(kind: &str) -> &'static str {
    match kind {
        "ollama" => "http://127.0.0.1:11434",
        _ => "https://api.openai.com/v1",
    }
}

fn agent_command(execute: bool) -> Result<(), String> {
    let mut prompt = String::new();
    io::stdin()
        .read_to_string(&mut prompt)
        .map_err(|error| error.to_string())?;
    if prompt.trim().is_empty() {
        return Err("provide a non-empty agent prompt through stdin".into());
    }

    let mut workspace =
        Workspace::create(env::var("HQ_WORKSPACE_ID").unwrap_or_else(|_| "native-cli".into()));
    let mut sources = Vec::new();
    if let Ok(note) = env::var("HQ_NOTE_TEXT") {
        let id = EntityId::scoped(IdNamespace::new(1), 1001);
        workspace.set_note(id, note);
        sources.push(SourceRef::Note(id));
    }
    if let Ok(path) = env::var("HQ_TEXT_FILE") {
        let id = EntityId::scoped(IdNamespace::new(1), 1002);
        workspace
            .import_text_file(id, Path::new(&path))
            .map_err(|error| format!("cannot import HQ_TEXT_FILE: {error:?}"))?;
        sources.push(SourceRef::File(id));
    }

    let mut gateway = Gateway::new();
    let primary_kind = env::var("HQ_PROVIDER").unwrap_or_else(|_| "ollama".into());
    let primary_model = required_env("HQ_MODEL")?;
    let primary_url = env::var("HQ_BASE_URL").unwrap_or_else(|_| default_url(&primary_kind).into());
    let mut candidates = vec![register_provider(
        &mut gateway,
        "primary",
        &primary_kind,
        &primary_url,
        &primary_model,
        env::var("HQ_API_KEY").ok(),
    )?];

    if let Ok(other_kind) = env::var("HQ_SECONDARY_PROVIDER") {
        let other_model = required_env("HQ_SECONDARY_MODEL")?;
        let other_url =
            env::var("HQ_SECONDARY_BASE_URL").unwrap_or_else(|_| default_url(&other_kind).into());
        candidates.push(register_provider(
            &mut gateway,
            "secondary",
            &other_kind,
            &other_url,
            &other_model,
            env::var("HQ_SECONDARY_API_KEY").ok(),
        )?);
    }

    let run_id = env::var("HQ_RUN_ID").unwrap_or_else(|_| "preview".into());
    let mut spec = RunSpec::new(run_id, prompt.clone(), sources, candidates);
    if let Ok(key_path) = env::var("HQ_JEV_KEY_FILE") {
        let key = std::fs::read_to_string(key_path)
            .map_err(|error| format!("cannot read HQ_JEV_KEY_FILE: {error}"))?;
        let url = env::var("HQ_JEV_URL").unwrap_or_else(|_| "http://127.0.0.1:4319".into());
        let policy = JevAskPolicy::new(&url, key.trim())
            .map_err(|error| format!("invalid Jev configuration: {error:?}"))?;
        let ranked = policy
            .rank(
                &InferenceRequest::new(
                    vec![ChatMessage::user(prompt.clone())],
                    spec.candidates.clone(),
                ),
                &prompt,
            )
            .map_err(|error| format!("Jev routing rejected: {error:?}"))?;
        spec.candidates = ranked.candidates;
    }

    let receipts =
        ReceiptStore::new(env::var("HQ_RECEIPTS_DIR").unwrap_or_else(|_| ".hq/receipts".into()));
    let engine = ExecutionEngine::new(&gateway, &receipts);
    if !execute {
        let context = engine
            .preview(&workspace, &spec)
            .map_err(|error| format!("{error:?}"))?;
        for message in context.messages {
            println!("=== {:?} ===\n{}", message.role, message.content);
        }
        println!("=== SELECTED CANDIDATE ORDER ===");
        for candidate in spec.candidates {
            println!("{} / {}", candidate.provider.as_str(), candidate.model);
        }
        return Ok(());
    }

    let receipt = engine
        .run(&workspace, &spec)
        .map_err(|error| format!("{error:?}"))?;
    println!("{}", receipt["output"].as_str().unwrap_or(""));
    eprintln!(
        "HQ receipt={} provider={} model={}",
        spec.id, receipt["selected"]["provider"], receipt["selected"]["model"]
    );
    Ok(())
}
