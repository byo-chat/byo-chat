# Versioning

byo-chat follows [Semantic Versioning 2.0.0](https://semver.org/): `MAJOR.MINOR.PATCH`, one version for the whole workspace (`[workspace.package] version` in `Cargo.toml`), tagged `vX.Y.Z` on `main`.

## The public API

SemVer counts breaks against what users and integrators depend on, which for byo-chat is:

- the Bridge Protocol: Canonical Event schemas (`byo.*.vN`) and their CloudEvents envelope;
- the local HTTP API and WebSocket messages;
- MCP tool names, inputs and outputs;
- the Connector Manifest format;
- the config file and the on-disk data layout (a change needs a migration);
- console commands and their flags, `--json` output, exit codes.

Crate APIs inside the workspace are internal: changing them is never a break.

## Bumping

While the major is 0 (the roadmap's v0.1 … v0.6), the public API is still settling:

- a break bumps MINOR (`0.3.2` → `0.4.0`);
- a feature or a fix bumps PATCH.

From `1.0.0`:

- a break bumps MAJOR;
- a backward-compatible feature bumps MINOR;
- a backward-compatible fix bumps PATCH.

A Canonical Event schema evolves additively: a new optional field stays `v1`; a removed or retyped field is a new schema (`byo.message.v2`) served beside the old one for at least one MINOR (one MAJOR after 1.0).

Pre-releases are `X.Y.Z-rc.N`. A released version is immutable: a fix ships as a new version.

## Commits

[Conventional Commits](https://www.conventionalcommits.org/) map each commit to its bump:

- `fix:` → PATCH; `feat:` → MINOR (PATCH while 0.x);
- `!` after the type, or a `BREAKING CHANGE:` footer → MAJOR (MINOR while 0.x);
- `docs:`, `test:`, `refactor:`, `chore:`, `ci:`, `build:` → no release on their own.

A scope names the crate or area: `feat(connector): ...`, `fix(store): ...`.

## Releasing

1. Every change since the last tag is classified by its commits; the highest bump wins.
2. Update `version` in `Cargo.toml` and the changelog in one `chore(release): vX.Y.Z` commit.
3. `cargo xtask check` passes, then tag `vX.Y.Z` on `main`.
