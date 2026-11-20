//! The only module allowed to read the process environment.

pub struct Config {
    pub listen_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        let listen_addr =
            std::env::var("BYO_LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:7878"));
        Self { listen_addr }
    }
}
