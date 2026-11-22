# Connectors run as separate processes, not linked into the binary

Status: proposed

Most Connectors are written in Go and the mautrix bridges are AGPL-3.0. Linking Go code into the Rust binary (cgo c-archive) would bring one Go runtime per library and fragile builds, and linking mautrix would force byo-chat to become AGPL, conflicting with its MIT license. Each Connector, plus the Conduit homeserver used for LinkedIn, therefore runs as a sidecar process: byo-chat downloads or bundles the binary at the version pinned in the Connector Manifest, then supervises, restarts and isolates it. A Connector that hangs or leaks memory cannot take the Bridge down.

## Considered Options

- Linking via cgo/FFI: rejected for licensing, size and build fragility.
- Rewriting Connectors in Rust: rejected for cost and to avoid tracking Network protocol changes ourselves.

## Amendment 2026-11-22 — wa-infinite shim and stdio Stream protocol

- byo-chat writes and maintains the `wa-infinite` shim, consuming `rsalcara/InfiniteAPI` as an upstream library dependency.
- In v0.1, the shim source lives in `connectors/wa-infinite/` in this repository; it moves to `byo-chat/connectors` when the Connector registry lands in v0.4 (ADR-0009).
- `wa-infinite` speaks bidirectional NDJSON over stdio (`Stream` Strategy): events and command responses on `stdout`, commands on `stdin`, and diagnostic logs on `stderr`.
- The NDJSON lines use a thin, versioned `byo-stream.v1` protocol owned by byo-chat. The shim unwraps Baileys' internal structures into typed lines, and `byo-connector` strictly parses every line into Canonical Events and commands at the adapter edge. The Connector Manifest declares `byo-stream.v1` directly; declarative field mapping stays for third-party CLIs we do not own.
- Media crosses the boundary by relative path inside a per-Account `--media-dir`, never inline base64 in NDJSON. `byo-connector` rejects any path that escapes `--media-dir`. Network session data stays in a per-Account `--session-dir`.
- Rejected options: local HTTP/WebSocket API (adds port allocation, Windows firewall prompts, and a loopback auth surface); raw Baileys JSON mapped declaratively by the Manifest; inline base64 media in NDJSON.
