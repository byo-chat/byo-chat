# One owner module per capability

Status: accepted

Some calls are allowed in one place only, and the lint gate enforces it (`clippy.toml` `disallowed-methods`, `print_stdout`/`print_stderr`):

- Process output: `byo-chat` `logging` module owns the sink (the `tracing` subscriber). Every crate emits through `tracing` macros; `print!`/`eprintln!` are forbidden everywhere else.
- Starting processes (Connector sidecars, ADR-0002): `byo_connector::spawn`. The `xtask` gate also spawns the checks it runs.
- Reading the environment: `byo-chat` `config` module. Nothing writes the environment.

Each owner carries the single `#[expect(..., reason = "... (ADR-0013)")]` for its capability. Concentrating these effects keeps them auditable and testable behind ports, which matters because Connectors run third-party binaries with access to personal Accounts.
