# Neurite Successor — Interaction & View Requirements

Status: **USER-DIRECTED REQUIREMENTS**

These requirements come from direct use of the legacy Neurite desktop build during runtime forensics. They are successor requirements, not claims about legacy behavior.

## 1. Discoverability and guidance

The successor should favor explicit, learnable interaction over hidden gestures.

Required interaction qualities:

- Tooltips for controls and non-obvious affordances.
- Contextual hints where the user is likely to get stuck.
- Lightweight reminders for shortcuts and alternate interaction methods.
- First-use guidance that can be dismissed and later recalled.
- Visible state transitions: the UI should make it clear what action just occurred and what object/state changed.
- Keyboard shortcuts may accelerate workflows, but important capability must not depend on memorizing a hidden shortcut.

The observed legacy note workflow — where spatial note creation requires `Shift + double-click` while the Notes panel looks like a note-creation surface — is a concrete example of behavior whose capability should be preserved but whose affordance should be redesigned.

## 2. Progressive disclosure

The UI should expose substantial power without permanently occupying the screen.

Requirements:

- Controls grouped into collapsible toolbars/palettes.
- Context-sensitive toolbars for the currently selected object/mode.
- Advanced controls can expand on demand while common actions remain immediately reachable.
- Panels should be dockable/collapsible where practical.
- The same capability should be reachable through direct manipulation, menus/toolbars, and natural-language agent interaction when appropriate.

## 3. Mobile and touch as first-class interaction

The user expects to operate the system from a phone frequently.

Requirements:

- Responsive layouts rather than a desktop canvas merely scaled down.
- Touch-sized controls and spacing.
- Important interactions must not depend on hover.
- Touch equivalents for selection, move, scale, connect, multi-select, context actions, and mode changes.
- Compact mobile views for monitoring agents, approving actions, chatting, browsing projects/repositories, and opening focused artifacts.
- Desktop spatial power and mobile operational access should share the same underlying workspace/state model.

## 4. Explicit view/mode system

The successor should support multiple projections of the same underlying objects/workspace rather than forcing every task into one canvas.

At minimum, the design should accommodate distinct modes/views such as:

- Spatial/fractal exploration view.
- 2D blackboard/whiteboard view.
- Workflow composition view.
- Agent operations / monitoring dashboard.
- Focused document/note/editor view.
- Compact mobile operational views.

View changes should not imply destructive conversion of underlying objects. A logical object may have multiple projections.

## 5. 2D Blackboard / Whiteboard mode

A dedicated 2D blackboard/whiteboard mode is required.

The exact implementation is open, but the interaction model should support:

- Freeze/pin a note or object into a 2D composition surface.
- Drag-and-drop placement.
- Diagramming primitives.
- Easier, explicit control over edges/connectors.
- Relationship type/direction editing.
- Labels/annotations.
- Grouping and multi-selection.
- Resize, align, distribute, snap, and layout assistance.
- Undo/redo.
- Fast creation of notes, cards, shapes, media, agents, workflows, repositories, and other supported objects.
- Tool palettes that can expand/collapse rather than permanently consume canvas area.
- Touch-friendly operation.

A whiteboard connection must remain distinguishable from semantic, authority, and runtime relationships. Drawing an edge cannot silently grant execution authority or create a privileged operational dependency.

## 6. Agent interaction

Agent interaction should be substantially more intuitive than the legacy prompt areas.

Requirements:

- A clearly identifiable conversation surface.
- Context-aware chat: the user can refer naturally to selected nodes, repositories, documents, workflows, agents, or regions of the canvas.
- Agent actions should surface proposed mutations before consequential execution when appropriate.
- Common object operations should be expressible conversationally without forcing the user to know implementation details.
- Agent responses should connect back to visible workspace objects/results rather than remain isolated chat text.

## 7. Workflow creation and editing

The UI must make agent/workflow composition easy rather than requiring low-level configuration.

Required user-level behavior includes requests such as:

> "That repo — let's turn it into a workflow."

The system should be capable of converting that intent into a usable workflow with minimal manual wiring. The resulting workflow should be inspectable and editable rather than opaque.

A repo-to-workflow interaction should be able to:

1. identify the selected repository/context;
2. inspect available capabilities/operations;
3. derive a proposed workflow or operational graph;
4. map required inputs, outputs, permissions, tools, and runtime bindings;
5. display the resulting workflow visually;
6. allow user approval/amendment where consequential;
7. instantiate/execute it once admitted;
8. preserve provenance back to the source repository and extraction evidence.

The UI should hide unnecessary plumbing while retaining inspectability.

