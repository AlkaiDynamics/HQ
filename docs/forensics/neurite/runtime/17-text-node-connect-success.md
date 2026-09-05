# Runtime Evidence 17 — Text-node connection succeeds and mutates Zettelkasten

## Status
Observed at runtime on the frozen Neurite baseline.

## Setup
Two spatial text nodes existed in `Archive 2`:
- title `26-09-04 ~ 18:29:15.586`, body `balls1`
- title `26-09-04 ~ 18:29:17.426`, body `balls2`

The earlier Shift-click discovery attempt had not produced a confirmed edge.

## Observed successful result
The two spatial nodes are now visibly connected by a rendered edge.

At the same time, both the spatial note bodies and the Notes/Zettelkasten editor show reciprocal references:

- under `26-09-04 ~ 18:29:15.586` / `balls1`:
  `[[26-09-04 ~ 18:29:17.426]]`
- under `26-09-04 ~ 18:29:17.426` / `balls2`:
  `[[26-09-04 ~ 18:29:15.586]]`

This is direct runtime evidence that connecting two text nodes is not only a visual graph operation. It also mutates the textual/Zettelkasten representation bidirectionally.

## Behavior trace established
`connect text node A to text node B`
→ rendered spatial edge appears
→ A receives textual reference to B
→ B receives textual reference to A
→ Notes/Zettelkasten editor reflects both mutations
→ spatial note bodies reflect the same reference mutations

## Source correlation
Frozen source provides two supported connection mechanisms:

1. Node-mode gesture: hold Shift + click to connect nodes.
2. Node action `connect`, which opens `Modal.Connect`; selecting another node invokes `connectNodes(node, originNode)`.

For edge interaction, the custom context menu exposes:
- `toggle direction`
- `delete`

This gives a deterministic next runtime probe for directionality and disconnection.

## Architectural implication
A Neurite text-node edge currently conflates at least two things:
- visual/spatial relationship
- semantic textual/Zettelkasten relationship

The Rust successor should preserve the useful synchronization behavior while separating the contracts, e.g. `VisualEdge` from `SemanticReference`, so a future visual connection cannot silently acquire semantic or authority meaning unless explicitly intended.

## UX implication
Connection success is much clearer once the edge and reciprocal references exist, but discovery is poor. The successor should expose visible connection handles/context actions/whiteboard edge tools and natural-language commands, with hidden modifier gestures treated only as expert shortcuts.

## Evidence class
- rendered edge: observed
- reciprocal references in both notes: observed
- reciprocal references in Notes panel: observed
- source mechanism correlation: source-derived
- semantic interpretation as separate contract concern: inferred architectural requirement
