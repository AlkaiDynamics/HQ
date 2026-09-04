# Neurite Static Decomposition — Pass 1

Pinned upstream: `satellitecomponent/Neurite@e62b270402b688a39a864007b9c0a02711b9573e`

This is an evidence-backed first decomposition of implementation responsibilities. It is not yet the complete behavior inventory.

## A. Application/bootstrap layer

Primary evidence: `js/main.js`.

Responsibilities currently mixed together:

- DOM helper definitions
- global event helper registration
- HTTP helper behavior
- application singleton construction
- dynamic HTML resource loading
- dynamic tab loading
- sequential script loading
- startup ordering
- readiness signaling to Electron

### Architectural seam

Target contract should be split into:

```text
ApplicationBootstrap
ResourceRegistry
SubsystemLifecycle
ReadinessSignal
```

The successor should preserve startup ordering requirements only where they are semantically necessary. Source-file ordering should not remain the dependency mechanism.

---

## B. Spatial graph model

Primary evidence:

- `js/nodes/nodeutilities.js`
- `js/nodes/nodeclass.js`
- `js/nodes/edgeclass.js`

### Existing `Graph` responsibilities

`Graph` currently owns or directly references:

- node registry
- edge registry
- node-view registry
- edge-view registry
- selected/moving/dragged interaction state
- camera pan
- camera zoom represented as a complex/vector quantity
- camera rotation
- DOM containers for nodes/edges
- coordinate transforms screen <-> complex plane
- view lookup through DOM datasets
- UUID allocation
- graph mutation

This is excessive responsibility for one object.

### Target seams

```text
SceneGraph
  EntityRegistry
  RelationshipRegistry

Camera2D
  pan
  zoom
  rotation
  world_to_screen
  screen_to_world

InteractionState
  dragged entity
  moving entity
  selection

ProjectionRegistry
  entity projection handles
  relationship projection handles
```

### Finding B-003 — model and projection are conflated

The current `Graph` stores both logical graph objects and DOM/SVG projection objects.

**Preserve:** graph mutation semantics, camera behavior, selection/movement behavior, world-coordinate behavior.

**Reject as target architecture:** DOM handles as graph state.

---

## C. Spatial entity / Node

Primary evidence: `js/nodes/nodeclass.js`.

The current `Node` object simultaneously contains:

- identity (`uuid`)
- spatial state (`pos`, `vel`, `force`, `anchor`, `scale`)
- simulation behavior
- fractal-force behavior
- mouse interaction state
- DOM content
- DOM event listeners
- node-window/view reference
- relationship collection
- serialization behavior
- per-node extension reconstruction callbacks
- node-type flags

### Target seams

```text
SpatialEntity
  id
  transform
  motion
  flags

EntityDynamics
  integration
  forces
  constraints

EntityProjection
  projection id
  presentation state

EntityInteraction
  drag / scale / select / anchor intents

EntityPersistenceRecord
  serializable entity state
  projection reference
  type-specific payload
```

### Finding B-004 — serialization currently depends on object introspection and DOM datasets

`Node.toJSON()` serializes a spread of the live object while excluding a hard-coded list of fields. `updateNodeData()` stores serialized state back into `data-*` attributes on the DOM element.

**Preserve:** the complete state necessary to restore a node.

**Reject:** live-object reflection + DOM attributes as the durable state schema.

The replacement requires an explicit, versioned Rust schema.

---

## D. Relationship / Edge

Primary evidence: `js/nodes/edgeclass.js`.

The current `Edge` object combines:

- logical endpoints
- desired physical length
- force strength
- directionality
- visual style
- spring/force simulation
- SVG projection
- mouse interaction
- Zettelkasten mutation behavior when direction changes

### Critical semantic coupling

For text nodes, changing edge direction can add/remove Zettelkasten references. Thus a visible edge is not merely presentation in the old application.

This must be captured behaviorally before redesign.

### Target seams

```text
SpatialRelationship
  endpoints
  physical layout parameters

SemanticRelationship
  direction / reference meaning

RelationshipProjection
  visual curve/arrow/style

RelationshipIntentHandler
  user requests to connect/disconnect/change direction
```

### Finding B-005 — one Edge instance spans multiple relationship meanings

The successor must distinguish at minimum:

```text
VisualEdge
SemanticReference
LayoutConstraint
RuntimeRelation
```

and only synchronize them through explicit rules.

---

## E. Fractal / coordinate substrate

Primary evidence: `js/mandelbrot/mandelbrot.js`.

Observed useful mechanisms include:

