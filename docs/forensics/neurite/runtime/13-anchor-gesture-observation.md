# Runtime Evidence — Anchor Gesture Observation

Status: **RUNTIME-OBSERVED + SOURCE-SUPPORTED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Active save: `p001`.
- Existing text node: `PERSISTENCE-TEST-001`.
- Prior runtime work already established move/resize/collapse/expand behavior.

## Observed action

The user performed the requested deterministic anchor gesture on the expanded text node:

```text
Alt + double-click inside the large gray note body
```

A screenshot was captured immediately afterward.

## Runtime observation

The node remained expanded and visible with the same content. The screenshot does not expose an unmistakable textual badge or other high-confidence visual marker for `anchorForce=1`, so the screenshot alone is not sufficient to claim the internal anchor state changed.

## Source evidence

The frozen upstream source at commit `e62b270402b688a39a864007b9c0a02711b9573e` defines the node-window double-click handler so that:

- text/contenteditable interaction normally suppresses anchoring unless the configured Alt/Option modifier is held;
- when admitted, the handler sets `node.anchor = node.pos`;
- toggles `node.anchorForce = 1 - node.anchorForce`;
- calls `node.toggleWindowAnchored(node.anchorForce === 1)`.

The source also defines a `.window-anchored` visual class. Therefore the performed gesture is the implementation-supported anchor toggle, but the current screenshot is treated conservatively as **gesture observed / anchor state source-supported**, not as a fully independent visual proof of internal state.

## Next verification

Use one persistence cycle:

```text
Save p001
→ Clear active workspace
→ Load p001
→ compare position, dimensions, content, and anchored presentation/state
```

The save path serializes node fields through `Node.toJSON()`, which includes `anchor` and `anchorForce` because they are not excluded from serialization. Runtime reload behavior will therefore provide the next empirical check that the anchor-related state survives persistence together with move/resize state.
