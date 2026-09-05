# Runtime Evidence — Manual Save + Restart Persistence

Status: **RUNTIME-OBSERVED**

## Setup

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`
- Test note created with spatial note gesture.
- Spatial note body: `PERSISTENCE-TEST-001`
- Matching Notes/Zettelkasten archive representation visible.
- Saves tab entry named `p001`.
- User explicitly clicked `Save` for `p001`.

## Restart observation

The user then closed Neurite completely and reopened it.

The first post-restart screenshot showed:

- the spatial note window present again;
- title `26-09-04 ~ 17:11:15.788` visible on the restored spatial note;
- body `PERSISTENCE-TEST-001` visible;
- Saves tab still containing `p001`;
- no evidence in the reported sequence of an explicit `Load` action between restart and the first screenshot.

Additional screenshots showed the Notes panel containing:

```text
## 26-09-04 ~ 17:11:15.788
PERSISTENCE-TEST-001
```

## Empirical conclusion

Explicit `Save` changes the behavior materially relative to the prior unsaved restart test.

Observed sequence:

```text
CREATE SPATIAL NOTE
→ edit body
→ note ↔ archive synchronized in-session
→ Save as p001
→ close application
→ reopen application
→ spatial projection restored
→ note content restored
→ archive representation restored
→ save entry p001 retained
```

This is evidence that the legacy desktop build has a durable manual-save path capable of restoring both spatial workspace state and its corresponding note/archive content across process restart.

## Contrast with unsaved restart

Previously observed without manual save:

```text
create/edit note
→ close application
→ reopen application
→ spatial note absent
→ archive reset
```

Therefore, for this tested workflow:

- in-session note/archive synchronization is not sufficient for durability;
- explicit workspace save is sufficient to persist the tested note across restart;
- the saved workspace appears to be restored automatically on startup in this test sequence, but explicit startup-selection semantics still require a dedicated test if multiple saves exist.

## Still unverified

- Whether restart always auto-loads the most recent save or only a selected/active save.
- Behavior with multiple saves.
- Explicit `Load` behavior from a fresh/cleared workspace.
- Whether node position, scale, anchor state, edges, and directionality restore exactly.
- Crash/power-loss behavior between edits and save.
- Save failure behavior under quota/storage/corruption conditions.
- Export/import parity with internal save/load.
