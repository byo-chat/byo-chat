# Chatwoot projection: auto-provisioned Channels and groups as contacts

Chatwoot has no group concept in any native channel. We follow the de-facto pattern of existing bridges (Evolution API, WAHA, unoapi-cloud) and fix their known weaknesses:

- Channels: when an Account is linked, byo-chat idempotently creates a Chatwoot API inbox (lookup before create); reusing an existing inbox by ID remains possible.
- Groups are disabled by default per Account, with allow/deny lists by group id.
- When enabled, each group becomes one Chatwoot contact ("Subject [Group]", identifier = group id) with a single conversation. The individual sender is rendered as a configurable template prefix (default `**Name (+phone):**`) and also stored structured in message attributes. Replies always go to the whole group; agent signature is optional.
- A per-Conversation lock prevents duplicate contacts/conversations under concurrent events, and the contact name follows group subject changes.

This is only a projection of Canonical Events; native group support in Chatwoot would change only the adapter.

## Considered Options

- One Chatwoot conversation per participant: rejected — breaks group context and replies.
- Groups on by default: rejected — high volume and noise in support inboxes.
