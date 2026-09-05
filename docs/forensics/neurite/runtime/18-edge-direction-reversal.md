# Runtime evidence — reverse directed text-node relationship

Status: observed at runtime, screenshot-backed.

## Setup
Two spatial text nodes are connected:
- `balls1` with title `26-09-04 ~ 18:29:15.586`
- `balls2` with title `26-09-04 ~ 18:29:17.426`

The edge had already been toggled once into a directed state where only `balls1` retained a Zettelkasten reference to `balls2`.

## Observed second toggle result
After toggling edge direction again:
- the edge remained directional with a visible arrow;
- `balls1` no longer showed a reciprocal reference;
- `balls2` showed `[[26-09-04 ~ 18:29:15.586]]`;
- therefore the semantic note reference flipped to the opposite endpoint together with the visual direction.

This confirms the direction cycle mutates both:
1. the rendered edge direction, and
2. the underlying Zettelkasten reference topology.

## Source correlation
`Edge.toggleDirection()` cycles through three states:
- default/undirected
- endpoint 0 -> endpoint 1
- endpoint 1 -> endpoint 0
- then back to undirected

For text nodes, each directional transition explicitly removes one Zettelkasten reference and adds the opposite reference. The edge direction is persisted separately in `Graph.edgeDirectionalities`.

## Architectural consequence for HQ
Do not model this legacy coupling as one undifferentiated relation. Preserve the behavior while separating the concepts:

`VisualEdge != SemanticRelationship`

A successor may synchronize them by policy, but it must be possible for either to exist or change independently, with explicit intent and observable state.
