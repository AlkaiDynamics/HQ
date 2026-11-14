import { DatabaseSync } from "node:sqlite";
import { createHash, randomUUID } from "node:crypto";
import { mkdirSync } from "node:fs";
import { dirname } from "node:path";
export const digest = (value) =>
  createHash("sha256").update(value).digest("hex");
export function normalizeContext(c) {
  if (
    !c ||
    typeof c.agentId !== "string" ||
    !c.agentId ||
    c.agentId.length > 200 ||
    typeof c.prompt !== "string" ||
    !c.prompt.trim() ||
    !Array.isArray(c.sources) ||
    !c.sources.length ||
    c.sources.length > 100
  )
    throw Error("Choose an agent, a prompt and at least one text source.");
  const ids = new Set();
  const sources = c.sources
    .map((s) => {
      if (
        !s ||
        typeof s.id !== "string" ||
        !s.id ||
        ids.has(s.id) ||
        typeof s.name !== "string" ||
        typeof s.text !== "string" ||
        s.id.length > 200 ||
        s.name.length > 500
      )
        throw Error("Invalid or duplicate source.");
      ids.add(s.id);
      return { id: s.id, name: s.name, kind: "text", text: s.text };
    })
    .sort((a, b) => a.id.localeCompare(b.id));
  const result = { agentId: c.agentId, prompt: c.prompt, sources };
  if (Buffer.byteLength(JSON.stringify(result)) > 48000)
    throw Error("Selected context exceeds 48 KB. Narrow the linked notes.");
  return result;
}
export const contextDigest = (c) => digest(JSON.stringify(normalizeContext(c)));
export class Store {
  constructor(path) {
    this.path = path;
    mkdirSync(dirname(path), { recursive: true });
    this.db = new DatabaseSync(path);
    this.db.exec(
      "PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; PRAGMA busy_timeout=5000;",
    );
    if (this.db.prepare("PRAGMA quick_check").get().quick_check !== "ok")
      throw Error("Database integrity check failed; original data retained.");
    this.db.exec(
      "CREATE TABLE IF NOT EXISTS workspace(revision INTEGER PRIMARY KEY,bundle TEXT NOT NULL,sha256 TEXT NOT NULL,created INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS runs(id TEXT PRIMARY KEY,status TEXT NOT NULL,body TEXT NOT NULL,created INTEGER NOT NULL);",
    );
  }
  acquireHost() {
    this.db.exec(
      "CREATE TABLE IF NOT EXISTS host_lease(id INTEGER PRIMARY KEY,pid INTEGER NOT NULL,token TEXT NOT NULL)",
    );
    return this.atomic(() => {
      const row = this.db.prepare("SELECT * FROM host_lease WHERE id=1").get();
      if (row) {
        let alive = true;
        try {
          process.kill(row.pid, 0);
        } catch (e) {
          alive = e.code !== "ESRCH";
        }
        if (alive) throw Error("A native host already owns this database.");
      }
      const token = randomUUID();
      this.db
        .prepare(
          "INSERT INTO host_lease VALUES(1,?,?) ON CONFLICT(id) DO UPDATE SET pid=excluded.pid,token=excluded.token",
        )
        .run(process.pid, token);
      return token;
    });
  }
  releaseHost(token) {
    this.db.prepare("DELETE FROM host_lease WHERE token=?").run(token);
  }
  close() {
    this.db.close();
  }
  atomic(fn) {
    this.db.exec("BEGIN IMMEDIATE");
    try {
      const out = fn();
      this.db.exec("COMMIT");
      return out;
    } catch (e) {
      this.db.exec("ROLLBACK");
      throw e;
    }
  }
  workspace() {
    const row = this.db
      .prepare("SELECT * FROM workspace ORDER BY revision DESC LIMIT 1")
      .get();
    if (!row) return { revision: 0, bundle: null };
    if (digest(row.bundle) !== row.sha256)
      throw Error("Workspace checksum failed; refusing to overwrite.");
    return row;
  }
  save(expected, bundle) {
    return this.atomic(() => {
      const prior = this.workspace();
      if (prior.revision !== expected)
        throw Error("conflict: reload the saved workspace before writing");
      if (
        typeof bundle !== "string" ||
        bundle.length > 12 * 1024 * 1024 ||
        !bundle.length ||
        !/^[A-Za-z0-9+/]*={0,2}$/.test(bundle) ||
        Buffer.from(bundle, "base64").toString("base64") !== bundle
      )
        throw Error("Invalid Neurite export; maximum 9 MB.");
      const sha = digest(bundle);
      if (sha === prior.sha256) return prior;
      const revision = prior.revision + 1;
      this.db
        .prepare("INSERT INTO workspace VALUES(?,?,?,?)")
        .run(revision, bundle, sha, Date.now());
      return this.workspace();
    });
  }
  enqueue(revision, context, previewHash, provider = {}) {
    return this.atomic(() => {
      const current = this.workspace();
      if (!current.bundle || current.revision !== revision)
        throw Error("conflict: save the workspace before running");
      const normalized = normalizeContext(context),
        sha = contextDigest(normalized);
      if (sha !== previewHash)
        throw Error("Context changed: preview again before running.");
      if (
        this.db
          .prepare(
            "SELECT count(*) AS n FROM runs WHERE status IN ('queued','running')",
          )
          .get().n >= 20
      )
        throw Error("Queue is full; wait or cancel a task.");
      const run = {
        id: randomUUID(),
        status: "queued",
        workspaceRevision: revision,
        workspaceSha256: current.sha256,
        context: normalized,
        contextSha256: sha,
        provider,
        created: Date.now(),
      };
      this.db
        .prepare("INSERT INTO runs VALUES(?,?,?,?)")
        .run(run.id, run.status, JSON.stringify(run), run.created);
      return run;
    });
  }
  run(id) {
    const row = this.db.prepare("SELECT body FROM runs WHERE id=?").get(id);
    if (!row) throw Error("Run not found.");
    return JSON.parse(row.body);
  }
  runs() {
    return this.db
      .prepare("SELECT body FROM runs ORDER BY created DESC")
      .all()
      .map((r) => JSON.parse(r.body));
  }
  transition(id, from, status, patch = {}) {
    return this.atomic(() => {
      const run = this.run(id);
      if (!from.includes(run.status)) return false;
      const next = {
        ...run,
        ...patch,
        id: run.id,
        status,
        updated: Date.now(),
      };
      this.db
        .prepare("UPDATE runs SET status=?,body=? WHERE id=?")
        .run(status, JSON.stringify(next), id);
      return true;
    });
  }
  reconcile() {
    for (const r of this.runs())
      if (r.status === "running")
        this.transition(r.id, ["running"], "unknown", {
          error:
            "Host stopped during execution. Check the outcome before creating a new run. No automatic replay.",
        });
  }
}
