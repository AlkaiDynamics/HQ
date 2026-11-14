# Native Neurite bridge — review checkpoint

This branch serves **actual Neurite**, pinned at `e62b270402b688a39a864007b9c0a02711b9573e`. Its fractal renderer, graph, node types, links, Zettelkasten and normal menus remain the application. The earlier `ui/` workbench remains reference material. It is not the served HQ environment.

The bridge adds a collapsible **HQ · Native activity** control, a local Node.js process host with SQLite, and supervised Rust inference workers using the existing provider gateway. The renderer does not own execution or committed storage. There is no new frontend framework or graph schema replacing Neurite.

## Run a packaged build

Unzip the complete platform artifact. On Windows, double-click **Start-HQ.cmd**. On Linux, run **Start-HQ.sh**. The package includes the Node runtime, Rust worker and pinned Neurite source. Keep the host window running when closing the browser view. Work does not survive turning off the host computer; interrupted work becomes **unknown**, with no automatic resubmission.

Start an installed local model service, such as Ollama or LM Studio. In **HQ · Native activity → Local model settings**, choose the provider and enter the exact installed model name. The defaults are Ollama at `127.0.0.1:11434`, or an OpenAI-compatible local server at `127.0.0.1:1234/v1`. This uses your local inference service; it does not require a paid API. Trusted host environment variables `HQ_OLLAMA_URL` and `HQ_COMPATIBLE_URL` can configure those endpoints. Credentials and endpoints cannot be changed by task text.

1. Add Neurite notes normally, or expand **Add notes or import text**. UTF-8 files become ordinary Neurite text nodes with their original filename as title.
2. Add or choose an agent. In **Connect text context**, check the notes and click **Connect checked notes to agent**. This calls Neurite's existing connection mechanism.
3. Enter a task and click **Preview exact context**. Inspect the frozen source text and prompt. **Run this task** rejects a changed preview rather than silently using different content.
4. Close the view while the host continues. Reopen HQ and use **Locate**, **Open** or **Cancel** in global activity. The result dialog shows frozen sources, provider/model, timestamps, workspace revision/hash and exact request messages. **Keep result as Neurite note** reuses Neurite's note/Zettelkasten creation path.

Generated text requires review against its sources. The 0.5B model tested here misidentified an owner. The 1.5B model answered the same sample correctly; that is a single observed task, not an accuracy guarantee.

## Storage and recovery

Native data is in `.hq/native/workspace.sqlite`. SQLite uses WAL and synchronous FULL. Workspace saves are optimistic revision transactions; the original binary Neurite export, including its native graph/view serialization, is stored without inventing a replacement graph model. Identical exports are deduplicated. Enqueue atomically binds a frozen context to a committed workspace revision. Receipts and state transitions use the same database. Native Rust kernel receipts are also retained under `kernel-receipts/`.

Only one host may own a database. A second host fails before reconciling another host's runs. Startup checks database integrity and export hashes and fails closed on damage. Running tasks interrupted by a restart become unknown, while queued jobs remain eligible to start. Cancellation wins over late results; provider-side stopping is best effort. No unknown task is automatically retried.

Neurite's existing browser saves remain. Loading the native workspace first exports the current browser graph into Neurite's save store, preserving that draft. Editor task text is also retained locally. Explicit native saves and a debounced save after user edits update the committed version. Close warnings identify pending edits. Browser storage failure, media-heavy exports, full disk and forced shutdown during an edit still require additional fault testing.

Back up the whole `.hq/native` folder with the host stopped. Do not discard SQLite `-wal` files from a live copy.

## Development and validation

Requires Node.js 24 and stable Rust:

```sh
git submodule update --init --recursive
npm ci
cargo build --release -p neurite-app --locked
npm start
```

Checks:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
npm test
npx prettier --check native-bridge/*.mjs native-bridge/client.js native-bridge/client.css package.json
npx playwright install --with-deps chromium
npm run test:browser
```

`server.test.mjs` verifies real HTTP → actual Rust child → a clearly labelled HTTP fixture → durable receipt → host reopen. It does not pretend the fixture is a model. `live-provider.mjs` separately launches a configured local llama.cpp server and official Qwen GGUF weights, checks the answer, and verifies receipt reopening. Test executables/weights are external inputs selected through `HQ_TEST_LLAMA_BIN`, `HQ_TEST_GGUF` and `HQ_TEST_MODEL`; they are not distributed in HQ.

`browser-smoke.mjs` runs actual pinned Neurite, creates/imports notes through controls, connects sources through Neurite edges, uses a fixture provider, closes the view during work, inspects provenance, retains the result and reopens from a fresh browser profile after host restart. Local graphical verification is blocked in this environment by Chromium socket restrictions; the separate headless-browser download also failed. CI must provide the graphical evidence. Unexecuted browser code is not an accepted user journey.

## Open gates

- Full browser journey and Windows installation/use validation.
- Genuine 3D nested frames, deep-scale picking, layout locks and Android continuity. This checkpoint preserves Neurite's existing fractal space; it does not falsely label that as a new 3D renderer.
- Offline bundling and compatibility checks for Neurite's existing external CDN dependencies. Initial startup currently needs those dependencies reachable.
- Broad Neurite feature/media parity, comprehensive crash/disk/network fault injection and performance/accessibility validation.
- Rich programmable definitions/edges/ticks, native app/CLI surfaces, scheduling and full plugin recovery.

Jev remains an optional decision-policy adapter in the existing gateway and is not required for this text-generation workflow. No Jev service was invoked by this bridge.

The tiny source hook is applied only to the exact pinned Neurite saver (LF-normalized SHA-256 checked). It exposes native export/import methods and an awaitable startup boundary. An unrecognized saver fails closed. Submodule files are never modified by the host. Upstream LICENSE and package metadata are preserved; the pre-existing MIT/ISC metadata discrepancy remains recorded in the forensic baseline.
