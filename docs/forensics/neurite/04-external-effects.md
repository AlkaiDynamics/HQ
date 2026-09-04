# Neurite External Effect / Privilege Ledger

Specimen: `satellitecomponent/Neurite`

Pinned main commit: `e62b270402b688a39a864007b9c0a02711b9573e`

Pinned Electron commit: `ab38b3f8d7edb53ba78affff91f30295d0282701`

This document records externally consequential behavior found in source. It is a source-level privilege map, not runtime certification.

## Classification fields

- **Trigger** — user/system action that initiates the effect.
- **Actor** — component that actually performs the effect.
- **Privilege** — authority required.
- **Blast radius** — bounded effect domain.
- **Reversibility** — reversible / compensatable / destructive / unknown.
- **Auditability** — whether the legacy implementation creates a durable structured audit record.
- **Native boundary** — capability that should mediate the effect.

---

# E-001 — Arbitrary host filesystem enumeration/read

**Trigger:** FileTree navigation or file open.

**Actor:** `localhost_servers/direct-access/direct-access.js`.

**Effect:**

- resolves caller-supplied path with `path.resolve()`
- can start at filesystem root
- `fs.stat`
- `fs.readdir`
- `fs.createReadStream`
- streams text and binary files

**Privilege:** ambient read access of the sidecar process.

**Blast radius:** any filesystem path readable by the Node process.

**Reversibility:** read-only, but confidentiality impact can be system-wide.

**Auditability:** no durable structured access ledger observed.

**Native boundary:** `FileReadCapability { granted_roots, handles, read, enumerate, expiry, quota }`.

A persisted file path is not a capability grant.

---

# E-002 — Dynamic dependency installation and code loading

**Trigger:** local-server startup when dependencies are absent.

**Actor:** `localhost_servers/start_servers.js`.

**Effect:**

- checks service folders for dependency state
- runs `npm install` via `execSync`
- dynamically imports each configured service module
- mounts imported Express applications into one process

**Privilege:** process execution, network/package registry access, filesystem writes, module execution.

**Blast radius:** local Node runtime + service directories + permissions of launching user.

**Reversibility:** package files can be removed, but arbitrary install scripts/modules may have wider effects; therefore not inherently reversible.

**Auditability:** console output only; no structured immutable effect log observed.

**Native boundary:** NEVER part of trusted core startup. Legacy/package runtimes enter through isolated provision/verify/runtime capabilities.

---

# E-003 — Electron-side dependency installation / server loading

**Trigger:** desktop startup when downloaded localhost-server bundle lacks expected dependencies.

**Actor:** Electron `modules/servermanager.js`.

**Effect:**

- spawns `npm install --prefer-offline --no-audit`
- inherits `process.env`
- writes installation log into Electron userData
- dynamically imports downloaded/local `start_servers.js`
- health-checks localhost:7070

**Privilege:** process execution, environment access, filesystem read/write, dynamic code load, possible network.

**Blast radius:** desktop user account and Node/Electron process authority.

**Reversibility:** not guaranteed.

**Auditability:** installation log exists, but not a complete structured effect ledger.

**Native boundary:** `LegacyRuntimeProvisionCapability` + isolated process/container, explicit environment allowlist.

---

# E-004 — Browser automation / screenshot capture

**Trigger:** Automation sidecar startup / screenshot request.

**Actor:** Playwright Automation service.

**Effect:**

- launches configured Playwright browser
- navigates to Neurite URL
- marks page `startedViaPlaywright`
- captures viewport screenshot
- exposes screenshot as base64 over HTTP on port 8081
- response server sets `Access-Control-Allow-Origin: *`

**Privilege:** browser automation + screen/application data observation.

**Blast radius:** automated Neurite browser context; potential exposure of whatever is visible in it.

**Reversibility:** observational.

**Auditability:** console logging only.

**Native boundary:** retain initially as a **legacy golden-test capture capsule**, not a normal application capability. Production screenshot/automation should require explicit scoped grants.

---

# E-005 — Arbitrary JavaScript execution in page context

**Trigger:** Function Call panel Run or equivalent generated function-call flow.

**Actor:** `View.Code` in `js/interface/functioncall/functioncallingpanel.js`.

**Effect:** constructs `AsyncFunction(...)` from supplied code and executes it inside the Neurite renderer/page context.

