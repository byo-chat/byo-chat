# Headless daemon with local web UI; Tauri is only a tray shell

Status: proposed

All work (Connector supervision, queue, Chatwoot sync, health checks) lives in one headless Rust daemon that serves a token-protected HTTP API and a Svelte web UI on 127.0.0.1:7878. On Linux it runs as a systemd user service (system service optional for servers); on Windows it runs in the background at login, because QR login and the OS keychain work best in the user session. The optional Tauri app only provides tray status and notifications and opens the same web UI, so there is a single UI to maintain and headless servers are first-class. The same binary offers console subcommands (`status`, `health`, `accounts`, `logs`) that talk to the daemon API.

## Considered Options

- Tauri app as the core process: rejected — ties the Bridge to a desktop session and breaks headless Linux.
- Leptos UI: rejected for development speed; Svelte is lighter to iterate on.
- Windows service by default: rejected — session-0 isolation complicates QR login and keychain access.

## Amendment 2026-11-20 — autostart

- Autostart uses each OS's native mechanism: on Windows a per-user `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` entry (no admin) that starts the tray, or the daemon when there is no tray; on Linux desktops a systemd user service for the daemon and an XDG `~/.config/autostart` entry for the tray; on Linux servers a system service installed by the administrator.
- It is on by default, offered as a pre-checked option at first setup, and can be turned off at any time in the web UI settings or with `byo-chat autostart enable|disable|status`.
- byo-chat reads the state from the OS (the Run entry, the systemd unit) instead of keeping its own flag, so a change made in Task Manager or systemctl shows correctly. A system service shows as "managed by the system service" and cannot be changed from byo-chat.
- Uninstalling removes the autostart entry.
