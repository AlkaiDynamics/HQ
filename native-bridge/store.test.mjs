import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { Store, contextDigest } from "./store.mjs";
const fresh = () =>
  new Store(
    join(mkdtempSync(join(tmpdir(), "hq-native-")), "workspace.sqlite"),
  );
const snapshot = Buffer.from(
  "actual Neurite export fixture\0binary bytes",
).toString("base64");
const context = {
  agentId: "neurite-agent",
  prompt: "Summarize the release.",
  sources: [
    {
      id: "note-uuid",
      name: "Release",
      kind: "text",
      text: "Owner Maya; Friday.",
    },
  ],
};
test("Neurite export bytes and workspace revisions survive reopen; stale mutation is atomic", () => {
  const s = fresh();
  assert.equal(s.workspace().revision, 0);
  s.save(0, snapshot);
  assert.throws(() => s.save(0, "changed"), /conflict/);
  const reopened = new Store(s.path);
  assert.equal(reopened.workspace().bundle, snapshot);
  assert.equal(reopened.workspace().revision, 1);
  s.close();
  reopened.close();
});
test("enqueue freezes exact context and committed revision; cancellation wins late completion", () => {
  const s = fresh();
  s.save(0, snapshot);
  const run = s.enqueue(1, context, contextDigest(context));
  context.sources[0].text = "Edited after enqueue";
  assert.equal(s.run(run.id).context.sources[0].text, "Owner Maya; Friday.");
  s.transition(run.id, ["queued"], "running");
  s.transition(run.id, ["running"], "cancelled");
  assert.equal(
    s.transition(run.id, ["running"], "completed", { result: "late" }),
    false,
  );
  assert.equal(s.run(run.id).status, "cancelled");
  s.close();
});
test("changed previews, detached revisions and malformed sources are rejected before enqueue", () => {
  const s = fresh();
  s.save(0, snapshot);
  assert.throws(() => s.enqueue(1, context, "wrong"), /preview/);
  assert.throws(
    () => s.enqueue(0, context, contextDigest(context)),
    /conflict/,
  );
  assert.throws(
    () => s.enqueue(1, { ...context, sources: [] }, "wrong"),
    /source/,
  );
  assert.deepEqual(s.runs(), []);
  s.close();
});
test("host interruption becomes unknown without resubmission; queued work remains queued", () => {
  const s = fresh();
  s.save(0, snapshot);
  const a = s.enqueue(1, context, contextDigest(context)),
    b = s.enqueue(1, context, contextDigest(context));
  s.transition(a.id, ["queued"], "running");
  s.reconcile();
  assert.equal(s.run(a.id).status, "unknown");
  assert.equal(s.run(b.id).status, "queued");
  s.close();
});
test("one native host owns a database; explicit release permits recovery", () => {
  const s = fresh();
  const lease = s.acquireHost();
  const other = new Store(s.path);
  assert.throws(() => other.acquireHost(), /already/);
  s.releaseHost(lease);
  const next = other.acquireHost();
  other.releaseHost(next);
  s.close();
  other.close();
});