**Privilege:** whatever ambient Web APIs/application globals are reachable from the renderer.

**Blast radius:** renderer state, application globals, network/storage capabilities available to page JS.

**Reversibility:** unknown/arbitrary.

**Auditability:** source/result/error history is persisted, but the external side effects of executed code are not comprehensively recorded.

**Native boundary:** `CodeExecutionCapability` through an isolated runtime. Execution history must not imply execution authority.

---

# E-006 — Python execution and package loading

**Trigger:** toggle/execute code on a text node containing Python code fences.

**Actor:** Pyodide integration in `js/nodes/nodeinteraction/bundlecode.js`.

**Effect:**

- dynamically imports Pyodide 0.23.0 from jsDelivr
- finds Python imports
- calls `pyodide.loadPackage(module)` for non-builtins
- executes code using `runPythonAsync`
- exposes a JavaScript callback (`window.outputHTML`) to Python

**Privilege:** code execution in Pyodide runtime + package/network acquisition + explicitly exposed JS bridge.

**Blast radius:** browser/Pyodide context and whatever bridge/import capabilities Pyodide exposes.

**Reversibility:** arbitrary code semantics.

**Auditability:** console/UI output, not full effect tracing.

**Native boundary:** isolated Python `RuntimeBinding`, explicit capability grants, package policy, no ambient host filesystem/network.

---

# E-007 — HTML/CSS/JavaScript execution from text-node code blocks

**Trigger:** execute code on text node with web code blocks.

**Actor:** `displayHTMLView()` / `bundleWebContent()`.

**Effect:** bundles code from the target node and connected nodes and assigns it to iframe `srcdoc`, including `<script>` content.

**Privilege:** browser iframe JavaScript execution. Source does not show a sandbox attribute being added to this iframe in this path.

**Blast radius:** iframe/browser surface; exact origin/parent-access behavior must be runtime-tested before stronger claims.

**Reversibility:** arbitrary within effective browser privileges.

**Auditability:** no structured effect ledger.

**Native boundary:** isolated WebRuntime/BrowserRuntime binding with explicit origin/network/storage policy.

---

# E-008 — Browser/webview navigation

**Trigger:** LinkNode open/search/back/forward/refresh/proxy-display.

**Actor:** browser iframe or Electron webview.

**Effect:** loads arbitrary resolved URL; Electron webview supports navigation history and page execution with configured sandbox/context isolation properties.

**Privilege:** network access + remote content rendering.

**Blast radius:** browser/webview runtime; may transmit requests/cookies/headers according to runtime policy.

**Reversibility:** navigation itself reversible; data disclosures/network side effects are not.

**Auditability:** URL/title state exists but no complete network audit trail.

**Native boundary:** `BrowserRuntimeBinding` + `NetworkCapability` + explicit persistent-session policy.

---

# E-009 — Raw web proxy / remote URL fetch

**Trigger:** LinkNode proxy display or web extraction/search workflows.

**Actor:** WebScrape sidecar.

**Routes/effects:**

- `GET /raw-proxy?url=` → fetch arbitrary supplied URL and return raw HTML
- `GET /proxy?url=` → fetch arbitrary supplied URL, parse HTML/PDF/plain text

**Privilege:** outbound network to caller-selected URLs.

**Blast radius:** network egress, exposure of server-side IP/network reachability, downloaded content processing.

**Reversibility:** network read is observational but data disclosure/request side effects are not necessarily reversible.

**Auditability:** console output only.

**Native boundary:** `WebFetchCapability { schemes, hosts, private_network_policy, byte_limit, timeout }`.

---

# E-010 — Local semantic/index database mutation

**Trigger:** webpage/embedding ingestion, additional embedding generation, delete-key action.

**Actor:** WebScrape sidecar SQLite `database.db`.

**Routes:**

- `GET /fetch-web-page-text`
- `POST /store-embedding-and-text`
- `POST /store-additional-embedding`
- `POST /fetch-embeddings-by-keys`
- `GET /get-keys`
- `DELETE /delete-chunks`

**Privilege:** local database read/write/delete.

**Blast radius:** WebScrape sidecar index database.

**Reversibility:** writes replace prior entries; deletion destructive unless separately backed up.

**Auditability:** no event-sourced change ledger observed.

**Native boundary:** local indexing subsystem with explicit source/content provenance and transactional mutation.

