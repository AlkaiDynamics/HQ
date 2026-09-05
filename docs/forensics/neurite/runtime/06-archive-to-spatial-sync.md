# Runtime Evidence 06 — Archive → Spatial Note Sync

## Specimen

- Application: Neurite Desktop
- Release: 2025.06.18.233853
- Windows installer SHA-256: `faa2506f579ec2bef1d790f39670c115ad1acb61588e207d7e9d0290a9b8d35c`
- Evidence class: `RUNTIME-OBSERVED`

## Preconditions

A spatial note created via Shift + double-click existed with title:

`26-09-04 ~ 17:04:22.388`

Its body had previously been set to:

`SPATIAL-BODY-001`

and that value was observed to appear in the active Notes/Zettelkasten archive.

## Interaction

The archive text body for the corresponding heading was edited from:

`SPATIAL-BODY-001`

to:

`ARCHIVE-EDIT-001`

## Observed visual result

The spatial note body updated to display:

`ARCHIVE-EDIT-001`

while the archive pane simultaneously displayed the same value under the same timestamp heading.

## Runtime conclusion

The legacy implementation supports bidirectional content synchronization for this note representation path:

```text
Spatial note body
    ↕
Zettelkasten/archive body
```

This observation verifies the reverse direction of the sync pair already established by the prior spatial-to-archive test.

## What this does NOT yet prove

- persistence across application restart;
- conflict semantics under simultaneous edits;
- behavior if the heading/title is changed from either surface;
- behavior for duplicated headings;
- failure handling if one representation is malformed;
- sync semantics across multiple Archives;
- whether every note-creation path enters the same synchronization contract.

## Contract implication

The successor should preserve the behavioral contract, not the current coupling:

```text
SemanticNoteState
    ↕ reconciliation/synchronization
TextualProjection
SpatialProjection
```

Neither projection should independently become the hidden authority merely because the user edited it last. The contract needs explicit change identity, reconciliation rules, and persistence semantics.

## Status

- Archive → spatial body synchronization: **RUNTIME-VERIFIED**
- Full bidirectional live content sync for the tested note: **RUNTIME-VERIFIED**
- Persistence/restart: **OPEN**
- Conflict/failure behavior: **OPEN**
