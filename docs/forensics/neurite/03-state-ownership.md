# Neurite Persistent State Ownership Ledger

Specimen: `satellitecomponent/Neurite`

Pinned main commit: `e62b270402b688a39a864007b9c0a02711b9573e`

This ledger covers the persistent browser-state surfaces observed in the pinned source. It separates workspace data, preferences, identity/session caches, credentials, execution history, and projection/UI caches so they cannot collapse into one native state domain.

## Persistence mechanisms

### LocalForage via `Stored`

`js/globals.js` defines `Stored`, which creates LocalForage instances. When called with one argument, database name is `Neurite` and store name is that argument. With two arguments, the first is the database name and the second is the store name.

No direct raw `indexedDB` use was found in the pinned source. No `sessionStorage` use was found.

### `window.localStorage`

Used for preferences, cached account/session information, provider credentials, custom model configuration, navigation/UI state, prompt library, filter state, and context-menu history.

### Serialized workspace representation

`js/interface/dropdown/savenet.js` serializes graph/projection state and separately stores blob data/metadata in LocalForage. Legacy export packages combine structured graph data with raw blobs.

---

# LocalForage stores

| Database | Store | Key family | Owner in legacy | Writers | Readers | Durability | Authority / regenerability | Native disposition |
|---|---|---|---|---|---|---|---|---|
| `Neurite` | `settings` | every `Settings.default` property by property name | global Settings | `Settings.set()` | Settings getters / UI | durable local preference | user preference; mostly regenerable from defaults | `UserPreferenceState` |
| `Neurite` | `functionCalls` | default `Stored` key (`functionCalls`) containing array of call records | `View.Code` | function/code execution and clear action | Function Call panel init | durable execution/history state | user history; regenerable only by re-execution, which may be unsafe/impossible | `ExecutionHistory`, NOT workspace authority |
| `blobs` | `blob-data` | `blobId` | `GraphsKeeper` | BlobSaver/save paths/import | exporter/load/import | durable workspace asset | workspace-owned binary content; not generally regenerable | `ExternalBlobState` |
| `graphs` | `blob-meta` | `graphId` → blob metadata dictionary | `GraphsKeeper` | save/import | exporter/UI | durable workspace metadata | workspace-owned; partly reconstructible from blobs, but metadata should be retained | `WorkspaceAssetIndex` |
| `graphs` | `graph-data` | `graphId` → serialized graph/workspace data | `GraphsKeeper` | save/import | load/export | durable workspace state | primary legacy workspace representation | legacy compatibility input only; normalize to explicit native schema |
| `graphs` | `graph-meta` | `graphId` → metadata | `GraphsKeeper` | add/save/rename | save-list UI/load | durable workspace metadata | metadata; partially regenerable except user title/timestamps/revision semantics | `WorkspaceSnapshotMetadata` |
| `state` | `GraphsView` | `latest-selected` | `View.Graphs` | graph selection/deletion | startup state loader | durable local UI/workspace pointer | local projection/navigation state; regenerable by choosing a save | `ProjectionState` |
| `state` | `GraphsView` | `autosave-enabled` | `View.Graphs` | autosave checkbox | startup state loader | durable preference | user preference; regenerable from default | `UserPreferenceState` |

## `functionCalls` record contents

Each persisted Function Call record can contain:

- `code`
- camera `zoom`
- camera `pan`
- derived/display `functionName`
- `isError`

The same subsystem executes code with JavaScript `AsyncFunction` in the page context. Therefore execution history and execution authority must be separated in the native system.

---

# Literal localStorage keys

## Account / remote-service cache

| Key | Owner | Writer | Reader | Meaning | Native disposition |
|---|---|---|---|---|---|
| `stripePublicKey` | sign-in/payment integration | `updatePublicKeys()` | payment/sign-in flow | Stripe public key cache | regenerable remote/public configuration cache |
| `turnstilePublicKey` | sign-in integration | `updatePublicKeys()` | sign-in flow | Turnstile public key cache | regenerable remote/public configuration cache |
| `userEmail` | authentication UI/session cache | `updateSignInState()` | sign-in UI, account-delete request, panel | cached identity display/session hint | `SessionCache`; remote backend remains authority |
| `lastMeCheck` | authentication session cache | `updateSignInState()` / `checkSessionValid()` | session-validity throttle | timestamp of last `/oauth/me` validation | regenerable cache, never authority |
| `userBalance` | Neurite account panel | `fetchUserBalance()` after backend result | panel | cached server-reported account balance | regenerable cache, remote backend remains authority |

