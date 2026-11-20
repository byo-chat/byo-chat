# One SQLite database per Account, single writer, at-least-once delivery

Expected load is tens of events per second at peak on one machine; SQLite in WAL mode handles far more, but allows one writer per file. Storage is therefore split into a small `control.db` (config, Accounts, Channels, Manifests) and one `accounts/{id}/history.db` per Account holding its events, queues and media index. This gives real write parallelism across Accounts, failure isolation, trivial per-Account backup/export, and crypto-shredding on removal (each Account has its own SQLCipher key). Cross-Account search fans out and merges.

Within each database a single writer task per Account batches writes in transactions; reads use a read-only connection pool (`journal_mode=WAL`, `synchronous=NORMAL`, `busy_timeout`), so `SQLITE_BUSY` is avoided by design.

Delivery uses a transactional outbox with at-least-once semantics: inbound events are deduplicated by their Network message id; outbound events carry an idempotency key and a Chatwoot↔Network id mapping; retries use exponential backoff with jitter up to 5 minutes; exhausted events go to a dead-letter list surfaced in the Channel and UI with manual retry. Ordering is preserved per Conversation while Conversations run in parallel, and sending is rate-limited per Account. No external broker is used (ADR-0001). Delivery rules (retry timing, dead letter, per-Conversation ordering, rate limiting) live in `byo-app`; `byo-store` only offers transactional primitives. Tests verify these guarantees through observable behaviour (what the fake Connector or Channel received, how many times, in which order), never by querying the database tables.

## Considered Options

- Single shared database: rejected — one writer would serialize all Accounts and removal could not crypto-shred.
- One database per Connector: rejected — a Connector serves many Accounts; the Account is the unit of isolation.
- PostgreSQL or an external broker: rejected — server-grade overhead for a single-user local bridge.
