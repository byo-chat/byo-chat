# Three-level health model and local message history

Status: proposed

Health is checked at three levels: Process (supervisor watches the Connector PID and restarts with backoff), Account (the Manifest's health command runs periodically and drives Account State) and Flow (queue depth, oldest pending event age, Chatwoot reachability). It is exposed via tray, web UI, `byo-chat health`, MCP, `/healthz`, `/readyz` (per-Account JSON) and optional Prometheus `/metrics`. Logs are structured JSON with rotation, captured per Connector; message content is never logged by default.

byo-chat stores the full Canonical Event history in its own SQLite database rather than querying Connectors on demand. This makes search and MCP `read_messages` work uniformly across Networks and while a Connector is down, at the cost of holding a copy of personal messages locally.

## Considered Options

- Pass-through with short retention, querying Connectors for history: rejected — Connectors expose history inconsistently (some not at all) and it would fail exactly when a Connector is unhealthy.

## Amendment 2026-11-20 — Connector output

- byo-chat does not manage a Connector's logs. It shows the Connector's stdout and stderr as written, beside its own supervisor lines, in the web UI and in `byo-chat logs --account <id>`.
- It keeps only the latest lines per Connector in a bounded in-memory buffer: never written to disk, never filtered or masked, lost on restart. Whatever the Connector writes, including contact names or message text, is the Connector's responsibility; files the Connector writes itself are outside byo-chat.
- byo-chat's own logs keep the rule above: no message content by default.
