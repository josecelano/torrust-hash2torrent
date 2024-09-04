use std::net::SocketAddr;

use camino::Utf8PathBuf;

#[derive(Clone)]
pub struct Config {
    pub api: Api,
    pub client: Client,
}

#[derive(Clone)]
pub struct Client {
    pub listen_port_range: Option<std::ops::Range<u16>>,
    pub session_output_dir: Utf8PathBuf,
}

#[derive(Clone)]
pub struct Api {
    pub bind_address: SocketAddr,
    pub torrents_cache_dir: Utf8PathBuf,
}
