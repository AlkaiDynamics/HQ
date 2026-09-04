# Neurite Core Behavior Trace Ledger

Upstream specimen: `satellitecomponent/Neurite`

Pinned commit: `e62b270402b688a39a864007b9c0a02711b9573e`

Pinned recursive tree: `d23fee7f1ade4886841a1e595540ad3a00a41ec6`

This ledger deliberately separates **source-derived behavior** from **runtime-observed behavior**. A source trace is not a golden fixture and is not sufficient to claim parity.

## Evidence statuses

- `SOURCE-CLOSED`: entrypoint, implementation chain, principal state mutations, semantic side effects, and relevant external boundary are identifiable from pinned source.
- `SOURCE-PARTIAL`: source path is known but one or more state/effect paths still need decomposition.
- `RUNTIME-OPEN`: visual output, timing-sensitive behavior, persistence result, failure behavior, or environment-specific behavior has not yet been captured from an executing legacy build.
- `GOLDEN-OPEN`: no representative replay fixture/golden capture exists yet.

All behaviors below remain `RUNTIME-OPEN` and `GOLDEN-OPEN` until executed against the frozen legacy application.

---

## B-CORE-001 — Create regular text node from canvas

**Source status:** SOURCE-CLOSED

### Interaction

Double-click the SVG background while node mode is active and no pending `Node.prev` connection state exists.

### Source chain

`document dblclick`
→ `createNodeFromWindow()`
→ `addNodeTagToZettelkasten(title, content)`
→ mutate active CodeMirror/Zettelkasten text
→ refresh mirror
→ locate associated Zettelkasten UI
→ `scrollToTitle(title)`
→ obtain/generated text node projection
→ assign content
→ dispatch `input` event.

### State/semantic effect

The text node is not merely inserted into the spatial graph. The create interaction first mutates the textual Zettelkasten representation, and the graph node is obtained through the Zettelkasten UI/parser path.

### Candidate contract

`CreateTextEntity { title, content, requested_position? }`
→ update semantic note model
→ emit/reconcile spatial projection.

### Evidence

- `js/nodes/createnodes/createnodes.js`
  - blob `fba7079be9ee3a42ac72a2585b94660c53ce2035`

### Runtime evidence still required

- exact initial position/scale under representative camera states
- visible transition
- behavior with missing/inactive Zettelkasten pane
- duplicate-title behavior
- persistence after save/reload

---

## B-CORE-002 — Create LLM node from canvas

**Source status:** SOURCE-PARTIAL

### Interaction

Modifier + double-click on SVG background using configured Alt modifier.

### Source chain

`document dblclick`
→ Alt modifier branch
→ `createLlmNode('', undefined, undefined, clientX, clientY)`
→ `.draw()`.

### Candidate contract

`CreateEntity(kind = AiConversation, position_hint)`

Creation should be independent from model execution authority. Creating an AI projection must not itself grant model/network execution capability.

### Evidence

- `js/nodes/createnodes/createnodes.js`
  - blob `fba7079be9ee3a42ac72a2585b94660c53ce2035`
- AI implementation family:
  - `js/nodes/nodetypes/ainodes/ainode.js`
  - `js/nodes/nodetypes/ainodes/ainodemessage.js`
  - `js/nodes/nodetypes/ainodes/responsehandler.js`

### Still source-open

Full AI creation lifecycle, provider selection, cancellation, streaming, and failure paths are tracked separately.

---

## B-CORE-003 — Create link/browser node from canvas

**Source status:** SOURCE-PARTIAL

### Interaction

Modifier + double-click on SVG background using configured Control modifier.

### Source chain

`document dblclick`
→ Control modifier branch
→ `returnLinkNodes()`
→ returned node has `followingMouse = 0`.

### Candidate contract

`CreateEntity(kind = WebResource)`
→ projection selected according to runtime environment (`BrowserProjection`, `EmbeddedWebProjection`, etc.).

### Evidence

- `js/nodes/createnodes/createnodes.js`
- `js/nodes/nodetypes/linknodes/linknode.js`
  - blob `f320e80ba686792db41e3aeb584d39602a954633`

