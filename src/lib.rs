use std::sync::Arc;

use camino::Utf8PathBuf;
use librqbit::Session;
use server::cache::Cache;

pub mod info_hash;
pub mod server;

pub struct Config {
    pub session_output_dir: Utf8PathBuf,
    pub cache_dir: Utf8PathBuf,
}

pub struct AppState {
    pub config: Arc<Config>,
    pub session: Arc<Session>,
    pub cache: Arc<Cache>,
}
