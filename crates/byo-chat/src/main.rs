//! byo-chat binary: composition root, daemon and console subcommands (ADR-0004).

mod config;
mod logging;

fn main() {
    let config = config::Config::from_env();
    logging::info(&format!("byo-chat starting on {}", config.listen_addr));
}
