# Agent tools read the local history; semantic search is optional and later

Status: proposed
Kind: feature
Protects: extensibility; constraint: privacy

## Context

Agents (ADR-0005) need to find messages, gather the context around them and build reports. byo-chat already keeps every Canonical Event in one SQLite database per Account (ADR-0007, ADR-0011), so these tools can answer from local data even when a Connector is down. Text from messages is untrusted: it may carry prompt injection.

## Decision

**D1.** MCP and the console expose the same read tools: `search_messages` (SQLite FTS5 full-text search with filters for Account, Conversation, Contact, Group, period and kind), `grep_messages` (regular expression, only within one Conversation or a period of at most 30 days), `get_context` (N events before and after a message, in Canonical form) and `message_stats` (volumes, first-reply times, unanswered Conversations, by period, Contact or Conversation).

**D2.** Results are paginated by cursor, 20 per page by default, with a highlighted excerpt instead of the whole message. Searches fan out over the Account databases and merge by rank.

**D3.** Message text in any tool result is marked untrusted and kept apart from metadata, so an Agent treats it as data.

**D4.** byo-chat retrieves and counts; it does not write reports. The Agent composes them from D1.

**D5.** Semantic search is not part of v0.3. It comes later as an opt-in per Account: a local embedding model (fastembed-rs, multilingual), vectors in the Account database (sqlite-vec), and `search_messages` gaining `mode: semantic | hybrid`. Embeddings never leave the machine. A spike first checks sqlite-vec with SQLCipher and measures indexing 100 000 messages.

**D6.** Received buttons and lists are shown by `get_context` with numbered options and their ids. `select_option(message_id, option)` (id, label or index) answers one; it is a write action under ADR-0005 (per-Account write grant, allowlist, rate limit, blocked after Handoff, audited in the Channel). A Connector without the `select` capability returns `capability_unsupported`.

**D7.** URLs in messages are extracted at ingest into their own table (URL, domain, message, Conversation, time, and the preview title and image when the Connector provides them). `list_links` filters by domain, Contact, Conversation and period. byo-chat never fetches a URL: an Agent that needs a page uses its own tools, so the daemon cannot be used to reach the user's network (SSRF) or reveal the user's address.

**D8.** byo-chat keeps a directory of Network facts only, synced from Connectors: for each Contact its display name, picture, phone or username, whether it is saved on the phone, the Accounts that know it, the Groups it belongs to with its role, its Conversations and last interaction; for each Group its name, picture, description and members. Tools: `list_contacts`, `get_contact`, `find_contact` (name, number or username), `list_groups`, `get_group`.

**D9.** byo-chat holds no CRM data of its own (tags, notes, lists, custom attributes). Organising people belongs to a Destination such as Chatwoot, so there is one source of truth per fact.

**D10.** Access has three tiers per Account: `read` (search, context, Contacts, Groups; on by default), `write` (send, react, `select_option`; off until granted) and `manage` (create, rename and describe Groups, set their picture, add and promote members, create invite links; edit the profile name, about and picture; off until granted, apart from `write`). Each action needs the matching Connector capability (`groups.manage`, `profile.edit`) or returns `capability_unsupported`.

**D11.** Destructive or exposing actions need a human approval even with `manage` granted: removing a member, demoting an admin, leaving a Group, changing the profile name or picture, revoking an invite link. The call returns `pending_approval` with an id; the user approves or denies from the tray or the web UI; the caller polls the outcome. Every request, approval and denial is audited.

**D12.** The same tools exist on MCP, the local HTTP API and WebSocket (ADR-0012), and the console. MCP clients and API tokens carry the tiers of D10, scoped per Account, and follow D11. A person acting in the console is the approver, so their actions run without a pending step. The web UI only shows profiles and Groups: it offers no `manage` action, and it is where approvals of D11 are given.

**D13.** When a Connector provides them, byo-chat stores the profile data and the profile picture of every Contact who writes to an Account: display name, about text, phone or username, business details, and the picture as media (ADR-0008: per Account, content-addressed, encrypted). Only the latest picture is kept, replaced when the Connector reports a change; the directory of D8 serves both.

## Considered options

- Regex over the whole history: rejected — no index, unbounded cost.
- Remote embedding APIs: rejected — message text would leave the machine.
- byo-chat fetching link previews itself: rejected — SSRF and IP exposure from links sent by strangers.
- Tags and notes on Contacts in byo-chat: rejected — a second source of truth beside Chatwoot's labels, notes and attributes.
- Semantic search in v0.3: rejected — model download, CPU and storage cost before FTS5 has been tried by real Agents.
