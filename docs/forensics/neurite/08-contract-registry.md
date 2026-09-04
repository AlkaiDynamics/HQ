# Neurite Technology-Neutral Contract Registry

Specimens:

- web/main: `e62b270402b688a39a864007b9c0a02711b9573e`
- Electron: `ab38b3f8d7edb53ba78affff91f30295d0282701`

This registry translates source-derived Neurite behavior into stable behavioral contracts. It does **not** assert runtime parity. Contract status must distinguish source-derived understanding from golden-verified behavior.

## Status vocabulary

```text
SOURCE-DERIVED
RUNTIME-VERIFIED
GOLDEN-VERIFIED
RUST-IMPLEMENTED
DIFFERENTIAL-PASS
PROMOTED
```

No contract may move directly from `SOURCE-DERIVED` to `PROMOTED`.

## Treatment vocabulary

```text
K  KEEP NATIVE
R  RETAIN / REFACTOR
S  SANDBOX
B  BRIDGE
X  REMOVE
```

A classification applies to a behavior/mechanism, not necessarily an entire legacy source file.

---

# C-001 — Scene

## Purpose

Own the local spatial environment independently of any projection backend, AI provider, ARGO connection, or legacy runtime.

## Contract

```text
Scene {
  entity_registry
  relationship_registry
  layout_state
  camera/reference_frame
  projection_index
}
```

Required operations:

```text
add_entity
remove_entity
lookup_entity
add_relationship
remove_relationship
query_relationships
apply_local_layout_change
create/remove projection
```

## Invariants

- Scene existence does not require a renderer.
- Scene existence does not require ARGO.
- Visual adjacency does not imply semantic or authority relations.
- Removing a projection is distinct from deleting the underlying entity.

## Legacy evidence

`Graph` currently combines entity/relationship storage, camera, DOM registries and interaction state.

## Treatment

**K/R** — keep the spatial scene capability; refactor ownership boundaries.

## Initial Rust target

`neurite-scene` + common identifiers in `neurite-core`.

---

# C-002 — SpatialEntity

## Purpose

Represent a logical object addressable in the scene without coupling identity to a DOM window or one content/runtime kind.

## Contract

```text
SpatialEntity {
  id
  spatial_state?
  semantic_entity_ref?
  projection_refs[]
  relationship_refs[]
  durable_metadata
}
```

`spatial_state` may include position, scale and layout-anchor state.

## Invariants

- Entity kind, projection kind, runtime binding kind and capability set are separate axes.
- One logical entity can have zero, one or many projections.
- Dormant logical entities need not imply live heavyweight UI/runtime resources.

## Treatment

**K/R**.

---

# C-003 — Workspace

## Purpose

Own the locally durable working set.

## Contract

```text
Workspace {
  id
  schema_version
  entities
  semantic_relationships
  selected_durable_layout_state
  note_content
  media_references
  user/workspace metadata
}
```

Operations:

```text
create
open
close
save_snapshot
import_legacy
export_portable
recover
migrate
```

## Invariants

- workspace data is distinct from UI preferences, credentials, remote-account cache and runtime state.
- a workspace can open without internet or ARGO.
- old workspace formats are migrated through compatibility readers, not interpreted as the new in-memory schema indefinitely.

## Treatment

**K/R**.

## Rust target

`neurite-workspace`, `neurite-store`.

---

# C-004 — Projection

## Purpose

Render or interact with an entity without owning its identity/authority.

## Contract

```text
Projection {
  projection_id
  entity_id
  kind
  local_view_state
  runtime_binding?
  action_surface
}
```

Examples:

```text
TextProjection
ImageProjection
VideoProjection
AudioProjection
PdfProjection
BrowserProjection
FileTreeProjection
AiAgentProjection
ComputationResultProjection
```

## Invariants

- projection state is not automatically domain state.
- projection failure cannot corrupt authoritative workspace state.
- browser/webview implementation is not required for non-browser projections.

## Treatment

**K/R** for projection architecture; individual browser/legacy projections may be **B/S**.

---

# C-005 — View

## Purpose

Describe user-facing composition, camera/layout, panels and selection independently of entity data.

## Contract

```text
View {
  camera
  visible_projection_set
  selection
  panel/layout state
  status/degradation indicators
}
```

## Invariants

- camera/view changes are normally local visual state unless explicitly persisted/shared.
- views can be saved without granting capabilities represented by objects visible within them.

## Treatment

**K/R**.

---

# C-006 — Command

## Purpose

Represent a requested consequential operation as typed data.

## Contract

```text
Command {
  id
  kind
  principal
  origin
  target
  typed_parameters
  requested_capabilities
  preconditions
  correlation_id
}
```

Examples:

```text
CreateEntity
DeleteEntity
CreateSemanticReference
RemoveSemanticReference
ImportWorkspace
ExportWorkspace
FetchWebResource
ExecutePython
ExecuteWebCode
RunModelInference
InstallModel
DeleteModel
CreatePaymentSession
DeleteRemoteAccount
```

## Invariants

- model output, UI gesture or plugin request produces intent/command data; none gains execution authority by being able to name a function.
- consequential commands pass admission/capability checks.
- purely visual/local operations may use a lighter local path.

## Legacy replacement

Replaces direct `executeNodeMethod`, arbitrary Neural API function invocation and AI-message direct graph mutation.

## Treatment

**K** new native architecture; legacy direct invocation **X/R** during migration.

---

# C-007 — Event

## Purpose

Record an observed result/state transition after execution.

## Contract

```text
Event {
  id
  command_id?
  kind
  principal
  source_runtime
  timestamp
  outcome
  state_delta_reference?
  external_target?
  error?
  compensation_metadata?
}
```

## Invariants

- consequential effects create structured events.
- secrets are referenced/redacted, not copied indiscriminately into logs.
- event history can support audit/replay/recovery where semantics permit.

## Treatment

**K**.

---

# C-008 — LocalState

## Purpose

Make ownership/durability/authority explicit for all state.

## Contract

Every state field/family declares:

```text
owner
writers
readers
durability
authority
regenerability
schema_version
```

Native domains:

```text
WorkspaceState
SnapshotState
ProjectionState
UserPreferenceState
CredentialReferenceState
SessionCache
ExecutionHistory
ExternalBlobState
CapabilityState
RuntimeState
```

## Invariants

- path ≠ filesystem authority
- code history ≠ execution authority
- provider selection ≠ credential/network authority
- remote cache ≠ remote authoritative state
- DOM identifiers are not durable schema

## Treatment

**K/R**.

---

# C-009 — Capability

## Purpose

Bound privilege independently of UI/entity/runtime identity.

## Contract

```text
CapabilityGrant {
  capability_kind
  principal
  resource_scope
  allowed_operations
  limits
  issued_at
  expiry?
  delegation_policy?
  audit_policy
}
```

Minimum capability families discovered from Neurite:

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

## Invariants

- no ambient filesystem/network/process/secrets for plugins or legacy runtimes.
- a broad `AI`, `Files`, `Browser` or `Execution` boolean is insufficient.

## Treatment

**K** new native capability broker.

---

# C-010 — Plugin

## Purpose

Allow future capabilities/projections to be added without modifying the architectural kernel.

## Contract

Conceptual surface:

```text
Plugin {
  identity()
  requested_capabilities()
  entity/projection descriptors()
  action descriptors()
  command handlers()
  event subscriptions()
  settings_schema()
  health/status()
}
```

Preferred untrusted extension boundary:

```text
Rust host
→ Wasmtime
→ WASI / Component Model
→ typed plugin interface
```

Other runtimes are represented through RuntimeBinding rather than pretending all software is a WASM plugin.

## Invariants

- plugin receives only explicit host imports/grants.
- no automatic filesystem, network, process, secret or ARGO authority.
- plugin crash cannot corrupt workspace durability boundary.

## Treatment

**K** new architecture.

---

# C-011 — MediaSource

## Purpose

Represent content separately from the projection used to view/play it.

## Contract

```text
MediaSource {
  id
  media_kind
  content_reference
  mime_type?
  content_digest?
  provenance
  portability
}
```

Kinds include:

```text
Image
Audio
Video
PDF/Document
```

## Invariants

- browser `blob:` URLs are transient projection/runtime details, not durable content identity.
- PDF does not need to inherit LinkNode semantics.

## Treatment

**K/R**.

---

# C-012 — ModelProvider

## Purpose

Decouple conversation/agent semantics from one API/provider and from network/credential authority.

## Contract

```text
ModelProvider {
  provider_id
  model_catalog()
  capabilities()
  infer(request, grant)
  stream(request, grant)
  cancel(request_id)
  health()
}
```

Optional provider-management extensions:

```text
install_model
remove_model
create_model
push_model
blob operations
```

Those require different capabilities from inference.

## Invariants

- provider selection does not grant credential/network access.
- context disclosure is explicit/policy-visible.
- local model use remains possible independently of cloud model providers.

## Treatment

provider abstraction **K/R**; external providers **B**; legacy AIProxy wiring **S/X** after migration.

---

# C-013 — Relationship

## Purpose

Prevent the legacy collapse of visual, semantic, layout and runtime connections.

## Contract family

```text
VisualEdge
SemanticReference
LayoutConstraint
RuntimeRelation
AuthorityRelation
```

Each has independent identity and lifecycle.

## Invariants

```text
VisualEdge != SemanticReference
VisualEdge != AuthorityRelation
VisualEdge != RuntimeRelation
```