---

# E-011 — Google Custom Search

**Trigger:** enabled web search during AI/context workflow.

**Actor:** browser client.

**Endpoint:** `https://www.googleapis.com/customsearch/v1` with API key, search-engine ID, and query in URL parameters.

**Privilege:** external network + credential use + query disclosure.

**Blast radius:** search terms and credential-bearing request to Google API.

**Reversibility:** disclosure not reversible.

**Auditability:** no comprehensive durable query/effect ledger observed.

**Native boundary:** `SearchCapability` backed by a credential reference and explicit disclosure policy.

---

# E-012 — Wikipedia retrieval

**Trigger:** Wikipedia-enabled AI context assembly.

**Actor:** WikiSearch sidecar.

**Effect:** queries English Wikipedia APIs for search results, categories, and extracts; returns summaries to client.

**Privilege:** external network.

**Blast radius:** query terms disclosed to Wikipedia.

**Reversibility:** disclosure not reversible.

**Native boundary:** `KnowledgeRetrievalCapability` / general web-fetch provider.

---

# E-013 — Wolfram Alpha request

**Trigger:** Wolfram-enabled search/AI workflow.

**Actor:** WolframAlpha sidecar.

**Effect:** sends query and API key to `https://api.wolframalpha.com/v2/query`; can retry using Wolfram `didyoumeans` response up to five attempts.

**Privilege:** external network + credential use + query disclosure.

**Blast radius:** query/API account.

**Native boundary:** provider-scoped `ComputationCapability`.

---

# E-014 — GitHub repository content retrieval

**Trigger:** Git/code search/parser workflow.

**Actor:** browser client code parser.

**Endpoint:** `https://api.github.com/repos/{owner}/{repo}/contents/{path}`.

**Privilege:** external network.

**Blast radius:** repository/path query disclosure; public API in observed source path.

**Native boundary:** repository-read capability/provider.

---

# E-015 — Direct cloud/local/custom model inference

**Trigger:** AI call execution.

**Actor:** browser client when proxy is unavailable/disabled.

**Observed direct targets:**

- OpenAI: `https://api.openai.com/v1/chat/completions`
- Groq: `https://api.groq.com/openai/v1/chat/completions`
- Ollama: `http://127.0.0.1:11434/api/chat`
- Custom: user-configured endpoint

Anthropic/Claude is intentionally restricted to the legacy proxy path in the observed client logic.

**Data sent can include:**

- identity/system prompts
- user prompt
- conversation history
- connected text-node contents
- connected image contents/URLs
- search/retrieval context
- Wikipedia/Wolfram/search results
- custom instructions

**Privilege:** model credential + outbound network + potentially broad workspace-content disclosure.

**Blast radius:** selected provider and all included contextual data.

**Native boundary:** `ModelInferenceCapability` with explicit model, credential, network, context/data-disclosure policy.

---

# E-016 — Local AI proxy credential ingestion and inference

**Trigger:** proxy-enabled AI call.

**Actor:** AIProxy sidecar.

**Credential effect:** client POSTs API keys and Ollama base URL to `/aiproxy/api-keys`; server stores them in process memory, initially falling back to environment variables.

**Inference routes:**

- `/openai`
- `/anthropic`
- `/groq`
- `/ollama/chat`
- `/custom`
- `/cancel`

**Privilege:** secret handling + outbound network/local-service access.

**Blast radius:** provider accounts, custom endpoint, Ollama service.

**Auditability:** active request map is transient; no durable request/effect ledger observed.

**Native boundary:** credential broker + provider-specific runtime binding.

---

# E-017 — Ollama model-management effects

**Trigger:** local-model management UI/workflows.

**Actor:** AIProxy sidecar.

**Routes/effects:**

- `GET /ollama/tags` — enumerate installed models
- `GET /ollama/library` — fetch ollama.com library page
- `POST /ollama/embeddings` — compute embedding
- `POST /ollama/pull` — download/install model
- `DELETE /ollama/delete` — delete model
- `POST /ollama/create` — create model
- `POST /ollama/show` — model info
- `HEAD /ollama/blobs/:digest` — inspect blob
- `POST /ollama/blobs/:digest` — create blob
- `POST /ollama/push` — push model

**Privilege:** local service control, potentially large network/disk consumption, deletion, remote push.

