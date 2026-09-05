# Neurite Native Kernel — Implementation Status

Manual legacy interaction testing is deferred unless it becomes a blocker.

The forensic corpus contains source decomposition, behavior traces, state ownership, external effects, Electron/runtime mapping, a node capability matrix, dependency manifest, contract registry, successor interaction requirements, and a machine-readable forensic knowledge graph. Existing runtime observations remain evidence; untested cases remain explicitly unverified rather than blocking construction.

## Evidence priority

1. source-derived evidence and the forensic knowledge graph;
2. existing runtime evidence;
3. automated and differential fixtures;
4. human legacy testing only when ambiguity materially blocks implementation or before a parity claim.

## Implemented foundation

- `neurite-core`: durable namespace-scoped identifiers and spatial primitives;
- `neurite-scene`: logical entities and explicitly separated relationship kinds;
- `neurite-events`: portable user/system intents and resulting events;
- `neurite-protocol`: open extension identifiers, scenario protocol bindings, client/runtime descriptors, partitions, and bounded stream policies;
- `neurite-control`: default-deny, policy-derived capability admission from intent to command;
- `neurite-workspace`: local workspace ownership and dirty-state tracking;
- `neurite-app`: minimal executable proving the crates compose.

`Visual`, `Semantic`, `Layout`, `Runtime`, and `Authority` relationships remain separate. Drawing or deleting a visual edge therefore does not inherently grant authority or mutate semantic meaning.

IDs are supplied at the boundary and combine a durable installation/runtime namespace with a local value. Imported workspaces and remote clients can reconstruct identity without process-global counters silently replacing it.

Extension-facing identifiers are validated strings rather than closed enums. A future node, UI, runtime, protocol, transport, scenario, projection, or platform can be registered without changing the architectural kernel.

## Android/mobile foundation

Android is represented in two independent roles:

1. a projection/control client using the same intent protocol as desktop;
2. a device execution host exposing individually gated capabilities.

Mobile clients declare requested capabilities such as camera, microphone, location, notifications, sensors, media, sharing, and scoped storage. A declaration never grants the capability. Authority is admitted separately for a specific principal and partition.

The trusted action-policy registry determines capability requirements. A client cannot lower its own requirements by sending an empty list, and unregistered actions are rejected.

See `docs/architecture/mobile-android-extensibility.md`.

## Revised implementation sequence

1. durable identity and open protocol envelopes — implemented foundation;
2. Android/mobile client and device-runtime contracts — implemented foundation;
3. capability admission from `Intent` to `Command` — implemented foundation;
4. command execution and `Command -> Event -> ObservedState` reconciliation;
5. append-only authoritative event log and versioned workspace snapshots;
6. deterministic scene tests and legacy fixture importer;
7. native window/input/render substrate using `winit` and `wgpu`;
8. fractal coordinate and recursive-navigation substrate;
9. projection layer for text notes, desktop, web, CLI, agent, chat, repository, media, database, graph, and Android nodes;
10. bounded concurrent streams, suspension/reactivation, and load testing;
11. differential verification against captured legacy fixtures;
12. CI and cross-platform desktop/Android build verification.
