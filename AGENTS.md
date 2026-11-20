# byo-chat

Local bridge: keeps personal messaging Accounts (WhatsApp, Telegram, Instagram, LinkedIn, X, Teams) connected through third-party Connectors and carries their events to Destinations (Chatwoot, webhooks, Agents via MCP). Rust daemon, hexagonal. Docs, code and commits are in English.

## Language

Name every type, function, test and commit with the terms of `CONTEXT.md` (Account, Connector, Canonical Event, Destination, Handoff...). Read it before naming anything; a term missing there is a question for the human, then a new glossary entry.

## Architecture map

| Crate | Role |
|---|---|
| `byo-domain` | Canonical Events, Account, Conversation, policies, port traits. Pure: no I/O, no async |
| `byo-app` | use cases; owns delivery rules (retry, dead letter, ordering, rate limit) |
| `byo-connector` | Connector supervisor, Stream/Poll/Matrix Strategies, Manifest; owns process spawns |
| `byo-store` | SQLite: `control.db` and one `history.db` per Account |
| `byo-chatwoot`, `byo-webhook` | `DestinationPort` adapters |
| `byo-system` | OS keychain, real `Clock` and `Rng` |
| `byo-http`, `byo-mcp` | local API + WebSocket + UI; MCP server |
| `byo-chat` | composition root: the only crate that wires adapters; owns config and logging |

Dependencies point inward to `byo-domain`; `cargo-deny` refuses an adapter used by anything but `byo-chat`.

Before changing an area, read its decision in `docs/adr/`:

- Connectors, sidecars, Manifests, updates → 0002, 0009
- event schema, groups, interactive messages → 0003, 0006
- daemon, UI, tray, console commands → 0004
- Agents, MCP, Handoff → 0005
- storage, queues, delivery, encryption, retention → 0001, 0007, 0008, 0011
- Bridge Protocol, webhooks, WebSocket, Chatwoot transport → 0012
- crates, ports, owner modules → 0010, 0013
- tests → 0014

A change that contradicts an ADR starts as a question to the human; an accepted answer becomes a new ADR.

## Workflow

Build in tracer bullets: one vertical slice through every layer, then the next. Each slice runs red → green: a failing test at one of the seams of ADR-0014, then the least code that turns it green, then refactor with the suite green. Use the `/tdd` skill for the loop. Use cases are the primary seam: real SQLite in a temp file, fakes only for Connector, Destination, SecretStore, Clock and Rng. Assert behaviour the caller observes, with expected values from a literal, a recorded Connector output or the spec.

Release and commit rules: `docs/versioning.md`, read before tagging a release, writing a commit message, or changing anything a user or integrator depends on (Bridge Protocol, HTTP API, MCP tools, Manifest format, config, CLI).

## Design (the 80/20)

- **Ubiquitous language**: one glossary term per concept, everywhere.
- **Deep modules**: a small interface over substantial behaviour; a port names a capability the domain needs, in domain words.
- **Functional core, imperative shell**: decisions live in pure functions of `byo-domain`/`byo-app`; I/O stays in adapters.
- **Parse, don't validate**: outside data becomes a typed value once, at the adapter edge; the core receives only valid types.
- **Make illegal states unrepresentable**: enums and newtypes (`AccountId`, `AccountState`) over strings and flags.
- **Errors as values**: a `Result` with a typed error per boundary, carried to the use case that can act on it.
- **Single source of truth**: each fact in one place; the config, the Manifest and the ADR are referenced, never copied.
- **YAGNI**: the current roadmap slice only (`docs/ROADMAP.md`).

## Tools

- Before reading a whole source file, to find code by its shape (a call with a given argument, an implementation), or to rewrite one shape at many sites, use ast-grep: `docs/ast-grep.md`.

## Lint gate

- Before you call a change done, run `cargo xtask check`; it passes when its last line is `gate: pass`.
- While you edit, the hook runs `cargo fmt` and clippy after each Rust file you change: read its output and fix it before you go on.
- When a rule fails, fix the cause:
  - an error or a `Result` that nobody reads goes to its caller or to the log;
  - a fallback that hides a failure goes away, and the failure reaches the use case in `byo-app` that can act on it;
  - `unwrap`, a cast or an index becomes a value narrowed once where it enters (the Connector, Destination, HTTP and MCP adapters);
  - logs go through `tracing`, whose sink `byo-chat`'s `logging` module owns; a process spawn goes through `byo_connector::spawn`; the environment is read in `byo-chat`'s `config` module (ADR-0013); main returns instead of calling `exit`;
  - a function that is too complex is a god function, and a file that is too long a god file: split each by the responsibilities it already carries, or rewrite it with guard clauses or a table of cases;
  - a comment is said by the code instead: a name, a type, a function, an assertion, a test. A comment that explains a workaround means the defect is still there: fix the defect. The reason no code can carry goes in an exception's reason or an ADR in `docs/adr/`.
- An exception is the last resort: `#[expect(clippy::<lint>, reason = "<why>")]` on the item, with a reason that says why the rule is wrong at that line.
- Linter configs (`Cargo.toml` lints, `clippy.toml`, `deny.toml`, `rustfmt.toml`), the gate (`xtask/`) and its scripts (`docs/lint-scripts/`) change only when the task is about them, and a count only goes down.
