# Local SQLite queue and Chatwoot polling by default

byo-chat runs on a PC behind NAT, while Chatwoot may be in the cloud or local. We use a persistent SQLite queue (inbox/outbox with retry) instead of a broker such as NATS, to keep a single binary with no extra processes. For outbound events (agent → contact) the default is polling the Chatwoot API, because Chatwoot's callback webhook cannot reach a machine behind NAT; webhook delivery through a tunnel (Cloudflare Tunnel, Tailscale Funnel, ngrok) is optional for real time.

## Considered Options

- Embedded NATS: rejected — overhead with no gain on a single machine.
- Webhook only: rejected — requires exposing the user's machine publicly.
