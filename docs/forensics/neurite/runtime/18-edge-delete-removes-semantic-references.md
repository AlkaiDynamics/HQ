# Runtime Evidence — Edge Delete Removes Semantic References

Evidence status: observed

Test context:
- Two spatial text nodes existed: `balls1` and `balls2`.
- They had previously been connected, cycled through directional states, and returned to undirected.
- The undirected state showed reciprocal Zettelkasten references in both notes and a visible curved spatial edge.

Observed action:
- User opened the edge context menu and selected `delete`.

Observed result:
- The visible spatial edge between `balls1` and `balls2` disappeared.
- `balls1` remained as a spatial node.
- `balls2` remained as a spatial node.
- In the Notes archive, the reciprocal `[[...]]` references were removed from both notes.
- The note bodies returned to their plain content (`balls1`, `balls2`) with no cross-reference lines visible.

Behavioral conclusion:

`delete visual edge between two text nodes -> remove visual relationship + remove reciprocal Zettelkasten semantic references`

This is strong runtime evidence that legacy Neurite treats the text-node edge as more than a projection artifact: deleting the edge mutates note semantics as well.

Successor requirement implication:
- Visual relationship deletion and semantic relationship deletion must be distinct operations/contracts even when they are intentionally synchronized.
- UI should make the semantic consequence explicit before destructive relationship changes.
- Relationship type/authority must be inspectable rather than inferred from visual line styling alone.
