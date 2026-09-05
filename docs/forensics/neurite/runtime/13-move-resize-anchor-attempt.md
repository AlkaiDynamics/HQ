# Runtime Evidence — Move / Resize / Anchor Attempt

Status: **RUNTIME-OBSERVED + ANCHOR UNVERIFIED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Loaded workspace `p001` contains the text node titled `26-09-04 ~ 17:11:15.788` with body `PERSISTENCE-TEST-001`.
- This test follows the earlier resize/collapse observations.

## Observed behavior

The supplied screenshots show the node at materially different positions and dimensions, confirming that the legacy UI supports:

- direct spatial movement of the node window;
- large changes in window dimensions through the resize affordance;
- preservation of the same logical note content/title while its projection geometry changes.

The screenshots also show a double-click attempt on the title/header region. The title text becomes selected, which makes the resulting anchor state visually ambiguous from the screenshot alone.

## Source correlation

Frozen source `js/nodes/createnodes/window.js` registers a `dblclick` listener on the node window. On an allowed double-click it:

```text
node.anchor = node.pos
node.anchorForce = 1 - node.anchorForce
node.toggleWindowAnchored(node.anchorForce === 1)
```

The same source explicitly prevents anchoring when the target is a text interaction surface unless the configured Alt/Option modifier is held. The README/user guide documents `Double Click` as `Anchor a node` and `Alt/Option` as allowing node interactions to pass through text-editing surfaces.

## Conclusion

- **MOVE:** runtime-observed.
- **RESIZE:** runtime-observed.
- **ANCHOR:** source-supported but **not yet empirically verified in this exact screenshot sequence** because the visible result could also be explained by title-text selection.

Do not mark the anchor golden fixture complete until a deterministic interaction is captured.

## Deterministic next probe

Use **Alt/Option + double-click inside the large text/body area**. This deliberately exercises the source path that permits anchoring through text interaction surfaces. Then capture the resulting node state before any save/reload test.