Creating or directing a visual edge cannot silently grant execution/authority.

For compatibility, a user gesture may intentionally create more than one relationship through an explicit compound command.

## Treatment

**K/R**.

---

# C-014 — Snapshot

## Purpose

Provide durable, versioned, recoverable workspace checkpoints.

## Contract

```text
Snapshot {
  snapshot_id
  workspace_id
  schema_version
  created_at
  parent?
  state_digest
  blob/content references
  migration metadata
}
```

Operations:

```text
create_atomic_snapshot
validate
restore
compare
export
migrate
```

## Legacy compatibility

Legacy import reader understands:

```text
JSON({data, blobMeta, offsets}) + NUL + blob bytes
```

and normalizes it into the native workspace schema.

## Treatment

**K/R**; legacy format reader **B** until retirement criteria are met.

---

# C-015 — Reconciliation

## Purpose

Reconcile independently changing local/remote/ARGO state without conflating authorities.

## Contract

```text
Reconciliation {
  local_version
  external_version
  authority_rules
  conflict_set
  proposed_resolution
  accepted_resolution
  resulting_events
}
```

## Invariants

- Neurite remains useful without ARGO.
- ARGO remains operational without Neurite.
- reconnect does not silently overwrite either state domain.
- projection of an ARGO entity does not make Neurite the authoritative source for that entity.

## Treatment

**K** architecture; actual ARGO bridge deferred until native/offline parity.

---

# C-016 — RuntimeBinding

This contract is required by the evidence even though it was not named explicitly in the original minimum contract list.

## Purpose

Represent execution/runtime location independently of the node/projection.

## Contract

```text
RuntimeBinding {
  id
  runtime_kind
  artifact/service identity
  lifecycle
  health
  IPC/protocol
  requested capabilities
  isolation class
}
```

Supported future runtime kinds can include:

```text
NativeRust
WASM/WASI
IsolatedProcess
PythonEnvironment
OCIContainer
RemoteService
MCP
BrowserRuntime
AgentRuntime
NativeApplication
```

## Invariants

- UI does not execute arbitrary repository/runtime code in-process.
- UI does not need implementation-specific logic for each backend.
- dormant entity/projection does not imply live runtime process.

## Treatment

**K**.

---

# C-017 — ActionDescriptor

Also added from source evidence because `NodeActions` and Neural API already demonstrate the need.

## Purpose

Make discoverable actions data-driven and capability-aware.

## Contract

```text
ActionDescriptor {
  id
  label
  aliases/synonyms
  parameter_schema
  applicability
  required_capabilities
  required_authority
  reversibility
  executor_binding
}
```

Legacy strengths retained:

- discoverable per-node action vocabulary
- aliases/synonyms
- recent/pinned action UX

Legacy weaknesses removed:

- parsing arbitrary method-call strings
- direct object method invocation
- mixing visual and high-authority actions in one untyped surface

## Treatment

**K/R**.

---

# C-018 — BehaviorFixture

Required to satisfy the migration method.

## Contract

```text
BehaviorFixture {
  fixture_id
  legacy_environment_manifest
  setup
  initial_state_digest
  interaction_sequence
  visual_capture_refs
  expected_state_delta
  expected_persistence
  expected_external_effects
  expected_failure_behavior
  timing/performance observations
}
```

## Invariants

- source inference cannot mark a fixture golden.
- same fixture runs against legacy and Rust implementation.
- comparison includes visible behavior, state, persistence, external effects, failure semantics and performance where relevant.

## Treatment

**K** test architecture.

---

# C-019 — Evidence

Required for the forensic KG.

## Contract

```text
Evidence {
  evidence_id
  specimen
  branch
  commit
  tree
  source_path?
  blob_sha?
  source_range_or_symbol?
  fixture_id?
  evidence_type
  confidence
  observation
}
```

Evidence types:

```text
SOURCE-DERIVED
RUNTIME-OBSERVED
GOLDEN-DIFFERENTIAL
INFERRED
HYPOTHESIZED
CONTRADICTED
```

## Treatment

**K** forensic/test infrastructure.

---

# Feature / mechanism K-R-S-B-X registry

These classifications are source-derived and may be refined by golden behavior evidence. `X` means remove the legacy **mechanism**, not automatically remove the user-visible value.

