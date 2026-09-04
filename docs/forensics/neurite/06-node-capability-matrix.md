# Neurite Node / Projection Capability Matrix

Specimen: `satellitecomponent/Neurite`

Pinned main commit: `e62b270402b688a39a864007b9c0a02711b9573e`

The frozen `js/nodes/nodetypes/` tree contains exactly seven top-level node families:

```text
ainodes
filetreenodes
imagenodes
linknodes
medianodes
textnodes
wolframnodes
```

This inventory separates **entity/content kind**, **projection**, **runtime binding**, and **capabilities**. Legacy `Node.getType()` only exposes the action families `base`, `text`, `llm`, and `link`; therefore the old notion of “node type” is demonstrably not one coherent type axis.

---

# Shared Base Node

## Legacy responsibilities

A `Node` can own or participate in:

- UUID identity
- position, velocity, force
- scale
- anchor state
- mouse/following state
- physics/fractal-gradient integration
- edge membership
- DOM content/projection
- serialization/reconstruction extras
- selection/collapse handling through associated view/state helpers

## Shared actions

The `NodeActions.base` surface exposes:

- `zoomTo`
- `follow`
- `delete`
- `toggleSelect`
- `toggleCollapse`
- `toggleAutomata`
- `spawnNode`
- `connect`

`moveNode` and `moveTo` methods exist in the action class although they are not currently advertised in `getActions()`.

## Native decomposition

```text
SpatialEntity
Projection
LayoutBody
SelectionState
ActionDescriptor[]
RelationshipEndpoint
```

A node projection should not imply the authority of any runtime/content capability it happens to display.

---

# 1. Text node family

Source family tree:

- `textnode.js`
- `contenteditable.js`

Cross-cutting execution behavior:

- `js/nodes/nodeinteraction/bundlecode.js`

## Content/entity behavior

Text nodes contain:

- title
- text body
- editable/syntax-highlighted projection
- Zettelkasten semantic identity/reference behavior

Creating ordinary canvas text nodes is mediated by the Zettelkasten textual representation rather than direct graph insertion.

Connecting/deleting text nodes also crosses the Zettelkasten semantic boundary.

## Projection modes

The same text node can present:

- ordinary text editing
- syntax-highlighted editing
- web-code execution result iframe
- Python execution output panel

## Advertised specialized action

`toggleCode`

This calls the code execution path; therefore it is not merely cosmetic projection switching.

## Code behavior

Python fences:

- dynamically acquire Pyodide
- inspect imports
- dynamically load packages
- execute Python asynchronously
- render output into the text node's Python result projection

HTML/CSS/JavaScript fences:

- combine code from the current node and connected nodes
- generate HTML/CSS/JS bundle
- place result into iframe `srcdoc`
- execute included JavaScript in the iframe environment

## Durable behavior

Observed serialized extras include:

- title
- text body

Semantic note content is also represented in Zettelkasten text and must reconcile with the spatial entity/projection.

## Native contracts

```text
TextDocument
SemanticNote
TextProjection
SyntaxProjection
CodeExecutionIntent
PythonRuntimeBinding
WebRuntimeBinding
SemanticReference
```

## Preliminary treatment

- text/notes: **K**
- Zettelkasten semantics: **K/R**
- syntax/code projection: **K/R**
- Python execution: **S**
- web-code execution: **S**

---

# 2. AI node family

Source family tree:

- `ainode.js`
- `ainodemessage.js`
- `promptlibrary.js`
- `responsehandler.js`

## Content/entity behavior

AI node contains conversation and inference configuration rather than merely a rendered chat box.

Observed state includes:

- title / identity
- prompt
- response/conversation transcript
- custom instructions
- model/provider selection
- temperature
- max tokens
- max context
- feature checkboxes
- responding/halted state
- auto-mode state
- connected-node/message-loop state

## Specialized actions

- `sendMessage`
- `settings`
- `halt`
- `refreshResponse`

## Context assembly

Before inference the node can assemble context from:

- identity prompt
- custom instructions
- prior conversation
- connected AI nodes
- connected text nodes
- connected image nodes
- connected link data
- local embedding retrieval
- Google search
- Wikipedia
- Wolfram Alpha
- code/instruction prompts

Token/context accounting trims context to configured limits.

## Inference behavior

Provider is selected dynamically. Legacy pathways include:

- Neurite hosted provider
- OpenAI
- Groq
- Anthropic through proxy path
- Ollama
- custom endpoint

Calls can stream and can be halted via AbortController/cancel logic.

## Multi-agent behavior

Connected AI nodes form a message graph. Model output can use mention/control syntax including:

