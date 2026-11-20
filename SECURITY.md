# Security

## Report a vulnerability

Report it privately through GitHub: **Security → Report a vulnerability** on this repository. Do not open a public issue.

## Supported versions

Only the latest release receives fixes while byo-chat is in 0.x.

## What byo-chat protects

- Message history and media on disk, encrypted per Account (SQLCipher), keys in the OS keychain.
- The local API, WebSocket and MCP behind a token; webhooks signed with HMAC.
- Connector binaries pinned by version and SHA-256; each Connector receives only what its Manifest declares.
- Agents read-only by default; writing is granted per Account.

## What you must do

- Keep the API token secret, and expose the daemon to the internet only through a tunnel you control.
- Use secondary Accounts where you can: automating personal Accounts may break a Network's terms of service and lead to a ban. That risk is yours.

## What byo-chat does not protect against

- Malware running as your user, which can read the keychain and the process memory.
- A Network deciding to ban an Account.
