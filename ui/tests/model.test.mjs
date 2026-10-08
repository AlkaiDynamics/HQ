import test from 'node:test';
import assert from 'node:assert/strict';
import {
  ACTIONS, QUEST_ACTIONS, NODE_KINDS, makeUIState, currentFrame, selectNode,
  enterFrame, returnToParent, openChildBeside, setCamera, moveNode,
  renameOrEditNode, addNode, addConnection, previewSelectedContext,
  contextBreadcrumb, visibleEdges, exportDraft,
} from '../model.mjs';

test('selected context contains only selected note/file nodes, never agent data', () => {
  const state=makeUIState();selectNode(state,'agent');assert.equal(previewSelectedContext(state).refs.length,0);
  selectNode(state,'note-a',true);const p=previewSelectedContext(state);
  assert.deepEqual(p.refs.map(x=>x.id),['note-a']);assert.match(p.content,/Neurite-style notes/);
  assert.doesNotMatch(p.content,/Receives explicitly selected references/);assert.equal(p.execution,'not-requested');
});

test('nested frame can open beside parent without moving parent, or enter and return with camera', () => {
  const state=makeUIState();selectNode(state,'agent');const before=state.camera;
  const overlay=openChildBeside(state,'agent');assert.equal(overlay.parent,'root');assert.equal(state.frame,'root');assert.deepEqual(state.camera,before);
  setCamera(state,{x:34,y:-18,zoom:1.1});enterFrame(state,'agent-world');assert.equal(state.frame,'agent-world');
  assert.deepEqual(contextBreadcrumb(state),['Research workspace','Reasoning agent / internal graph']);
  assert.equal(state.camera.zoom,.84);assert.deepEqual(state.cameraByFrame.root,{x:34,y:-18,zoom:1.1});
  assert.equal(returnToParent(state),true);assert.equal(state.frame,'root');assert.equal(state.camera.zoom,1.1);
  assert.equal(returnToParent(state),false);
});

test('camera zoom is bounded and never changes runtime status or graph records', () => {
  const state=makeUIState();const original=structuredClone(state.project);
  setCamera(state,{x:103,y:-82,zoom:20});assert.equal(state.camera.zoom,2);
  setCamera(state,{x:0,y:0,zoom:.01});assert.equal(state.camera.zoom,.35);
  assert.throws(()=>setCamera(state,{x:NaN,y:0,zoom:1}),/Invalid camera/);
  assert.deepEqual(state.project,original);
});

test('moving and editing a selected node changes only the UI draft revision', () => {
  const state=makeUIState();const before=state.project.revision;
  moveNode(state,'note-a',1301,102);renameOrEditNode(state,'note-a',{title:'New note',body:'Updated'});
  const node=currentFrame(state).nodes.find(n=>n.id==='note-a');assert.equal(node.x,1301);assert.equal(node.body,'Updated');
  assert.equal(state.project.revision,before+2);assert.equal(node.status,'not-run');
  assert.throws(()=>renameOrEditNode(state,'note-a',{body:123}),/Invalid content/);
});

test('explicit connection channel protects note references from silent execution', () => {
  const state=makeUIState();const edge=addConnection(state,'note-a','agent','reference');
  assert.equal(edge.channel,'reference');assert.equal(edge.authority,undefined);
  assert.throws(()=>addConnection(state,'note-a','agent','reference'),/Duplicate/);
  assert.throws(()=>addConnection(state,'note-a','agent','exec'),/Invalid edge channel/);
  assert.throws(()=>addConnection(state,'note-a','missing','data'),/Invalid endpoints/);
  const count=visibleEdges(state).length;state.showReferences=false;
  assert.ok(visibleEdges(state).every(e=>e.channel!=='reference'));
  assert.ok(visibleEdges(state).length<count);assert.equal(currentFrame(state).edges.length,count);
});

test('node catalog is open-ended: new instances are not limited to mockup element counts', () => {
  const state=makeUIState();const before=currentFrame(state).nodes.length;
  for(let i=0;i<31;i++) addNode(state,'tool',200+i*4,200);
  assert.equal(currentFrame(state).nodes.length,before+31);
  assert.equal(new Set(currentFrame(state).nodes.map(n=>n.id)).size,before+31);
  assert.throws(()=>addNode(state,'invented',0,0),/Unknown template/);
  assert.ok(NODE_KINDS.some(k=>k.role==='terminal'));
});

test('parent-child boundary ports stay visible and remain declarations, not grants', () => {
  const state=makeUIState();const frame=openChildBeside(state,'agent');
  assert.deepEqual(frame.boundary.inputs,['Goal','Research data','User context']);
  assert.deepEqual(frame.boundary.outputs,['To tools','To review']);
  assert.equal(frame.runtime,undefined);
});

test('export identifies snapshot as nonauthoritative and no execution receipts are invented', () => {
  const state=makeUIState();const out=JSON.parse(exportDraft(state));
  assert.equal(out.authoritative,false);assert.equal(out.uiDraft,true);assert.equal(out.project.schema,'hq.ui.fixture.v1');
  assert.equal(out.project.runs,undefined);assert.equal(out.project.receipts,undefined);
});

test('recognition-first controls and full prompt set are retained rather than hard-capped',()=>{
  assert.ok(ACTIONS.includes('Chat with Selection'));
  assert.ok(ACTIONS.includes('Custom Node'));
  assert.ok(QUEST_ACTIONS.includes('Show All N'));
  assert.ok(QUEST_ACTIONS.includes('Show Next K'));
  assert.ok(QUEST_ACTIONS.includes('Missing Shape'));
  assert.ok(ACTIONS.length>9);
});

test('returning from a nested world reselects its containing node',()=>{
  const s=makeUIState();enterFrame(s,'agent-world');selectNode(s,'reason');assert.equal(returnToParent(s),true);
  assert.equal(s.frame,'root');assert.deepEqual(s.selection,['agent']);assert.equal(s.inspector,true);
});
