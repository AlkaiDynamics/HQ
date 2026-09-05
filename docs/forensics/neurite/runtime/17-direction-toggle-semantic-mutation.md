# Runtime Evidence 17 — Edge direction changes semantic note references

## Status

**Observed + source-supported.**

## User interaction

Starting from two connected text nodes (`balls1`, `balls2`) with:

- a visible undirected spatial edge, and
- reciprocal Zettelkasten references in both note bodies,

user opened the edge context action and selected **toggle direction**.

## Observed visual effect

The formerly curved/undirected edge became a straight directed edge with a large arrow marker. In the supplied runtime screenshot the arrow points from the `balls1` side toward the `balls2` side.

## Observed semantic/state mutation

Before direction toggle, both notes contained reciprocal references.

After direction toggle:

- `balls1` still contains a reference to the `balls2` timestamp/title.
- `balls2` no longer contains the reciprocal reference to `balls1`.

Therefore edge direction is not merely projection styling in legacy Neurite. Toggling direction mutates the textual/Zettelkasten relationship representation as well as the visual edge state.

## Source confirmation

Frozen upstream: `satellitecomponent/Neurite@e62b270402b688a39a864007b9c0a02711b9573e`.

`js/nodes/edgeclass.js` implements `Edge.toggleDirection()` as a three-state cycle:

1. default/undirected -> `pts[0] -> pts[1]`
2. `pts[0] -> pts[1]` -> `pts[1] -> pts[0]`
3. `pts[1] -> pts[0]` -> undirected

For two text nodes, the same method explicitly adds/removes Zettelkasten references to match the new direction, then stores directionality in `Graph.edgeDirectionalities[edgeKey]`.

`EdgeView.draw()` also switches rendering behavior:

- undirected: curved path, no arrow;
- directed: straight path + visible arrow/border.

## Behavioral trace

```text
edge context action: toggle direction
  -> Edge.toggleDirection()
  -> edge.directionality mutated
  -> reciprocal textual refs changed to one-way ref
  -> Graph.edgeDirectionalities[edgeKey] updated
  -> EdgeView.draw()
  -> straight directed edge + arrow rendered
```

## Contract implication

Legacy Neurite conflates at least two relationship classes:

```text
VisualEdge
SemanticReference
```

A direction edit on the visual edge can mutate the semantic note graph. The successor must make this coupling explicit rather than accidental.

Recommended native contract split:

```text
Relationship {
  semantic_kind
  directionality
  projection_style
  sync_policy
}

VisualEdge = projection of Relationship
SemanticReference = semantic representation of Relationship
```

A UI action that changes direction should state whether it changes:

- visual direction only,
- semantic direction only, or
- both under an explicit synchronization rule.

This is particularly important for ARGO/HQ because a visual connection must never silently imply authority, runtime dependency, or other privileged relationship classes.

## Golden assertion candidate

Given two reciprocal text-note references and an undirected edge:

```text
toggle direction once
=> directed arrow appears
=> exactly one textual reference remains
=> directionality state is persisted in graph state
```

Follow-up goldens should cycle direction a second and third time to verify reversal and return-to-undirected behavior.
