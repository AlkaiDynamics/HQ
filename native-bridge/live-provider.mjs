import { spawn } from "node:child_process";
import {
  createWriteStream,
  mkdtempSync,
  writeFileSync,
  readFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { startHost } from "./server.mjs";
import assert from "node:assert/strict";
const envDir = resolve("..", "inference-test"),
  model = process.env.HQ_TEST_GGUF || join(envDir, "qwen.gguf"),
  exe =
    process.env.HQ_TEST_LLAMA_BIN ||
    join(envDir, "llama-b12375", "llama-server"),
  evidence = resolve("test-evidence");
const { mkdirSync } = await import("node:fs");
mkdirSync(evidence, { recursive: true });
const provider = spawn(exe, [
  "-m",
  model,
  "--alias",
  process.env.HQ_TEST_MODEL || "qwen2.5-0.5b-instruct",
  "--host",
  "127.0.0.1",
  "--port",
  "1234",
  "--ctx-size",
  "2048",
  "--parallel",
  "2",
  "--n-predict",
  "96",
  "--threads",
  "2",
  "--no-warmup",
  "--temp",
  "0",
]);
const log = createWriteStream(join(evidence, "live-provider.log"));
provider.stdout.pipe(log);
provider.stderr.pipe(log);
let host;
async function wait(fn, ms = 60000) {
  const end = Date.now() + ms;
  let last;
  while (Date.now() < end) {
    try {
      const r = await fn();
      if (r) return r;
    } catch (e) {
      if (e.fatal) throw e;
      last = e;
    }
    await new Promise((r) => setTimeout(r, 200));
  }
  throw Error("Timed out " + last);
}
try {
  await wait(async () => {
    const r = await fetch("http://127.0.0.1:1234/health");
    return r.ok;
  }, 20000);
  const dir = mkdtempSync(join(tmpdir(), "hq-live-"));
  host = await startHost({ port: 0, data: dir });
  let session = (await (await fetch(host.url + "/hq-api/session")).json())
    .session;
  const api = async (p, b) => {
    const r = await fetch(host.url + "/hq-api/" + p, {
      method: b ? "POST" : "GET",
      headers: b
        ? { "Content-Type": "application/json", "X-HQ-Session": session }
        : {},
      body: b ? JSON.stringify(b) : undefined,
    });
    const j = await r.json();
    if (!r.ok) throw Error(j.error);
    return j;
  };
  await api("settings", {
    kind: "openai",
    model: process.env.HQ_TEST_MODEL || "qwen2.5-0.5b-instruct",
  });
  const w = await api("workspace", {
    revision: 0,
    bundle: Buffer.from("API live-test bundle fixture\0").toString("base64"),
  });
  const context = {
    agentId: "live-agent",
    prompt:
      "Using the selected note, name the person responsible for the Friday release. Answer in one short sentence and cite the source ID.",
    sources: [
      {
        id: "release-note",
        name: "Release brief",
        text: "Maya is responsible for the Friday release. Noah verifies cancellation.",
      },
    ],
  };
  const preview = await api("preview", { context });
  const run = await api("runs", {
    revision: w.revision,
    context,
    hash: preview.hash,
  });
  const done = await wait(async () => {
    const r = (await api("runs")).find((x) => x.id === run.id);
    if (["failed", "unknown"].includes(r.status))
      throw Object.assign(Error(JSON.stringify(r)), { fatal: true });
    return r.status === "completed" && r;
  });
  writeFileSync(
    join(evidence, "live-provider-last-run.json"),
    JSON.stringify(done, null, 2),
  );
  assert(done.result.includes("Maya"));
  assert.equal(
    done.receipt.selected.model,
    process.env.HQ_TEST_MODEL || "qwen2.5-0.5b-instruct",
  );
  await host.close();
  host = await startHost({ port: 0, data: dir });
  session = (await (await fetch(host.url + "/hq-api/session")).json()).session;
  assert.equal((await api("runs"))[0].result, done.result);
  writeFileSync(
    join(evidence, "live-provider-result.json"),
    JSON.stringify(
      {
        verifiedAt: new Date().toISOString(),
        provider: "llama.cpp b12375 CPU, actual Qwen2.5 GGUF; no mock model",
        scope:
          "API -> Rust worker -> real local inference -> durable receipt -> host restart; workspace bundle is a fixture, browser journey tested separately",
        receipt: done,
      },
      null,
      2,
    ),
  );
  console.log("REAL INFERENCE AND RESTART PASSED: " + done.result);
} finally {
  await host?.close();
  provider.kill();
}
