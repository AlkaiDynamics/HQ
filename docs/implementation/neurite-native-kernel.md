# Neurite Native Kernel — Implementation Start

Manual legacy interaction testing is now **deferred unless it becomes a blocker**.

The forensic corpus already contains source decomposition, behavior traces, state ownership, external effects, Electron/runtime mapping, node capability matrix, dependency manifest, contract registry, successor interaction requirements, and a machine-readable forensic knowledge graph. Runtime observations already captured remain evidence; untested cases remain explicitly unverified rather than blocking all forward motion.

## Rule going forward

Use this priority order:

1. source-derived evidence and the forensic KG;
2. existing runtime evidence;
3. automated/differential fixtures when available;
4. human legacy testing only when ambiguity materially blocks implementation or before a parity/promotion claim.

## First implementation slice

The first Rust slice establishes the non-UI domain kernel:

- `neurite-core`: typed identifiers and spatial primitives;
- `neurite-scene`: logical entities and explicitly separated relationship kinds;
- `neurite-events`: portable user/system intents and resulting events;
- `neurite-workspace`: local workspace ownership and dirty-state tracking;
- `neurite-app`: minimal executable proving the crates compose.

The design intentionally separates `Visual`, `Semantic`, `Layout`, `Runtime`, and `Authority` relationships. This prevents the legacy coupling where drawing or deleting a visual edge can silently imply or destroy a semantic relationship.

## Next implementation slice

1. command admission/execution layer: `Intent -> Command -> Event`;
2. persistent versioned workspace store;
3. deterministic scene tests and legacy fixture importer;
4. native window/input/render substrate (`winit`/`wgpu`);
5. fractal coordinate substrate;
6. projection layer for text notes;
7. differential verification against the captured legacy fixtures.
