# Runtime Evidence — Modified Save / Load Anomaly

Status: **RUNTIME-OBSERVED / FAILURE-ADJACENT / PARTIALLY CONFOUNDED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Existing save: `p001`.
- Existing node content: `PERSISTENCE-TEST-001`.
- Prior runtime tests established node creation, bidirectional note/archive sync, manual save persistence, workspace clear/load, move, resize, collapse/expand, and the source-supported anchor gesture.
- The user supplied the screenshot sequence in reverse chronological order, consistent with the prior capture convention.

## Intended workflow

```text
Save p001
→ Clear
→ create/select temporary empty save
→ Load p001
→ compare restored node state
```

The target comparison was position, dimensions, content, and anchor-related presentation/state.

## Observed sequence

The screenshots show all of the following:

1. `PERSISTENCE-TEST-001` visible in a large resized spatial node before the clear/load cycle.
2. Clear confirmation and the `Create a new save?` branch.
3. A save-title prompt containing `p001`.
4. A collision dialog stating that a save titled `p001` already exists, with choices to overwrite or create a duplicate.
5. A new save row `save2` appearing alongside `p001` and `save1`.
6. A blank workspace after the clear/new-save branch.
7. A subsequent attempt to load `p001`.
8. The camera/fractal state changes when `p001` is targeted/loaded, but the spatial node is not visibly present in the final blank-canvas captures.

## What this proves

### Runtime-observed

- The Clear flow is stateful and can branch into creation of another save.
- Creating a new save with a colliding title invokes an explicit overwrite-or-duplicate confirmation.
- A duplicate/new-save branch can result in an additional save row (`save2`).
- Loading/toggling between saves can restore/change camera/fractal coordinates independently of whether the expected node projection is visible.
- In this run, the expected `PERSISTENCE-TEST-001` node was **not visibly restored** after the final `p001` load attempt.

### Source-supported

The frozen `js/interface/dropdown/savenet.js` implementation confirms:

- `Clear` explicitly prompts `Create a new save?` before clearing/creating a fresh save branch.
- `Load` first autosaves the currently selected graph, then reads the target graph data and calls the graph loader.
- Empty saves are treated specially and require an additional confirmation before load.
- Save metadata and graph data are stored separately through `GraphsKeeper`.

## Interpretation

Do **not** classify this as a clean proof that move/resize/anchor persistence fails.

The run is partially confounded by the save-title collision and duplicate/new-save branch. The strongest safe statement is:

> After a modified-node save followed by Clear/new-save/title-collision handling and then a `p001` load attempt, the expected node was not visibly restored, even though the camera/fractal state changed.

This is useful failure-behavior evidence in its own right. It shows that the save/clear/load interaction model can produce ambiguous workspace state and that the visual restoration of entities can diverge from restoration of view/camera state.

## Successor requirement derived from this observation

The successor should make workspace state transitions explicit and transactional:

```text
Current workspace
→ dirty/clean indicator
→ Save / Save As / Duplicate / Switch
→ explicit target workspace
→ atomic restore result
→ visible success/failure status
```

A user should never have to infer which save is active, whether a click overwrote or duplicated, or whether only camera state versus entity state was restored.

## Test status

- Position persistence: **not cleanly verified in this run**.
- Dimension persistence: **not cleanly verified in this run**.
- Content persistence: **previously verified in simpler save/restart and load-after-clear cases; not cleanly re-verified here**.
- Anchor persistence: **not empirically closed**.
- Save/clear/load failure/ambiguity behavior: **runtime-observed**.

This run should be retained as a golden failure/ambiguity fixture rather than discarded or repeated until it looks successful.
