# Runtime Evidence — Edge direction cycle returns to undirected

Status: observed

Specimen: legacy Neurite runtime, user-driven manual test.

## Preconditions
- Two text nodes existed: `balls1` and `balls2`.
- The nodes were already connected.
- Prior observations established the edge direction cycle:
  - undirected with reciprocal Zettelkasten references
  - directional state A→B with one-sided reference
  - directional state B→A with the opposite one-sided reference

## Interaction
The user invoked `toggle direction` on the edge a third time.

## Observed visual effect
- The directional arrow disappeared.
- The edge returned to the curved undirected rendering.

## Observed semantic state
The Notes/Zettelkasten projection again showed reciprocal references:
- `balls1` contained a reference to `balls2`.
- `balls2` contained a reference to `balls1`.

## Behavioral conclusion
The legacy cycle is empirically closed:

`undirected ↔ A→B ↔ B→A ↔ undirected`

For text nodes, each direction transition is not merely a visual edge-style change; the semantic note-reference state is rewritten to match the selected directionality.

## Successor implication
Treat these as separate domains even when a user action synchronizes them:

- visual relationship/projection
- semantic knowledge relationship
- directionality metadata

The successor should not assume changing a visual edge must mutate semantic knowledge unless the command explicitly requests or policy authorizes that coupling.
