# Runtime Evidence — Connection Gesture Discovery

Status: **RUNTIME-OBSERVED + SOURCE-SUPPORTED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Two spatial text notes were created in the active workspace:
  - body `balls1`
  - body `balls2`
- The Notes/Zettelkasten panel showed matching note entries and timestamp headings for both notes.

## Runtime observation

The user attempted to discover how to connect the two nodes through the visible UI and could not determine the interaction. During the attempt, a filesystem browser/modal was opened, demonstrating that adjacent controls can route the user into an unrelated privileged file-navigation surface without making the graph-connection gesture apparent.

This is useful behavioral evidence about discoverability and interaction ambiguity. It is not evidence that connection failed internally because the intended connection gesture was not yet executed.

## Source-supported connection interaction

The frozen upstream source at commit `e62b270402b688a39a864007b9c0a02711b9573e` shows:

- `settings.nodeModeKey` defaults to the configured Shift key.
- While that key is down, `App.nodeMode` is enabled.
- On node `mousedown`, if no previous node is pending and node mode is active, the current node is stored in `Node.prev`.
- On the next node `mousedown`, if `Node.prev` exists, `connectNodes(this, Node.prev)` is called and `Node.prev` is cleared.
- For two text nodes, `connectNodes` writes reciprocal Zettelkasten references via `addEdgeToZettelkasten(title1, title2)` and `addEdgeToZettelkasten(title2, title1)` rather than directly calling the generic edge constructor.

The UI source labels the corresponding control as `Freeze and Connect Nodes (Hold Shift)`.

Therefore the deterministic interaction to test next is:

```text
Hold Shift
→ single-click / mouse-down first node without dragging
→ release Shift
→ single-click / mouse-down second node without dragging
```

Because `Node.prev` is intentionally not cleared on Shift keyup, the second click does not need Shift to remain held.

## Success criteria

For two text notes, verify both:

1. a visual edge appears between the spatial nodes;
2. reciprocal references are added to the Notes/Zettelkasten representation.

## UX finding

The successor should not rely on this hidden stateful gesture as the only way to create relationships. The capability is valuable; the affordance is weak. Provide visible connect handles, explicit edge tools, context actions, tooltips, touch-friendly controls, and conversational commands such as `connect these two` while retaining keyboard accelerators for expert use.
