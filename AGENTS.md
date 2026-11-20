# byo-chat

Local bridge: keeps personal messaging Accounts (WhatsApp, Telegram, Instagram, LinkedIn, X, Teams) connected through third-party Connectors and carries their events to Destinations (Chatwoot, webhooks, Agents via MCP). Rust daemon, hexagonal (ports and adapters). Docs, code and commits are in English.

Operational guide for agents working on byo-chat. It holds what an agent gets wrong without being told and points to the documents that hold the details. When a rule here and its source document disagree, the source is correct: fix this file in the same change. Write docs in plain English: short sentences, common words, no idioms.

- **[CONTEXT.md](./CONTEXT.md)**: the ubiquitous language. Before you name a type, function, test, flag or anything a user sees, search it for the term (`**Term**:`). A missing term is a question for the human, then a new entry.
- **[docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)**: crates, allowed edges, fitness functions, trust boundaries. Read its §1 before you add a crate, a dependency between crates, a port, or a call to an outside system.
- **[docs/adr/](./docs/adr/)**: the decisions; ARCHITECTURE.md §3 names the ADR of each crate. A change that contradicts an ADR, or crosses a boundary no ADR covers, starts as a question to the human; the answer becomes an ADR from [the template](./docs/adr/TEMPLATE.md).
- **[docs/TESTING.md](./docs/TESTING.md)**: when a test is done. Read it before you add, change or review a test.
- **[SECURITY.md](./SECURITY.md)**: read it with ARCHITECTURE.md §6 before you add a secret, a token check, a path from the network, or anything a Connector or an Agent receives.

## Workflow

Build in tracer bullets: one vertical slice through every layer, then the next. Each slice runs red → green: a failing test at one of the seams of ADR-0014, then the least code that turns it green, then refactor with the suite green. Use the `/tdd` skill for the loop. Use cases are the primary seam: real SQLite in a temp file, fakes only for Connector, Destination, SecretStore, Clock and Rng. A test is done by the rules of `docs/TESTING.md`.

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