- `vec2` complex/vector arithmetic
- Mandelbrot iteration (`Fractal.step`)
- inverse/square-root path (`Fractal.unstep`)
- escape/distance calculations
- finite-difference gradient
- complex-plane <-> screen transforms
- recenter/rezoom mechanics to keep SVG coordinates numerically manageable

The Node physics layer directly samples `Fractal.grad(...)` to influence node motion.

### Target seams

```text
Complex2 / math primitives
FractalField
FractalNavigation
CameraTransform
FractalRenderer
FractalInfluenceField (optional behavior-preserving dynamics)
```

### Finding B-006 — fractal is both visual substrate and behavior field

The successor cannot treat the fractal as a decorative background only. Existing node motion can depend on the fractal gradient.

This needs golden tests covering representative positions/iterations and node-motion effects.

---

## F. Simulation loop

Primary evidence: `js/nodes/nodeinteraction/nodestep.js`.

One `requestAnimationFrame` loop currently performs:

```text
selected-node input processing
-> autopilot camera update
-> SVG viewBox update
-> mouse-path fractal/orbit update
-> FPS measurement/UI mutation
-> every-node simulation step
-> every-edge simulation step
-> incremental fractal regeneration
-> request next frame
```

### Target seams

```text
FrameClock
InputIntegrator
CameraController
SceneDynamics
LayoutDynamics
RenderScheduler
TelemetrySampler
FractalWorkScheduler
```

### Finding B-007 — render, simulation, UI telemetry and fractal work share one frame loop

Preserve interactive timing behavior, but the Rust target should have explicit scheduling/budgets and should keep blocking I/O/background services out of this loop.

---

## G. Node projection/window system

Primary evidence: `js/nodes/createnodes/window.js`.

`NodeView` currently provides:

- window chrome
- title field
- copy button
- delete/fullscreen/collapse buttons
- anchoring interaction
- selection interaction
- resize behavior
- focus/hover styling
- dynamic node-specific controls
- DOM binding/rebinding

### Target seam

```text
NodeProjection
NodeChrome
ProjectionActions
ProjectionLayoutState
```

The same logical entity should be able to support alternative projections without changing entity state.

---

## H. Existing node kinds

Observed node-type families under `js/nodes/nodetypes/`:

```text
AI nodes
file-tree nodes
image nodes
link nodes
media nodes
text nodes
Wolfram nodes
```

Current `Node.getType()` exposes only a smaller action-oriented type set (`text`, `llm`, `link`, `base`), showing that presentation families and behavior/action families are already not identical.

### Finding B-008 — node type is not one dimension

The successor should avoid a single enum attempting to encode all of:

- entity kind
- projection kind
- runtime binding kind
- capability kind
- action surface

These should be composable dimensions.

---

## I. Text node / executable text surface

Primary evidence: `js/nodes/nodetypes/textnodes/textnode.js` plus code-execution scripts loaded by `main.js`.

A text node constructs and holds:

- textarea/editor state
- syntax-display surface
- HTML iframe surface
- Python output/frame surface
- code editing state
- title and text persistence callbacks

`NodeActions.text` exposes `toggleCode`, which calls code-execution behavior.

### Target seam

```text
TextDocument
TextProjection
CodeProjection
CodeExecutionIntent
ExecutionCapability
```

Execution must not be implied merely because text content contains code.

---

## J. Link/browser node

Primary evidence: `js/nodes/nodetypes/linknodes/linknode.js`.

Behavior differs by host:

```text
Electron -> webview
Browser  -> iframe
```

Observed capabilities:

- open/view URL
- navigate URL
- back/forward (Electron webview; iframe best-effort)
- refresh
- proxy display through local web-scrape sidecar
- extract/store page text
- import page text into Zettelkasten

Electron webviews explicitly set security-related web preferences such as context isolation, sandboxing and no node integration.

### Target seam

```text
WebResourceEntity
BrowserProjection
BrowserRuntimeBinding
WebFetchCapability
WebExtractionCapability
```

The native UI should not become a browser engine. Browser capability belongs behind a replaceable runtime binding.

---

## K. Action surface

Primary evidence: `js/nodes/nodeinteraction/nodeactioninterface.js`.

Current behavior already contains an important abstraction:

```text
Node -> NodeActions.forNode(node) -> action vocabulary -> execute action
```

Base actions include focus/follow/delete/select/collapse/automata/spawn/connect. Type-specific actions extend that surface for text, LLM and link nodes.

### Finding B-009 — an embryonic capability/action contract already exists

