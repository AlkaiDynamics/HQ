# Runtime Observation 07 — Restart Persistence (Partial)

## Specimen

Official Neurite Desktop Windows build corresponding to release `electron-2025.06.18.233853`.

## Setup before restart

A spatial note was created with `Shift + double-click`, producing timestamp title:

`26-09-04 ~ 17:04:22.388`

Its body was edited through the spatial projection and then through the Notes/Zettelkasten archive. Immediately before restart, both surfaces displayed:

`ARCHIVE-EDIT-001`

No manual save action was performed before exit.

## Restart observation

After closing Neurite completely and reopening it, the application returned to the fractal canvas without the previously created spatial note visibly restored in the viewport.

Observed:

- application restarts successfully;
- fractal canvas renders normally;
- no fatal recovery dialog appears;
- the previously created spatial note is not visibly restored in the initial viewport;
- the Notes panel is not open by default after restart.

## What this does NOT yet prove

This observation does **not** yet establish that the note data was lost. The note/archive content may still persist in storage while its spatial projection or workspace graph is not automatically restored.

Therefore persistence status remains **PARTIAL / UNRESOLVED** until the Notes archive and save/load surfaces are inspected after restart.

## Next empirical check

Open the Notes panel and determine whether the timestamp heading and `ARCHIVE-EDIT-001` content remain present after restart.

## Evidence status

- Restart succeeds: `RUNTIME-OBSERVED`
- Spatial note visible on immediate restart: `CONTRADICTED` for automatic restoration in the initial viewport
- Archive text persistence: `OPEN`
- Workspace graph persistence: `OPEN`
- Manual-save requirement: `OPEN`
