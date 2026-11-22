# byo-chat

Local bridge that keeps messaging-network accounts connected through third-party CLIs/bridges and exposes them to Chatwoot as support channels.

## Language

**Network**:
External messaging service (WhatsApp, Telegram, Instagram, LinkedIn, X, Teams).
_Avoid_: platform, provider

**Account**:
One authenticated user identity on a Network. A Network may have many Accounts.
_Avoid_: session, profile

**Account State**:
New, Authenticating, Connected, Reconnecting, Disconnected, Expired (requires re-login) or Paused (by the user). Expired raises an alert in the tray and a system message in the Channel.

**Connector**:
Third-party software (CLI or bridge) that byo-chat supervises to operate Accounts on a Network.
_Avoid_: driver, plugin

**Self-Update**:
Replacing the running byo-chat with a newer published release, only after verifying its signature. Distinct from a Connector update (ADR-0009).
_Avoid_: upgrade, auto-update

**Strategy**:
How byo-chat talks to a Connector: Stream, Poll or Matrix.

**Connector Manifest**:
Declarative description of a Connector: Network, Strategy, pinned binary and version, event/send/health commands, login kind, and mapping to Canonical Events.

**Canonical Event**:
The common input/output unit of the Bridge (message, media, reaction, edit, deletion, receipt, reply, member join/leave), following Matrix event semantics with an own versioned schema (`byo.*.v1`).
_Avoid_: payload, Chatwoot message

**Contact**:
A person's identity on one Network (Network + its id there, such as a phone number or a username). The same identity seen by several of the user's Accounts on that Network is one Contact. byo-chat never links Contacts across Networks; unifying people is a Destination's job (for example Chatwoot).
_Avoid_: person, customer, lead

**Group**:
A multi-member chat that exists on a Network, with Contacts as members. Like a Contact, it belongs to the Network, so several Accounts may take part in the same Group.

**Conversation**:
A dialogue within an Account: direct (1:1) or group. Groups are supported even when the destination does not support them.

**Channel**:
A Chatwoot inbox bound to exactly one Account.
_Avoid_: inbox (in domain code)

**Agent**:
An AI client acting on Accounts through byo-chat's MCP adapter. Read-only by default; write access is granted per Account, optionally limited to an allowlist of Conversations and rate-limited.

**Responder**:
Who currently answers a Conversation: the Agent or a human in the Channel.

**Handoff**:
The moment a human takes over a Conversation in the Channel; from then on the Agent can no longer write to it.

**Reopen Policy**:
Who answers first when a contact writes to a resolved Conversation: the Agent (default), the last human, or never the Agent. Configured per Account; without an Agent the Conversation goes straight to the Channel.

**Bridge Protocol**:
byo-chat's own contract with destinations: Canonical Events in a CloudEvents envelope, delivered by signed webhook or local WebSocket, accepted through the local API or WebSocket, and exposed to Agents via MCP.

**Destination**:
Any system consuming the Bridge Protocol (Chatwoot, n8n, a CRM). Chatwoot is one Destination, not the contract.

**Bridge**:
byo-chat's role: carrying events between Account and Channel in both directions.

## Relationships

- A **Network** has one or more possible **Connectors**; each **Account** uses exactly one **Connector**
- Each **Connector** is described by a **Connector Manifest** and uses one **Strategy**
- Each **Conversation** has exactly one **Responder** at a time; **Handoff** moves it from Agent to human
- Each **Account** maps to exactly one **Channel** (1:1)
- A **Contact** or **Group** belongs to a **Network** and may be seen by many of the user's **Accounts** there; a group **Conversation** is an **Account**'s view of a **Group**

## Flagged ambiguities

- "nat" — resolved: neither NATS nor embedded NAT traversal; the queue is local (see ADR-0001).
- Coverage reference: WhatsApp (wa-infinite) and Telegram; other Networks adapt to them.
- License: MIT (kept; AGPL Connectors stay isolated as separate processes).
- Network session data belongs to the Connector (one folder per Account); byo-chat secrets live in the OS keychain.
- Pending outbound events wait for reconnection up to a configurable deadline, then surface as a visible failure in the Channel.
- Ban risk: the user's responsibility, shown as a disclaimer; byo-chat does not try to mitigate it.
