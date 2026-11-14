import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { patchSaveModule } from "./upstream-hooks.mjs";
test("pinned Neurite saver gains an export/import seam without rewriting existing method bodies", () => {
  const source = readFileSync(
    new URL(
      "../upstream/neurite/js/interface/dropdown/savenet.js",
      import.meta.url,
    ),
    "utf8",
  );
  const normalized = source.replace(/\r\n/g, "\n");
  const patched = patchSaveModule(source);
  assert.equal(patchSaveModule(normalized.replace(/\n/g, "\r\n")), patched);
  assert(patched.includes("nativeExport"));
  assert(patched.includes("nativeImport"));
  assert(
    patched.includes(
      normalized.slice(
        normalized.indexOf("    #loadGraph(text, importer){"),
        normalized.indexOf("    #populateDirectionalityMap"),
      ),
    ),
  );
});
test("unrecognized upstream saver fails closed before applying a patch", () =>
  assert.throws(() => patchSaveModule("unknown upstream"), /pinned/));