**Blast radius:** configured Ollama instance and its model/blob store; remote registry where push is used.

**Reversibility:** pull/create may be compensatable; delete is destructive; push/disclosure is not generally reversible.

**Native boundary:** split into granular capabilities: `ModelList`, `ModelInfer`, `ModelInstall`, `ModelDelete`, `ModelCreate`, `ModelPush`, `ModelBlobWrite`.

---

# E-018 — Neurite hosted backend API

**Actor:** `window.NeuriteBackend`; Electron-local mode routes these through secure-proxy IPC.

**Observed endpoints:**

- `GET /api/public-keys`
- `POST /api/oauth/sign-out`
- `GET /api/oauth/me`
- `POST /api/oauth/delete-account`
- `GET /api/stripe/get-balance`
- `POST /api/stripe/create-checkout-session`
- `/api/ai/get-response` via generic request path, including streaming

**Privilege/effects:** session cookies, account mutation, payment-session creation, hosted inference.

**Blast radius:** Neurite account/remote backend; account deletion is high-consequence and destructive.

**Native boundary:** hosted-account/payment integration is an optional remote capability; it must not be required for standalone startup.

---

# E-019 — Electron secure network proxy

**Trigger:** hosted backend request in Electron-local frontend.

**Actor:** Electron main + hidden BrowserWindow loading Neurite proxy page.

**Effect chain:**

renderer
→ preload `electronAPI.secureFetch` / stream IPC
→ Electron main
→ hidden sandboxed persistent `secure-proxy` BrowserWindow
→ remote proxy page performs request
→ response IPC
→ renderer `Response`.

Auth messages are also forwarded from proxy to renderer.

**Privilege:** Electron IPC + persistent browser session + network.

**Blast radius:** requests made in the proxy partition/session.

**Auditability:** console logs; no durable structured effect log.

**Native boundary:** replace with ordinary typed HTTP/provider clients where possible; retain browser-mediated auth only where genuinely required and isolate it as a browser runtime capability.

---

# E-020 — Account/payment popup flows

**Trigger:** sign-in/add-funds.

**Actor:** browser `window.open` or Electron secure proxy popup.

**Effect:** opens verification/payment redirect pages and receives auth/Stripe status messages.

**Privilege:** external browser navigation, auth/payment session initiation.

**Blast radius:** remote account/payment flow.

**Native boundary:** explicit account/payment capability, with human confirmation for consequential operations.

---

# E-021 — Account deletion

**Trigger:** explicit delete-account UI + confirmation.

**Actor:** hosted backend request.

**Effect:** `POST /oauth/delete-account` including cached email; UI states remaining balance will also be deleted.

**Privilege:** destructive account authority.

**Blast radius:** remote account and associated balance/data according to service behavior.

**Reversibility:** destructive; source UI describes it as final.

**Auditability:** backend behavior not present in this repository, so server-side auditability is unknown.

**Native boundary:** high-risk remote command requiring explicit human confirmation and server-side authentication.

---

# E-022 — Browser filesystem export/import of API-key file

**Trigger:** save/load keys UI.

**Actor:** browser File System Access API.

**Effect:**

- `showSaveFilePicker` writes JSON containing provider keys to plaintext `.txt`
- `showOpenFilePicker` reads a selected JSON key file into UI fields

**Privilege:** user-selected filesystem file read/write + secret disclosure.

**Blast radius:** selected file and included credentials.

**Native boundary:** secret import/export, if retained at all, must be explicitly labeled and separately confirmed; default should be secure credential storage/reference.

---

# E-023 — Workspace file export/import

**Trigger:** graph download/drop/import.

**Actor:** browser download/FileReader/blob APIs.

**Effect:** exports legacy graph package and imports user-selected package/text.

**Privilege:** user-selected filesystem I/O mediated through browser.

**Blast radius:** selected package only.

**Native boundary:** `WorkspaceImportCapability` / `WorkspaceExportCapability` with format validation, size limits, migration, checksums.

---

# E-024 — AI-mediated graph/note mutation

**Trigger:** model output interpreted by `AiNode.MessageLoop`.

**Actor:** client message-loop command/mention registry.

**Observed semantic actions:**

- `@memory` → create a new Zettelkasten text/memory node and connect it
- `/disconnect <title>` → remove a relationship
- `/exit` → disconnect conversation relationships
- `/rewrite <title>` → rewrite target text node content
- directed `@recipient` / `@all` → send content to connected AI nodes
- `@user` → request human input

