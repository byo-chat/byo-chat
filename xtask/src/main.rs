#![expect(
    clippy::print_stdout,
    reason = "the gate is a command-line tool: its report is its output"
)]
#![expect(
    clippy::disallowed_methods,
    reason = "the gate owns spawning the checks it runs (ADR-0013)"
)]

mod adr;
mod checks;

use std::process::{Command, ExitCode};

use checks::{Check, Pin};

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some("check") {
        return run_gate();
    }
    println!("usage: cargo xtask check");
    ExitCode::FAILURE
}

fn run_gate() -> ExitCode {
    let pins_ok = checks::PINS.iter().all(pin_matches);
    let base = merge_base();
    let results: Vec<bool> = checks::gate(&base).iter().map(run_check).collect();
    if pins_ok && results.iter().all(|ok| *ok) {
        println!("gate: pass");
        ExitCode::SUCCESS
    } else {
        println!("gate: FAIL");
        ExitCode::FAILURE
    }
}

fn pin_matches(pin: &Pin) -> bool {
    let output = Command::new(pin.program).args(pin.version_args).output();
    let found = output.map(|out| String::from_utf8_lossy(&out.stdout).into_owned());
    let ok = found.as_ref().is_ok_and(|text| text.contains(pin.version));
    let status = if ok { "ok" } else { "WRONG VERSION" };
    println!("pin   {:<14} {:<10} {status}", pin.program, pin.version);
    ok
}

fn run_check(check: &Check) -> bool {
    let mut command = Command::new(check.program);
    command.args(&check.args);
    for (key, value) in check.env {
        command.env(key, value);
    }
    let ok = command.status().is_ok_and(|status| status.success());
    let status = if ok { "ok" } else { "FAIL" };
    println!("check {:<24} {status}", check.name);
    ok
}

fn merge_base() -> String {
    ["origin/main", "main"]
        .iter()
        .find_map(|branch| {
            let out = Command::new("git")
                .args(["merge-base", "HEAD", branch])
                .output()
                .ok()?;
            out.status
                .success()
                .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
        })
        .unwrap_or_else(|| String::from("HEAD"))
}
