# How to package a Node Connector without bun

Question (issue #21): if a Connector runs on Node, how do we ship it with byo-chat on Windows x64 and Linux x64? Does InfiniteAPI run as a Node single executable application (SEA)?

Date checked: 2026-11-20.

## 1. What InfiniteAPI needs

Source: `package.json` of https://github.com/rsalcara/InfiniteAPI (default branch, read on 2026-11-20).

- The package is named `baileys`, version `7.0.0-rc.13`. It is a Baileys library, not a program. We must write a small Node entry script that speaks the Bridge Protocol and calls it.
- `"type": "module"`: the code is ESM. `engines.node` is `>=20.0.0`.
- Runtime `dependencies`: `@cacheable/node-cache`, `@hapi/boom`, `async-mutex`, `fflate`, `libsignal`, `lru-cache`, `music-metadata`, `p-queue`, `pino`, `prom-client`, `protobufjs`, `whatsapp-rust-bridge@0.5.4`, `ws`.
- None of these ship a native addon (`.node` file):
  - `libsignal@6` depends only on `protobufjs` and `curve25519-js` (pure JS). Source: `npm view libsignal dependencies`.
  - `whatsapp-rust-bridge@0.5.4` is "powered by Rust and WebAssembly": it ships a `.wasm` file, ESM only (`exports["."].import`), 5 files, about 2 MB. Source: `npm view whatsapp-rust-bridge@0.5.4`.
  - `music-metadata` is pure JS ESM.
- Native code is only in **optional peer** dependencies: `sharp` (images, native), `better-sqlite3` (native), `@roamhq/wrtc` (native), `jimp` (pure JS), `audio-decode`, `link-preview-js`, `qrcode-terminal`. Baileys works without them. If we want thumbnails, `jimp` is the pure-JS choice.

Conclusion: the core of InfiniteAPI has no native addon. It can be bundled into one file. The `.wasm` of `whatsapp-rust-bridge` must be carried as an asset or inlined by the bundler.

## 2. Options

### A. Node SEA (single executable application)

Source: https://nodejs.org/api/single-executable-applications.html.

- Status: "Stability: 1.1 - Active development". Tested on CI for Windows and Linux (all architectures Node supports except s390x; not Alpine).
- Build: since v25.5.0, `node --build-sea sea-config.json`. On Node 22/24 the older flow is used: `node --experimental-sea-config`, then copy the `node` binary and inject the blob with `postject`.
- ESM: the config field `"mainFormat": "module"` makes the main script ESM (cannot be combined with `useSnapshot`). This field is documented in the current docs; check that the Node line we pin has it. On Node 22 the main script is CommonJS only, so the bundler must output CJS (esbuild can, but top-level `await` in ESM code then breaks).
- Module loading: `require`/`import` in the main script only load built-in modules. Everything must be bundled into one script (esbuild/rollup), or loaded from the SEA virtual file system (VFS, newer Node).
- Native addons: cannot load from the VFS. They must be added as `assets`, written to a temp file and loaded with `process.dlopen()`. Works, but it is extra code, and temp files on Windows may be blocked by antivirus. Not needed for InfiniteAPI core.
- WASM: carry it as an asset and read it with `node:sea` `getRawAsset()`, or inline it in the bundle.
- Size: the whole `node` binary plus the bundle. The local `node.exe` 22.22.2 is 83 MB; a SEA is about 85-95 MB per platform. No tree shaking of the runtime.
- Update path: rebuild the SEA for each Node security release and each Connector release. One file to replace.
- Code signing: the docs say to remove the existing signature before injecting (`signtool remove /s` on Windows), then sign the result: `signtool sign /fd SHA256 sea.exe` (Windows). Linux binaries are not signed by the OS; we use our own checksum/signature from the release.

### B. Bundled Node runtime plus a JS bundle

- Ship the official `node` binary (from https://nodejs.org/dist/, with `SHASUMS256.txt` signed by the Node release keys) next to a bundled `connector.mjs` (and the `.wasm`).
- ESM: full, no restrictions. Native addons: normal `require` of a `.node` file next to it, no temp file.
- Size: the same runtime cost (about 83 MB `node.exe`, a similar size on Linux; the archives compress to about 30 MB), plus a bundle of a few MB.
- Update path: the runtime and the Connector bundle update separately. A Node security fix does not need a Connector rebuild. Several Node Connectors can share one runtime.
- Code signing: `node.exe` is already signed by the OpenJS Foundation on Windows; we keep its signature untouched. The `.mjs` file is not signed by the OS; our release checksum covers it.
- byo-chat starts it as `node connector.mjs` through `byo_connector::spawn`. The Manifest names the runtime and the entry file.

### C. Others

- **pkg (vercel)**: archived, no longer maintained. **`@yao-pkg/pkg`** is the community fork: it patches Node, supports ESM only partly (it transpiles to CJS), native addons by extracting to a temp folder. Another patched runtime to trust; no reason to pick it over SEA now.
- **Deno compile** (`deno compile --target x86_64-pc-windows-msvc|x86_64-unknown-linux-gnu`): cross-compiles, ESM native, binaries about 60-80 MB. Node-API addons work through `npm:` only partly, and Baileys on Deno's Node compatibility is untested. Risk: a second runtime with different bugs.
- **Bun compile**: excluded by the issue title.
- **nexe**: old, slow updates. Not recommended.

## 3. Summary table

| Option | Size (x64) | Native addons | ESM | Update path | Signing |
|---|---|---|---|---|---|
| Node SEA | ~85-95 MB, one file | Asset + temp file + `dlopen` | Node 22: CJS only; newer: `mainFormat: module` | Rebuild per Node or Connector release | Remove then re-sign (Windows) |
| Node runtime + bundle | ~83 MB runtime + few MB | Normal | Full | Runtime and Connector separate | Keep Node's signature; checksum the bundle |
| @yao-pkg/pkg | ~50-90 MB | Temp extraction | Partial | Wait for fork to patch Node | Re-sign |
| Deno compile | ~60-80 MB | Partial | Full | Rebuild per Deno release | Re-sign |

## 4. Does InfiniteAPI run as a SEA?

Yes, in principle: no native addons in its runtime dependencies, and the `.wasm` can be an asset. It needs a CJS bundle on Node 22, or `mainFormat: "module"` on a newer Node. Not yet tested by a build; a spike should build it and log in once.

## 5. Recommendation

Use option B: ship the official Node runtime (pin a LTS line, 24) plus one esbuild bundle per Connector. It has full ESM, plain native addon loading if an optional peer like `sharp` is ever needed, keeps Node's own signature, and lets a Node security fix ship without rebuilding Connectors. Keep SEA as a later option for a one-file download once `--build-sea` and `mainFormat: "module"` reach an LTS line. Avoid pkg forks and Deno for Baileys.
