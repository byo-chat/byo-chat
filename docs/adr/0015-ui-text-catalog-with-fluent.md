# User-facing text lives in a Fluent catalog, English first

Status: proposed
Kind: structural
Protects: operability, extensibility

## Context

byo-chat writes text in many places: the web UI, the tray, OS notifications, system messages in a Chatwoot Channel, and the human output of console commands. The product starts in English and must accept other languages later without hunting strings across crates and the Svelte code.

## Decision

**D1.** Every text a person reads in the web UI, the tray, OS notifications, Channel system messages and human console output comes from one catalog in Project Fluent format (`.ftl`), with English as the source locale.

**D2.** The same `.ftl` files serve Rust (`fluent` crates) and the Svelte UI (`@fluent/bundle`).

**D3.** The locale comes from the OS, can be overridden in settings, and falls back to English for any missing key. Channel system messages use the locale configured for each Destination, since the human agent may read another language.

**D4.** API and MCP errors and `--json` output carry a stable English code (`account_expired`) and an English message; they are part of the public API (docs/versioning.md) and are never translated. Logs are always English.

## Considered options

- JSON key-value files: rejected — no plural or gender rules per language.
- gettext: rejected — weaker tooling in Svelte and no shared runtime with Rust.

## Compliance

- D1: not checked by code yet: needs the catalog lint below once the first UI text exists.
- D2: not checked by code yet.
- Planned check: a gate test fails when code uses a key missing from the English catalog, or the catalog holds a key no code uses.
