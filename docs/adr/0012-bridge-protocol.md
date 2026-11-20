# byo-chat Bridge Protocol: CloudEvents over webhook and local API; Chatwoot as one adapter

Chatwoot does not define the contract; it is one destination among others (n8n, CRMs, own products). byo-chat exposes its own Bridge Protocol: Canonical Events wrapped in a CloudEvents JSON envelope, over deliberately few transports:

- Outbound webhooks (byo-chat → subscriber), HMAC-signed, using the delivery guarantees of ADR-0011.
- Local HTTP API (subscriber → byo-chat) for sending and querying, token-protected; remote systems reach it through a tunnel the user provides (Cloudflare Tunnel, Tailscale Funnel, ngrok).
- WebSocket on the local API (`/v1/ws`, token-protected): one bidirectional connection where the subscriber receives the live event stream (with resume from the last acknowledged event id) and sends commands such as send message, with acknowledgements. Works through the same tunnel.
- MCP for Agents (ADR-0005).

The Chatwoot adapter translates the protocol: Chatwoot's REST API for inbound, and Chatwoot's official callback webhook (via the user's tunnel) for agent replies, with polling (ADR-0001) as the fallback when no tunnel is configured.

To avoid overengineering, we deliberately do not build now: a dial-out WebSocket transport (byo-chat connecting to the subscriber), synchronous "actions in webhook response", or subscribing to Chatwoot's undocumented ActionCable socket. Each can be added later as another transport without changing the event format.

## Considered Options

- Chatwoot's API as the contract: rejected — ties every integration to Chatwoot's poorer model.
- Custom envelope: rejected — CloudEvents is a CNCF standard many tools already accept.
