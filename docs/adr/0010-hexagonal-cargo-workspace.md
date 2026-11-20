# Hexagonal Cargo workspace with dependency-checked layers

The code is a Cargo workspace with one crate per layer: `byo-domain` (Canonical Events, Account, Conversation, Account State, port traits; no I/O, no async runtime), `byo-app` (use cases orchestrating ports), driven adapters `byo-connector`, `byo-store`, `byo-chatwoot`, driving adapters `byo-mcp`, `byo-http`, and the `byo-chat` binary as composition root. The domain never depends on an adapter, enforced by a build-failing dependency check. Fakes exist only for ports at the system boundary: `ConnectorPort`, `ChannelPort`, `SecretStore`, and the `Clock` and `Rng` ports that make time (backoff, retention, polling, deadlines) and jitter deterministic. Ports to our own code (`EventStore`, `OutboundQueue`) are never faked: tests run them on real SQLite in a temporary file. Every Connector adapter must pass a shared contract test suite fed with recorded real Connector output (e.g. wacli NDJSON). Stack: tokio, axum, serde, rmcp, and rusqlite (not sqlx) because SQLCipher arrives in v0.2 and rusqlite supports it via `bundled-sqlcipher`.

## Considered Options

- Single crate with modules: rejected — layer boundaries would not be enforced by the compiler.
- sqlx: rejected — limited SQLCipher support; switching drivers later is costly.
