# Neurite Forensic Baseline

Status: **IN PROGRESS — Gate A forensic preservation**

This directory records evidence for the native Neurite successor. It is not a rewrite plan and it is not a source-port map. The governing unit is observable behavior and the evidence chain that implements it.

## Frozen upstream

- Repository: `satellitecomponent/Neurite`
- Branch observed: `main`
- Commit: `e62b270402b688a39a864007b9c0a02711b9573e`
- Tree: `d23fee7f1ade4886841a1e595540ad3a00a41ec6`
- Commit timestamp: `2025-06-20T01:40:40Z`
- Commit message: `Update server-release.yml`
- Repository LICENSE blob: `16fcb872b8ccc84359aa939c2004009c528728f4`
- `index.html` blob: `d1d340ae2f0661153703d0d31959352635217ec9`
- `package.json` blob: `44c73a7ae0ed8b518a8612ee460b1762219c463f`
- `js/main.js` blob: `ceb2ed3d5faa03612c11dd02da884a3ec1651528`

All source observations in this forensic pass are pinned to the commit above unless a later record explicitly names another revision.

## Acquisition note

The analysis environment could not perform a direct network `git clone`, so the frozen tree and source files were read through GitHub's immutable commit/tree/content APIs. That gives us commit- and blob-addressed source evidence, but it does **not** satisfy runtime reproduction. Gate A remains open until the frozen application is executed in controlled environments and its behavior is captured.

## Top-level repository shape

Observed top-level surfaces include:

```text
.github/
LICENSE
README.md
index.html
js/
localhost_servers/
package.json
public/
resources/
vite.config.js
```

The principal browser application lives in `js/` and dynamically loads resource templates plus an ordered global-script list. The local privileged/runtime helpers live under `localhost_servers/`.

## License/provenance anomaly to preserve

The repository root `LICENSE` is the MIT License, while `package.json` declares `"license": "ISC"`.

This record does not resolve that discrepancy. The successor should preserve both facts in provenance until the legal/source-of-truth question is resolved separately.

## Runtime dependency inventory — first pass

`package.json` declares no ordinary runtime dependencies; it declares Vite and `shx` as development dependencies. That is **not** a complete description of browser runtime dependencies.

`index.html` loads runtime libraries from public CDNs, including:

- PDF.js (`pdfjs-dist@3.9.179`)
- localForage (`1.10.0`)
- CodeMirror 5 core and add-ons/modes
- Prism.js (`1.29.0`) and autoloader
- DOMPurify (`2.3.3`)
- marked

Therefore:

```text
package manifest dependencies
!=
actual runtime dependency surface
```

Offline-parity work must capture and either bundle, replace, or deliberately supersede these dependencies.

## Browser startup chain

Observed startup sequence:

```text
index.html
  -> CDN/runtime libraries
  -> js/main.js
       -> dynamic resource HTML templates
       -> dynamic tab HTML
       -> ordered global JS scripts
       -> Graph = new Graph()
       -> App = new App()
       -> App.init()
```

`App.init()` then initializes server checks, tags, body input, fractal controls, node simulation, AI, embeddings, recorder, interface, edit/code/graph views, Zettelkasten panes/path, controls/settings, saved views and readiness signaling.

### Finding B-001 — load order is architecture

The existing application is not cleanly partitioned into ordinary ES modules. `js/main.js` sequentially injects a long list of scripts into one shared global namespace. Many scripts assume earlier globals already exist.

**Preserve:** startup behavior and subsystem availability.

**Do not preserve:** implicit global namespace, load-order dependency, and singleton construction as architectural contracts.

## First subsystem inventory

```text
Fractal / coordinate system
  js/mandelbrot/

Graph / spatial model
  js/nodes/nodeclass.js
  js/nodes/edgeclass.js
  js/nodes/nodeutilities.js
  js/nodes/nodeinteraction/

Node projections / node kinds
  js/nodes/createnodes/
  js/nodes/nodetypes/

Zettelkasten / note graph
  js/zettelkasten/

Interface / action surfaces
  js/interface/

AI / model providers / agent behavior
  js/ai/

Workspace persistence
  js/interface/dropdown/savenet.js
  localForage/localStorage state scattered elsewhere

Privileged local services
  localhost_servers/

Static resources / templates / styles
  resources/
```

## Local sidecar/runtime inventory

`localhost_servers/servers.json` declares six services:

1. `Automation`
2. `WebScrape`
3. `WikiSearch`
4. `WolframAlpha`
5. `AIProxy`
6. `DirectAccess`

The main sidecar server mounts these under a single Express service on port `7070` by default. `Automation` is conditional on the `neurite` launch flag.

### Finding B-002 — privileged effects are sidecars, not core semantics

The sidecars include filesystem reading/navigation, browser automation, scraping, external model/API proxying and external knowledge services.

**Preserve:** the useful capability behaviors.

**Do not preserve:** ambient authority or the assumption that these capabilities belong inside the spatial application's trust boundary.

## Gate A status

| Requirement | Status | Evidence / next action |
|---|---|---|
| Repository frozen | DONE | commit + tree pinned above |
| License captured | DONE, anomaly open | root MIT vs package ISC |
| Top-level structure captured | DONE first pass | immutable tree/content API |
| Runtime dependency surface | PARTIAL | CDN dependencies identified; deeper transitive inventory pending |
| Assets/configuration | PARTIAL | tree frozen; asset inventory/checksums not yet enumerated |
| External APIs | PARTIAL | sidecar families identified; endpoints/providers still being mapped |
| Runbooks | PARTIAL | Vite and localhost-server startup paths observed |
| Full checksums | PARTIAL | Git blob SHAs available; complete manifest pending |
| Fresh-profile reproduction | NOT DONE | requires runnable frozen checkout |
| Existing-workspace reproduction | NOT DONE | requires representative saved workspace fixtures |
| Online/offline matrix | NOT DONE | runtime test phase |
| Missing-service degradation | NOT DONE | runtime test phase |
| Full behavior inventory | IN PROGRESS | see behavior ledger |
| Golden behavior corpus | NOT DONE | requires runtime capture |

**Gate A is not closed. No parity claim should be made from source inspection alone.**
