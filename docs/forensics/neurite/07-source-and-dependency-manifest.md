# Neurite Immutable Source / Dependency Manifest

This document closes the **source-level repository/dependency/checksum manifest** for the frozen Neurite specimens. Runtime reproduction still requires an executing legacy environment.

---

# 1. Primary web/main specimen

Repository: `satellitecomponent/Neurite`

Branch: `main`

Pinned commit:

```text
e62b270402b688a39a864007b9c0a02711b9573e
```

Pinned recursive tree:

```text
d23fee7f1ade4886841a1e595540ad3a00a41ec6
```

GitHub returned this recursive tree with:

```text
truncated: false
```

Therefore the Git tree is the immutable checksum manifest for the entire tracked main specimen. Every tracked path is represented by:

```text
path
mode
type
Git object SHA
size (for blobs)
```

The root tree SHA is a Merkle commitment over all descendant trees/blobs. Any tracked-file content change, addition, deletion, rename, or tree restructuring produces a different object path/tree lineage and ultimately a different root tree SHA.

## Important root object checks

| Path | Git blob SHA | Size |
|---|---|---:|
| `LICENSE` | `16fcb872b8ccc84359aa939c2004009c528728f4` | 1,075 |
| `README.md` | `6532b71e300d28564e322f12ec80b91dfa478b68` | 27,179 |
| `index.html` | `d1d340ae2f0661153703d0d31959352635217ec9` | 3,183 |
| `package.json` | `44c73a7ae0ed8b518a8612ee460b1762219c463f` | 569 |
| `js/main.js` | `ceb2ed3d5faa03612c11dd02da884a3ec1651528` | 12,549 |
| `js/globals.js` | `bfa0802451b92f7dd0db4b8695aee0f474d48825` | 17,310 |
| `js/mandelbrot/mandelbrot.js` | `69afcd4d33219fafc01205b995dbf51e4af2e7c4` | 33,996 |
| `js/nodes/nodeclass.js` | `990972e7cf640ec44ba0d3121cc945c77687a6a9` | 17,256 |
| `js/nodes/edgeclass.js` | `ceca876f6820e9c9b7613fd351a7f12e196e97e6` | 15,395 |
| `js/nodes/nodeutilities.js` | `d1f4d50ca71c0d47582f18b7ffc03dce73360659` | 10,660 |
| `js/interface/dropdown/savenet.js` | `0f0574196e88450b40fc70fbad0a150f8243439d` | 33,936 |
| `js/interface/neuralapi.js` | `40c680d544858aafa3f3502c601647cc31947e25` | 25,877 |
| `js/nodes/nodetypes/ainodes/ainode.js` | `481b61385c4ebce28698ed142cb77145fd9d330d` | 25,521 |
| `js/nodes/nodetypes/ainodes/ainodemessage.js` | `689b465abd9ca380911505dd616c76cb39756173` | 33,228 |
| `localhost_servers/start_servers.js` | `a4cf5fb97e18809d2a518cf0be0b0885e4eca431` | 3,062 |
| `localhost_servers/servers.json` | `83bc4076d22f8832100a211a91075d38e008dab4` | 560 |

These are convenience anchors only; the complete recursive tree is the full manifest.

---

# 2. Branch inventory at capture

Observed branches and tips:

| Branch | Commit | Protected |
|---|---|---|
| `dev` | `2d08ae418cde8d248ca620601033fd2371e219f1` | no |
| `electron` | `ab38b3f8d7edb53ba78affff91f30295d0282701` | no |
| `gh-pages` | `525b087932e8fad219194078415887780467ee82` | no |
| `main` | `e62b270402b688a39a864007b9c0a02711b9573e` | yes |
| `test-page` | `8ad2f85ba37392705f3704c7b993e17920e9ef64` | no |

`main` and `electron` are the two specimens currently frozen for behavioral reconstruction. Other branches are preserved in the upstream ledger but are not yet treated as parity baselines.

---

# 3. Release families observed

The repository uses at least three release/tag families:

```text
electron-<timestamp>
frontend-<timestamp>
servers-<timestamp>
```

Observed release examples at capture:

- `electron-2025.06.18.233853`
- `frontend-2025.05.02.191757`
- `servers-2025.04.12.023909`

The Electron updater/downloader source discovers these families dynamically via GitHub Releases.

Release assets can have GitHub-supplied SHA-256 digests when GitHub exposes them. Example observed Electron release assets include SHA-256 digests for DMG, AppImage, and EXE artifacts. The legacy updater code itself does not explicitly verify those digests before launching/extracting downloaded artifacts.

---

# 4. Root package manifest — browser/Vite app

Root package manifest blob:

```text
44c73a7ae0ed8b518a8612ee460b1762219c463f
```

Declared package dependencies:

```text
runtime dependencies: none

devDependencies:
  vite: ^6.1.1
  shx: ^0.3.3

overrides:
  glob: ^11.0.1
  vite.esbuild: ^0.25.0
```

Scripts:

