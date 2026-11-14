import { test } from "node:test";
import assert from "node:assert/strict";
import http from "node:http";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { startHost } from "./server.mjs";
const wait = async (fn) => {
  for (let i = 0; i < 150; i++) {
    const r = await fn();
    if (r) return r;
    await new Promise((r) => setTimeout(r, 50));
  }
  throw Error("Worker timed out.");
};
test("real HTTP bridge -> Rust process -> fixture provider -> persisted receipt -> host reopen", async () => {
  let providerDown = false;
  let providerCalls = 0,
    providerBody;
  const provider = http.createServer(async (req, res) => {
    let body = "";
    for await (const b of req) body += b;
    providerBody = JSON.parse(body);
    providerCalls++;
    if (providerDown) {
      res.writeHead(503, { "Content-Type": "application/json" });
      res.end('{"error":"fixture outage"}');
      return;
    }
    res.writeHead(200, { "Content-Type": "application/json" });
    res.end(
      JSON.stringify({
        choices: [
          {
            message: {
              role: "assistant",
              content: "Fixture response: Maya owns the Friday release.",
            },
          },
        ],
      }),
    );
  });
  await new Promise((r) => provider.listen(0, "127.0.0.1", r));
  const endpoint = "http://127.0.0.1:" + provider.address().port + "/v1";
  process.env.HQ_COMPATIBLE_URL = endpoint;
  const data = mkdtempSync(join(tmpdir(), "hq-wire-"));
  let host = await startHost({ port: 0, data });
  let token = (await (await fetch(host.url + "/hq-api/session")).json())
    .session;
  const call = async (path, body) => {
    const r = await fetch(host.url + "/hq-api/" + path, {
      method: body ? "POST" : "GET",
      headers: body
        ? { "Content-Type": "application/json", "X-HQ-Session": token }
        : {},
      body: body ? JSON.stringify(body) : undefined,
    });
    return { status: r.status, data: await r.json() };
  };
  try {
    const index = await fetch(host.url);
    assert((await index.text()).includes("hq-native.js"));
    assert.equal((await fetch(host.url + "/Cargo.toml")).status, 404);
    const denied = await fetch(host.url + "/hq-api/settings", {
      method: "POST",
      headers: {
        Origin: "https://untrusted.invalid",
        "Content-Type": "application/json",
      },
      body: "{}",
    });
    assert.equal(denied.status, 403);
    await call("settings", { kind: "openai", model: "fixture-model" });
    const saved = (
      await call("workspace", {
        revision: 0,
        bundle: Buffer.from("Neurite bundle fixture\0").toString("base64"),
      })
    ).data;
    const context = {
      agentId: "native-agent-1",
      prompt: "Who owns release?",
      sources: [
        {
          id: "native-note-1",
          name: "Release brief",
          text: "Maya owns Friday release.",
        },
      ],
    };
    const preview = (await call("preview", { context })).data;
    const queued = (
      await call("runs", {
        revision: saved.revision,
        context,
        hash: preview.hash,
      })
    ).data;
    const run = await wait(async () => {
      const r = (await call("runs")).data.find((r) => r.id === queued.id);
      if (r.status === "unknown" || r.status === "failed")
        throw Error(JSON.stringify(r));
      return r.status === "completed" && r;
    });
    assert.equal(providerCalls, 1);
    assert(
      providerBody.messages.some(
        (m) =>
          m.content.includes("native-note-1") && m.content.includes("Maya"),
      ),
    );
    assert.equal(run.receipt.selected.model, "fixture-model");
    assert.equal(run.context.sources[0].id, "native-note-1");
    assert.equal(run.result, "Fixture response: Maya owns the Friday release.");
    await host.close();
    host = await startHost({ port: 0, data });
    token = (await (await fetch(host.url + "/hq-api/session")).json()).session;
    assert.equal((await call("workspace")).data.bundle, saved.bundle);
    assert.equal((await call("runs")).data[0].result, run.result);
    assert.equal(providerCalls, 1);
    providerDown = true;
    const unavailable = (
      await call("runs", {
        revision: saved.revision,
        context,
        hash: preview.hash,
      })
    ).data;
    const failed = await wait(async () => {
      const r = (await call("runs")).data.find((x) => x.id === unavailable.id);
      return r.status !== "queued" && r.status !== "running" && r;
    });
    assert.equal(failed.status, "waiting");
    await host.close();
    host = await startHost({ port: 0, data });
    token = (await (await fetch(host.url + "/hq-api/session")).json()).session;
    assert.equal(
      (await call("runs")).data.find((x) => x.id === unavailable.id).status,
      "waiting",
    );
    assert.equal(providerCalls, 2);
    assert.equal(
      (await call("cancel", { id: unavailable.id })).data.status,
      "cancelled",
    );
  } finally {
    await host.close();
    await new Promise((r) => provider.close(r));
    delete process.env.HQ_COMPATIBLE_URL;
  }
});
