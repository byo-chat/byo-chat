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

**Strategy**:
How byo-chat talks to a Connector: Stream, Poll or Matrix.

**Connector Manifest**:
Declarative description of a Connector: Network, Strategy, pinned binary and version, event/send/health commands, login kind, and mapping to Canonical Events.

**Canonical Event**:
The common input/output unit of the Bridge (message, media, reaction, edit, deletion, receipt, reply, member join/leave), following Matrix event semantics with an own versioned schema (`byo.*.v1`).
_Avoid_: payload, Chatwoot message

**Conversation**:
A dialogue within an Account: direct (1:1) or group. Groups are supported even when the destination does not support them.

**Channel**:
A Chatwoot inbox bound to exactly one Account.
_Avoid_: inbox (in domain code)

**Bridge**:
byo-chat's role: carrying events between Account and Channel in both directions.

## Relationships

- A **Network** has one or more possible **Connectors**; each **Account** uses exactly one **Connector**
- Each **Connector** is described by a **Connector Manifest** and uses one **Strategy**
- Each **Account** maps to exactly one **Channel** (1:1)

## Flagged ambiguities

- "nat" — resolved: neither NATS nor embedded NAT traversal; the queue is local (see ADR-0001).
- Coverage reference: WhatsApp (wacli) and Telegram; other Networks adapt to them.
- License: MIT (kept; AGPL Connectors stay isolated as separate processes).
- Network session data belongs to the Connector (one folder per Account); byo-chat secrets live in the OS keychain.
- Pending outbound events wait for reconnection up to a configurable deadline, then surface as a visible failure in the Channel.
- Ban risk: the user's responsibility, shown as a disclaimer; byo-chat does not try to mitigate it.
