# Connectors run as separate processes, not linked into the binary

Most Connectors are written in Go and the mautrix bridges are AGPL-3.0. Linking Go code into the Rust binary (cgo c-archive) would bring one Go runtime per library and fragile builds, and linking mautrix would force byo-chat to become AGPL, conflicting with its MIT license. Each Connector, plus the Conduit homeserver used for LinkedIn, therefore runs as a sidecar process: byo-chat downloads or bundles the binary at the version pinned in the Connector Manifest, then supervises, restarts and isolates it. A Connector that hangs or leaks memory cannot take the Bridge down.

## Considered Options

- Linking via cgo/FFI: rejected for licensing, size and build fragility.
- Rewriting Connectors in Rust: rejected for cost and to avoid tracking Network protocol changes ourselves.
