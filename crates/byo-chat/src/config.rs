#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) listen_addr: String,
}

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:7878";

impl Config {
    #[expect(
        clippy::disallowed_methods,
        reason = "config owns reading the environment (ADR-0013)"
    )]
    pub(crate) fn from_env() -> Self {
        let listen_addr =
            std::env::var("BYO_LISTEN_ADDR").unwrap_or_else(|_| String::from(DEFAULT_LISTEN_ADDR));
        Self { listen_addr }
    }
}
