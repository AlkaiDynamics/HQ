# Runtime Evidence — Move / Resize / Collapse, with Anchor Correction

Status: **RUNTIME-OBSERVED + SOURCE-CORRELATED**

## Context

- Exact verified Neurite Desktop Windows build: `electron-2025.06.18.233853`.
- Existing save `p001` contains spatial note `PERSISTENCE-TEST-001`.
- User was asked to batch MOVE + SCALE/RESIZE + ANCHOR and supplied six screenshots showing successive spatial/window states.

## Runtime observations from screenshots

The supplied screenshots visibly demonstrate that the tested note can:

- occupy substantially different positions in the canvas;
- be resized between a large rectangular window and a much smaller rectangular window;
- be collapsed into Neurite's circular collapsed representation;
- be expanded back into a rectangular window.

Because the fractal camera itself can pan/zoom independently, the screenshots alone are not sufficient to prove exact world-coordinate movement or exact persistence of the moved position. Those remain for a save/reload differential check.

## Important correction: circle icon is collapse, not anchor

The prior test instruction incorrectly identified the circle icon in the node title bar as an anchor/pin control.

Frozen source proves otherwise:

- `NodeView.rewindowify()` binds `#button-collapse` to `toggleCollapse()`.
- `resources/svg/icons.html` defines `button-collapse` as the circular title-bar icon.
- Collapsing transforms the node window into a 60x60 circular representation and later restores the original dimensions when expanded.

Therefore the screenshots showing a large circular node are empirical evidence of **collapse/expand behavior**, not anchoring.

## Actual anchor interaction

Frozen source and the README agree that anchoring is toggled by **double-clicking the node/window** (outside editable text unless Alt is held):

```text
double-click node/window
→ node.anchor = node.pos
→ node.anchorForce = 1 - node.anchorForce
→ toggle `window-anchored` visual state
```

The README describes `Double Click` as `Anchor a node`.

So anchor remains runtime-open until the user performs that actual gesture and we observe the resulting visual/physical behavior.

## Behavioral conclusions

Source + runtime now support these distinctions:

```text
Resize handle / resize interaction
→ changes window dimensions

Circle title-bar control
→ collapse / expand projection state

Double-click node/window
→ anchor / unanchor physical state
```

These must remain separate successor contracts. Collapse is a projection/UI state; anchor is a spatial/physics state.

## Successor UX implication

The fact that the circle icon was easy to misread as an anchor control is itself evidence of weak affordance semantics in the legacy UI. The successor should use explicit tooltips/labels and distinguish:

- Collapse/Minimize
- Pin/Anchor
- Resize/Scale
- Focus/Zoom

without relying on icon interpretation alone.

## Still unverified

- exact runtime anchor visual state;
- whether anchored nodes resist graph/fractal forces as expected during live interaction;
- whether move/world position persists exactly through save/load;
- whether resized window dimensions persist exactly through save/load;
- whether collapse state persists through save/load;
- whether anchor state persists through save/load.