**Privilege:** mutation of workspace entities/relationships and agent-to-agent messaging.

**Blast radius:** reachable connected graph/workspace nodes under legacy logic.

**Reversibility:** some graph edits can be reversed conceptually but there is no universal transactional rollback shown.

**Auditability:** resulting workspace state changes exist, but no uniform command/event audit trail governs them.

**Native boundary:** model output produces **proposed intents**, not direct authority. Route through `Intent → Command → Capability/Authority Check → Executor → Event`.

---

# E-025 — Remote content/CDN acquisition required by legacy browser core

**Trigger:** initial page/code-feature load.

**Actor:** browser module/script loading.

**Observed examples:** runtime libraries loaded by `index.html`; Pyodide imported from jsDelivr when Python execution is used.

**Privilege:** external network/code acquisition.

**Blast radius:** runtime code supply chain and offline availability.

**Native boundary:** core dependencies/assets bundled and content-pinned locally. Optional runtime downloads must be explicit capability-managed artifacts with integrity verification.

---

# Consequence classes for native capability broker

The source audit implies the following minimum capability families:

```text
FilesystemEnumerate
FilesystemRead
WorkspaceImport
WorkspaceExport
CredentialRead
CredentialImportExport
NetworkFetch
WebSearch
BrowserNavigate
BrowserAutomation
ScreenshotCapture
ModelInfer
ModelInstall
ModelDelete
ModelCreate
ModelPush
ModelBlobWrite
PythonExecute
WebCodeExecute
JavaScriptExecute
RepositoryRead
LocalIndexRead
LocalIndexWrite
LocalIndexDelete
RemoteAccountRead
RemoteAccountMutate
PaymentSessionCreate
WorkspaceEntityMutate
WorkspaceRelationshipMutate
AgentMessageSend
LegacyRuntimeProvision
```

A single broad `AI`, `Browser`, `Files`, or `Execution` boolean is insufficient.

---

# Auditability requirement derived from legacy gaps

All consequential native commands should produce a structured event containing at minimum:

```text
command_id
principal
originating_interaction_or_agent
capability_grant
runtime_binding
input_digest / bounded payload reference
started_at
completed_at
outcome
external_target
state mutations
failure
reversibility/compensation metadata
```

Sensitive values should be referenced/redacted rather than copied into logs.

---

# Evidence paths

Main branch:

- `localhost_servers/start_servers.js`
- `localhost_servers/servers.json`
- `localhost_servers/direct-access/direct-access.js`
- `localhost_servers/automation/automation.js`
- `localhost_servers/webscrape/index.js`
- `localhost_servers/wiki-search/novelty.js`
- `localhost_servers/wolfram-alpha/index.js`
- `localhost_servers/ai-proxy/ai-proxy.js`
- `js/interface/filetree.js`
- `js/interface/functioncall/functioncallingpanel.js`
- `js/nodes/nodeinteraction/bundlecode.js`
- `js/nodes/nodetypes/linknodes/linknode.js`
- `js/nodes/nodetypes/ainodes/ainodemessage.js`
- `js/ai/ai-utility/handleapikeys.js`
- `js/ai/ai_v2.js`
- `js/interface/searchapi/searchapi.js`
- `js/interface/searchapi/wikipedia.js`
- `js/interface/searchapi/wolframapi.js`
- `js/interface/searchapi/codeparser/gitparsed.js`
- `js/interface/dropdown/signin.js`
- `js/interface/dropdown/neuritepanel.js`

Electron branch:

- `main.js`
- `modules/preload.js`
- `modules/proxypreload.js`
- `modules/securefetch.js`
- `modules/servermanager.js`
- `modules/windowmanager.js`
- updater/downloader/frontend modules (separate desktop ledger)

# Remaining runtime evidence

Source establishes reachable effect mechanisms, but runtime testing is still required for:

- actual origin/cookie/session behavior
- CORS and proxy failure modes
- iframe/srcdoc effective isolation
- Pyodide package/network behavior offline
- exact filesystem permissions per OS/package
- provider timeout/cancellation/error handling
- storage/database corruption and concurrency
- model-operation side effects against real Ollama versions
- Electron updater/download verification and failure modes
- audit/log artifacts actually emitted during representative runs
