# Own canonical event schema with Matrix semantics; Chatwoot is just an adapter

Status: proposed

The Bridge's internal format is an own versioned schema (`byo.*.v1`) following Matrix event semantics (message, reaction, edit, redaction, receipt, reply, group membership). Chatwoot's protocol covers neither groups, reactions nor edits; using it as the model would tie the domain to the poorest destination, so it becomes an outbound adapter. WhatsApp (wacli) and Telegram define minimum coverage.

## Considered Options

- Chatwoot's model: rejected, insufficient coverage.
- Full Matrix (every Connector as a bridge, byo-chat as a Matrix client): rejected, it would require Conduit always running and writing bridges for wacli/Telegram. Compatible semantics keep that door open.

## Addendum: interactive messages and capabilities

The schema includes `byo.interactive.v1` (buttons, lists, polls, CTA, carousel). Each Connector Manifest declares its capabilities (e.g. `buttons`, `lists`, `polls`). When a Connector lacks a capability, byo-chat degrades automatically: buttons/lists → native poll → numbered text menu, mapping the contact's reply back to a selection. wacli cannot send buttons or lists (only polls and selecting inbound options), so an optional second WhatsApp Connector (`wa-infinite`, a bun-compiled shim over the InfiniteAPI Baileys fork) provides native interactive messages, chosen per Account.

WhatsApp Connectors are wacli (default) and wa-infinite. WAHA was considered and rejected: it is a Docker-hosted HTTP server that does not fit byo-chat's managed-sidecar model (ADR-0002), and it adds no capability beyond the two chosen Connectors.