---

## B-CORE-004 — Move selected nodes by keyboard

**Source status:** SOURCE-CLOSED

### Interaction

Hold arrow-key combinations while one or more nodes are selected.

### Source chain

window `keydown`/`keyup`
→ global `keyState`
→ animation frame `NodeSimulation.nodeStep()`
→ `processSelectedNodes()`
→ `getDirectionAngleFromKeyState()`
→ `App.selectedNodes.forEach(Node.moveAtThisAngle, movementAngle)`
→ `node.moveNode(angle)`
→ force is applied to node dynamics
→ regular node simulation integrates position and redraws.

### State effect

This is force/dynamics based, not a simple direct coordinate assignment.

### Candidate contract

Two distinct concepts should exist in native code:

- `NudgeEntity/ApplyLayoutImpulse` for legacy dynamics-compatible movement
- `SetEntityPosition` for deterministic placement

### Evidence

- `js/nodes/nodeinteraction/movenodes.js`
  - blob `71ff51619d9f1b1ba0a20a513fc799f753149dac`
- `js/nodes/nodeinteraction/nodestep.js`
  - blob `decbc1bae45ea229672f27c0a432851ea351a3cd`
- `js/nodes/nodeclass.js`
  - blob `990972e7cf640ec44ba0d3121cc945c77687a6a9`

### Runtime evidence still required

- displacement curve across scale values
- frame-rate sensitivity
- interaction with fractal force, anchors, and edges
- multi-selection movement

---

## B-CORE-005 — Scale selected nodes by keyboard

**Source status:** SOURCE-CLOSED

### Interaction

Hold configured scale keys (`f` / `d` in pinned source) with selected nodes.

### Source chain

window key state
→ animation frame
→ `processScalingKeys()`
→ choose factor `1.04` or `0.96`
→ compute selected-node centroid
→ `App.selectedNodes.scale(scaleFactor, centroid)`.

### Candidate contract

`ScaleSelection { factor, pivot }`

### Evidence

- `js/nodes/nodeinteraction/movenodes.js`
- `js/nodes/nodeinteraction/nodestep.js`

### Runtime evidence still required

- exact centroid behavior
- min/max scaling constraints
- interaction with collapsed/anchored projections
- persistence after reload

---

## B-CORE-006 — Anchor/unanchor node

**Source status:** SOURCE-CLOSED for model/view state coupling; runtime appearance still open.

### Source behavior

`Node` anchor state is reflected by projection CSS class `window-anchored` while expanded. Collapse transfers the visible anchor marker to `collapsed-anchor`; expanding restores `window-anchored`.

### Candidate contract

`SetLayoutAnchor { entity_id, anchored }`

Anchor is a layout/dynamics property. Its visual decoration is projection state and should not be the authoritative representation of anchoring.

### Evidence

- `js/nodes/nodeclass.js`
  - blob `990972e7cf640ec44ba0d3121cc945c77687a6a9`
- `js/nodes/nodeinteraction/togglenodestate.js`
  - blob `8c30e1e56862fc179bf96d232d0fbda516eb78cf`

---

## B-CORE-007 — Connect text node to text node

**Source status:** SOURCE-CLOSED

### Interaction/API

`connectNodes(node1, node2)` with two distinct text nodes.

### Source chain

`connectNodes()`
→ read titles
→ `addEdgeToZettelkasten(title1, title2)`
→ `addEdgeToZettelkasten(title2, title1)`
→ Zettelkasten parser/reconciliation subsequently materializes graph relationship/projection.

### Critical semantic property

For text nodes, connect is not simply `Graph.addEdge`. It mutates the textual/semantic representation bidirectionally.

### Candidate contracts

- `CreateSemanticReference`
- `CreateVisualEdgeProjection`
- optional `CreateLayoutConstraint`

These must be separable in the successor.

### Evidence

- `js/nodes/nodeinteraction/connect.js`
  - blob `1459c81a8fe17d02c590117aa7e16886662e98dc`
- `js/zettelkasten/*`

---

## B-CORE-008 — Connect non-text nodes

