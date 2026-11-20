# Headless daemon with local web UI; Tauri is only a tray shell

All work (Connector supervision, queue, Chatwoot sync, health checks) lives in one headless Rust daemon that serves a token-protected HTTP API and a Svelte web UI on 127.0.0.1:7878. On Linux it runs as a systemd user service (system service optional for servers); on Windows it runs in the background at login, because QR login and the OS keychain work best in the user session. The optional Tauri app only provides tray status and notifications and opens the same web UI, so there is a single UI to maintain and headless servers are first-class. The same binary offers console subcommands (`status`, `health`, `accounts`, `logs`) that talk to the daemon API.

## Considered Options

- Tauri app as the core process: rejected — ties the Bridge to a desktop session and breaks headless Linux.
- Leptos UI: rejected for development speed; Svelte is lighter to iterate on.
- Windows service by default: rejected — session-0 isolation complicates QR login and keychain access.
