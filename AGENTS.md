# byo-chat

Local bridge between personal messaging Accounts and support Destinations. Domain language: `CONTEXT.md`. Decisions: `docs/adr/`. Docs and code are in English.

## Tools

- Before reading a whole source file, to find code by its shape (a call with a given argument, an implementation), or to rewrite one shape at many sites, use ast-grep: `docs/ast-grep.md`.

## Lint gate

- Before you call a change done, run `cargo xtask check`; it passes when its last line is `gate: pass`.
- While you edit, the hook runs `cargo fmt` and clippy after each Rust file you change: read its output and fix it before you go on.
- When a rule fails, fix the cause:
  - an error or a `Result` that nobody reads goes to its caller or to the log;
  - a fallback that hides a failure goes away, and the failure reaches the use case in `byo-app` that can act on it;
  - `unwrap`, a cast or an index becomes a value narrowed once where it enters (the Connector, Chatwoot, HTTP and MCP adapters);
  - output goes through `byo-chat`'s `logging` module, a process spawn through `byo_connector::spawn`, and reading the environment through `byo-chat`'s `config` module (ADR-0013); nothing calls `exit`, main returns;
  - a function that is too complex is a god function, and a file that is too long a god file: split each by the responsibilities it already carries, or rewrite it with guard clauses or a table of cases;
  - a comment is said by the code instead: a name, a type, a function, an assertion, a test. A comment that explains a workaround means the defect is still there: fix the defect. The reason no code can carry goes in an exception's reason or an ADR in `docs/adr/`.
- An exception is the last resort: `#[expect(clippy::<lint>, reason = "<why>")]` on the item, with a reason that says why the rule is wrong at that line.
- Linter configs (`Cargo.toml` lints, `clippy.toml`, `deny.toml`, `rustfmt.toml`), the gate (`xtask/`) and its scripts (`docs/lint-scripts/`) change only when the task is about them, and a count only goes down.
