import http from "node:http";
import { readFile, realpath } from "node:fs/promises";
import { existsSync, mkdirSync } from "node:fs";
import { resolve, join, extname, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { randomUUID } from "node:crypto";
import { Store, contextDigest, normalizeContext } from "./store.mjs";
import { patchSaveModule, UPSTREAM_COMMIT } from "./upstream-hooks.mjs";
const here = fileURLToPath(new URL(".", import.meta.url)),
  root = resolve(here, "..");
export async function startHost({
  port = 4317,
  data = join(root, ".hq/native"),
  upstream = join(root, "upstream/neurite"),
  worker = join(
    root,
    "target/release/neurite-app" + (process.platform === "win32" ? ".exe" : ""),
  ),
} = {}) {
  const saver = patchSaveModule(
    await readFile(join(upstream, "js/interface/dropdown/savenet.js"), "utf8"),
  );
  const assets = await realpath(upstream);
  mkdirSync(data, { recursive: true });
  const store = new Store(join(data, "workspace.sqlite"));
  const children = new Map(),
    session = randomUUID();
  let closed = false;
  const config = {
    kind: process.env.HQ_PROVIDER || "ollama",
    model: process.env.HQ_MODEL || "qwen2.5:0.5b",
  };
  store.db.exec(
    "CREATE TABLE IF NOT EXISTS settings(id INTEGER PRIMARY KEY,body TEXT NOT NULL)",
  );
  const saved = store.db.prepare("SELECT body FROM settings WHERE id=1").get();
  if (saved) Object.assign(config, JSON.parse(saved.body));
  let lease;
  try {
    lease = store.acquireHost();
  } catch (e) {
    store.close();
    throw e;
  }
  const endpoint = (kind) =>
    kind === "ollama"
      ? process.env.HQ_OLLAMA_URL || "http://127.0.0.1:11434"
      : process.env.HQ_COMPATIBLE_URL || "http://127.0.0.1:1234/v1";
  function pump() {
    if (closed) return;
    for (const run of store
      .runs()
      .filter((r) => r.status === "queued")
      .reverse()) {
      if (children.size >= 2) break;
      if (!existsSync(worker)) {
        store.transition(run.id, ["queued"], "failed", {
          error:
            "Native worker is missing. Build neurite-app or install the complete runtime package.",
        });
        continue;
      }
      if (
        !store.transition(run.id, ["queued"], "running", {
          started: Date.now(),
        })
      )
        continue;
      const child = spawn(worker, ["--agent-run-json"], {
        cwd: root,
        env: {
          ...process.env,
          HQ_PROVIDER: run.provider.kind,
          HQ_MODEL: run.provider.model,
          HQ_BASE_URL: run.provider.url,
          HQ_RECEIPTS_DIR: join(data, "kernel-receipts"),
        },
        stdio: ["pipe", "pipe", "pipe"],
      });
      children.set(run.id, child);
      let output = "",
        error = "",
        oversized = false;
      const stop = () => {
        oversized = true;
        child.kill();
      };
      child.stdout.on("data", (b) => {
        output += b;
        if (output.length > 1024 * 1024) stop();
      });
      child.stderr.on("data", (b) => {
        error = (error + b).slice(-4000);
      });
      child.stdin.on("error", () => {});
      const timer = setTimeout(() => {
        store.transition(run.id, ["running"], "unknown", {
          error:
            "Worker exceeded four minutes; provider outcome unknown. Not retried.",
        });
        child.kill();
      }, 240000);
      child.on("error", () => {
        clearTimeout(timer);
        children.delete(run.id);
        store.transition(run.id, ["running"], "failed", {
          error: "Could not start native worker.",
        });
        pump();
      });
      child.on("exit", (code) => {
        clearTimeout(timer);
        children.delete(run.id);
        try {
          const receipt = JSON.parse(output);
          if (
            receipt.run_id !== run.id ||
            !["succeeded", "failed"].includes(receipt.status)
          )
            throw Error("Invalid kernel receipt");
          store.transition(
            run.id,
            ["running"],
            receipt.status === "succeeded"
              ? "completed"
              : receipt.error_class === "exhausted" &&
                  receipt.attempts?.length &&
                  receipt.attempts.every((a) =>
                    ["transport", "rate_limited", "server"].includes(a.failure),
                  )
                ? "waiting"
                : "failed",
            { receipt, result: receipt.output || null, finished: Date.now() },
          );
        } catch {
          store.transition(
            run.id,
            ["running"],
            oversized ? "failed" : "unknown",
            {
              error: oversized
                ? "Worker output exceeded 1 MB."
                : "Worker exited without a complete receipt; outcome unknown. " +
                  (code === 0 ? "" : error),
              finished: Date.now(),
            },
          );
        }
        pump();
      });
      child.stdin.end(JSON.stringify({ id: run.id, ...run.context }));
    }
  }
  async function body(req) {
    let text = "";
    for await (const b of req) {
      text += b;
      if (Buffer.byteLength(text) > 13 * 1024 * 1024)
        throw Error("Request exceeds 13 MB.");
    }
    return JSON.parse(text);
  }
  function send(res, status, data, type = "application/json") {
    res.writeHead(status, {
      "Content-Type": type,
      "X-Content-Type-Options": "nosniff",
      "Cache-Control": "no-store",
      "Referrer-Policy": "no-referrer",
    });
    res.end(type === "application/json" ? JSON.stringify(data) : data);
  }
  const server = http.createServer(async (req, res) => {
    try {
      const actualPort = server.address().port,
        hosts = [`127.0.0.1:${actualPort}`, `localhost:${actualPort}`];
      if (
        !hosts.includes(req.headers.host) ||
        (req.headers.origin &&
          !hosts.some((h) => req.headers.origin === "http://" + h))
      )
        return send(res, 403, { error: "Foreign origin/host rejected." });
      const path = new URL(req.url, "http://127.0.0.1").pathname;
      if (path.startsWith("/hq-api/")) {
        if (req.method === "POST" && req.headers["x-hq-session"] !== session)
          return send(res, 403, { error: "Session token required." });
        const key = req.method + " " + path;
        if (key === "GET /hq-api/session") return send(res, 200, { session });
        if (key === "GET /hq-api/health")
          return send(res, 200, {
            host: "online",
            upstream: UPSTREAM_COMMIT,
            worker: existsSync(worker),
            provider: { ...config, url: endpoint(config.kind) },
            active: children.size,
          });
        if (key === "GET /hq-api/workspace")
          return send(res, 200, store.workspace());
        if (key === "POST /hq-api/workspace") {
          const b = await body(req);
          return send(res, 200, store.save(b.revision, b.bundle));
        }
        if (key === "GET /hq-api/runs") return send(res, 200, store.runs());
        if (key === "POST /hq-api/preview") {
          const b = await body(req),
            context = normalizeContext(b.context);
          return send(res, 200, {
            context,
            hash: contextDigest(context),
            provider: { ...config, url: endpoint(config.kind) },
          });
        }
        if (key === "POST /hq-api/runs") {
          const b = await body(req),
            run = store.enqueue(b.revision, b.context, b.hash, {
              ...config,
              url: endpoint(config.kind),
            });
          send(res, 202, run);
          pump();
          return;
        }
        if (key === "POST /hq-api/cancel") {
          const b = await body(req);
          store.transition(
            b.id,
            ["queued", "running", "waiting"],
            "cancelled",
            {
              finished: Date.now(),
              error:
                "Cancelled locally. Provider-side stopping is best effort.",
            },
          );
          children.get(b.id)?.kill();
          return send(res, 200, store.run(b.id));
        }
        if (key === "POST /hq-api/settings") {
          const b = await body(req);
          if (
            !["ollama", "openai"].includes(b.kind) ||
            typeof b.model !== "string" ||
            !b.model.trim() ||
            b.model.length > 128 ||
            /[\x00-\x1f]/.test(b.model)
          )
            throw Error("Choose a provider and installed model.");
          Object.assign(config, { kind: b.kind, model: b.model.trim() });
          store.db
            .prepare(
              "INSERT INTO settings VALUES(1,?) ON CONFLICT(id) DO UPDATE SET body=excluded.body",
            )
            .run(JSON.stringify(config));
          return send(res, 200, config);
        }
        return send(res, 404, { error: "Unknown operation." });
      }
      if (req.method !== "GET")
        return send(res, 405, { error: "Method not allowed." });
      if (path === "/hq-native.js")
        return send(
          res,
          200,
          await readFile(join(here, "client.js")),
          "text/javascript",
        );
      if (path === "/hq-native.css")
        return send(
          res,
          200,
          await readFile(join(here, "client.css")),
          "text/css",
        );
      if (path === "/js/interface/dropdown/savenet.js")
        return send(res, 200, saver, "text/javascript");
      if (path === "/" || path === "/index.html") {
        const index = await readFile(join(assets, "index.html"), "utf8");
        return send(
          res,
          200,
          index.replace(
            "</body>",
            '<link rel="stylesheet" href="/hq-native.css"><script src="/hq-native.js"></script></body>',
          ),
          "text/html",
        );
      }
      if (
        !/^\/(js|resources|public|wiki)\//.test(path) &&
        !["/favicon.ico", "/favicon.svg"].includes(path)
      )
        return send(res, 404, { error: "Asset not found." });
      if (path.split("/").some((p) => p.startsWith(".")))
        return send(res, 403, { error: "Private path rejected." });
      const file = await realpath(
        resolve(assets, "." + decodeURIComponent(path)),
      );
      if (!file.startsWith(assets + sep))
        return send(res, 403, { error: "Path rejected." });
      const mime = {
        ".js": "text/javascript",
        ".css": "text/css",
        ".html": "text/html",
        ".json": "application/json",
        ".svg": "image/svg+xml",
        ".png": "image/png",
        ".jpg": "image/jpeg",
        ".ico": "image/x-icon",
        ".woff2": "font/woff2",
        ".txt": "text/plain",
        ".mp4": "video/mp4",
        ".webm": "video/webm",
        ".webp": "image/webp",
      };
      send(
        res,
        200,
        await readFile(file),
        mime[extname(file)] || "application/octet-stream",
      );
    } catch (e) {
      if (!res.headersSent)
        send(
          res,
          e.code === "ENOENT"
            ? 404
            : String(e.message).startsWith("conflict:")
              ? 409
              : 400,
          { error: e.message },
        );
      else res.end();
    }
  });
  try {
    await new Promise((ok, no) => {
      server.once("error", no);
      server.listen(port, "127.0.0.1", ok);
    });
  } catch (e) {
    store.releaseHost(lease);
    store.close();
    throw e;
  }
  store.reconcile();
  const timer = setInterval(pump, 1000);
  pump();
  return {
    server,
    store,
    url: `http://127.0.0.1:${server.address().port}`,
    async close() {
      closed = true;
      clearInterval(timer);
      for (const child of children.values()) {
        child.kill();
        await new Promise((r) => child.once("exit", r));
      }
      await new Promise((r) => server.close(r));
      store.releaseHost(lease);
      store.close();
    },
  };
}
if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const host = await startHost({
    port: Number(process.env.HQ_PORT || 4317),
    ...(process.env.HQ_DATA_DIR
      ? { data: resolve(process.env.HQ_DATA_DIR) }
      : {}),
  });
  if (process.env.HQ_OPEN_BROWSER === "1") {
    const opener =
      process.platform === "win32"
        ? spawn("cmd.exe", ["/c", "start", "", host.url], { stdio: "ignore" })
        : spawn("xdg-open", [host.url], { stdio: "ignore" });
    opener.on("error", () =>
      console.log("Open " + host.url + " in your browser."),
    );
    opener.unref();
  }
  console.log(
    "HQ Neurite: " +
      host.url +
      " — leave this host running while tasks continue.",
  );
  process.once("SIGINT", () => host.close().then(() => process.exit()));
  process.once("SIGTERM", () => host.close().then(() => process.exit()));
}
