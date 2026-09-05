# Runtime Evidence — Spatial Note Creation

## Specimen

- Product: Neurite Desktop
- Official release asset: `electron-2025.06.18.233853`
- Windows installer SHA-256: `faa2506f579ec2bef1d790f39670c115ad1acb61588e207d7e9d0290a9b8d35c`
- Source specimen: `satellitecomponent/Neurite@e62b270402b688a39a864007b9c0a02711b9573e`
- Evidence class: `RUNTIME-OBSERVED`

## Interaction

User held `Shift` and double-clicked an empty location on the fractal canvas, using the legacy spatial-note creation gesture.

## Observed visual effect

A new spatial note window appeared at the interaction location. The window contained:

- a generated title: `26-09-04 ~ 17:04:22.388`
- an empty body area
- legacy note-window controls/chrome

A second screenshot showed the same note at a different camera scale. Camera-scale change is not attributed to note creation because it was not isolated as part of this interaction.

## Observed state mutation

At the same time, the active Notes/Zettelkasten archive (`Archive 3`) was mutated. Beneath the pre-existing text `GOLDEN-TEST-NODE-001`, Neurite inserted a new markdown-style heading:

```text
## 26-09-04 ~ 17:04:22.388
```

This is direct runtime evidence that spatial-note creation is coupled to a textual Zettelkasten/archive mutation.

## What this proves

`Shift + double-click canvas`
→ creates a spatial text/note projection
→ assigns an automatic timestamp-derived title
→ creates corresponding textual representation in the active Notes archive.

This is stronger than source inference and confirms one direction of the legacy spatial/text synchronization contract:

```text
Spatial note creation
→ textual archive representation
```

## What remains open

This capture does **not** yet prove:

- whether body edits in the spatial node propagate into the archive;
- whether archive edits propagate into the spatial node body/title;
- exact debounce/timing semantics;
- persistence across application restart;
- duplicate-title behavior;
- failure behavior;
- behavior when no Notes pane is active;
- behavior with multiple archives/panes;
- whether the generated timestamp title is configurable.

## Migration implication

Preserve the semantic contract, not the hidden gesture:

```text
Create Note intent
→ create note entity
→ create spatial projection
→ create/update textual projection
→ keep both representations synchronized
```

The native successor should expose creation through discoverable actions while retaining optional expert shortcuts.
