# MCP adapter with per-Conversation handoff

Agents interact with Accounts through an MCP adapter that sits beside the Chatwoot adapter on the same core and Canonical Events, exposed over stdio (`byo-chat mcp`) and Streamable HTTP (`/mcp`, token-protected). Because Agents act on personal Accounts and inbound messages may carry prompt injection, access is read-only by default; writing is enabled per Account, optionally restricted to a Conversation allowlist, rate-limited and audited as a note in the Channel. When an Agent and Chatwoot share an Account, each Conversation has one Responder: the Agent answers until a human takes over in Chatwoot (Handoff), after which the Agent is blocked from writing to that Conversation.

## Considered Options

- Both may answer freely: rejected — duplicate or conflicting replies to contacts.
- Per-Account ownership (Agent-only or Chatwoot-only): rejected — prevents the common "bot first, human on escalation" flow.
