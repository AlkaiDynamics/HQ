# Neurite Electron Runtime Decomposition

Electron is a separate upstream specimen from the web/main branch.

## Frozen desktop baseline

Repository: `satellitecomponent/Neurite`

Branch: `electron`

Pinned commit: `ab38b3f8d7edb53ba78affff91f30295d0282701`

Pinned tree: `d5231b9fd5cd0e67d290a08d17dbaa82434382d6`

The recursive Electron tree was returned complete (`truncated: false`).

The main branch desktop release workflow explicitly checks out `ref: electron`, so Electron behavior must be treated as a distinct source baseline rather than inferred from `main`.

---

# Desktop component map

```text
Electron main.js
  ├─ update.js
  ├─ loadingwindow.js
  ├─ windowmanager.js
  │    ├─ frontendupdater.js
  │    ├─ frontendserver.js
  │    └─ contextmenu.js
  ├─ securefetch.js
  │    └─ proxypreload.js
  ├─ serverdownloader.js
  ├─ servermanager.js
  ├─ shortcuts.js
  └─ logging.js

Renderer BrowserWindow
  └─ preload.js
       └─ window.electronAPI
```

---

# Startup workflow

`app.whenReady()`

1. `initializeUpdater()`
2. if updater initiates restart, stop startup
3. create loading window
4. initialize secure-fetch proxy window/IPC
5. create main BrowserWindow
6. register shortcuts
7. concurrently:
   - obtain/start localhost server bundle
   - await renderer `renderer-ready`
8. after both complete, execute `Host?.checkServer?.()` in renderer
9. close loading window
10. show main window

The desktop application therefore has two separate readiness domains:

- local privileged services
- browser renderer

The successor should preserve the useful readiness/degradation semantics without requiring all optional capabilities before the native shell can become usable.

---

# Main BrowserWindow security configuration

Observed settings:

```text
nodeIntegration: false
contextIsolation: true
nodeIntegrationInWorker: false
nodeIntegrationInSubFrames: false
enableRemoteModule: false
webSecurity: true
webviewTag: true
preload: modules/preload.js
```

`devTools: true` is enabled.

On macOS, title-bar details differ.

No standard application menu is used.

These are useful security intentions, but they do not eliminate the separate privileged effects exposed through preload IPC, sidecars, runtime downloads, and browser/webview capabilities.

---

# Renderer preload bridge

`modules/preload.js` exposes `window.electronAPI`:

```text
startedViaElectron
sendReady()
secureFetch(endpoint, options)
sendStreamRequest(id, endpoint, options)
_addStreamListener(handler)
_removeStreamListener(handler)
openPopupViaProxy(url)
```

It also:

- forces webFrame zoom to 1.0
- forwards sanitized `auth-message` IPC payloads into renderer `window.postMessage`
- forwards context-menu coordinates to Electron main

This is a narrow bridge relative to enabling Node integration directly, but it still provides meaningful network/auth/browser authority.

---

# Frontend acquisition and fallback

`windowmanager.createMainWindow()` calls `ensureFrontendDownloaded()`.

## `frontendupdater.js`

Uses GitHub Releases API:

```text
satellitecomponent/neurite
release prefix: frontend-
asset: frontend.zip
```

Behavior:

1. query up to 100 releases
2. select newest final `frontend-*` release
3. use cached version if release lookup fails
4. if version is not extracted locally:
   - download ZIP
   - extract into Electron userData/frontend/<version>
   - write version cache
   - delete old extracted versions
5. return extracted frontend root

If neither network release lookup nor cached frontend is available, it returns null.

`windowmanager.js` then falls back to:

`https://neurite.network`

Therefore a fresh desktop instance, as represented by this branch, is not statically demonstrated to satisfy the target requirement of fully offline fresh startup. Runtime tests are still required, but source already identifies the dependency path that must be removed in the successor.

## Integrity finding

The observed frontend download path does not perform an explicit content hash/signature verification before extraction.

Native replacement:

- core frontend/UI is compiled/bundled with the native app
- optional downloadable artifacts use signed/content-addressed manifests
- cached optional assets cannot determine whether the core shell starts

---

# Local server bundle acquisition

`serverdownloader.js` uses GitHub Releases API:

```text
release prefix: servers-
asset: servers.zip
```

Behavior:

- cache lives under Electron userData/servers
- tracks selected version in `version.json`
- if latest lookup fails, can use cached version
- if required server launcher is absent and no network release exists, startup throws `No usable server version available`
- downloads `servers.zip`
- extracts into version directory
- preserves selected paths from old server tree:
  - `database.db`
  - `node_modules`
- deletes old version directories after successful update

## Important migration consequence

The sidecar database and dependency directory are explicitly carried forward between downloaded server versions. That is an implicit migration strategy and needs explicit data/version contracts in the native successor.

## Integrity finding

No explicit asset signature/hash verification is observed before ZIP extraction.

---

# Local server runtime

`servermanager.js`:

1. checks `http://localhost:7070/check`
2. prepares log at Electron userData/server-install.log
3. tests whether `node_modules`, Express, and package lock exist
4. if not, spawns:

```text
npm install --prefer-offline --no-audit
```

with inherited process environment and shell enabled
5. dynamically imports `start_servers.js`
6. polls `/check` every 500ms up to 30 seconds

This is not suitable as a trusted native-core mechanism.

Native replacement:

```text
Capability declaration
→ approved RuntimeBinding
→ provisioned isolated artifact
→ health/readiness state
→ typed IPC
```

No runtime dependency installation inside the native UI process.

---

# Server shutdown observation

`servermanager.js` declares a module-level `childProcess`, and `stopLocalServers()` attempts to tree-kill it if set.

