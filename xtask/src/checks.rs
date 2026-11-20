pub(crate) struct Pin {
    pub(crate) program: &'static str,
    pub(crate) version_args: &'static [&'static str],
    pub(crate) version: &'static str,
}

pub(crate) const PINS: &[Pin] = &[
    Pin {
        program: "cargo-deny",
        version_args: &["deny", "--version"],
        version: "0.22.0",
    },
    Pin {
        program: "cargo-machete",
        version_args: &["--version"],
        version: "0.10.1",
    },
    Pin {
        program: "typos",
        version_args: &["--version"],
        version: "1.52.2",
    },
    Pin {
        program: "taplo",
        version_args: &["--version"],
        version: "0.10.0",
    },
    Pin {
        program: "zizmor",
        version_args: &["--version"],
        version: "1.32.0",
    },
];

pub(crate) struct Check {
    pub(crate) name: &'static str,
    pub(crate) program: &'static str,
    pub(crate) args: Vec<String>,
    pub(crate) env: &'static [(&'static str, &'static str)],
}

const SCRIPTS: &str = "docs/lint-scripts";

fn check(name: &'static str, program: &'static str, args: &[&str]) -> Check {
    Check {
        name,
        program,
        args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        env: &[],
    }
}

pub(crate) fn gate(merge_base: &str) -> Vec<Check> {
    let tally = format!("{SCRIPTS}/tally.py");
    let gate_tests = format!("{SCRIPTS}/gate.py");
    let comments = ["comments", "crates", "xtask", "--ext", ".rs"];
    let mut new_comments: Vec<&str> = vec![&tally];
    new_comments.extend(comments);
    new_comments.extend(["--base", merge_base]);
    let mut held_comments: Vec<&str> = vec![&tally];
    held_comments.extend(comments);
    held_comments.extend(["--expect", "0"]);
    vec![
        check("format", "cargo", &["fmt", "--all", "--check"]),
        check(
            "clippy",
            "cargo",
            &[
                "clippy",
                "--locked",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        Check {
            env: &[("RUSTDOCFLAGS", "-D warnings")],
            ..check(
                "docs",
                "cargo",
                &["doc", "--locked", "--no-deps", "--workspace"],
            )
        },
        check("tests", "cargo", &["test", "--locked", "--workspace"]),
        check("dependencies", "cargo", &["deny", "--locked", "check"]),
        check("unused deps", "cargo-machete", &[]),
        check("typos", "typos", &[]),
        check("toml format", "taplo", &["fmt", "--check"]),
        check("workflows", "zizmor", &["--offline", ".github"]),
        check("no new comment", "python", &new_comments),
        check("comment count", "python", &held_comments),
        check("gate scripts", "python", &[&gate_tests, "--tests"]),
    ]
}

#[cfg(test)]
mod tests {
    use super::PINS;

    const CI: &str = include_str!("../../.github/workflows/ci.yml");

    #[test]
    fn ci_runs_the_local_gate() {
        assert!(CI.contains("cargo xtask check"));
    }

    #[test]
    fn ci_installs_every_tool_at_its_pin() {
        for pin in PINS {
            let line = format!("{}@{}", pin.program, pin.version);
            assert!(CI.contains(&line), "ci.yml lacks {line}");
        }
    }
}