## Credentials

The provider registry plus generic LocalStorage key helper yields these credential keys:

| Key | Provider / purpose | Native disposition |
|---|---|---|
| `anthropicApiKey` | Anthropic | secret reference / credential vault only |
| `googleApiKey` | Google APIs | secret reference / credential vault only |
| `googleSearchEngineId` | Google Programmable Search engine ID | provider config; may not be secret but should remain provider-scoped |
| `GROQApiKey` | Groq | secret reference / credential vault only |
| `openaiApiKey` | OpenAI | secret reference / credential vault only |
| `wolframApiKey` | Wolfram Alpha | secret reference / credential vault only |

Legacy code loads/saves these directly to/from browser LocalStorage. It can also export them to a plaintext JSON `.txt` file through browser file-picker APIs. Native code must not reproduce this as ordinary workspace persistence.

## Custom model configuration

| Key | Owner | Contents | Native disposition |
|---|---|---|---|
| `customModelDropdown` | custom model selector | serialized model options including `value`, `text`, `key`, and `endpoint` | split endpoint/model metadata from credential reference |
| `customModelDropdown_selected` | custom model selector | selected option ID | user/provider preference |

**Critical:** the legacy serialized option includes `option.dataset.key`, so a custom endpoint credential can be persisted inside `customModelDropdown` rather than only in the provider-key family.

## UI/input/preferences

| Key | Owner | Writer / reader | Meaning | Native disposition |
|---|---|---|---|---|
| `inputValues` | dropdown/global input persistence | dropdown controls; save/load also embeds/restores it | map of input element IDs to values | schema-versioned user/workspace preferences after classifying each field |
| `modalInputValues` | Modal | `Modal.storeInputValue` / `Modal.loadInputValues` | map of modal input IDs to values | user preference / feature config, not generic arbitrary map long term |
| `currentPath` | FileTree/global | FileTree + globals | most recent filesystem path | local projection/navigation state; must not itself confer filesystem authority |
| `controls` | custom controls | controls UI | input binding configuration | user preference |
| `promptLibrary` | AI prompt library | prompt-library mutations / initialization | user prompt templates | user-owned library state |
| `keyFilters` | embeddings/search UI | filter toggle / initialization | serialized Map of embedding-key visibility filters | local search/UI preference |
| `savedViews` | saved-coordinate UI | view cache writer/reader | saved fractal/camera views | workspace/user view state; classify during migration |
| `pinnedContextMenuItems` | context menu | `Manager.PinnedItems` | pinned node-action IDs | user UI preference |
| `nodeMethodCalls` | context menu | `Manager.RecentSuggestions` | recent node-action IDs | user UI history/preference |

---

# Dynamic localStorage key families

A literal-key inventory alone is insufficient because legacy code deliberately derives keys at runtime.

## `selectElement.id`

`CustomDropdown.setupModelSelect()` stores and restores the selected value under the DOM ID of every `select.custom-select:not(.ignoreSetup)` element.

Native interpretation:

`DynamicSelectPreference { semantic_setting_id, selected_value }`

The DOM ID must not become the durable schema identifier by accident.

## `Select.storeSelectedValue(selectId)`

Stores selected values using arbitrary caller-supplied select IDs. This overlaps the preceding family and must be normalized into named settings/contracts.

## Provider key family: `provider.storageId || provider.inputId`

The provider registry controls the effective key. The full currently observed provider family is recorded above.

## Context-menu manager `storageId`

Two concrete constructor values are currently observed:

- `pinnedContextMenuItems`
- `nodeMethodCalls`

Any future constructor value is another persistence key, so the native equivalent must declare a storage namespace rather than silently allocate LocalStorage keys.

## Custom-dropdown `storageId` and `${storageId}_selected`

