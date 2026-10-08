# HQ Model Gateway — First Working Slice

**Status:** Rust workspace tests, formatting and Clippy passed in GitHub Actions on
2026-10-07 (27 passing tests across the workspace). Live inference against a user's
real model/provider installations remains unverified.

## Boundaries

- `neurite-model-gateway` is a Rust workspace crate; neither Ollama nor Jev is a
  dependency of the HQ kernel, scene, workspace, control, or protocol crates.
- `Gateway` accepts ordered, explicitly named provider/model candidates,
  checks trusted capability declarations, and records failures and selection.
- `ProviderAdapter` is the extensibility contract. The initial adapters are
  `OllamaAdapter` (`/api/tags`, `/api/chat`) and
  `OpenAiCompatibleAdapter` (`/models`, `/chat/completions`).
- OpenAI-compatible servers can include OpenAI, LM Studio, llama.cpp server,
  and vLLM **only when the deployment actually implements the corresponding
  Chat Completions endpoints**. Each installation requires its own verification.
- This slice supports **synchronous, text-only, non-streamed chat**. It does
  not implement tool calling, media, embeddings, token accounting, provider-side
  cancellation, retries within a provider, or HTTP streaming. Do not treat
  the open adapter boundary as evidence that those capabilities work.
- Native Anthropic and Google/Gemini APIs are **not yet implemented**.
- Jev is a **future routing-policy adapter**: it may rank or supply the candidate
  list but must not acquire ownership of HQ's runtime, state, or admission.
- `discover` returns model names with **empty capability assertions**.
  Trusted host configuration must explicitly certify capabilities before a
  requirement-gated route may select a model.
- Candidate order is caller-supplied, not an automatic quality score.
  Fallback only follows transport errors, HTTP 408/429, or HTTP 5xx;
  auth failures, malformed payloads and unsupported responses stop.
  Tool-effect replay is explicitly outside this slice.

## Local smoke test

```sh
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings

# Example: local Ollama. Prompt is passed via stdin, not a command-line argument.
export HQ_PROVIDER=ollama
export HQ_MODEL=qwen3:8b
export HQ_BASE_URL=http://127.0.0.1:11434
printf 'Give one short sentence.\n' | cargo run -p neurite-model-gateway --example infer

# Example: OpenAI-compatible endpoint. Set HQ_MODEL to a model supported by the endpoint.
export HQ_PROVIDER=openai
export HQ_MODEL=your-model-id
export HQ_BASE_URL=http://127.0.0.1:1234/v1
printf 'Give one short sentence.\n' | cargo run -p neurite-model-gateway --example infer

# Cloud OpenAI example: use HQ_BASE_URL=https://api.openai.com/v1
# and HQ_API_KEY from a secret store/environment, never source control.
```

For a local-vs-cloud switching gate, configure two `ProviderAdapter` registrations,
supply two `ModelSpec` candidates, then verify selected provider/model and
`failed_attempts` in the returned `InferenceResult`. Provider discovery does not
automatically enable selection. Tests cover fallback ordering and capability rejection.

## Release gates not yet closed

1. Actual Ollama inference observed against the user's local installation.
2. Actual OpenAI-compatible inference observed against a separate configured endpoint.
3. Inject a transient outage and verify fallback with recorded provenance outside test doubles.
4. Persist provider configuration (without secrets), run receipts and discovered models.
5. Wire HQ notes/files to context assembly, then run from the native UI.
6. Extend protocol-level capability metadata and provider-specific models as required.
7. Integrate Jev as an optional policy component after verifying its current interface.

**Security:** Only trusted administrators should configure provider URLs or key references.
Do not let agent-supplied prompts change provider endpoints, secret material, or
capability declarations. Production integrations need secret-store references,
HTTP host restrictions, request cancellation and audit-safe error handling.

## Second integration slice: provider wire fallback and receipt recovery

- `neurite-execution/tests/provider_wire.rs` covers the complete chain
  (selected workspace note -> Ollama-shaped HTTP 503 -> OpenAI-compatible
  HTTP response -> selected provider and failed-attempt receipt -> reopen).
  The test uses **loopback HTTP fixtures**: real request/response handling,
  not real model inference.
