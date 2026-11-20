# Roadmap

Each release is a vertical slice; v0.1 is a tracer bullet through every layer with a single Network.

| Release | Scope |
|---|---|
| v0.1 | WhatsApp via wacli (Stream Strategy, Connector Manifest); Canonical Event schema designed in full, text + media implemented; SQLite queue and history (unencrypted); Chatwoot adapter (auto-provisioned inbox, 1:1, outbound polling); daemon + console (`status`, `health`, `accounts`); web UI main screen of the ADR-0004 amendment (Accounts by Network, Account detail with metrics, Delivery health and Connector logs, QR linking) with the Fluent text catalog (ADR-0015) |
| v0.2 | Telegram (Poll Strategy); reactions, edits, redactions, receipts; SQLCipher; `byo.interactive.v1` with fallback; optional `wa-infinite` Connector |
| v0.3 | MCP adapter with the Agent tools of ADR-0017: search, scoped grep, context, stats, links, Contact and Group directory with profile data and pictures; write with Handoff and `select_option`; groups |
| v0.4 | Tauri tray shell; Windows background/Linux systemd service; Connector registry with pinned hashes and approved updates; signed Self-Update (ADR-0016); native autostart (ADR-0004 amendment); `manage` tier for Groups and profile with human approvals (ADR-0017 D10–D11) |
| v0.5 | Instagram and Teams (Poll Strategy) |
| v0.6 | LinkedIn (Matrix Strategy + embedded Conduit) |
