/** HQ presentation model. These view operations never authorize or execute a run.
 * IDs are stable within this fixture; production identity/revisions remain Rust-owned.
 */
export const NODE_KINDS = Object.freeze([
  { role: 'agent', label: 'AI agent', symbol: 'Ag', family: 'AI', tone: 'cyan' },
  { role: 'model', label: 'Model', symbol: 'Lm', family: 'AI', tone: 'blue' },
  { role: 'prompt', label: 'Prompt', symbol: 'Pr', family: 'AI', tone: 'violet' },
  { role: 'note', label: 'Note', symbol: 'Nt', family: 'Knowledge', tone: 'amber' },
  { role: 'file', label: 'File / docs', symbol: 'Fs', family: 'Knowledge', tone: 'blue' },
  { role: 'browser', label: 'Browser', symbol: 'Bw', family: 'Surfaces', tone: 'cyan' },
  { role: 'terminal', label: 'Terminal', symbol: 'Cl', family: 'Surfaces', tone: 'slate' },
  { role: 'calendar', label: 'Schedule', symbol: 'Sc', family: 'Time', tone: 'teal' },
  { role: 'workflow', label: 'Workflow', symbol: 'Wf', family: 'Logic', tone: 'violet' },
  { role: 'tool', label: 'Tool', symbol: 'Tl', family: 'Logic', tone: 'blue' },
  { role: 'review', label: 'Human review', symbol: 'Rv', family: 'Logic', tone: 'amber' },
  { role: 'output', label: 'Output', symbol: 'Ot', family: 'Knowledge', tone: 'teal' },
]);

const item = (id, title, role, x, y, body, extras = {}) => ({
  id, title, role, x, y, body, subtitle: extras.subtitle ?? '',
  rows: extras.rows ?? [], child: extras.child ?? null,
  status: extras.status ?? 'not-run',
});

export function makeFixture() {
  return {
    schema: 'hq.ui.fixture.v1',
    revision: 0,
    frames: {
      root: {
        name: 'Research workspace',
        parent: null,
        nodes: [
          item('goal', 'Research objective', 'prompt', 92, 104,
            'Understand the topic, preserve source evidence, and turn findings into inspectable choices.',
            { subtitle: 'Goal / objective', rows: ['Objective', 'Success criteria'] }),
          item('research', 'Research inputs', 'file', 390, 104,
            'Imported materials, citations, and selected context. Nothing is sent to a model until an approved run.',
            { subtitle: 'Evidence / knowledge', rows: ['Source inventory', 'Extracted notes', 'Version history'] }),
          item('agent', 'Reasoning agent', 'agent', 688, 104,
            'Receives explicitly selected references, evaluates options, and proposes a plan.',
            { subtitle: 'Agent / composite node', child: 'agent-world', rows: ['Inspect internal graph', 'Context policy', 'Provider: not connected'] }),
          item('tools', 'Tools and actions', 'tool', 120, 430,
            'External effects are governed by separate permission and execution contracts.',
            { subtitle: 'Tools / actions', rows: ['Browser', 'Script', 'MCP connection'] }),
          item('review', 'Human review', 'review', 418, 430,
            'Approval is an explicit wait state. A visual connection does not authorize an external effect.',
            { subtitle: 'Decision / human in loop', rows: ['Proposed actions', 'Approve or revise'] }),
          item('output', 'Output and receipts', 'output', 716, 430,
            'Inspect proposed responses and execution receipts here when a native runtime is connected.',
            { subtitle: 'Result / provenance', rows: ['Response', 'Execution history', 'Evidence'] }),
          item('note-a', 'Field note', 'note', 1030, 130,
            'Neurite-style notes coexist with workflows on the same spatial canvas.',
            { subtitle: 'Editable note', rows: ['References', 'Backlinks'] }),
          item('calendar', 'Schedule', 'calendar', 1030, 450,
            'A scheduled action belongs to its authorized host. A canvas card is a view, not a clock.',
            { subtitle: 'Time / planning', rows: ['Occurrences', 'Time zone'] }),
        ],
        edges: [
          { id: 'e1', from: 'goal', to: 'research', channel: 'data' },
          { id: 'e2', from: 'research', to: 'agent', channel: 'data' },
          { id: 'e3', from: 'agent', to: 'tools', channel: 'control' },
          { id: 'e4', from: 'tools', to: 'review', channel: 'control' },
          { id: 'e5', from: 'review', to: 'output', channel: 'control' },
          { id: 'e6', from: 'note-a', to: 'research', channel: 'reference' },
        ],
      },
      'agent-world': {
        name: 'Reasoning agent / internal graph',
        parent: 'root',
        parentNode: 'agent',
        boundary: { inputs: ['Goal', 'Research data', 'User context'], outputs: ['To tools', 'To review'] },
        nodes: [
          item('planner', 'Planner', 'prompt', 90, 130, 'Break objective into inspectable tasks.', { subtitle: '3.1 • Plan', rows: ['Decompose goal', 'Identify dependencies'] }),
          item('reason', 'Reasoning', 'agent', 382, 130, 'Evaluate alternatives and evidence.', { subtitle: '3.2 • Evaluate', rows: ['Weigh options', 'Check unknowns'] }),
          item('router', 'Router', 'tool', 674, 130, 'Propose which declared capability may be invoked.', { subtitle: '3.3 • Route', rows: ['Select candidate', 'Require admission'] }),
          item('memory', 'Memory', 'note', 230, 420, 'Bounded references and durable claims have separate provenance.', { subtitle: '3.4 • Context', rows: ['Retrieve', 'Record sources'] }),
          item('reflection', 'Reflection', 'review', 570, 420, 'Evaluate the result without silently replaying effects.', { subtitle: '3.5 • Verify', rows: ['Check result', 'Revise next step'] }),
        ],
        edges: [
          { id: 'c1', from: 'planner', to: 'reason', channel: 'data' },
          { id: 'c2', from: 'reason', to: 'router', channel: 'data' },
          { id: 'c3', from: 'memory', to: 'reason', channel: 'reference' },
          { id: 'c4', from: 'router', to: 'reflection', channel: 'control' },
        ],
      },
    },
  };
}