**Source status:** SOURCE-CLOSED

### Source chain

`connectNodes(node1, node2)`
→ non-text branch
→ `connectDistance()`
→ reject existing edge
→ compute Euclidean distance
→ desired edge distance = half current distance
→ instantiate `Edge`
→ push edge into both endpoint edge arrays
→ `Graph.addEdge(edge)`.

### Candidate contracts

- `CreateVisualEdge`
- `CreateLayoutConstraint`

No semantic relationship should be inferred merely from the presence of this edge.

### Evidence

- `js/nodes/nodeinteraction/connect.js`
- `js/nodes/edgeclass.js`

---

## B-CORE-009 — Toggle edge direction

**Source status:** SOURCE-CLOSED at entrypoint/semantic coupling level

### Interaction

Edge context menu
→ `edge.toggleDirection()`.

### Source behavior

Directionality cycles through endpoint direction states. For text-node edges, direction changes are coupled to Zettelkasten reference mutation.

### Candidate contracts

- `SetVisualEdgeDirection`
- `SetSemanticReferenceDirection`

These must be explicit and independently addressable.

### Evidence

- `js/nodes/edgeclass.js`
  - blob `ceca876f6820e9c9b7613fd351a7f12e196e97e6`
- `js/interface/dropdown/customui/rightclick/customcontextmenu.js`
  - blob `0093cfe67a28b1e804a5e91d54716cd7b3d1ba5a`

---

## B-CORE-010 — Disconnect/delete edge

**Source status:** SOURCE-CLOSED at entrypoint/semantic coupling level

### Interaction

Edge context menu
→ delete
→ `edge.removeInstance()`.

### Source behavior

For text-text endpoints, removal first calls `removeEdgeFromAllInstances(...)`, coupling deletion to textual/Zettelkasten representations. The edge instance is then removed from graph/projection structures.

### Candidate contracts

- `RemoveSemanticReference`
- `RemoveVisualEdge`
- `RemoveLayoutConstraint`

### Evidence

- `js/nodes/edgeclass.js`
- `js/interface/dropdown/customui/rightclick/customcontextmenu.js`

---

## B-CORE-011 — Delete node

**Source status:** SOURCE-CLOSED for principal graph + text semantic path

### Source chain

Projection/action delete
→ `node.remove()`
→ `Graph.deleteNode(node)`
→ remove incident edge views/edge registrations and node graph/projection state.

For text nodes, deletion also invokes the corresponding Zettelkasten parser's `deleteNodeByTitle(title)` path.

### Candidate contract

`DeleteEntity`
→ determine semantic ownership
→ remove entity
→ remove/reconcile relationships
→ remove projections
→ emit deletion event.

Deletion of a projection alone must be modeled separately from deletion of the underlying entity.

### Evidence

- `js/nodes/nodeutilities.js`
  - blob `d1f4d50ca71c0d47582f18b7ffc03dce73360659`
- `js/nodes/nodeclass.js`
- `js/nodes/createnodes/window.js`
  - blob `8508e764211ca9930de67d7037996cf661127d27`
- `js/zettelkasten/zetcodemirror.js`
  - blob `a3ab8c6fafc8b2ec9ddf055a347bd61f56452932`

### Runtime evidence still required

- delete confirmation behavior if any by surface
- selected-node batch deletion
- edge cleanup visual timing
- persistence after reload
- failure/partial-state behavior

---

## B-CORE-012 — Save workspace

**Source status:** SOURCE-CLOSED for storage architecture; serialization details still under dedicated persistence ledger

### Storage objects

`GraphsKeeper` owns LocalForage-backed stores:

- `blobs / blob-data`
- `graphs / blob-meta`
- `graphs / graph-data`
- `graphs / graph-meta`

Graph metadata includes creation/update timestamps, graph ID, revision count, size, and title.

`saveMetaAndData(meta, data)` increments revisions, calculates serialized size, stores graph data, then graph metadata.

Binary/blob assets are stored separately and associated through blob metadata.

### Candidate contract