However, the shown startup path dynamically imports `start_servers.js` rather than assigning a spawned server process to `childProcess`.

This creates a source-level lifecycle mismatch worth testing at runtime: the intended shutdown mechanism may not actually own the imported local-server lifetime.

Do not promote this from finding to confirmed bug until runtime reproduction verifies actual shutdown behavior.

---

# Secure fetch/browser proxy architecture

Electron-local frontend backend requests do not call remote hosted APIs directly from the ordinary renderer path.

## Flow

```text
Renderer
  ↓ preload IPC
Electron main
  ↓
Hidden secure-proxy BrowserWindow
  ↓
https://neurite.network/resources/proxy/proxy.html
  ↓ network request
response/auth data
  ↓ IPC
Renderer
```

`securefetch.js` creates hidden BrowserWindow with:

```text
nodeIntegration: false
contextIsolation: true
sandbox: true
webSecurity: true
allowRunningInsecureContent: false
enableRemoteModule: false
partition: persist:secure-proxy
devTools: false
```

It supports:

- non-streaming fetch
- streaming fetch
- popup forwarding
- auth-message forwarding

The hidden proxy uses a persistent Electron partition, meaning browser session/cookie state is intentionally separated from the normal renderer while surviving within that partition.

Native direction:

- ordinary HTTP/provider clients for normal API traffic
- browser-mediated auth isolated to a BrowserRuntime only where required
- session state explicitly owned by that runtime
- no browser proxy as a general-purpose authority bridge

---

# Desktop application self-update

`update.js`:

1. queries GitHub Releases API for newest final `electron-*` release
2. compares timestamp-style embedded version
3. selects platform installer:
   - Windows `.exe`
   - macOS `.dmg`
   - Linux `.AppImage`
4. prompts user `Update & Restart` / `Later`
5. downloads selected asset to OS temp directory
6. calls `child_process.spawn(downloaded_asset, [], { detached: true })`
7. quits current application

## Integrity finding

The observed path does not perform explicit hash/signature verification of the downloaded executable before spawning it.

This is a high-authority software-supply-chain effect and must not be copied into the trusted native core as-is.

Native updater requirements should include:

- signed release metadata
- content digest verification
- rollback
- atomic replacement
- migration compatibility check
- explicit channel/version policy
- recorded update event

---

# Desktop failure/degradation paths visible in source

## Updater release lookup fails

Application logs warning and continues startup.

## Frontend release lookup fails

- cached frontend → use cache
- no cache → return null → window manager falls back to hosted neurite.network

## Server release lookup fails

- cached server version may be usable
- no usable server files → throws

## Server dependency installation fails

Startup server promise catches/logs failure in `main.js`; main awaits server promise and renderer readiness. The caught async wrapper allows startup sequence to continue to the post-wait stage, then renderer Host check determines local-service connectivity.

## Renderer hook failure

Caught/logged after readiness.

## Main-window load local frontend fails

Attempts hosted neurite.network fallback.

## Secure proxy cleanup failure

Window is force-destroyed after timeout.

These source-visible degradation paths need runtime fixtures, especially offline/no-cache behavior.

---

# Desktop state domains

```text
ElectronAppState
  build version
  update state

DownloadedArtifactState
  frontend version cache
  server version cache
  extracted artifacts

LegacyServiceState
  preserved database.db
  node_modules (legacy only)
  server install log

BrowserSessionState
  persist:secure-proxy partition

RendererState
  browser LocalStorage / LocalForage / workspace
```

The native system should not retain `node_modules` as durable application state. It is an artifact of the legacy runtime.

---

# Candidate native contracts extracted from Electron

```text
ApplicationReadyState
CapabilityReadyState
RuntimeBindingHealth
OptionalArtifactManifest
ArtifactCache
SignedUpdateManifest
UpdateIntent
UpdateResult
BrowserAuthSession
SecureHttpRequest
StreamedHttpResponse
LocalServiceBinding
LegacyRuntimeBinding
```

---

# K/R/S/B/X preliminary treatment

| Legacy desktop mechanism | Preliminary treatment |
|---|---|
| native desktop shell concept | K — KEEP NATIVE, reimplemented in Rust |
| narrow renderer/main authority bridge pattern | R — retain concept, replace Electron IPC with typed native interfaces |
| hidden browser auth session where unavoidable | S/B — sandbox/bridge |
| runtime download of entire frontend | X after native UI parity; not needed in native core |
| runtime download of legacy servers | S during migration, then shrink/retire |
| `npm install` during app startup | X from trusted core; legacy capsule only |
| direct dynamic import of sidecar launcher into desktop process | X from trusted core |
| legacy Electron webview browsing | B/S until replaced by explicit BrowserRuntime binding |
| automatic executable download/spawn updater as implemented | R — retain update capability, replace mechanism with verified native updater |

These are preliminary source-derived classifications; final feature classification occurs against behavioral/golden evidence.

---

# Runtime tests still required

1. fresh install with network available
2. fresh install with network unavailable
3. existing frontend + server caches, offline
4. frontend cache only
5. server cache only
6. corrupted frontend ZIP/extracted cache
7. corrupted server ZIP/extracted cache
8. package install failure
9. localhost:7070 already occupied
10. one local service fails to mount
11. renderer never emits ready
12. secure proxy page unavailable
13. auth popup flow
14. update declined
15. update download failure
16. app shutdown with local servers active
17. validate whether local server actually terminates on application quit
18. cookie/session persistence of secure-proxy partition
19. hosted-frontend fallback behavior
20. Electron webview LinkNode behavior

Until these execute against the frozen Electron baseline, desktop Gate A remains open.
