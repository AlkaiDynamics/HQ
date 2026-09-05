# Runtime Evidence — Spatial Persistence Cycle (Partial)

Status: **RUNTIME-OBSERVED / INCOMPLETE VERIFICATION**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Active durable save under test: `p001`.
- Spatial text node: `PERSISTENCE-TEST-001`.
- Prior observations established note content persistence and successful Clear → Load for the simpler baseline state.
- The node had subsequently been moved, resized, and an anchor-toggle gesture had been attempted.

## Requested test sequence

```text
Save p001
→ Clear
→ Load p001
→ compare content, position, dimensions, and anchor-related presentation/state
```

## Screenshot sequence observed

The screenshots show:

1. `Save` on `p001` being activated while the enlarged `PERSISTENCE-TEST-001` node is visible.
2. `Clear` being invoked.
3. The `Are you sure?` confirmation state.
4. The subsequent `Create a new save?` prompt while the prior node is still visible behind the modal.
5. A new save named `save2` appears in the save list.
6. The active canvas becomes empty.
7. The most recent screenshot shows the **Load** control on the `save2` row highlighted/targeted while the canvas remains empty.

## What this proves

The test reconfirms the Clear workflow behavior:

```text
Clear
→ confirmation
→ optional new-save creation
→ new save (`save2`)
→ empty active workspace
```

The pre-clear state was successfully saved to `p001` before the clear sequence.

## What is NOT yet proven

The screenshots do **not** yet show `p001` loaded after the clear/new-save transition. Therefore this capture does not yet prove restoration of:

- moved position;
- resized dimensions;
- anchor/anchorForce state;
- the complete modified spatial projection.

The final reload target visible in the screenshot is `save2`, not `p001`.

## Minimal next action

From the current empty workspace:

```text
click Load on p001
→ do not touch anything else
→ capture one screenshot
```

That one capture is sufficient to complete the spatial-persistence comparison for position/dimensions/content. Anchor persistence can then be classified based on the restored presentation plus source serialization evidence.