`SaveWorkspaceSnapshot`
→ serialize explicit versioned workspace schema
→ persist durable state atomically
→ persist referenced blobs/content
→ update snapshot metadata/event history.

### Evidence

- `js/interface/dropdown/savenet.js`
  - blob `0f0574196e88450b40fc70fbad0a150f8243439d`

### Runtime evidence still required

- exact UI/autosave triggers
- interruption behavior
- failure behavior under storage quota/corruption
- content fidelity after reload

---

## B-CORE-013 — Load saved workspace

**Source status:** SOURCE-CLOSED for principal path

### Source chain

Saved graph Load button
→ optional autosave current selection
→ load graph data by graph ID
→ set selected graph
→ `#loadGraph(data)`
→ clear current graph
→ reconstruct content/state from serialized representation
→ refresh saved-graph UI.

### Candidate contract

`LoadWorkspaceSnapshot`
→ validate/version/migrate
→ stage reconstructed state
→ atomically promote loaded workspace
→ rebuild projections
→ report recoverable failures rather than silently partially loading.

### Evidence

- `js/interface/dropdown/savenet.js`

---

## B-CORE-014 — Export workspace package

**Source status:** SOURCE-CLOSED

### Source behavior

`GraphExporter` gathers graph data, blob metadata, and each blob. It computes byte offsets and returns:

`JSON({ data, blobMeta, offsets }) + NUL + concatenated blob bytes`.

### Candidate contract

`ExportWorkspacePackage`

The native successor should preserve importability of legacy exports but use an explicitly versioned portable package format for new workspaces.

### Evidence

- `js/interface/dropdown/savenet.js`

---

## B-CORE-015 — Import workspace package

**Source status:** SOURCE-CLOSED for principal parser path

### Source behavior

`GraphImporter` reads an ArrayBuffer, scans for the NUL delimiter, parses leading JSON, reconstructs blobs from offset/size metadata, repairs blob references into the graph data, then passes the reconstructed data into the regular load/save path. A plain-text fallback path exists when the binary package structure is not detected.

### Candidate contract

`ImportLegacyWorkspace`
→ parse in isolated compatibility reader
→ validate bounds/types/references
→ normalize to native workspace IR
→ save native snapshot.

### Evidence

- `js/interface/dropdown/savenet.js`

---

# Cross-cutting findings from these traces

## 1. Graph operations are frequently semantic operations

Text CREATE, CONNECT, DIRECTION, DISCONNECT, and DELETE cross the graph/Zettelkasten boundary. A direct port of graph methods would therefore lose semantics.

## 2. Projection classes currently carry authoritative-looking state

Collapsed and anchored states are partly encoded as DOM classes/datasets. Native contracts must move authoritative behavior state out of projection implementation details.

## 3. Movement is dynamic

Keyboard movement applies force into simulation rather than simply assigning coordinates. Golden tests must distinguish interaction value from implementation accident.

## 4. Save/load is a compound persistence protocol

Workspace data and blobs are separate stores and exports combine structured metadata with raw blob bytes. A compatibility reader is required even if new native persistence is SQLite/event/snapshot based.

## 5. The recursive source manifest is available

The pinned recursive Git tree returned `truncated: false`. Blob SHAs therefore provide an immutable repository-level content manifest for every tracked file. A generated path/blob/size manifest should be retained as the forensic checksum index.

---

# Remaining Gate A/B work after this ledger

## Source work

- exhaustive node-subtype behavior matrix
- exhaustive LocalStorage/LocalForage key ownership matrix
- exhaustive network/API/effect inventory
- Electron bridge behavior decomposition
- AI lifecycle and Neural API trace
- privilege/effect classification with blast radius/reversibility/auditability
- technology-neutral contract registry and K/R/S/B/X decisions

## Runtime work

- frozen legacy execution
- fresh-profile capture
- existing-workspace capture
- online/offline matrix
- missing-sidecar matrix
- local AI scenario
- cloud AI scenario
- screenshots/video/state snapshots
- failure injection
- persistence/reload verification
- representative golden fixtures for each behavior above

No behavior in this ledger is considered parity-proven until runtime and differential evidence exists.
