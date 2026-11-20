# Three-level health model and local message history

Health is checked at three levels: Process (supervisor watches the Connector PID and restarts with backoff), Account (the Manifest's health command runs periodically and drives Account State) and Flow (queue depth, oldest pending event age, Chatwoot reachability). It is exposed via tray, web UI, `byo-chat health`, MCP, `/healthz`, `/readyz` (per-Account JSON) and optional Prometheus `/metrics`. Logs are structured JSON with rotation, captured per Connector; message content is never logged by default.

byo-chat stores the full Canonical Event history in its own SQLite database rather than querying Connectors on demand. This makes search and MCP `read_messages` work uniformly across Networks and while a Connector is down, at the cost of holding a copy of personal messages locally.

## Considered Options

- Pass-through with short retention, querying Connectors for history: rejected — Connectors expose history inconsistently (some not at all) and it would fail exactly when a Connector is unhealthy.