Concrete observed custom-model storage ID:

- `customModelDropdown`
- `customModelDropdown_selected`

The helper is generic and could support further instances.

---

# Serialized per-node state

Persistent node state is not confined to LocalStorage/LocalForage keys. `Node.toJSON()`, DOM datasets, `save_extras`, and type-specific `push_extra_cb` callbacks serialize model/projection details.

Observed type-specific durable fields include:

## Base/spatial node

- UUID
- position
- velocity/force-related fields as selected by legacy serializer
- anchor state
- scale
- edge membership/edge data
- type flags and extras necessary for reconstruction

## Text node

- title
- text content
- code editing/projection state where serialized elsewhere

## AI node

- title
- custom instructions
- current prompt text
- response/conversation text
- option checkbox states
- temperature
- max tokens
- max context
- provider/model selections via associated controls/state

## Link node

- title
- URL / link state
- projection depends on runtime (`iframe` vs Electron `webview`)

## Image node

- title
- image source metadata
- blob reference for blob-backed image content

## File-tree node

- path
- projection/browser state

The native schema must make these fields explicit and versioned. Projection-only implementation artifacts (DOM class names, element paths, dataset encodings) must not become domain state.

---

# State-domain classification for the Rust successor

```text
WorkspaceState
  entities
  semantic relationships
  spatial/layout state selected for durability
  note/Zettelkasten content
  workspace-owned media references

SnapshotState
  revision
  migration/version data
  atomic snapshot metadata

ProjectionState
  collapsed/open presentation
  selected panes/views
  temporary browser/media projection details

UserPreferenceState
  controls
  model/provider defaults
  UI settings
  saved UI preferences

CredentialReferenceState
  references to secrets held by a credential broker/vault
  NEVER raw secret values in workspace state

SessionCache
  user identity display
  last backend validation
  cached remote balance/config
  explicitly non-authoritative

ExecutionHistory
  prior function/code executions
  results/errors
  separate from authority to execute

ExternalBlobState
  workspace media/blob content
  content hashes
  ownership and portability metadata
```

---

# Migration invariants derived from the state audit

1. A persisted filesystem path must not imply authority to read that path.
2. A persisted code snippet/history record must not imply authority to execute it.
3. A model/provider selection must not imply network permission or credential access.
4. Cached remote identity/balance/configuration is not authoritative remote state.
5. Secrets are references to separately protected capability-scoped credentials, never ordinary workspace fields.
6. DOM element IDs/classes/datasets are not durable domain schema.
7. Legacy workspace import is a compatibility parsing problem; new workspace persistence uses explicit versioned types.
8. Saved graph selection/autosave state is projection/preference state, not workspace truth.
9. Blob ownership, graph snapshot metadata, and semantic workspace data remain separable.

---

# Evidence paths

Primary evidence:

- `js/globals.js`
- `js/interface/dropdown/savenet.js`
- `js/interface/dropdown/signin.js`
- `js/interface/dropdown/neuritepanel.js`
- `js/interface/dropdown/dropdown.js`
- `js/interface/dropdown/customui/customdropdown.js`
- `js/interface/dropdown/customui/customcontrols.js`
- `js/interface/dropdown/customui/custommodal.js`
- `js/interface/dropdown/customui/rightclick/suggestions.js`
- `js/interface/dropdown/customui/displaysavedcoords.js`
- `js/interface/searchapi/embeddingsdb.js`
- `js/ai/ai-utility/aihelpers.js`
- `js/ai/ai-utility/handleapikeys.js`
- `js/nodes/nodetypes/ainodes/promptlibrary.js`
- `js/interface/functioncall/functioncallingpanel.js`
- node type serializers under `js/nodes/`

The pinned recursive tree is complete (`truncated: false`), so these paths can be joined to immutable blob SHAs in the forensic KG/manifest.

# Remaining runtime evidence

This closes the **source-level persistence surface inventory**. It does not establish:

- actual browser storage backend selected by LocalForage on each environment
- quota behavior
- corruption behavior
- atomicity under crash
- stale-version compatibility
- exact legacy save/reload fidelity

Those remain runtime/golden-test obligations.
