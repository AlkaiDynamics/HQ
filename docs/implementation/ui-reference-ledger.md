# HQ UI implementation — source/image continuity ledger

**Status:** initial 2D interactive browser preview/fixture, **not** the HQ native client, GPU world, persisted Rust workspace, 3D renderer, or live inference UI. This ledger is intended to travel with implementation changes and be updated for each UI engineering unit. Source-derived constraints are distinguished from proposed implementation choices.

## Authority order

1. Explicit user corrections and original supplied images (visual evidence).
2. `HQ-Mockup-Evidence-and-Design-Corrections.md` and `HQ-Native-Neurite-Specification.md` supplied to ChatGPT Files; `HQ-Phase-4-Architecture.md` and `HQ-Phase-4-Reference-Study.md` in the same Library.
3. Existing HQ Rust crates and Neurite behavior-forensic notes in this repository.
4. Public source/official product documentation for donor implementation patterns. The code examples below are references, **not copied dependencies or verified whole-product parity**.

## Visual sources used

| Supplied image | Direct visual evidence retained | UI feature / path | Not inferred |
|---|---|---|---|
| `image-gen-1(8).png` | Dark 2D infinite-flow workspace; element palette, editable workflow cards, prompt tools, inspector and generator | `ui/index.html` three-zone workbench, `ui/styles.css` quiet dark shell, `ui/app.mjs` graph canvas and ports | Sample products, execution success dots and exact color values are not authority |
| `image-gen-2(1).png` | Grounded Space theme selector and more spacious arrangement | Theme selector and reduced-neon default; alternate palettes in `styles.css` | Decorative planets/starfields are optional, not mandated |
| `image-gen-3(1).png` | Periodic symbol/family palette, many specific node families, workflow editing | Optional `◇ Periodic` layout of the **open-ended** node definition catalog | A fixed periodic-table capacity is not a node limit |
| `image-gen-4(1).png` | Top-level workflow remains visible while reasoning agent's internal graph is expanded below; `Back to parent` and boundary inputs/outputs | `openChildBeside`, `enterFrame`, `returnToParent`; same root frame/camera, child overlay, named boundary ports | Still image does not prove animation or actual effect execution |
| `image-gen-2.png`, `image-gen-3.png`, `image-gen-4.png` | Goal → choices → execution, mixed content types and planning/status presentation | QuestN selection tools and note/schedule/agent/terminal/browser templates | Sample numbers/automations are placeholders, not working integration |
| `image(20260927-222139).png`, `image(20260927-222139-1).png`, `image(20260927-222143).png`, `image(20260927-222245).png` | Neurite heterogeneous node windows, text references, spatial zoom and different object scales | Note/file nodes alongside agent and time objects; zoom/pan, explicit semantic reference edges | Genuine Neurite behavior parity not yet verified |
| `image(20260928-003038).png`, `image(20260928-003041).png`, `image(9).png`, `image(10).png` | Ordered-to-free periodic organization, multiple visual treatments, extended prompt workbench, multiagent spatial chat | Periodic palette, 19 staged actions, selection-aware context and custom prompts | No assumption that these stills implement runnable AI tools |

**User visual corrections:** default is dark, grounded/sleek, less neon, **no hot pink**, muted blue, softer edges, not gratuitously futuristic. Theme changes are presentation-only and do not grant actions. Retain discoverable one-click capabilities without pretending that a button grants permission or immediately executes code.

## Code/interaction donors and actual inspected paths

| Donor | Observed code/doc | Adopted pattern | Boundary |
|---|---|---|---|
| **Neurite** | [satellitecomponent/Neurite](https://github.com/satellitecomponent/Neurite); user-supplied ingest and HQ forensic docs for `zettelkasten.js`, `edgeclass.js`, `connect.js` | Preserve heterogeneous canvas objects, text↔reference links and navigation. Distinguish references from executable channels in `ui/model.mjs` | No wholesale migration/copy; source and runtime behavior need parity tests |
| **Flowise** | [Agentflow V2](https://docs.flowiseai.com/using-flowise/agentflowv2); [Flowise `buildAgentflow.ts`](https://github.com/FlowiseAI/Flowise/blob/main/packages/server/src/utils/buildAgentflow.ts), inspected | Readable connected agent stages, visible waits/approvals; typed model execution is separate from canvas | UI does not implement Flowise's execution engine |
| **n8n** | [n8n `workflow-execute.ts`](https://github.com/n8n-io/n8n/blob/master/packages/core/src/execution-engine/workflow-execute.ts), inspected | Clear authoring ports/edges and separation of workflow plan from active run | Connections do not imply execution; the HQ Rust execution and admission path is authoritative |
| **XYFlow** | [XYFlow edge geometry utilities](https://github.com/xyflow/xyflow/blob/main/packages/system/src/utils/edges/general.ts), inspected | Simple cubic connection curves, editor-style pan/zoom; implementation is small SVG view code in `ui/app.mjs` | XYFlow is **not** HQ's primary canvas or graph database |
| **ComfyUI** | [Subgraphs](https://docs.comfy.org/interface/features/subgraph), referenced in Phase 4 study | Boundary ports and editable internals for composed nodes | No lazy evaluator or run-scoped expansion claimed |

### Acceptance/verification boundary

- `ui/model.mjs` is a browser **presentation adapter fixture**. It operates on mock frames and explicitly exports `authoritative:false`. It does not write HQ's Rust scene/notes/receipts. `--agent-run` remains a separate CLI path until an actual controller/IPC bridge is built and tested.
- `ui/app.mjs` provides interaction and source preview but never calls provider or Jev URLs. `Run` explains the missing integration; no fabricated success feedback.
- Node edges preserve `data`, `control`, and `reference` channels. Selection never executes or changes grants; zoom and hidden reference edges never stop processes.
- Accessibility and mobile behavior are included as design accommodations, not certified through assistive-technology/platform testing.

### Next unit / real integration

Port this visual/control surface to the agreed **Tauri 2 + Svelte + Three.js** UI layers as the team implements the renderer and workspace controller. Preserve stable IDs and typed commands; let Rust remain owner of transaction, context selection and receipts. Perform native 2D/3D runtime screenshot and interaction comparison against the supplied images; do not rebrand the browser demo as the native application.
