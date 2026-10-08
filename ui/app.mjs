import {
  ACTIONS, QUEST_ACTIONS, NODE_KINDS, makeUIState, currentFrame, currentNode,
  selectNode, enterFrame, returnToParent, openChildBeside, setCamera, moveNode,
  renameOrEditNode, addNode, addConnection, previewSelectedContext,
  contextBreadcrumb, visibleEdges, exportDraft,
} from './model.mjs';

const state = makeUIState();
const $ = id => document.getElementById(id);
const symbol = role => NODE_KINDS.find(t => t.role === role) ?? { symbol:'◇', tone:'slate', label:'Custom' };
const S = 'http://www.w3.org/2000/svg';
const make = (tag, className, text) => { const el = document.createElement(tag); if(className)el.className=className; if(text != null)el.textContent=text; return el; };
const svg = (tag, props={}) => { const el=document.createElementNS(S,tag); for(const [k,v] of Object.entries(props))el.setAttribute(k,String(v)); return el; };
let drag = null, pan = null, toastTime = null, periodic = false, inspectorTab = 'Content';

function toast(message) {
  clearTimeout(toastTime); const el=$('toast'); el.textContent=message; el.classList.remove('hidden');
  toastTime=setTimeout(()=>el.classList.add('hidden'),3500);
}
function showDialog(title, content) {
  $('dialog-title').textContent=title; const area=$('dialog-content'); area.replaceChildren();
  if(typeof content==='string')area.append(make('p',null,content));else area.append(content);
  $('dialog').classList.remove('hidden'); $('dialog-close').focus();
}
function closeDialog(){ $('dialog').classList.add('hidden'); }
function infoNotice(title,message){showDialog(title,message);}
function promptDraft(action) {
  const names=state.selection.map(id=>currentFrame(state).nodes.find(n=>n.id===id)?.title).filter(Boolean);
  const p=make('div');p.append(make('p',null,`Action: ${action}. Scope: ${names.length?names.join(', '):'No selected object'}. This stages a prompt, not an inference call or tool effect.`));
  const textarea=make('textarea','inspector-field inspector-text');textarea.setAttribute('aria-label','Staged prompt');
  textarea.value=`${action}: ${names.length?'work with '+names.join(', '):'select the relevant objects first'}. Preserve source evidence, options, and unresolved questions.`;
  p.append(textarea);p.append(make('p',null,'Agent execution is not connected in this UI fixture. The staged text is editable and remains local.'));
  showDialog(`QuestN / ${action}`,p);
}
function renderPalette(){
  const fams=['All',...new Set(NODE_KINDS.map(n=>n.family))]; const tabs=$('family-tabs');tabs.replaceChildren();
  for(const fam of fams){const b=make('button',fam===state.paletteFamily?'active':'',fam);b.type='button';b.setAttribute('aria-pressed',String(fam===state.paletteFamily));b.onclick=()=>{state.paletteFamily=fam;renderPalette();};tabs.append(b);}
  const list=$('palette-items');list.replaceChildren();list.classList.toggle('periodic',periodic);
  const query=state.paletteQuery.toLowerCase().trim();let count=0;
  for(const kind of NODE_KINDS.filter(k=>(state.paletteFamily==='All'||state.paletteFamily===k.family)&&(!query||`${k.label} ${k.role} ${k.family}`.toLowerCase().includes(query)))){
    const b=make('button',`element tone-${kind.tone}`);b.type='button';b.title=`Create ${kind.label} node in this frame`;
    b.append(make('span','element-symbol',kind.symbol),make('span',null,kind.label));
    b.onclick=()=>{const node=addNode(state,kind.role,(380-state.camera.x)/state.camera.zoom,(220-state.camera.y)/state.camera.zoom);renderAll();toast(`${kind.label} added to UI draft`);};
    list.append(b);count++;
  }
  if(!count)list.append(make('p','reference-copy','No matching elements. Create a custom node instead.'));
  $('palette-family-caption').textContent=periodic?'PERIODIC ORGANIZATION':'NODE DEFINITIONS';
}
function renderNav(){
  $('breadcrumbs').replaceChildren();
  const path=contextBreadcrumb(state);path.forEach((part,i)=>{if(i)$('breadcrumbs').append(make('span',null,'›'));$('breadcrumbs').append(make('span',null,part));});
  $('stage-title').textContent=currentFrame(state).name;
  $('mini-frame').textContent=state.frame==='root'?'Root':'Child';
  $('zoom-label').textContent=`${Math.round(state.camera.zoom*100)}%`;
  $('lens-label').textContent=state.view==='graph'?'KNOWLEDGE GRAPH':'PLANAR LENS';
  $('planar-mode').classList.toggle('active',state.view==='planar');
  $('graph-mode').classList.toggle('active',state.view==='graph');
  $('planar-mode').setAttribute('aria-pressed',String(state.view==='planar'));
  $('graph-mode').setAttribute('aria-pressed',String(state.view==='graph'));
  $('refs-toggle').checked=state.showReferences;
  $('theme-select').value=state.theme;
  document.body.dataset.theme=state.theme;
}
function addPort(nodeEl,node,type,reference=false){
  const p=make('button',`node-port ${type}${reference?' reference':''}`);p.type='button';p.title=`${type==='output'?'Begin':'Finish'} ${reference?'reference':'data/control'} connection`;
  p.setAttribute('aria-label',`${type} port of ${node.title}`);
  p.onclick=event=>{event.stopPropagation();handlePort(node.id,type,reference);};nodeEl.append(p);
}
function handlePort(nodeId,type,ref){
  if(type==='output'){state.connectFrom={id:nodeId,channel:ref?'reference':'data'};toast('Source chosen. Select an input port to connect.');return;}
  if(!state.connectFrom){toast('Choose an output port first.');return;}
  try{addConnection(state,state.connectFrom.id,nodeId,state.connectFrom.channel);toast('UI draft relationship created; no execution implied.');}catch(err){toast(err.message);}finally{state.connectFrom=null;renderAll();}
}
function createNode(node){
  const kind=symbol(node.role), el=make('article',`node tone-${kind.tone}${state.selection.includes(node.id)?' selected':''}`);
  el.dataset.node=node.id;el.setAttribute('tabindex','0');el.setAttribute('aria-label',`${node.title}, ${kind.label}, ${node.status}`);
  el.style.left=`${node.x}px`;el.style.top=`${node.y}px`;
  const head=make('div','node-header');head.append(make('span','node-symbol',kind.symbol));
  const info=make('div','node-info');info.append(make('strong',null,node.title),make('small',null,node.subtitle||kind.label));head.append(info);
  head.append(make('i','status-dot'));el.append(head,make('div','node-divider'));
  const body=make('div','node-body');body.append(make('p','node-summary',node.body));
  for(const row of node.rows.slice(0,3))body.append(make('div','node-row',`◇  ${row}`));
  if(node.child){const footer=make('div','node-footer');
    const beside=make('button','node-action','▣ Open beside parent');beside.onclick=e=>{e.stopPropagation();openChildBeside(state,node.id);renderAll();};
    const inside=make('button','node-action','↗ Enter');inside.onclick=e=>{e.stopPropagation();enterFrame(state,node.child);renderAll();};
    footer.append(beside,inside);body.append(footer);
  }
  if(node.role==='note'||node.role==='file'){const footer=make('div','node-footer');const preview=make('button','node-action','Preview context');preview.onclick=e=>{e.stopPropagation();selectNode(state,node.id);displayContextPreview();renderAll();};footer.append(preview);body.append(footer);}
  el.append(body);addPort(el,node,'input');addPort(el,node,'output');
  if(['note','file'].includes(node.role))addPort(el,node,'output',true);
  el.addEventListener('click',e=>{if(e.target.closest('button'))return;selectNode(state,node.id,e.shiftKey);renderAll();});
  el.addEventListener('keydown',e=>{if(e.key==='Enter'&&e.target===el){selectNode(state,node.id,e.shiftKey);renderAll();}if(e.key==='Delete'&&e.target===el)toast('Delete requires native workspace transaction; UI draft removal is disabled.');});
  head.addEventListener('pointerdown',e=>{if(e.button!==0)return;e.stopPropagation();selectNode(state,node.id,e.shiftKey);const position={x:node.x,y:node.y};drag={id:node.id,pointerX:e.clientX,pointerY:e.clientY,position};head.setPointerCapture(e.pointerId);renderInspector();});
  head.addEventListener('pointermove',e=>{if(!drag||drag.id!==node.id)return;moveNode(state,node.id,drag.position.x+(e.clientX-drag.pointerX)/state.camera.zoom,drag.position.y+(e.clientY-drag.pointerY)/state.camera.zoom);el.style.left=`${node.x}px`;el.style.top=`${node.y}px`;renderEdges();renderMinimap();});
  const end=()=>{if(drag?.id===node.id){drag=null;renderAll();}};head.addEventListener('pointerup',end);head.addEventListener('pointercancel',end);
  return el;
}
function renderEdges(){const area=$('wires');area.replaceChildren();const nodes=new Map(currentFrame(state).nodes.map(n=>[n.id,n]));
  for(const edge of visibleEdges(state)){
    const from=nodes.get(edge.from), to=nodes.get(edge.to);if(!from||!to)continue;
    const x1=from.x+228,y1=from.y+(edge.channel==='reference'?96:70),x2=to.x,y2=to.y+70;
    const bend=Math.max(72,Math.abs(x2-x1)*.45);const d=`M ${x1} ${y1} C ${x1+bend} ${y1}, ${x2-bend} ${y2}, ${x2} ${y2}`;
    const path=svg('path',{d,class:`edge ${edge.channel}${state.view==='graph'?' graph-view':''}`});
    area.append(path);
  }
}
function renderNodes(){const el=$('nodes');el.replaceChildren();for(const n of currentFrame(state).nodes)el.append(createNode(n));renderEdges();}
function renderMinimap(){const m=$('minimap-svg');m.replaceChildren();const nodes=currentFrame(state).nodes;const maxX=Math.max(1400,...nodes.map(n=>n.x+228));const maxY=Math.max(680,...nodes.map(n=>n.y+170));const sx=174/maxX,sy=66/maxY;
  for(const e of visibleEdges(state)){const a=nodes.find(n=>n.id===e.from),b=nodes.find(n=>n.id===e.to);if(a&&b)m.append(svg('line',{x1:(a.x+110)*sx,y1:(a.y+70)*sy,x2:(b.x+110)*sx,y2:(b.y+70)*sy,stroke:e.channel==='reference'?'#5c9f9e':'#5487c5','stroke-width':'1'}));}
  for(const n of nodes)m.append(svg('rect',{x:n.x*sx,y:n.y*sy,width:Math.max(7,228*sx),height:Math.max(5,115*sy),rx:2,fill:state.selection.includes(n.id)?'#80c6f7':'#305676',stroke:'#699ab7','stroke-width':'.6'}));
}
function updateCamera(){const c=state.camera;$('world').style.transform=`translate(${c.x}px, ${c.y}px) scale(${c.zoom})`;$('zoom-label').textContent=`${Math.round(c.zoom*100)}%`;}
function renderChild(){const panel=$('child-panel');panel.classList.toggle('hidden',!state.childOverlay);if(!state.childOverlay)return;
  const frame=state.project.frames[state.childOverlay];$('child-title').textContent=frame.name;const boundary=frame.boundary;$('child-boundary').textContent=`${boundary.inputs.join(' · ')} → ${boundary.outputs.join(' · ')}`;
  const area=$('child-preview');area.replaceChildren();const w=Math.max(450,area.clientWidth),scale=Math.min(.68,(w-170)/880);
  const lines=svg('svg',{class:'child-connector',viewBox:`0 0 ${w} 222`,preserveAspectRatio:'none'});area.append(lines);
  const locate=n=>({x:16+n.x*scale,y:16+(n.y-110)*.38});
  for(const e of frame.edges){const a=frame.nodes.find(n=>n.id===e.from),b=frame.nodes.find(n=>n.id===e.to);if(!a||!b)continue;const p=locate(a),q=locate(b);lines.append(svg('path',{d:`M ${p.x+146} ${p.y+30} C ${p.x+190} ${p.y+30}, ${q.x-40} ${q.y+30}, ${q.x} ${q.y+30}`,fill:'none',stroke:e.channel==='reference'?'#5b9c9c':'#628fc6','stroke-width':1.5}));}
  for(const n of frame.nodes){const p=locate(n),t=make('button','child-map-node');t.style.left=`${p.x}px`;t.style.top=`${p.y}px`;t.append(make('strong',null,n.title),make('small',null,n.subtitle));t.onclick=()=>{enterFrame(state,frame.parent?state.childOverlay:state.frame);selectNode(state,n.id);renderAll();};area.append(t);}
}
function renderInspector(){const region=$('inspector-content');region.replaceChildren();const selected=state.selection.map(id=>currentFrame(state).nodes.find(n=>n.id===id)).filter(Boolean);
  $('selection-count').textContent=selected.length?`${selected.length} selected`:'No selection';
  if(!selected.length){const empty=make('div','empty-inspector');empty.append(make('span','empty-symbol','◇'),make('strong',null,'Choose any object'),make('p',null,'Edit content, inspect its declared role and relationships, or expand a nested world.'));region.append(empty);return;}
  if(selected.length>1){const box=make('div','inspector-section');box.append(make('h3',null,'Multi-selection'),make('p',null,selected.map(n=>n.title).join(' · ')));const preview=make('button','full subtle-button','Preview selected references');preview.onclick=displayContextPreview;box.append(preview);region.append(box);return;}
  const node=selected[0];const tabs=make('div','inspector-tabs');for(const name of ['Content','Context','Connections','Program','History']){const b=make('button',name===inspectorTab?'active':'',name);b.onclick=()=>{inspectorTab=name;renderInspector();};tabs.append(b);}region.append(tabs);
  if(inspectorTab==='Content'){
    const titleLabel=make('label','inspector-label','Name');const title=make('input','inspector-field');title.value=node.title;title.setAttribute('aria-label','Node title');
    const bodyLabel=make('label','inspector-label','Content');const body=make('textarea','inspector-field inspector-text');body.value=node.body;body.setAttribute('aria-label','Node content');
    const b=make('button','full subtle-button','Apply to UI draft');b.style.marginTop='10px';b.onclick=()=>{renameOrEditNode(state,node.id,{title:title.value,body:body.value});renderAll();toast('UI draft updated. Not yet saved to Rust workspace.');};
    region.append(titleLabel,title,bodyLabel,body,b);
  }else if(inspectorTab==='Context'){
    const box=make('div','inspector-section');box.append(make('h3',null,`Selected source: ${node.id}`),make('p',null,'Only note/file sources chosen explicitly are included in the context preview. Execution is disabled.'));const btn=make('button','full subtle-button','Show exact selected context');btn.onclick=displayContextPreview;box.append(btn);region.append(box);
  }else if(inspectorTab==='Connections'){
    const box=make('div','inspector-section');box.append(make('h3',null,'Declared edges'));const edges=currentFrame(state).edges.filter(e=>e.from===node.id||e.to===node.id);if(!edges.length)box.append(make('p',null,'No connections. Use an output port, then an input port to create one.'));for(const e of edges)box.append(make('p',null,`${e.from} → ${e.to}  ·  ${e.channel}`));region.append(box);
  }else if(inspectorTab==='Program'){
    const box=make('div','inspector-section');box.append(make('h3',null,'Editable node definition (future adapter)'),make('p',null,`Role: ${node.role}\nNo executable implementation is installed in this UI fixture.`));region.append(box);
  }else{
    const box=make('div','inspector-section');box.append(make('h3',null,'UI draft provenance'),make('p',null,`View revision: ${state.project.revision}. No durable Rust workspace receipt exists for these edits.`));region.append(box);
  }
  if(node.child){const action=make('div','inspector-actions');const beside=make('button','subtle-button','▣ Open beside parent');beside.onclick=()=>{openChildBeside(state,node.id);renderAll();};const enter=make('button','subtle-button','↗ Enter world');enter.onclick=()=>{enterFrame(state,node.child);renderAll();};action.append(beside,enter);region.append(action);}
}
function renderActions(){const tools=$('prompt-tools');tools.replaceChildren();const q=$('action-search').value.toLowerCase().trim();for(const action of ACTIONS.filter(x=>!q||x.toLowerCase().includes(q))){const b=make('button');b.title=`Stage ${action} for the current selection`;b.append(make('b',null,'✧'),make('span',null,action));b.onclick=()=>promptDraft(action);tools.append(b);}
  const quest=$('quest-actions');quest.replaceChildren();for(const action of QUEST_ACTIONS){const b=make('button',null,action);b.onclick=()=>promptDraft(action);quest.append(b);}
}
function renderAll(){renderNav();renderPalette();renderNodes();renderMinimap();renderChild();renderInspector();renderActions();updateCamera();}
function displayContextPreview(){const preview=previewSelectedContext(state),box=make('div');box.append(make('p',null,`${preview.refs.length} selected note/file source(s). This is a local preview; nothing has been sent to a provider.`));const pre=make('pre');pre.textContent=preview.content||'No note/file nodes selected. Select a source, then preview again.';box.append(pre);showDialog('Exact selected context (UI draft)',box);}
function zoomTo(nextZoom,px,py){const c=state.camera, v=$('viewport').getBoundingClientRect();const x=px??v.width/2,y=py??v.height/2;
  const nx=(x-c.x)/c.zoom,ny=(y-c.y)/c.zoom;
  setCamera(state,{zoom:nextZoom,x:x-nx*nextZoom,y:y-ny*nextZoom});updateCamera();}
