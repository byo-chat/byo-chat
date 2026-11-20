# ast-grep

ast-grep reads code as a syntax tree. `ast-grep` runs it at the version the repo pins (0.51.3, installed with `cargo install --locked ast-grep@0.51.3`). It sees syntax only: no types, no imports, nothing across files. A match is a candidate; the compiler and the linters decide what it means.

## Look before you read

Before reading a whole source file, outline it, then read only the range you need:

    ast-grep outline <file>                    # items and their members, with line numbers
    ast-grep outline <dir> --items exports     # the public surface of a folder
    ast-grep outline <file> --items imports    # what a file depends on

## Search by shape

Search with ast-grep when the question is about the shape of code: every call of a function with a literal argument, every `impl` of a trait. Search with grep when it is about text: a name, a message, a file.

    ast-grep run -p '<pattern>' -l rust <paths>
    ast-grep run -p '<pattern>' -l rust <paths> --json=stream   # one match per line, to count

The pattern is Rust code: `$X` matches one node, `$$$ARGS` any number of them.

A pattern that matches nothing is not proof the code is absent:

- A path is part of the pattern: `Command::new($P)` does not match `std::process::Command::new(...)`. Write the path as a metavariable (`$C::new($P)`) and read the matches.
- A pattern does not match inside a macro's arguments (`assert_eq!(x.unwrap(), 1)`). Search those with grep.
- `--debug-query=pattern` prints how ast-grep parsed the pattern; test it on a snippet you know contains the code.

## Rewrite

One shape changed at many sites:

    ast-grep run -p '<old>' -r '<new>' -l rust <paths>      # prints the diff, changes nothing
    ast-grep run -p '<old>' -r '<new>' -l rust <paths> -U   # applies it

Read the diff before `-U`. The rewrite drops the line breaks of the code it replaces: run `cargo fmt --all` after it. A rewrite goes in a commit of its own, with the command in its message, and the gate passes after it.

For rule syntax and anything else, see ast-grep's docs for agents: https://ast-grep.github.io/llms.txt