| Behavior / mechanism | Treatment | Native destination / reason |
|---|---|---|
| spatial scene and graph experience | K/R | native scene/entity/relationship architecture |
| Mandelbrot/fractal navigation | K/R | native fractal math + renderer; golden math/navigation tests |
| graph physics/layout | K/R | native layout subsystem; preserve useful dynamics, decouple from frame/render loop |
| camera pan/zoom/rotation | K/R | native camera/input |
| node/window spatial interaction | K/R | native projection/chrome/action surface |
| Zettelkasten note model | K/R | native semantic-note/textual projection + explicit reconciliation |
| text ↔ graph synchronization | K/R | retain behavior, formalize semantic relationship/reconciliation contract |
| visual edges | K/R | VisualEdge |
| text semantic references | K/R | SemanticReference |
| edge spring behavior | K/R | LayoutConstraint |
| DOM dataset/reflection serialization | X | replace with typed versioned schemas |
| global/load-order module system | X | Rust module/crate boundaries |
| LocalForage graph persistence | R/B | compatibility import only; native store/snapshots replace it |
| legacy graph export format | B | import/export compatibility reader/writer during migration |
| image/audio/video UX | K/R | native MediaSource + projections |
| PDF represented as LinkNode | X/R | preserve PDF behavior via DocumentAsset + PdfProjection |
| web/link resource concept | K | WebResourceEntity |
| iframe/Electron-webview implementation | B/S | BrowserRuntime binding |
| webpage extraction | R/S | safe network/extraction service |
| raw arbitrary web proxy | S/X | bounded fetch runtime, private-network restrictions |
| file-tree UX | K/R | native FileTreeProjection |
| DirectAccess ambient filesystem read | X | scoped file capabilities |
| text-node code editing | K/R | text/code projection |
| in-page AsyncFunction execution | X/S | isolated JS/runtime capability |
| Pyodide Python execution | S/R | isolated Python runtime binding; optional compatibility path |
| web-code iframe execution | S/R | isolated WebRuntime |
| AI conversation/agent node | K/R | ConversationEntity/AiAgentProjection |
| model provider selection | K/R | ModelProvider abstraction |
| cloud model APIs | B | provider bindings |
| local Ollama inference | B/R | local provider/runtime binding |
| legacy AIProxy | S/B then X | compatibility provider bridge only |
| AI context assembly | K/R | native context/retrieval pipeline |
| AI streaming/cancel | K/R | inference stream contract |
| agent-to-agent messaging | K/R | typed AgentMessage/relationship |
| model-output direct graph mutation | X/R | proposed intent → admission → command |
| AI auto-mode recursion | R | continuity/governance-aware agent loop |
| Google/Wikipedia/Wolfram retrieval | B | provider capability bindings |
| local semantic/embedding index | K/R | native local search/index subsystem |
| WebScrape SQLite database | B/R | migrate useful index data; replace service-bound storage |
| Wolfram result node | R | generic computation result projection + provider bridge |
| Function Call action discovery | K/R | ActionDescriptor registry |
| direct method-string execution | X | typed commands/executors |
| Neural API aliases/action vocabulary | K/R | typed action/command registry |
| Playwright legacy automation | S/K test-only | keep as frozen legacy golden capture harness initially |
| runtime `npm install` during application startup | X | provisioned isolated runtime artifacts |
| dynamic import of sidecars into trusted process | X | isolated RuntimeBindings |
| Electron desktop-shell experience | K/R | native Rust shell |
| Electron preload IPC pattern | R | typed native capability interfaces |
| hidden browser proxy for normal API traffic | X/R | native HTTP clients; browser auth only where needed |
| downloaded frontend runtime | X after parity | native renderer/UI is compiled/bundled |
| downloaded legacy server bundle | S/B temporary | compatibility capsule only |
| legacy executable updater | R | verified native updater with signatures/digests/rollback |
| remote Neurite account/payment integration | B | optional provider; not core startup dependency |
| plaintext API-key browser storage/export | X | credential broker/vault references |
| ARGO integration | B/K interface | replaceable bridge after native/offline parity |

---

# Dependency direction for Rust

The contracts imply this dependency discipline:

```text
neurite-core
  ↑
neurite-scene    neurite-events    neurite-store
  ↑                   ↑                 ↑
neurite-render   neurite-capability  neurite-workspace
  ↑                   ↑                 ↑
neurite-ui       neurite-sandbox     compatibility/import

optional providers/runtimes/plugins
  ↓ typed contracts only

neurite-argo
  ↓ replaceable bridge only
```

Hard constraints:

```text
render does not know ARGO
fractal does not know AI
store does not know DOM/UI implementation details
ARGO bridge does not own local workspace state
plugins/runtimes do not inherit ambient authority
```

---

# Contract closure status

## Source-level contract extraction

**CLOSED enough to begin the forensic KG and Rust architectural skeleton definition**, with one critical limitation:

All contracts are still `SOURCE-DERIVED` until runtime behavior fixtures verify the portions of behavior that source alone cannot establish.

## Gate C

**NOT CLOSED.**

Gate C requires technology-neutral behavioral contracts for every feature selected for migration, grounded by the preceding behavioral characterization. The registry provides the contract framework and source-derived mappings, but golden/runtime evidence is still missing.

That distinction must remain explicit in the KG/status views.
