#[expect(
    clippy::print_stderr,
    reason = "logging owns process output (ADR-0013)"
)]
pub(crate) fn info(message: &str) {
    eprintln!("{message}");
}
