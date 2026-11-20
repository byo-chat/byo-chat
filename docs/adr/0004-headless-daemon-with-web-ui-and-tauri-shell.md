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

## Amendment 2026-11-20 — main screen layout

Chosen from the prototype on branch `prototype/main-screen` (variant I, see its `NOTES.md`):

- Left sidebar: Accounts grouped by Network in a fixed order, each Network with its official logo and a Connect action; each Account shows an avatar with a Network badge, its label and an Account State dot.
- Right panel: the selected Account's detail, with two tabs, Overview and Connector logs.
- Overview: a metric strip with 7-day sparklines and the day-over-day change, coloured by whether the change is good; Delivery health rings (delivered, online in the last 7 days, replied within 5 minutes, queue), each with a one-line context; the Account's Destinations.
- Connector logs: the Connector output as-is, from the in-memory buffer (ADR-0007 amendment).
- Visual language: neutral palette on a soft lilac-grey canvas, white cards; official logos and status dots are the only colours. Colours and surfaces are CSS variables in `web/src/tokens.css`.
- Logos come from each brand's official kit and follow its usage rules.
- The metrics need data the Bridge must record: time to first reply per Conversation, and Account connect and disconnect times.
