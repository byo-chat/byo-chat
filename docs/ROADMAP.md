# Roadmap

Each release is a vertical slice; v0.1 is a tracer bullet through every layer with a single Network.

| Release | Scope |
|---|---|
| v0.1 | WhatsApp via wacli (Stream Strategy, Connector Manifest); Canonical Event schema designed in full, text + media implemented; SQLite queue and history (unencrypted); Chatwoot adapter (auto-provisioned inbox, 1:1, outbound polling); daemon + console (`status`, `health`, `accounts`); minimal web UI (QR linking, Account State) |
| v0.2 | Telegram (Poll Strategy); reactions, edits, redactions, receipts; SQLCipher |
| v0.3 | MCP adapter (read-only, then write with Handoff); groups |
| v0.4 | Tauri tray shell; Windows background/Linux systemd service; Connector registry with pinned hashes and approved updates |
| v0.5 | Instagram and Teams (Poll Strategy) |
| v0.6 | LinkedIn (Matrix Strategy + embedded Conduit) |