```text
start       vite
start:host  vite --host
build       vite build
postbuild   copy js/resources/wiki into dist
```

## License metadata discrepancy

- root `LICENSE` file: MIT
- root `package.json`: ISC

Preserve this discrepancy as provenance. Do not silently normalize it.

---

# 5. Browser dependencies absent from package.json

The browser runtime directly loads the following from public CDNs in `index.html`.

## Explicitly versioned

```text
pdfjs-dist 3.9.179
localforage 1.10.0
CodeMirror major 5 (mixed exact CDN paths; some files use 5, 5.0.0, or 5.62.3)
Prism 1.29.0
DOMPurify 2.3.3
```

## Unpinned / insufficiently pinned

```text
marked/marked.min.js
```

The URL does not encode a version in the pinned source.

## Loaded CodeMirror assets

Core:

```text
codemirror@5/lib/codemirror.js
codemirror@5/lib/codemirror.css
```

Add-ons:

```text
addon/scroll/simplescrollbars.js
addon/mode/overlay.js
addon/mode/loadmode.js
addon/runmode/runmode.js
addon/hint/show-hint.js
addon/hint/anyword-hint.js
placeholder addon via cdnjs CodeMirror 5.0.0
```

Modes:

```text
meta.js
htmlmixed via cdnjs 5.62.3
xml
javascript
css
python
```

Additional styles/plugins:

```text
simplescrollbars.css
show-hint.css
Prism Tomorrow theme 1.29.0
Prism autoloader 1.29.0
```

This proves that `package.json` is not a complete runtime dependency declaration and that a fresh legacy browser startup can depend on network-hosted code/assets.

---

# 6. Dynamically acquired runtime dependency

Python/code execution dynamically imports:

```text
https://cdn.jsdelivr.net/pyodide/v0.23.0/full/pyodide.mjs
```

Then Pyodide may dynamically load packages discovered from Python imports.

This dependency is feature-conditional but is part of the legacy execution capability surface.

---

# 7. Localhost server dependency graph

The legacy local services are independently packaged Node applications. `start_servers.js` can run `npm install` inside each service directory when it considers dependencies absent.

No tracked sidecar lockfiles are present in the frozen main recursive tree. Therefore semver ranges can resolve to newer transitive package versions when dynamically installed.

## 7.1 Orchestrator — `localhost_servers/package.json`

Blob:

```text
4cc4694f3e69d0eb28e5cd4ceb0b6c4649b91276
```

Dependencies:

```text
express ^4.21.2
http-proxy-middleware ^3.0.3
cors ^2.8.5
```

## 7.2 AIProxy

Manifest blob:

```text
5b9579f9d691e063eeaf4d1f2629b52886b996b6
```

Dependencies:

```text
axios ^1.6.7
express ^4.21.2
dotenv 16.4.5
cheerio 1.0.0-rc.12
```

External/local runtime targets include OpenAI, Anthropic, Groq, Ollama, arbitrary custom endpoints, and ollama.com.

## 7.3 Automation

Manifest blob:

```text
6658685d61ff26b2e75d185f21e4cd86e3a75b2f
```

Dependency:

```text
playwright 1.42.1
```

Playwright browser binaries/runtime assets are additional provisioned artifacts beyond the package manifest itself.

## 7.4 DirectAccess

Manifest blob:

```text
af1ace56ad434fe1755cf9fda8ecffc24da8ecfe
```

Dependency:

```text
express ^4.21.2
```

## 7.5 WebScrape

Manifest blob:

```text
96414aac26785b0cc1287727382d14a21efc10be
```

Dependencies:

```text
cheerio ^1.0.0-rc.12
express ^4.21.2
node-fetch ^2.6.9
pdf-parse ^1.1.1
sqlite3 ^5.1.6

override:
  glob ^11.0.1
```

It also creates/uses a mutable local SQLite `database.db`, which is runtime state rather than a frozen source dependency.

## 7.6 WikiSearch

Manifest blob:

```text
d4f4cedb7b97387903f94813204a7d0d6c850a21
```

Dependencies:

```text
axios ^1.6.7
express ^4.21.2
```

External target: English Wikipedia APIs.

## 7.7 WolframAlpha

Manifest blob:

```text
4d0443986f183f23d0c4e311d40ccf56e7d742be
```

Dependencies:

```text
axios ^1.3.6
express ^4.21.2
```

Package metadata license is ISC while several sibling sidecars use MIT.

External target: Wolfram Alpha v2 query API.

---

# 8. Service topology

`localhost_servers/servers.json` defines these mounted services:

```text
Automation
WebScrape
WikiSearch
WolframAlpha
AIProxy
DirectAccess
```

`start_servers.js` mounts services into one Express process on localhost port 7070, except Automation is flagged to start only under the `neurite` mode and itself exposes screenshot HTTP service on port 8081 when run.

This is an important distinction:

```text
source package boundary
≠
process/isolation boundary
```

Several logically different privilege domains are dynamically imported into the same Node process.

---

# 9. Electron specimen

