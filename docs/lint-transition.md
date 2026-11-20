# Lint transition plan

Walking skeleton: every rule of the Rust target is an error with no debt list, and `cargo xtask check` is the one gate, mirrored by CI. Only Phase 0 is left: gate rows not on yet.

## Phase 0 â€” remaining gate rows (lane: gate)

| Row | Why not on yet | Done when |
|---|---|---|
| actionlint | binary download with checksum, not in install-action | pinned in CI by checksum and in the gate |
| gitleaks | binary download with checksum, not in install-action | pinned in CI by checksum and in the gate |
| jscpd (no new clone, count ratchet) | needs npm/package.json | pinned and run in the gate against the base branch |
| File size and inline test block limits | script not written | xtask check fails over 500 non-blank lines |
| Scheduled pin job | not written | weekly CI job compares pins with latest releases and opens an issue |
| Dependabot with cooldown | not written | `.github/dependabot.yml` grouped, with cooldown |
| Mutation testing on the diff | slow; first tests only | `cargo mutants --in-diff` in CI |
| Agent hook refusing bypasses and unasked config edits | not written | PreToolUse hook in `.claude/settings.json` |

## Decisions for the human

- D1: done 2026-11-20 — `main` requires both CI gate jobs, up to date, through a pull request, admins included; no force push or deletion.
