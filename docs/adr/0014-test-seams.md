# Test seams

Status: accepted

Tests verify behaviour through public interfaces, at seams agreed in advance; no test is written at another seam without a new decision. For v0.1:

1. Domain rules (`byo-domain`): Account State transitions, Handoff, Reopen Policy, redaction. Pure and fast.
2. Use cases (`byo-app`) with real SQLite and fake Connector, Channel, Clock and Rng. The primary seam: inbound event reaches the Channel, a reply reaches the Connector, retry, deduplication, ordering, dead letter.
3. Connector contract: Stream Strategy and Connector Manifest fed with recorded real wa-infinite output.
4. Chatwoot adapter against a real Chatwoot in Docker, in CI only.
5. Local HTTP API through HTTP: token auth and Bridge Protocol format.

The Svelte UI and the Tauri tray are out of scope for automated tests in v0.1. Work proceeds in vertical slices: one failing test, the minimal code to pass it, repeat. Expected values come from an independent source (a literal, a recorded Connector output, the spec), never recomputed the way the code does.

## Considered Options

- Fakes for every port: rejected — tests would pass against an imagined queue and miss the transaction and concurrency bugs of ADR-0011.
