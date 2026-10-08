# HQ Canvas — first source-grounded interaction slice

This **browser-hosted UI development fixture** demonstrates the native HQ canvas design direction while the real Tauri/Svelte/Three.js client and Rust controller transport remain unimplemented. It is not a parallel workspace authority or an inference runtime. The actual HQ kernel remains in the Rust workspace.

## Run locally

From the repository root on Windows with Python installed:

```powershell
python -m http.server 8765
# Then visit http://127.0.0.1:8765/ui/
```

Or start any static HTTP server at the repo root. No dependencies or CDN access are required. Running by `file://` may prevent ES-module loading; use HTTP.

## Verify

```sh
node --test ui/tests/*.test.mjs
```

The main GitHub PR workflow runs these tests alongside existing Rust workspace checks. The UI logic and graph fixture are intentionally pure JavaScript and dependency-free until the native client decisions are wired into the Rust authority.

## What is interactive

- Node creation, editing, selection, dragging, panning, pointer-centered zoom, minimap, and draft graph export.
- Typed-channel selection for explicit semantic references versus data/control connections (no implicit execution).
- Context preview lists *only* selected note/file nodes; no provider call occurs.
- Nested child-world inspection beside the parent and full-screen enter/back with camera bookmarks.
- Searchable element palette, a periodic organizational mode, prompt-action staging, QuestN actions, alternate purely visual themes, and accessible keyboard equivalents.

## Deliberately not represented as working

- No Svelte/Three.js/desktop shell yet; 3D remains disabled rather than simulated by styling.
- UI edits do **not** mutate the Rust `Workspace`, SQLite, provider gateway, or receipts; there is no autosave.
- **Run** is a disclosure that native execution is not yet connected; it does not run an agent or forge a receipt.
- Export marks `authoritative: false`. It is a review artifact, not a persisted workspace record.
- Imported repo, image-to-node generation, browser/CLI/Android and live data all require separate host bridges and approval boundaries.

See `docs/implementation/ui-reference-ledger.md` for explicit, image-by-image and source-by-source implementation traceability.