function setupEvents(){
  $('palette-search').addEventListener('input',e=>{state.paletteQuery=e.target.value;renderPalette();});
  $('periodic-toggle').onclick=()=>{periodic=!periodic;renderPalette();};
  $('palette-close').onclick=()=>{$('palette').classList.remove('open');if(window.innerWidth>700){document.body.classList.toggle('panels-hidden');}};
  $('add-custom').onclick=()=>{addNode(state,'workflow',300,270);renderAll();};
  $('action-search').oninput=renderActions;$('custom-prompt').onclick=()=>promptDraft('Custom Prompt');
  $('preview-context').onclick=displayContextPreview;$('run-disabled').onclick=()=>infoNotice('Real execution is not wired to this UI','The Rust provider gateway and CLI exist, but this graphical fixture has no authority to start a run. No fake receipt or result was created.');
  $('tool-help').onclick=()=>infoNotice('Prompt workbench','Actions stage an editable proposal scoped to your selection. They do not call a model, alter permissions, or execute effects.');
  $('theme-select').onchange=e=>{state.theme=e.target.value;renderNav();};$('refs-toggle').onchange=e=>{state.showReferences=e.target.checked;renderEdges();renderMinimap();};
  $('planar-mode').onclick=()=>{state.view='planar';renderAll();};$('graph-mode').onclick=()=>{state.view='graph';renderAll();};
  $('toggle-panels').onclick=()=>{if(window.innerWidth<=1080){$('right-panel').classList.toggle('open');}else{document.body.classList.toggle('panels-hidden');}};
  $('zoom-in').onclick=()=>zoomTo(state.camera.zoom*1.2);$('zoom-out').onclick=()=>zoomTo(state.camera.zoom/1.2);$('zoom-home').onclick=()=>{setCamera(state,{x:0,y:0,zoom:.84});updateCamera();};
  $('child-close').onclick=()=>{state.childOverlay=null;renderChild();};$('child-enter').onclick=()=>{if(state.childOverlay){enterFrame(state,state.childOverlay);renderAll();}};
  $('export-draft').onclick=()=>{const blob=new Blob([exportDraft(state)],{type:'application/json'});const url=URL.createObjectURL(blob);const link=make('a');link.href=url;link.download='hq-ui-nonauthoritative-draft.json';link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);toast('Downloaded review-only UI draft; not persisted to HQ kernel.');};
  $('edge-mode').onclick=()=>{state.connectFrom=null;toast('Select a node’s output port, then another node’s input port. Links do not execute.');};
  $('dialog-close').onclick=closeDialog;$('dialog-done').onclick=closeDialog;$('dialog').onclick=e=>{if(e.target===$('dialog'))closeDialog();};
  $('nav-notes').onclick=()=>{if(state.frame!=='root')returnToParent(state);selectNode(state,'note-a');renderAll();};
  $('nav-agents').onclick=()=>{if(state.frame!=='root')returnToParent(state);selectNode(state,'agent');renderAll();};
  $('nav-worlds').onclick=()=>{while(returnToParent(state)){};setCamera(state,{x:0,y:0,zoom:.84});renderAll();};
  $('nav-help').onclick=()=>infoNotice('Navigation','Click to select · Shift-click to select several · Drag node header to move · Drag background to pan · Wheel to zoom at pointer · Output port then input port to connect · Enter a composite node or open beside its parent. Shortcuts: / search; Esc close; +/− zoom.');
  $('global-search').addEventListener('input',e=>{const query=e.target.value.toLowerCase().trim();if(!query)return;const n=currentFrame(state).nodes.find(n=>`${n.title} ${n.body} ${n.role}`.toLowerCase().includes(query));if(n){selectNode(state,n.id);renderAll();}});
  const vp=$('viewport');
  vp.addEventListener('pointerdown',e=>{if(e.button!==0||e.target.closest('.node,.canvas-corner'))return;pan={x:e.clientX,y:e.clientY,camera:{...state.camera}};vp.classList.add('panning');vp.setPointerCapture(e.pointerId);});
  vp.addEventListener('pointermove',e=>{if(!pan)return;setCamera(state,{...state.camera,x:pan.camera.x+e.clientX-pan.x,y:pan.camera.y+e.clientY-pan.y});updateCamera();});
  const endPan=()=>{pan=null;vp.classList.remove('panning');};vp.addEventListener('pointerup',endPan);vp.addEventListener('pointercancel',endPan);
  vp.addEventListener('wheel',e=>{e.preventDefault();const r=vp.getBoundingClientRect();zoomTo(state.camera.zoom*(e.deltaY>0?.9:1.1),e.clientX-r.left,e.clientY-r.top);},{passive:false});
  document.addEventListener('keydown',e=>{if(e.key==='Escape'){closeDialog();state.connectFrom=null;return;}
    if(e.key==='/'&&!['INPUT','TEXTAREA'].includes(document.activeElement?.tagName)){e.preventDefault();$('global-search').focus();}
    if(e.target===vp&&(e.key==='+'||e.key==='=')){e.preventDefault();zoomTo(state.camera.zoom*1.2);}
    if(e.target===vp&&e.key==='-'){e.preventDefault();zoomTo(state.camera.zoom/1.2);}
    if(e.key==='Backspace'&&e.altKey){e.preventDefault();returnToParent(state);renderAll();}
  });
  window.addEventListener('resize',()=>{if(state.childOverlay)renderChild();});
}
setupEvents();selectNode(state,'agent');renderAll();