Branch:

```text
electron
```

Pinned commit:

```text
ab38b3f8d7edb53ba78affff91f30295d0282701
```

Pinned recursive tree:

```text
d5231b9fd5cd0e67d290a08d17dbaa82434382d6
```

The recursive tree was returned complete (`truncated: false`).

Electron package manifest blob:

```text
f571f58494e29e83cf361c2fd328bca30f27e3c9
```

Dependencies:

```text
adm-zip ^0.5.16
axios ^1.9.0
tree-kill ^1.2.2
uuid ^11.1.0
```

Dev/build dependencies:

```text
electron ^36.5.0
electron-builder ^25.1.8
```

No root `package-lock.json` exists at the pinned Electron commit.

Packaging targets:

```text
macOS: dmg
Windows: nsis
Linux: AppImage
```

---

# 10. Runtime-downloaded Electron artifacts

Electron adds a second dependency plane not represented by its npm manifest.

## Frontend payload

GitHub release family:

```text
frontend-*
asset: frontend.zip
```

Downloaded/extracted under Electron userData.

## Local server payload

GitHub release family:

```text
servers-*
asset: servers.zip
```

Downloaded/extracted under Electron userData.

The updater preserves legacy:

```text
database.db
node_modules
```

between server versions.

## Desktop application update

GitHub release family:

```text
electron-*
```

Downloads and launches platform installer/artifact.

## Hosted frontend fallback

If no usable local frontend is available, the desktop code can fall back to:

```text
https://neurite.network
```

## Secure proxy page

Electron's hidden secure network BrowserWindow loads a hosted proxy page under neurite.network and uses a persistent browser partition.

---

# 11. External service/runtime dependency inventory

The frozen source refers to or can communicate with these external/local systems:

```text
GitHub Releases API
GitHub repository contents API
neurite.network
test.neurite.network
OpenAI API
Anthropic API
Groq API
Google Custom Search API
Wikipedia API
Wolfram Alpha API
Ollama local API (default 127.0.0.1:11434)
ollama.com library
arbitrary custom model endpoint
arbitrary caller-selected web URLs through WebScrape
npm registry / configured npm source during dynamic installs
jsDelivr
cdnjs
```

Some are optional capabilities, but they are all dependencies/effect targets of reachable legacy behavior.

---

# 12. Mutable/runtime artifacts intentionally excluded from source checksums

The Git checksum trees freeze tracked source, not mutable runtime state. Important non-source artifacts include:

```text
LocalStorage data
LocalForage/IndexedDB-backed data
WebScrape database.db
Electron userData frontend cache
Electron userData server cache
Electron version.json files
Electron server-install.log
sidecar node_modules
Pyodide-loaded packages
Playwright browser binaries
Ollama models/blobs
credential/environment values
browser cookies / secure-proxy persistent partition
imported workspace/media files
```

These require runtime fixture capture, not source hashing.

---

# 13. Reproducibility risks identified by dependency manifest

1. **No tracked lockfiles for the dynamic sidecars.** `npm install` can resolve semver ranges differently over time.
2. **Browser dependencies live outside package.json.** Several are fetched from CDN on demand/startup.
3. **Some browser assets are only major-version pinned or unpinned.** `marked` is loaded without a source-encoded version.
4. **Pyodide package resolution is runtime-dependent.** Imported Python packages can trigger additional acquisition.
5. **Electron frontend/server payloads are independently versioned from desktop executable.**
6. **Electron carries mutable `database.db` and `node_modules` across server payload upgrades.**
7. **Legacy updater/download paths do not explicitly validate downloaded asset hashes/signatures before extraction/execution.**
8. **Hosted fallback can change behavior independently of the pinned local source.**
9. **Provider APIs/Ollama behavior are external versioned systems outside the repository.**

These are exactly why runtime golden capture must record the complete executing environment, not merely the Git commit.

---

# 14. Runtime environment manifest required for each golden run

Every golden legacy run should add:

```text
fixture_id
OS + version
architecture
browser/Electron version
main commit/tree
Electron commit/tree if applicable
frontend release tag/digest if downloaded
server release tag/digest if downloaded
Node version
npm version
resolved sidecar dependency lock/snapshot
CDN response URL + content digest for every externally loaded asset
Pyodide version + loaded package set/digests
Playwright/browser build if used
Ollama version + model digests if used
provider/model IDs
service availability matrix
network mode
workspace fixture digest
storage-state digest before/after
```

That converts the static source manifest into an executable reproducibility manifest.

---

# Closure status

## Source-level manifest

**CLOSED** for the two frozen tracked specimens:

- main commit/tree
- Electron commit/tree
- complete recursive Git object manifests
- branch tips
- declared package manifests
- browser/CDN runtime dependencies
- dynamic Pyodide dependency
- local service packages
- runtime-downloaded Electron payload families
- external dependency/effect targets

## Runtime dependency manifest

**OPEN** until frozen Neurite is executed and the actually resolved/downloaded artifacts are captured and hashed.