export const ACTIONS = Object.freeze(['What', 'How', 'Why', 'Who', 'Origin', 'Elaborate', 'Example', 'Pros / Cons', 'Compare', 'Extract', 'Concepts', 'Analogy', 'Research', 'Split', 'Join', 'SOP', 'Custom Prompt', 'Custom Node', 'Chat with Selection']);
export const QUEST_ACTIONS = Object.freeze(['Select', 'Select Several', 'Combine', 'Compare', 'Exclude', 'Defer', 'Expand Family / Node', 'Show Next K', 'Show All N', 'Show Constrained', 'Show Unknowns', 'Reopen Excluded', 'Missing Shape']);

function assertFrame(state, frame) {
  if (!state.project.frames[frame]) throw new Error('Unknown workspace frame');
}

export function makeUIState(project = makeFixture()) {
  return {
    project: structuredClone(project), frame: 'root', selection: [],
    camera: { x: 0, y: 0, zoom: 0.84 },
    cameraByFrame: {}, childOverlay: null, view: 'planar', theme: 'grounded',
    showReferences: true, paletteFamily: 'All', paletteQuery: '',
    inspector: false, connectFrom: null, choiceMode: false,
    history: [],
  };
}
export function currentFrame(state) { return state.project.frames[state.frame]; }
export function currentNode(state, id = state.selection[0]) { return currentFrame(state).nodes.find(n => n.id === id) ?? null; }
export function selectNode(state, id, additive = false) {
  if (!currentNode(state, id)) throw new Error('Unknown node');
  state.selection = additive ? (state.selection.includes(id) ? state.selection.filter(x => x !== id) : [...state.selection, id]) : [id];
  state.inspector = state.selection.length > 0;
  return state.selection;
}
export function saveCamera(state) { state.cameraByFrame[state.frame] = { ...state.camera }; }
export function enterFrame(state, frame) {
  assertFrame(state, frame);
  saveCamera(state);
  state.history.push(state.frame);
  state.frame = frame;
  state.camera = { ...state.cameraByFrame[frame] ?? { x: 0, y: 0, zoom: 0.84 } };
  state.selection = [];
  state.inspector = false;
  state.childOverlay = null;
}
export function returnToParent(state) {
  const child = state.frame;
  const parent = currentFrame(state).parent;
  if (!parent) return false;
  saveCamera(state);
  state.frame = parent;
  state.camera = { ...state.cameraByFrame[parent] ?? { x: 0, y: 0, zoom: 0.84 } };
  state.history.pop();
  const parentNodeId = state.project.frames[child].parentNode;
  state.selection = parentNodeId && currentFrame(state).nodes.some(n => n.id === parentNodeId) ? [parentNodeId] : [];
  state.inspector = state.selection.length > 0;
  state.childOverlay = null;
  return true;
}
export function openChildBeside(state, nodeId) {
  const node = currentNode(state, nodeId);
  if (!node?.child) throw new Error('Node has no nested world');
  assertFrame(state, node.child);
  state.childOverlay = node.child;
  return state.project.frames[node.child];
}
export function setCamera(state, camera) {
  const zoom = Math.min(2, Math.max(0.35, camera.zoom));
  if (![camera.x, camera.y, zoom].every(Number.isFinite)) throw new Error('Invalid camera');
  state.camera = { x: camera.x, y: camera.y, zoom };
}
export function moveNode(state, id, x, y) {
  const node = currentNode(state, id);
  if (!node || !Number.isFinite(x) || !Number.isFinite(y)) throw new Error('Invalid placement');
  node.x = Math.round(x); node.y = Math.round(y);
  state.project.revision++;
}
export function renameOrEditNode(state, id, changes) {
  const node = currentNode(state, id);
  if (!node) throw new Error('Unknown node');
  const fields = ['title', 'body'];
  for (const key of fields) if (key in changes) {
    if (typeof changes[key] !== 'string' || changes[key].length > 10000) throw new Error('Invalid content');
    node[key] = changes[key];
  }
  state.project.revision++;
}
export function addNode(state, role, x = 380, y = 250) {
  const definition = NODE_KINDS.find(n => n.role === role);
  if (!definition) throw new Error('Unknown template');
  const ids = new Set(Object.values(state.project.frames).flatMap(f => f.nodes.map(n => n.id)));
  let index = 1;
  while (ids.has(`draft-${index}`)) index++;
  const next = item(`draft-${index}`, definition.label, role, Math.round(x), Math.round(y), 'Edit this node in the Inspector.', { subtitle: 'Draft • UI only' });
  currentFrame(state).nodes.push(next);
  state.project.revision++;
  selectNode(state, next.id);
  return next;
}
export function addConnection(state, from, to, channel = 'data') {
  const frame = currentFrame(state);
  if (from === to || !frame.nodes.some(n => n.id === from) || !frame.nodes.some(n => n.id === to)) throw new Error('Invalid endpoints');
  if (!['data', 'control', 'reference'].includes(channel)) throw new Error('Invalid edge channel');
  if (frame.edges.some(e => e.from === from && e.to === to && e.channel === channel)) throw new Error('Duplicate relationship');
  let id = `ui-edge-${state.project.revision + 1}`;
  while (frame.edges.some(e => e.id === id)) id += 'x';
  frame.edges.push({ id, from, to, channel });
  state.project.revision++;
  return frame.edges.at(-1);
}
export function previewSelectedContext(state) {
  const frame = currentFrame(state);
  const selected = frame.nodes.filter(n => state.selection.includes(n.id) && ['note','file'].includes(n.role));
  return { refs: selected.map(n => ({ id: n.id, role: n.role, title: n.title })), content: selected.map(n => `[${n.role} ${n.id}: ${n.title}]\n${n.body}`).join('\n\n'), execution: 'not-requested' };
}
export function contextBreadcrumb(state) {
  const out = [];
  let frame = state.frame;
  while (frame) { out.unshift(state.project.frames[frame].name); frame = state.project.frames[frame].parent; }
  return out;
}
export function visibleEdges(state) { return currentFrame(state).edges.filter(e => state.showReferences || e.channel !== 'reference'); }
export function exportDraft(state) {
  return JSON.stringify({ schema: state.project.schema, uiDraft: true, authoritative: false, project: state.project }, null, 2);
}