This is a high-value behavior to preserve and formalize.

Target:

```text
Entity
  -> ActionDescriptor[]
       id
       label/synonyms
       parameter schema
       required capability/authority
       reversibility
       execution binding
```

The old implementation directly invokes JS methods. The successor should route consequential actions through typed commands and capability admission.

---

## L. Zettelkasten / graph-text dual representation

Primary evidence:

- `js/zettelkasten/*`
- edge code calling `addEdgeToZettelkasten` / remove counterparts

Observed responsibilities:

- syntax definition / node/reference tags
- CodeMirror-backed textual workspace
- text parsing
- node placement
- path/workspace behavior
- synchronization between text references and graph edges

### Finding B-010 — graph and textual note representation are bidirectionally coupled

This is a defining behavior, not incidental UI.

The Rust successor needs an explicit projection/synchronization contract so that:

```text
textual representation
<-> semantic note graph
<-> spatial projection
```

are separate but reconcilable.

---

## M. Persistence

Primary evidence:

- `js/globals.js`
- `js/interface/dropdown/savenet.js`
- scattered `localStorage` references

Observed state mechanisms:

1. localForage-backed `Stored` abstraction
2. graph metadata/data/blob stores
3. state such as latest-selected graph
4. localStorage for controls, modal values, API keys, paths, suggestions, prompts, saved views, filters, user-related UI values, etc.
5. graph export/import as a JSON header plus a NUL separator and concatenated blobs

### Finding B-011 — persistence is fragmented across multiple implicit stores

The target requires a state-ownership ledger before migration.

Initial target categories:

```text
WorkspaceState
UserPreferenceState
CredentialReferenceState
UIEphemeralState
CapabilityState
ProjectionState
ExternalBlobState
```

No new schema should be implemented until each current state item is assigned an owner, writer, reader, durability requirement and migration rule.

---

## N. Privileged local services

Primary evidence:

- `localhost_servers/servers.json`
- `localhost_servers/start_servers.js`
- service directories

Services:

```text
Automation
WebScrape
WikiSearch
WolframAlpha
AIProxy
DirectAccess
```

The launcher can automatically run `npm install` inside service directories before importing them.

### Finding B-012 — service bootstrap performs installation and execution dynamically

This behavior should not migrate into the trusted native UI core.

Target:

```text
CapabilityBroker
  -> admitted provider
  -> isolated runtime
  -> explicit health/readiness
  -> typed IPC
```

---

## O. Filesystem sidecar

Primary evidence: `localhost_servers/direct-access/direct-access.js`.

Current API accepts arbitrary paths and resolves them against the host filesystem for directory navigation and file reads.

### Finding B-013 — current filesystem capability is effectively ambient read authority

Target contract requires explicit scoped roots/handles. File browsing is valuable; unrestricted root-relative access is not an architectural requirement.

---

## P. Automation sidecar

Primary evidence: `localhost_servers/automation/automation.js`.

Observed behavior:

- launches Playwright browser
- opens Neurite
- sets `window.startedViaPlaywright`
- exposes screenshot capture over HTTP

This is useful both as a product capability and, critically, as a **golden-behavior capture mechanism** for the rewrite.

### Migration use

The old automation service should be preserved initially as part of the compatibility/test capsule so we can capture:

```text
interaction
-> screenshot/video/state observation
```

from the legacy application.

---

# First-pass target contracts discovered

The source is already pointing toward these technology-neutral contracts:

```text
ApplicationLifecycle
SceneGraph
SpatialEntity
Relationship
Camera
Projection
ActionDescriptor
Intent
Command
Event
Workspace
WorkspaceSerializer
BlobStore
TextDocument
SemanticReferenceGraph
BrowserRuntimeBinding
ModelProvider
FileCapability
WebCapability
AutomationCapability
ExternalKnowledgeCapability
CapabilityBroker
```

These are **candidate contracts**. They are not promoted until runtime/behavior evidence confirms them.

# Next forensic work

1. Complete recursive source inventory/checksum manifest.
2. Map every persistent state key/store and its writers/readers.
3. Map every network/API endpoint and external effect.
4. Trace create/move/scale/connect/delete/save/load workflows end-to-end.
5. Trace Zettelkasten text -> graph and graph -> text synchronization.
6. Trace node action dispatch and function-calling/Neural API paths.
7. Trace AI node provider/context/stream behavior.
8. Trace link/media/file drop/import behavior.
9. Build representative legacy fixtures and golden behavior specs.
10. Only then promote stable contracts for the Rust skeleton.