- named `@recipient`
- `@all`
- `@user`
- `@memory`

Observed commands include:

- `/rewrite <node>`
- `/disconnect <node>`
- `/exit`

`@memory` can spawn a new memory/text node and connect it. Commands can mutate note/graph relationships.

Therefore AI output is presently capable of becoming a workspace action, not merely text.

## Auto mode

An AI node can recursively derive and send a new self-prompt after completing an inference response.

## Halt behavior

Halt can:

- abort current request
- set response state
- clean message loop
- close an incomplete rendered code block
- recursively halt connected LLM nodes
- remove active request bookkeeping

## Response projection

`ResponseHandler` manages rendered responses, code blocks, restoration, and resend/regenerate-related behavior.

## Durable behavior

Node extras persist prompt, response, instructions, checkboxes and model tuning state. Provider preferences/credentials are also stored through separate global state mechanisms.

## Native contracts

```text
ConversationEntity
AiAgentProjection
ModelProvider
InferenceRequest
InferenceStream
ContextAssembly
RetrievalRequest
AgentMessage
ProposedIntent
ExecutionAdmission
ConversationTopology
```

## Preliminary treatment

- conversation/agent projection: **K/R**
- local model interface: **K/R**
- remote inference: **B**
- model-generated workspace mutations: **R** behind command/admission layer
- legacy direct proxy/provider wiring: **X/R** after provider abstraction
- recursive auto mode: **R**, governed by continuity/authority policy

---

# 3. Link / browser node family

Principal source:

- `linknode.js`

Related extraction/import behavior exists in search/embed modules.

## Entity behavior

Represents a URL/web resource with mutable current navigation URL/title.

## Projection behavior

Environment-dependent:

- ordinary browser → iframe
- Electron → webview

Electron webview enables:

- navigation
- history back/forward
- refresh
- zoom factor

Browser iframe provides more constrained equivalents where origin rules permit.

## Specialized actions

- `toggleLink`
- `extractText`
- `importText`

## Extract behavior

`extractText` routes remote content through web extraction/vector storage flows.

## Import behavior

`importText` converts web-resource text into Zettelkasten/note material.

## Proxy display

Can ask the WebScrape sidecar for raw proxied HTML and display it via object URL.

## Native contracts

```text
WebResourceEntity
BrowserProjection
BrowserRuntimeBinding
WebFetchCapability
WebExtractionCapability
ImportAsTextCommand
NavigationState
```

## Preliminary treatment

- URL resource entity: **K**
- browser projection: **B/S**
- extraction: **R/S**
- raw legacy proxy: **S/X** after safe fetch service exists

---

# 4. File-tree node family

Principal source:

- `filetreenode.js`

Related implementation:

- `js/interface/filetree.js`
- DirectAccess sidecar

## Entity/projection behavior

Creates a resizable file-tree browser rooted at a path.

State includes:

- current `filePath`
- current projection navigation

Path changes update node path state.

## Runtime behavior

Actual enumeration/read authority is performed by DirectAccess sidecar, which accepts resolved host paths.

Dropping a folder can create a FileTreeNode rooted at that folder.

## Native contracts

```text
FilesystemLocationReference
FileTreeProjection
DirectoryEnumerateIntent
FileOpenIntent
ScopedFileCapability
```

`filePath` is data, not authority.

## Preliminary treatment

- file-tree UX: **K/R**
- host filesystem access mechanism: **X/S**, replaced by scoped file capability

---

# 5. Image node family

Principal source:

- `imagenode.js`

## Entity/projection behavior

- displays image
- derives initial dimensions from natural image aspect ratio
- normalizes initial height to 600px in the legacy projection
- creates standard Node/NodeView wrapper

## Durable state

- title
- image source (`imageData`)
- if using `blob:` source, records blob association/title

## Import behavior

OS/file-tree drop creates blob URL and image projection after image load.

## Native contracts

```text
MediaAsset(Image)
ImageProjection
BlobReference
MediaImportCommand
```

## Preliminary treatment

**K/R** — straightforward native media projection; browser object URLs are implementation detail only.

---

# 6. Media node family

Principal source:

- `medianode.js`

## Variants

- audio
- video

They are one implementation family rather than separate domain node architectures.

## Behavior

- create HTML media element
- attach playback controls
- assign source URL
- video projection limits initial height to 600px while preserving aspect ratio
- title derives from filename/metadata

## Import behavior

OS/file-tree drops create blob URLs and then media nodes.

## Native contracts

```text
MediaAsset(Audio|Video)
AudioProjection
VideoProjection
PlaybackState
MediaImportCommand
```

