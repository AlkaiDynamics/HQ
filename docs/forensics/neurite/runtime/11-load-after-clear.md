# Runtime Evidence — Load Existing Save After Clear

Status: **RUNTIME-OBSERVED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Existing save `p001` previously persisted the spatial note `PERSISTENCE-TEST-001`.
- User had invoked Clear, created a new empty save `save1`, and reached an empty active canvas while both saves remained listed.

## Observed action

The user clicked **Load** for `p001`.

## Observed result

The previously saved spatial note returned to the active canvas with:

- title `26-09-04 ~ 17:11:15.788`;
- body `PERSISTENCE-TEST-001`;
- `p001` and `save1` both still visible in the Saves panel.

## Behavioral conclusion

The internal save/load system supports switching from a cleared/new workspace state back to a prior durable workspace:

```text
p001 saved
→ Clear active graph
→ create/select empty save1
→ empty canvas
→ Load p001
→ prior spatial projection restored
→ prior note content restored
```

This closes the basic manual **Clear → Load existing workspace** loop for the tested note-only workspace.

## Still unverified

- Exact restoration of node position, scale, anchor state, edges, directionality, and mixed node types.
- Whether switching saves autosaves pending edits depending on Autosave state.
- Load behavior when target save data is malformed/corrupted.
- Export/import parity with the internal save/load path.
