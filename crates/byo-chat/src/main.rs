mod config;
mod logging;

fn main() {
    let config = config::Config::from_env();
    logging::info(&format!("byo-chat starting on {}", config.listen_addr));
}
