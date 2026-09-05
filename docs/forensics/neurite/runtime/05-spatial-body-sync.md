# Runtime Observation 05 — Spatial Body → Notes Archive Sync

## Specimen
- Application: Neurite Desktop
- Release: electron-2025.06.18.233853
- Installer SHA-256: `faa2506f579ec2bef1d790f39670c115ad1acb61588e207d7e9d0290a9b8d35c`
- Observation status: RUNTIME-OBSERVED

## Setup
A spatial note had already been created by `Shift + double-click`, producing the auto-generated title:

`26-09-04 ~ 17:04:22.388`

The Notes/Zettelkasten archive simultaneously contained:

`## 26-09-04 ~ 17:04:22.388`

## Interaction
User clicked inside the spatial note body and entered:

`SPATIAL-BODY-001`

## Observed visual effect
The spatial note body immediately displayed `SPATIAL-BODY-001`.

## Observed state/projection effect
The Notes/Zettelkasten panel immediately displayed `SPATIAL-BODY-001` directly beneath the matching timestamp heading.

This is runtime evidence that editing a spatial note body propagates into the text/Zettelkasten representation.

## What this proves
- Spatial note body mutation is reflected in the Notes/Zettelkasten archive.
- The spatial note and archive text are linked representations of the same note content, not independent copies.
- At least one direction of synchronization is runtime-verified: `spatial projection → textual/Zettelkasten projection`.

## Still unverified
- Reverse-direction sync: editing the Notes/Zettelkasten text and observing the spatial note update.
- Persistence across application restart.
- Conflict behavior if both projections are edited rapidly or concurrently.
- Failure behavior when synchronization/parsing fails.

## Evidence class
`RUNTIME-OBSERVED`