## Preliminary treatment

**K/R** — native media playback/projection; source/blob mechanism can change.

---

# 7. Wolfram result node family

Principal source:

- `wolframnode.js`

## Behavior

Consumes already-produced Wolfram data and creates a standard spatial node containing the returned result table.

Title is derived from reformulated query plus a result suffix.

New result node initially follows the mouse and uses an adjusted mouse anchor.

## Important distinction

The node is a **result projection**. It does not own Wolfram network authority. Query execution belongs to the Wolfram/provider capability.

## Native contracts

```text
ComputationResultEntity
TableProjection
WolframProviderBinding
```

## Preliminary treatment

- generic computation/result projection: **K/R**
- Wolfram-specific provider: **B**

---

# Imported forms that are NOT distinct node architectures

The UI advertises more content types than the seven node-family folders because several are represented through existing families.

## PDF

Drop handling treats PDF as an application blob and creates a `LinkNode` over a blob URL. Therefore PDF is currently a **Web/Link projection specialization**, not an independent domain node family.

Native direction:

```text
DocumentAsset(PDF)
→ PdfProjection
```

No reason for its logical entity to inherit browser/link semantics.

## Source-code files

Code MIME types are read as text and wrapped in fenced code, then created as ordinary text nodes.

Native direction:

```text
TextDocument
+ LanguageMetadata
+ CodeProjection
+ optional ExecutionCapability
```

## Plain files

Text/application content judged textual becomes a text node.

## Raw pasted HTML

Pasted HTML can create a generic Node with an HTML div projection directly. This is another legacy projection path rather than a distinct stable entity kind.

## URLs / pasted iframe snippets

Become LinkNodes.

## Folder drop

Becomes FileTreeNode.

---

# Node action architecture finding

Legacy action dispatch already contains the seed of the target capability/action model:

```text
Node
→ Node.getType()
→ NodeActions.<type>
→ getActions()
→ executeNodeMethod()
```

But it currently:

- conflates harmless UI actions with consequential effects
- parses method names/arguments from strings
- invokes methods directly
- uses a coarse node action type (`base/text/llm/link`)

Native replacement:

```text
Entity + Projection + Context
  ↓
ActionDescriptor {
  id
  label
  parameter_schema
  applicability
  required_capabilities
  required_authority
  reversibility
  executor_binding
}
  ↓
Intent
  ↓
Admission
  ↓
Command
  ↓
Executor
  ↓
Event
```

Purely local visual actions can take a short path where appropriate.

---

# Capability matrix

| Family / form | Core data | Projection | External/runtime capability | Semantic graph effect | Native treatment |
|---|---|---|---|---|---|
| Base spatial entity | identity, position, scale, layout | native spatial chrome | none inherently | edges/layout | K |
| Text | note/text | text/syntax | optional code runtimes | Zettelkasten references | K/R |
| Text + Python | text/code | Python output | Python execute/package load | none inherently | S |
| Text + web code | text/code | iframe/web result | Web/JS execute | none inherently | S |
| AI | conversation/config | chat/agent | model/search/retrieval | agent messaging, rewrite/memory/disconnect | K/R + B/S effects |
| Link | URL | browser/webview | network/browser | import can create note content | K entity, B/S projection |
| FileTree | path reference | tree browser | filesystem enumerate/read | imported/opened content can create nodes | K/R UI, capability-gated FS |
| Image | asset/blob ref | image | media decode | none inherently | K/R |
| Audio | asset/blob ref | player | media decode/playback | none inherently | K/R |
| Video | asset/blob ref | player | media decode/playback | none inherently | K/R |
| Wolfram result | result/table | table | none for projection; provider for query | none inherently | K/R result + B provider |
| PDF import | document/blob | legacy LinkNode | PDF/browser rendering | extraction may enter index | K document + native PDF projection |
| Code-file import | text + language | text/code | optional execution | note semantics if imported as text | K/R + S execution |
| Raw pasted HTML | markup | direct DOM projection | browser HTML behavior | none inherently | R/S; sanitize/isolate |

---

# Source-level node-family closure

The source-level **node-family inventory is now closed** for the pinned main tree: all seven folder families and the additional import forms that users experience as media/document/code nodes have a mapped data/projection/capability decomposition.

This does **not** close runtime/golden behavior for:

- exact visual dimensions/transitions
- malformed media/document handling
- navigation failures
- codec/platform differences
- provider failures
- AI streaming timing
- execution sandbox behavior
- dropped-file edge cases
- restoration from legacy workspaces

Those remain golden-test obligations.
