# Architecture

The map of byo-chat's architecture. It points to the decisions in [adr/](./adr/) and the words in [GLOSSARY.md](../GLOSSARY.md); it does not repeat them. When this file and an ADR disagree, the ADR is correct: fix this file.

## 1. How to use this file

- Before you add a crate, a dependency between crates, or a port: §3 and §4.
- Before you add a call to an outside system (a Connector binary, Chatwoot, a webhook, the OS keychain): §4 and §6.
- Before you add a path that carries data in from a less trusted zone, a secret, or a setting that turns a protection off: §6 and [SECURITY.md](../SECURITY.md).
- A rule here that blocks your change is answered by a decision: an ADR or an amendment.

## 2. Constraints and characteristics

**Constraints**, never traded:

- **Portability**: Windows and Linux, x64 and arm64, built and tested in CI.
- **Privacy**: personal messages stay on the user's machine, encrypted at rest (ADR-0008); nothing leaves except to Destinations the user configured.

**Characteristics**, ranked: when two conflict, the higher one wins unless an ADR says otherwise.

| # | Characteristic | Meaning in byo-chat |
|---|---|---|
| 1 | **Delivery integrity** | An event is never lost or sent twice to a contact; order holds per Conversation (ADR-0011). |
| 2 | **Isolation** | A failing Connector, Account or Destination cannot take the others down (ADR-0002, ADR-0011). |
| 3 | **Recoverability** | The Bridge keeps running unattended: reconnects, retries, and surfaces what needs a human (ADR-0007). |
| 4 | Extensibility | A new Network or Destination enters as a Manifest or an adapter, without touching the core (ADR-0003, ADR-0012). |
| 5 | Testability | Seams with boundary-only fakes (ADR-0014). |
| 6 | Observability | Health at three levels, structured logs without message content (ADR-0007). |
| 7 | Operability | One binary, service or tray, setup from the web UI (ADR-0004). |

A new ADR names the characteristic it serves in its `Protects:` line.

## 3. Crates

| Crate | Role | Decided by |
|---|---|---|
| `byo-domain` | Canonical Events, Account, Conversation, policies, port traits. Pure: no I/O, no async | ADR-0010, ADR-0003 |
| `byo-app` | use cases; owns delivery rules | ADR-0010, ADR-0011 |
| `byo-connector` | Connector supervisor, Strategies, Manifest; owns process spawns | ADR-0002, ADR-0009, ADR-0013 |
| `byo-store` | SQLite: `control.db`, one `history.db` per Account | ADR-0011, ADR-0008 |
| `byo-chatwoot`, `byo-webhook` | `DestinationPort` adapters | ADR-0006, ADR-0012 |
| `byo-system` | OS keychain, real `Clock` and `Rng` | ADR-0010 |
| `byo-http`, `byo-mcp` | local API, WebSocket and UI; MCP server | ADR-0004, ADR-0005, ADR-0012 |
| `byo-chat` | composition root; owns config and the log sink | ADR-0010, ADR-0013 |
| `xtask` | the gate | ADR-0013 |
| `web/` | Svelte + TypeScript web UI; text from the Fluent catalog `web/locales/en.ftl`; built to static files that `byo-http` serves | ADR-0004, ADR-0015 |

## 4. Allowed edges

- Every crate may depend on `byo-domain`; `byo-domain` depends on no workspace crate.
- `byo-app` depends only on `byo-domain`.
- Only `byo-chat` depends on `byo-app` and on adapter crates; no adapter depends on another.

## 5. Fitness functions

A fitness function fails when the architecture breaks, for any code, including code not written yet. A behaviour test is not one.

| Rule | Check |
|---|---|
| Only `byo-chat` uses an adapter crate | `cargo-deny` `[bans] deny` with `wrappers` (`deny.toml`) |
| No declared, unused dependency | Cargo lints `unused_dependencies`, `unused_workspace_dependencies` |
| Output, spawns and the environment have one owner each | clippy `print_*` and `disallowed-methods` (`clippy.toml`), ADR-0013 |
| CI runs the local gate with the same tool pins | `ci_runs_the_local_gate`, `ci_installs_every_tool_at_its_pin` (`xtask`) |
| Every ADR has a status from the closed set | `every_adr_has_a_closed_status` (`xtask`) |
| `byo-domain` has no I/O and no async runtime | not checked by code yet: it has no dependencies to ban |

## 6. Trust boundaries

A new path across a zone needs a row here before it has code.

| Boundary | What crosses | Control |
|---|---|---|
| Connector binary → Bridge | third-party output about personal Accounts | pinned, hashed binaries (ADR-0009); output parsed into typed Canonical Events at the adapter edge; media paths stay relative inside the Account's media directory (ADR-0002) |
| Bridge → Connector | commands and a minimal environment | only what the Manifest declares; no byo-chat secret (ADR-0009) |
| Destination → local API, WebSocket, Chatwoot webhook | replies and commands from the network, through the user's tunnel | token on the API and WebSocket, HMAC on webhooks (ADR-0012) |
| Agent → MCP | tool calls; inbound messages may carry prompt injection | read-only by default, per-Account write, allowlist, rate limit, Handoff (ADR-0005) |
| Disk | history, media, keys | SQLCipher per Account, keys in the OS keychain (ADR-0008, ADR-0011) |
