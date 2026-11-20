# Self-Update from signed GitHub releases

Status: proposed
Kind: structural
Protects: operability; constraint: security

## Context

byo-chat holds the keys to personal Accounts and their message history. A Self-Update that trusted whatever the GitHub release page serves would turn a leaked GitHub token into code execution with access to everything.

## Decision

**D1.** A release is accepted only when its minisign signature verifies against the public key embedded in the running binary, and its SHA-256 matches the signed manifest. Anything else is refused and reported.

**D2.** The release workflow signs with a private key held outside the repository, in a protected environment that requires a manual approval.

**D3.** byo-chat checks the latest stable GitHub release at start and every 24 hours, and on demand (UI button, `byo-chat update --check`). Pre-releases are offered only on an opt-in beta channel.

**D4.** A newer release is announced in the tray, the web UI and an OS notification, with its version and release notes; a breaking release (docs/versioning.md) is marked as such.

**D5.** A Self-Update is applied only by a user action (UI or `byo-chat update`); nothing installs on its own.

**D6.** Applying it: download and verify (D1), back up the databases, drain the outbox, stop Connectors, replace the executable (rename-in-place on Windows), restart the daemon.

**D7.** The previous executable and the database backup are kept; if the new version is not ready within 2 minutes, byo-chat restores both, restarts the previous version and tells the user. Events received meanwhile are read again from the Connector and deduplicated (ADR-0011).

**D8.** When a package manager installed byo-chat, Self-Update is off: byo-chat only announces the new release.

## Considered options

- SHA-256 only: rejected — whoever publishes a fake binary publishes a fake hash.
- Silent automatic install: rejected — it can drop live support sessions mid-shift, as ADR-0009 already decided for Connectors.
- Sigstore/cosign keyless: rejected for now — verifying it inside the binary is far more complex than minisign.

## Compliance

- D1: not checked by code yet.
- D2: not checked by code: a repository setting.
- D5, D8: not checked by code yet.
