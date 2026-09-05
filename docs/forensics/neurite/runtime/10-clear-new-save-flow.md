# Runtime Evidence — Clear / New Save Flow

Status: **RUNTIME-OBSERVED + SOURCE-CORRELATED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Existing saved workspace `p001` contained the spatial note `PERSISTENCE-TEST-001`.
- User supplied screenshots in reverse chronological order; the sequence below is reconstructed oldest → newest.

## Observed sequence

1. Saved workspace is open and `PERSISTENCE-TEST-001` is visible.
2. User invokes **Clear** from the Saves panel.
3. The Saves UI changes the control to an inline confirmation state: **Are you sure? / Yes / No**.
4. On confirmation, Neurite presents a second confirmation: **Create a new save?**.
5. Choosing to create a save opens a title prompt: **Enter a title for this save:**.
6. User enters `save1`.
7. The active canvas becomes empty.
8. The Saves panel now contains both `p001` and `save1`.

## Source correlation

Frozen source `js/interface/dropdown/savenet.js` confirms:

- `Clear` first exposes an inline `Are you sure?` state.
- Confirming Clear invokes `window.confirm("Create a new save?")`.
- The handler clears the graph after deselecting the currently selected graph.
- The save list is then refreshed.

Source specimen: `satellitecomponent/Neurite@e62b270402b688a39a864007b9c0a02711b9573e`.

## Behavioral conclusion

For this tested path, **Clear is not a simple destructive reset button**. It behaves more like a workspace transition flow:

```text
Clear
→ inline destructive confirmation
→ optional new-save confirmation
→ optional save naming
→ active graph cleared
→ prior save remains available
→ optional new empty save is added
```

This is behavior worth preserving semantically but not necessarily as the same interaction sequence.

## UX observation

The legacy flow requires several modal/inline steps and does not make the resulting state transition obvious before the user commits. In the successor, the same capability should be represented more directly, for example as an explicit **New Workspace / Clear Current View / Preserve Current Save** flow with visible consequences and undo/recovery where feasible.

## Still unverified

- Clicking `Load` on `p001` after this exact Clear/new-save flow.
- Whether `save1` is created as a zero-size/empty save immediately or only becomes durable after a later Save action.
- How autosave changes the Clear behavior.
- Failure behavior if new-save creation fails.
