# Writing tests

A test exists to fail when the behaviour it names breaks, and only then. Where tests go is decided in [ADR-0014](./adr/0014-test-seams.md); this file says what makes one worth keeping.

## Words

- **Mutation**: one small edit to production code that breaks the behaviour (delete a call, `>=` → `>`).
- **Red**: the test fails. **Seen red**: you watched it fail under a mutation.
- **Vacuous**: a pass that would also happen with the behaviour broken.
- **Control**: an assertion that proves a negative assertion could have failed.

## Definition of done

A test is done when every item holds:

1. **Seen red.** In the TDD loop the first red counts only when the assertion fails, not a compile error or a `todo!()`. The commit body names it: `red: <test> fails before <change>`.
2. **It drives production.** It calls the real entry point of its seam and asserts what comes out. Delete the line the test name promises, and it goes red.
3. **Negatives have a control.** Every "X did not happen" also shows X could happen: "not sent twice" sends once first and asserts the one delivery.
4. **Values are exact.** `assert_eq!` on the whole value, and the boundary value when the code has a boundary.
5. **It tests byo-chat.** Library behaviour (std, serde, rusqlite) belongs to its owner; test the byo-chat function that uses it.
6. **One path, one test.** Cases that differ only in input are rows of one table-driven test.
7. **It checks behaviour.** A refactor that keeps the behaviour keeps it green.

## What may be faked

Everything byo-chat owns runs for real, SQLite included (a temporary file). A fake replaces only an edge byo-chat does not own: a Connector, a Destination, the OS keychain, time and randomness (`Clock`, `Rng`). A Connector whose output a test must control is a recorded NDJSON fixture or a Rust helper binary run from an integration test through `CARGO_BIN_EXE_<name>`: a shell script child does not run on Windows.

## Security guards

A test that guards a token, an HMAC signature, the Agent write permission, Handoff, or the minimal environment of a Connector:

1. attempts the violation for real;
2. asserts the refusal;
3. includes a legitimate case in the same test and asserts it succeeds;
4. is seen red with the guard removed.
