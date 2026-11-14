/* Small native bridge on the pinned Neurite application. No alternate scene model. */
(async () => {
  const pause = (ms) => new Promise((r) => setTimeout(r, ms));
  for (let i = 0; !window.appReady && i < 600; i++) await pause(100);
  if (!window.appReady) return;
  await App.viewGraphs.nativeReady;
  const section = document.createElement("section");
  section.id = "hq-native";
  section.innerHTML = `<details><summary>HQ · Native activity</summary><p id="hq-host-status" role="status">Connecting…</p><div class="hq-row"><button id="hq-save">Save workspace</button><button id="hq-restore">Reload saved workspace</button></div><details><summary>Local model settings</summary><label>Provider<select id="hq-provider"><option value="ollama">Ollama</option><option value="openai">LM Studio / compatible local server</option></select></label><label>Installed model<input id="hq-model"></label><button id="hq-settings">Save model settings</button></details><details><summary>Add notes or import text</summary><label>Note title<input id="hq-note-title" value="New note"></label><label>Note text<textarea id="hq-note-text"></textarea></label><button id="hq-add-note">Add Neurite note</button><label>Import text file<input id="hq-import" type="file" accept=".txt,.md,.csv,.json"></label><button id="hq-add-agent">Add agent</button></details><label>Agent<select id="hq-agent"></select></label><details><summary>Connect text context</summary><div id="hq-connect-list"></div><button id="hq-connect">Connect checked notes to agent</button></details><label>Task<textarea id="hq-prompt" placeholder="What should this agent do with its connected text notes?"></textarea></label><button id="hq-preview">Preview exact context</button><button id="hq-run" disabled>Run this task</button><details id="hq-context"><summary>Prompt and connected sources</summary><pre></pre></details><div id="hq-runs" aria-label="Persistent agent runs"></div><dialog id="hq-result"><button id="hq-close-result">Close result</button><h3></h3><pre class="hq-output"></pre><details><summary>Source snapshots and provenance</summary><pre class="hq-provenance"></pre></details><button id="hq-keep">Keep result as Neurite note</button></dialog></details>`;
  document.body.append(section);
  for (const type of ["mousedown", "pointerdown", "keydown", "keyup", "wheel"])
    section.addEventListener(type, (e) => e.stopPropagation());
  const el = (id) => section.querySelector("#hq-" + id);
  let revision = 0,
    token = "",
    preview = null,
    ready = false,
    saving = Promise.resolve(),
    saveTimer,
    currentResult = null,
    dirty = false,
    editGeneration = 0;
  const status = (message) => (el("host-status").textContent = message);
  const api = async (path, data) => {
    const r = await fetch("/hq-api/" + path, {
      method: data === undefined ? "GET" : "POST",
      headers:
        data === undefined
          ? {}
          : { "Content-Type": "application/json", "X-HQ-Session": token },
      body: data === undefined ? undefined : JSON.stringify(data),
    });
    const result = await r.json();
    if (!r.ok) throw Error(result.error);
    return result;
  };
  const report = (e) =>
    status("Action failed: " + e.message + " · browser draft retained.");
  const act = (fn) => async () => {
    try {
      await fn();
    } catch (e) {
      report(e);
    }
  };
  function listAgents() {
    const select = el("agent"),
      prior = select.value;
    const agents = [];
    Graph.forEachNode((n) => {
      if (n.isLLM) agents.push(n);
    });
    select.replaceChildren(
      ...agents.map((n) => new Option(n.getTitle() || n.uuid, n.uuid)),
    );
    if (agents.some((n) => n.uuid === prior)) select.value = prior;
    const holder = el("connect-list"),
      checked = new Set(
        [...holder.querySelectorAll("input:checked")].map((e) => e.value),
      );
    const labels = [];
    Graph.forEachNode((n) => {
      if (!n.isTextNode) return;
      const label = document.createElement("label"),
        input = document.createElement("input");
      input.type = "checkbox";
      input.value = n.uuid;
      input.checked = checked.has(n.uuid);
      input.setAttribute("aria-label", "Context: " + n.getTitle());
      label.append(input, document.createTextNode(n.getTitle()));
      labels.push(label);
    });
    holder.replaceChildren(...labels);
  }
  function addNote(title, text) {
    let unique = title || "Untitled note",
      i = 1;
    const titles = new Set();
    Graph.forEachNode((n) => titles.add(n.getTitle()));
    while (titles.has(unique)) unique = title + " (" + ++i + ")";
    return createNodeFromWindow(unique, text);
  }
  async function exportBase64() {
    const blob = await App.viewGraphs.nativeExport();
    if (blob.size > 9 * 1024 * 1024)
      throw Error(
        "Workspace exceeds this build’s 9 MB limit. Use Neurite export to retain it.",
      );
    const bytes = new Uint8Array(await blob.arrayBuffer());
    let binary = "";
    for (let i = 0; i < bytes.length; i += 16384)
      binary += String.fromCharCode(...bytes.subarray(i, i + 16384));
    return btoa(binary);
  }
  function save() {
    const next = saving
      .catch(() => {})
      .then(async () => {
        const generation = editGeneration;
        const bundle = await exportBase64();
        const saved = await api("workspace", { revision, bundle });
        revision = saved.revision;
        if (generation === editGeneration) dirty = false;
        status("Saved native workspace · revision " + revision);
      });
    saving = next;
    return next;
  }
  async function restore() {
    await saving.catch(() => {});
    const saved = await api("workspace");
    revision = saved.revision;
    if (saved.bundle) {
      await App.viewGraphs.nativeExport();
      const bytes = Uint8Array.from(atob(saved.bundle), (c) => c.charCodeAt(0));
      await App.viewGraphs.nativeImport(
        new File([bytes], "HQ native workspace.txt"),
      );
    }
    dirty = false;
    listAgents();
    status(
      "Opened native workspace · revision " +
        revision +
        " · prior browser draft retained in Neurite saves.",
    );
  }
  async function context() {
    const agent = Node.byUuid(el("agent").value);
    if (!agent?.isLLM) throw Error("Add or choose an existing Neurite agent.");
    const connected = await agent.getAllConnectedNodesData(true);
    const text = connected.filter(
      (info) => !info.isLLM && info.data?.type === "text",
    );
    const unsupported = connected.filter(
      (info) => !info.isLLM && info.data && info.data.type !== "text",
    );
    if (unsupported.length)
      throw Error(
        "This worker supports text context only. Other connected media or links need their native Neurite provider path.",
      );
    return {
      agentId: agent.uuid,
      prompt: el("prompt").value,
      sources: text.map((info) => ({
        id: info.node.uuid,
        name: info.node.getTitle(),
        text: info.data.data,
        kind: "text",
      })),
    };
  }
  function invalidate() {
    preview = null;
    el("run").disabled = true;
  }
  el("save").onclick = act(save);
  el("restore").onclick = act(async () => {
    await restore();
    invalidate();
  });
  el("settings").onclick = act(async () => {
    await api("settings", {
      kind: el("provider").value,
      model: el("model").value,
    });
    invalidate();
    status("Local model settings saved.");
  });
  el("add-note").onclick = act(async () => {
    addNote(el("note-title").value, el("note-text").value);
    await save();
    listAgents();
    invalidate();
  });
  el("import").onchange = act(async () => {
    const file = el("import").files[0];
    if (!file) return;
    if (file.size > 256000) throw Error("Text import is limited to 256 KB.");
    const text = new TextDecoder("utf-8", { fatal: true }).decode(
      await file.arrayBuffer(),
    );
    if (text.includes("\0")) throw Error("Choose a UTF-8 text file.");
    addNote(file.name, text);
    await save();
    listAgents();
    invalidate();
  });
  el("add-agent").onclick = act(async () => {
    createLlmNode("Companion");
    listAgents();
    await save();
  });
  el("preview").onclick = act(async () => {
    await save();
    preview = await api("preview", { context: await context() });
    el("context").querySelector("pre").textContent = JSON.stringify(
      preview,
      null,
      2,
    );
    el("context").open = true;
    el("run").disabled = false;
    status(
      "Preview ready · " +
        preview.context.sources.length +
        " connected text sources.",
    );
  });
  el("run").onclick = act(async () => {
    if (!preview) throw Error("Preview context first.");
    await save();
    const fresh = await api("preview", { context: await context() });
    if (
      fresh.hash !== preview.hash ||
      JSON.stringify(fresh.provider) !== JSON.stringify(preview.provider)
    ) {
      invalidate();
      throw Error("Context or provider changed. Preview again.");
    }
    await api("runs", { revision, context: fresh.context, hash: preview.hash });
    invalidate();
    await refresh();
    status("Queued in native host. You can close this browser view.");
  });
  el("connect").onclick = act(async () => {
    const agent = Node.byUuid(el("agent").value);
    if (!agent?.isLLM) throw Error("Choose an agent first.");
    const ids = [...el("connect-list").querySelectorAll("input:checked")].map(
      (e) => e.value,
    );
    if (!ids.length) throw Error("Check at least one text note.");
    for (const id of ids) {
      const note = Node.byUuid(id);
      if (note?.isTextNode) connectNodes(note, agent);
    }
    await save();
    invalidate();
    status("Connected " + ids.length + " notes using Neurite edges.");
  });
  el("agent").onchange = invalidate;
  el("prompt").oninput = () => {
    invalidate();
    try {
      localStorage.setItem("hq-native-task-draft", el("prompt").value);
    } catch {}
  };
  try {
    el("prompt").value = localStorage.getItem("hq-native-task-draft") || "";
  } catch {}
  el("close-result").onclick = () => el("result").close();
  el("keep").onclick = act(async () => {
    if (currentResult?.result) {
      addNote("Agent result", currentResult.result);
      await save();
      el("result").close();
    }
  });
  async function refresh() {
    const runs = await api("runs");
    el("runs").replaceChildren(
      ...runs.map((run) => {
        const row = document.createElement("article");
        const label = document.createElement("span");
        label.textContent =
          (Node.byUuid(run.context.agentId)?.getTitle() ||
            run.context.agentId) +
          " · " +
          run.status;
        row.append(label);
        for (const name of ["Locate", "Open", "Cancel"]) {
          const button = document.createElement("button");
          button.textContent = name;
          button.disabled =
            name === "Cancel" &&
            !["queued", "running", "waiting"].includes(run.status);
          button.onclick = act(async () => {
            if (name === "Cancel") {
              await api("cancel", { id: run.id });
              await refresh();
            } else if (name === "Locate") {
              const node = Node.byUuid(run.context.agentId);
              if (!node)
                throw Error(
                  "Agent is absent from this workspace. Its receipt remains available.",
                );
              App.selectedNodes.clear();
              App.selectedNodes.restoreNodeById(node.uuid);
              Animation.zoomToNodeTitle(node);
            } else {
              currentResult = run;
              el("result").querySelector("h3").textContent =
                "Result · " + run.status;
              el("result").querySelector(".hq-output").textContent =
                run.result || run.error || "Task is " + run.status;
              el("result").querySelector(".hq-provenance").textContent =
                JSON.stringify(run, null, 2);
              el("keep").disabled = !run.result;
              el("result").showModal();
            }
          });
          row.append(button);
        }
        return row;
      }),
    );
  }
  try {
    token = (await api("session")).session;
    const health = await api("health");
    el("provider").value = health.provider.kind;
    el("model").value = health.provider.model;
    await restore();
    ready = true;
    section.dataset.ready = "true";
    await refresh();
    if (!health.worker)
      status("Native worker missing. Build or install the runtime package.");
  } catch (e) {
    report(e);
  }
  section.querySelector("details").addEventListener("toggle", listAgents);
  for (const event of ["input", "change", "pointerup", "keyup"])
    document.addEventListener(event, (e) => {
      if (!ready || section.contains(e.target)) return;
      dirty = true;
      editGeneration++;
      invalidate();
      clearTimeout(saveTimer);
      saveTimer = setTimeout(() => save().catch(report), 1800);
    });
  window.addEventListener("beforeunload", (e) => {
    if (dirty) {
      e.preventDefault();
      e.returnValue = "";
    }
  });
  setInterval(() => {
    if (ready)
      refresh().catch(() =>
        status("Native host unavailable. Workspace draft retained."),
      );
  }, 2000);
})();
