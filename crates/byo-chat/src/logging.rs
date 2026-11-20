//! The only module allowed to write to stdout or stderr.

pub fn info(message: &str) {
    eprintln!("{message}");
}
