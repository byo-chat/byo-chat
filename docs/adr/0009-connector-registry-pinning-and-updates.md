# Connector registry with pinned, hashed binaries and user-approved updates

Status: proposed

byo-chat downloads and runs third-party binaries that hold access to personal Accounts, so Connector distribution is treated as a supply-chain risk. Official Connector Manifests live in a separate, PR-reviewed repository (`byo-chat/connectors`), bundled with byo-chat and refreshable; local Manifests are allowed but flagged "unverified". Each Manifest pins an exact version and a SHA-256 per platform (Windows/Linux, x64/arm64); mismatches abort installation and "latest" is never used. Binaries come from the upstream project's official releases or, when none exist, are built by `byo-chat/connectors` CI with artifact attestations; Node/TypeScript Connectors are packaged as single executables with `bun build --compile`. Updates are offered, approved per Connector by the user, and rolled back automatically if the Connector fails health after updating. Connectors run with their own data directory and a minimal environment, receiving only what their Manifest declares and never byo-chat's secrets.

## Considered Options

- Auto-updating to latest: rejected — silent behaviour changes and session breakage on personal Accounts.
- Go/Rust-only Connectors: rejected — would exclude Instagram and Beeper CLIs, which only exist in TypeScript.

## Amendment 2026-11-22 — Node Connectors

- A Node Connector ships as the official Node LTS runtime, pinned with its SHA-256 like any binary, plus one esbuild bundle per Connector. This replaces `bun build --compile`.
- A Node security fix updates the runtime without rebuilding a Connector.
- A Node single executable application stays a later one-file option.
- Evidence: `docs/research/node-connector-packaging.md` on branch `research/node-connector-packaging`.
