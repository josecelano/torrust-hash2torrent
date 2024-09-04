use std::sync::Arc;

use api::cache::Cache;
use bit_torrent::client::Client;
use config::Config;

pub mod api;
pub mod bit_torrent;
pub mod config;

pub struct AppState {
    pub config: Arc<Config>,
    pub client: Arc<Client>,
    pub cache: Arc<Cache>,
}

#[must_use]
pub fn run_app_for_integration_tests() -> String {
    "No integration tests yet :-(".to_string()
}
