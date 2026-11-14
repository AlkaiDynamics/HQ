import { chromium } from "playwright";
import assert from "node:assert/strict";
import http from "node:http";
import { startHost } from "./server.mjs";
import { mkdtempSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
const evidence = resolve("test-evidence");
mkdirSync(evidence, { recursive: true });
const results = [],
  errors = [];
let host, browser, provider;
async function wait(fn, ms = 30000) {
  const end = Date.now() + ms;
  while (Date.now() < end) {
    const r = await fn();
    if (r) return r;
    await new Promise((r) => setTimeout(r, 100));
  }
  throw Error("Timed out waiting for workflow state.");
}
async function check(name, fn) {
  await fn();
  results.push({ name, status: "passed" });
  console.log("PASS " + name);
}
try {
  provider = http.createServer(async (req, res) => {
    let text = "";
    for await (const b of req) text += b;
    const body = JSON.parse(text);
    assert(body.messages.some((m) => m.content.includes("Maya")));
    setTimeout(() => {
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(
        JSON.stringify({
          choices: [
            {
              message: {
                role: "assistant",
                content:
                  "Fixture response: Maya owns Friday release; Noah tests cancellation.",
              },
            },
          ],
        }),
      );
    }, 2500);
  });
  await new Promise((r) => provider.listen(0, "127.0.0.1", r));
  process.env.HQ_COMPATIBLE_URL =
    "http://127.0.0.1:" + provider.address().port + "/v1";
  const data = mkdtempSync(join(tmpdir(), "hq-browser-"));
  host = await startHost({ port: 0, data });
  browser = await chromium.launch({
    ...(process.env.HQ_BROWSER_BIN
      ? { executablePath: process.env.HQ_BROWSER_BIN }
      : {}),
    headless: true,
    args: [
      "--no-sandbox",
      "--use-angle=swiftshader",
      "--enable-unsafe-swiftshader",
    ],
  });
  const context = await browser.newContext({
    viewport: { width: 1400, height: 950 },
  });
  let page = await context.newPage();
  page.on("pageerror", (e) => errors.push(e.message));
  const open = async () => {
    await page.goto(host.url, { waitUntil: "domcontentloaded" });
    await page.waitForSelector("#hq-native[data-ready=true]", {
      timeout: 60000,
    });
    await page.locator("#hq-native > details > summary").click();
  };
  await check(
    "Start actual pinned Neurite with fractal controls and native host",
    open,
  );
  await check(
    "Create note and import real UTF-8 file through controls",
    async () => {
      await page
        .locator("#hq-add-note")
        .evaluate((e) => (e.closest("details").open = true));
      await page.locator("#hq-note-title").fill("Release brief");
      await page.locator("#hq-note-text").fill("Maya owns the Friday release.");
      await page.locator("#hq-add-note").click();
      await wait(async () => {
        const saved = host.store.workspace();
        return saved.revision > 0;
      });
      await page.locator("#hq-import").setInputFiles({
        name: "meeting.md",
        mimeType: "text/markdown",
        buffer: Buffer.from(
          "Noah tests cancellation. Review the result before publishing.",
        ),
      });
      await wait(() => host.store.workspace().revision > 1);
      await page.locator("#hq-add-agent").click();
      await page.waitForFunction(
        () => document.querySelector("#hq-agent").options.length > 0,
      );
    },
  );
  await check(
    "Connect notes with native Neurite edges and preview exact context",
    async () => {
      await page
        .locator("#hq-connect")
        .evaluate((e) => (e.closest("details").open = true));
      await page.getByLabel("Context: Release brief", { exact: true }).check();
      await page.getByLabel("Context: meeting.md", { exact: true }).check();
      await page.locator("#hq-connect").click();
      await page
        .locator("#hq-settings")
        .evaluate((e) => (e.closest("details").open = true));
      await page.locator("#hq-provider").selectOption("openai");
      await page.locator("#hq-model").fill("fixture-model");
      await page.locator("#hq-settings").click();
      await page
        .locator("#hq-prompt")
        .fill("Who owns Friday release and who tests cancellation?");
      await page.locator("#hq-preview").click();
      await page.waitForFunction(
        () => !document.querySelector("#hq-run").disabled,
      );
      const preview = JSON.parse(
        await page.locator("#hq-context pre").innerText(),
      );
      assert.equal(preview.context.sources.length, 2);
      assert(preview.context.sources.some((s) => s.name === "meeting.md"));
    },
  );
  await check(
    "Run in Rust worker; close browser view while work continues",
    async () => {
      await page.locator("#hq-run").click();
      await wait(() => host.store.runs().length === 1);
      await page.close();
      await wait(() => host.store.runs()[0].status === "completed");
      assert.match(host.store.runs()[0].result, /Maya/);
    },
  );
  await check(
    "Reopen workspace, inspect provenance and retain result as a Neurite note",
    async () => {
      page = await context.newPage();
      page.on("pageerror", (e) => errors.push(e.message));
      await open();
      await page
        .locator("#hq-runs")
        .getByRole("button", { name: "Open", exact: true })
        .first()
        .click();
      assert.match(await page.locator(".hq-output").innerText(), /Maya/);
      await page.locator("#hq-result summary").click();
      assert.match(
        await page.locator(".hq-provenance").innerText(),
        /request_messages/,
      );
      await page.locator("#hq-keep").click();
      await wait(async () => {
        const w = host.store.workspace();
        return Buffer.from(w.bundle, "base64").includes(
          Buffer.from("Agent result"),
        );
      });
    },
  );
  await check("Cancel a second task from global activity", async () => {
    await page.locator("#hq-prompt").fill("Who owns Friday release?");
    await page.locator("#hq-preview").click();
    await page.waitForFunction(
      () => !document.querySelector("#hq-run").disabled,
    );
    await page.locator("#hq-run").click();
    await wait(() => host.store.runs().length === 2);
    await page
      .locator("#hq-runs article")
      .first()
      .getByRole("button", { name: "Cancel", exact: true })
      .click();
    await wait(() => host.store.runs()[0].status === "cancelled");
  });
  await check(
    "Restart host and open a fresh browser profile with retained notes, links and receipts",
    async () => {
      const before = host.store.workspace();
      await page.close();
      await context.close();
      await host.close();
      host = await startHost({ port: 0, data });
      assert.equal(host.store.workspace().bundle, before.bundle);
      const fresh = await browser.newContext({
        viewport: { width: 1400, height: 950 },
      });
      page = await fresh.newPage();
      page.on("pageerror", (e) => errors.push(e.message));
      await open();
      const count = await page.evaluate(() => {
        const list = [];
        Graph.forEachNode((n) =>
          list.push({ name: n.getTitle(), edges: n.edges.length }),
        );
        return list;
      });
      assert(count.some((n) => n.name === "Release brief" && n.edges > 0));
      assert(count.some((n) => n.name === "meeting.md"));
      assert(count.some((n) => n.name === "Agent result"));
      assert.equal(host.store.runs().length, 2);
    },
  );
  await check("No browser runtime errors", () => assert.deepEqual(errors, []));
} catch (e) {
  results.push({
    name: "workflow",
    status: "failed",
    error: String(e),
    browserErrors: errors,
  });
  console.error(e);
  process.exitCode = 1;
} finally {
  writeFileSync(
    join(evidence, "browser-results.json"),
    JSON.stringify(
      {
        date: new Date().toISOString(),
        provider: "HTTP fixture, not a model",
        results,
      },
      null,
      2,
    ),
  );
  await browser?.close();
  await host?.close();
  if (provider) await new Promise((r) => provider.close(r));
  delete process.env.HQ_COMPATIBLE_URL;
}
