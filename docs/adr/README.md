# Architecture Decision Records

Each ADR records one decision about a boundary in byo-chat. Before changing a boundary, read the ADR that governs it (the map is in [ARCHITECTURE.md](../ARCHITECTURE.md)).

- **One number, one decision.** `NNNN-<slug>.md`, numbered in order, never reused. Prose cites `ADR-NNNN`, and a rule inside it as `ADR-NNNN D2`.
- **Write one only when all three hold**: hard to reverse, surprising without context, a real trade-off.
- **Start from [TEMPLATE.md](./TEMPLATE.md).**
- **A status from a closed set**, kept true to the code: `proposed | accepted | deferred | rejected | superseded by ADR-NNNN`. `proposed` means not built yet; the commit that ships the code sets `accepted`. `cargo xtask check` fails on a missing or unknown status.
- **An accepted decision is amended, not rewritten**: add `## Amendment YYYY-MM-DD — <what changed>` at the end, and a line `Amended by ADR-NNNN (<summary>)` under the Status of an ADR that a later one changes.
- **A structural ADR** (a boundary, a dependency rule, a disk or wire format, the owner of a fact) has a `## Compliance` section: per rule, the check that fails when code breaks it, or `not checked by code: <why>`.

ADR-0001 to ADR-0014 predate the template: they get Kind, Protects, numbered rules and Compliance when they are next amended.
