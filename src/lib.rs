use std::sync::Arc;

use camino::Utf8PathBuf;
use librqbit::Session;

pub mod info_hash;
pub mod server;

pub struct Config {
    pub session_output_dir: Utf8PathBuf,
    pub cache_dir: Utf8PathBuf,
}

pub struct AppState {
    pub session: Arc<Session>,
    pub config: Arc<Config>,
}
