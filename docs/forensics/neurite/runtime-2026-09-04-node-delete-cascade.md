# Runtime Evidence — Text Node Delete Cascade

## Scenario
Legacy Neurite runtime, Windows desktop build. Two text nodes previously existed: `balls1` and `balls2`. Their connecting edge had already been deleted, so no relationship references remained before this test.

## Interaction
User clicked the `X` control on the `balls2` spatial text-node window.

## Observed visual effect
- The `balls2` spatial node disappeared from the canvas.
- The `balls1` spatial node remained present.

## Observed Notes/Zettelkasten effect
- The Notes panel no longer displayed the `balls2` section (`## 26-09-04 ~ 18:29:17.426` / `balls2`).
- The `balls1` section remained present.

## Empirical conclusion
For this tested text-node case, deleting the spatial text node deletes its corresponding Notes/Zettelkasten note section rather than merely removing the spatial projection.

Observed lifecycle distinction now includes:

`delete edge -> preserve both notes, remove relationship`

`delete text node -> remove spatial node + remove corresponding note section`

## Evidence status
- Interaction: observed
- Visual node deletion: observed
- Corresponding Notes entry deletion: observed
- Generalization to every node subtype: unknown
- Durable persistence behavior after this deletion: not tested in this step
