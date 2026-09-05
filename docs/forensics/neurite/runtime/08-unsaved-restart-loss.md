# Runtime Evidence — Unsaved Restart Persistence

## Specimen

Neurite Desktop 2025.06.18.233853, Windows installer SHA-256:
`faa2506f579ec2bef1d790f39670c115ad1acb61588e207d7e9d0290a9b8d35c`

## Preconditions

A spatial note had been created via Shift + double-click and had synchronized bidirectionally with the Notes/Zettelkasten archive.

Observed content before restart:
- spatial note title: `26-09-04 ~ 17:04:22.388`
- spatial/archive body after reverse-sync test: `ARCHIVE-EDIT-001`

No explicit manual workspace save was performed before application exit.

## Interaction

1. Close Neurite completely.
2. Reopen Neurite.
3. Open the Notes panel.
4. Inspect the available archive and note contents.

## Runtime observation

After restart:
- the previously visible spatial note was not restored to the canvas;
- the Notes panel reset to `Archive 1`;
- the archive showed the default empty-note placeholder/help text;
- neither `26-09-04 ~ 17:04:22.388` nor `ARCHIVE-EDIT-001` was present;
- the Notes search UI had no visible matching result.

## Evidence classification

**RUNTIME-OBSERVED**

Confidence: high for the tested sequence.

## Behavioral conclusion

For this tested fresh-profile sequence, a note created and edited without an explicit manual workspace save did **not** survive a full application restart. Both the spatial projection and its Notes/Zettelkasten textual representation were absent on relaunch.

This means the earlier spatial↔archive synchronization is an in-session behavior and must not be interpreted as durable persistence by itself.

## Scope / limits

This does **not** establish that Neurite has no persistence mechanism. It establishes only that the tested note/archive state was not automatically restored after restart when no explicit workspace save had been performed.

Manual Save/Load behavior must be tested separately.