## 8. Agent monitoring dashboard

A dedicated operational view is required for understanding what agents are doing.

It should make it easy to see, at minimum:

- running / waiting / blocked / failed / complete agents and tasks;
- current goal/task;
- parent/child or delegated work;
- active tools/capabilities;
- current workflow stage;
- recent actions/events;
- errors and retry state;
- resource/cost/latency signals where relevant;
- approvals or user input required;
- pause/resume/stop/retry controls;
- links back to generated artifacts, workspace objects, and source evidence.

The dashboard should support both a broad system overview and drill-down into one agent/workflow.

## 9. Interaction design principle

Preserve Neurite's valuable spatial/cognitive capabilities, but do not preserve accidental interaction friction.

Target principle:

```text
powerful capability
+ visible affordance
+ contextual guidance
+ progressive disclosure
+ multiple projections
+ touch/mobile parity
+ natural-language control
+ inspectable system state
```

The successor should feel like a system that teaches the user what can be done while they use it, rather than requiring prior knowledge of hidden gestures.

## 10. Primary experiential target: dynamic neural knowledge environment

The long-term primary experience should feel less like an application and more like a **dynamic, interactive neural knowledge environment with continuous AI conversation**.

Requirements:

- A 3D-capable knowledge graph / spatial cognition view should be supported as a first-class projection, alongside 2D modes.
- Nodes may represent knowledge, people, projects, repositories, files, agents, workflows, tools, media, decisions, goals, events, or other system entities.
- Relationships should be visually legible and semantically typed.
- The scene should be able to expand, collapse, cluster, filter, focus, and re-project without changing underlying authoritative state.
- AI conversation should be spatially contextual: the user should be able to point at, select, look at, mention, or otherwise indicate an object and speak naturally about it.
- Agent outputs should be able to appear as graph changes, annotations, proposed workflows, generated artifacts, highlighted relationships, or other visible mutations rather than only chat messages.
- The interface should support smooth movement between broad situational awareness and focused work.

The target experience is approximately:

```text
see / hear / point / select / speak
→ system resolves context
→ agent reasons over relevant graph + state
→ proposes or performs allowed action
→ result appears in the shared environment
→ user can inspect, approve, redirect, or continue naturally
```

## 11. Ambient and wearable interaction

The desktop and phone are transitional clients, not the assumed final interaction model.

The architecture must accommodate a future in which the user frequently interacts through ambient/wearable devices such as:

- smart glasses;
- earbuds/headsets;
- small voice-first companion devices;
- watches or other glanceable approval surfaces;
- future spatial-computing interfaces.

Requirements:

- Conversation and command semantics must not be coupled to a desktop window.
- Core interaction intents must be representable independently of mouse/keyboard UI events.
- Voice-first interaction must support conversational continuity, object reference, follow-up, interruption, correction, confirmation, and resumption.
- Visual clients should be optional projections of the same underlying session/context where feasible.
- The system should support concise spoken summaries and progressive drill-down rather than requiring visual inspection for every decision.
- Approval requests should be deliverable through minimal surfaces with enough context to make a safe decision.
- A wearable client must be able to hand off seamlessly to phone, tablet, desktop, whiteboard, or spatial view without losing task/session context.
- Important tasks should remain operable when no rich visual surface is currently available.

Design target: over time, routine operation should require progressively less direct interaction with a phone or computer.

## 12. Human participation is structural, not incidental

The user intends to remain **human in the loop, for the loop, and of the loop**. This is a system-design requirement, not merely a UI preference.

The successor should distinguish several forms of human participation:

### Human in the loop
The user can approve, reject, amend, redirect, pause, stop, or take over consequential actions.

### Human for the loop
The system should optimize automation around the user's goals, constraints, preferences, judgment, attention, and desired outcomes rather than treating autonomy as an end in itself.

### Human of the loop
The user is part of the operating system's ongoing state and feedback structure: observations, decisions, corrections, preferences, priorities, exceptions, and judgments become explicit inputs into future behavior and adaptation.

Requirements:

- Human intervention points must be explicit and inspectable.
- The user should be able to set delegation boundaries by capability, context, risk, project, agent, or workflow.
- The system should learn from user corrections without silently expanding authority.
- Low-risk reversible work may become increasingly autonomous; high-impact or ambiguous actions should remain appropriately reviewable.
- The user must retain the ability to inspect why an action happened, what information informed it, what changed, and what can be reversed.
- The interface should minimize unnecessary approval fatigue while preserving meaningful control.

The goal is not maximum autonomy. The goal is **maximum useful leverage while preserving human agency, judgment, and continuity of intent**.