- `ReceiptStore::status` now distinguishes `Missing`, `Pending`, and
  `Complete`. A pending claim surviving interruption produces
  `ExecutionError::PendingRun`, never silently `None`. Recovery is
  intentionally **manual** until replay safety can be established.
- CI now runs automatically from the draft PR only, with concurrency
  cancellation for superseded runs; standalone manual workflow dispatch
  remains available. This avoids duplicate push/PR runs for one commit.
- This does **not** confirm two live configured providers, prompt redaction
  through Jev, native 2D/3D UI integration, or transaction-safe workspace
  persistence. These remain release gates.

## Jev privacy gate

HQ's headless CLI never sends the entire user prompt to Jev as a routing
summary. To opt into Jev, configure both `HQ_JEV_KEY_FILE` (the local
relay's bearer credential file) and a separately authored, bounded
`HQ_JEV_TASK_SUMMARY` (single line, at most 1000 characters). If the
routing summary is absent or invalid, `--agent-run` fails before making
a router or model request; there is no automatic prompt fallback.

Example PowerShell configuration, with no secrets embedded in commands:

```powershell
$env:HQ_JEV_KEY_FILE = 'C:\path\to\jev-caller-secret'
$env:HQ_JEV_TASK_SUMMARY = 'Routine short-answer text question with selected reference notes'
```

`--agent-preview` never calls the Jev endpoint, even when Jev is
configured. Candidate order displayed during preview is the static order;
a Jev-assisted run can change it at execution time. The router receives
the explicit task summary and eligible model descriptors, not workspace
notes, attached file contents or the prompt supplied through stdin.
Operators remain responsible for ensuring their chosen task summary
does not itself disclose sensitive material.

Unit and CLI integration tests cover rejection of absent, oversized and
multiline summaries and the offline preview path; no real Jev or model
service has yet been exercised.

## Current priority: direct live-provider proof

Jev is an optional routing enhancement and **not** a prerequisite for HQ.
Do not configure `HQ_JEV_KEY_FILE` for the first working agent run. Prove
workspace notes/files -> a real configured inference provider -> model response
-> receipt saved and read back before extending Jev or model routing.

A reproducible **Windows PowerShell** smoke procedure for a running local
OpenAI-compatible server (e.g., LM Studio with its chat endpoint enabled):

```powershell
# Run these from the HQ repository root, with Rust/Cargo installed.
Remove-Item Env:HQ_JEV_KEY_FILE -ErrorAction SilentlyContinue
$env:HQ_PROVIDER = 'openai'
$env:HQ_BASE_URL = 'http://127.0.0.1:1234/v1'
$env:HQ_MODEL = 'REPLACE_WITH_ACTUALLY_LOADED_MODEL_ID'
$env:HQ_NOTE_TEXT = 'HQ live smoke reference note'
$env:HQ_RUN_ID = 'hq-live-' + [guid]::NewGuid().ToString('N')
'Using the selected note, reply in exactly one sentence.' |
    cargo run --locked -p neurite-app -- --agent-preview
'Using the selected note, reply in exactly one sentence.' |
    cargo run --locked -p neurite-app -- --agent-run
Get-Content (Join-Path '.hq/receipts' ($env:HQ_RUN_ID + '.json')) -Raw
```

Change `HQ_PROVIDER` to `ollama`, `HQ_BASE_URL` to
`http://127.0.0.1:11434`, and `HQ_MODEL` to an actually loaded Ollama
model to test that alternative. These commands are **instructions**,
not evidence that a user's installed provider has responded. Any failed
run must preserve its failure receipt; do not silently retry with the
same run ID. If `HQ_JEV_KEY_FILE` is set, the optional Jev policy is
enabled for `--agent-run`, so remove it for the baseline proof.

The first release acceptance criterion is an observed real provider
response alongside a readable `succeeded` receipt that names the
selected provider/model. Do not call that verified based on fixture tests.
