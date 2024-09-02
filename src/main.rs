use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use torrust_hash2torrent::bit_torrent::client::Client;
use torrust_hash2torrent::config::Config;
use torrust_hash2torrent::{
    api::{self, cache::Cache},
    AppState,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    let session_output_dir = "./storage/session";
    let cache_dir = "./storage/torrents";

    let config = Config {
        session_output_dir: session_output_dir.into(),
        cache_dir: cache_dir.into(),
    };

    info!("creating BitTorrent client and starting the session ...");

    let mut client = Client::new(session_output_dir.into());
    client.start_session().await?;

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    info!("starting HTTP server on: http://{server_addr} ..."); // DevSkim: ignore DS137138

    let app_state = AppState {
        config: Arc::new(config),
        client: Arc::new(client),
        cache: Arc::new(Cache::new(cache_dir.into())),
    };

    api::start(&server_addr, app_state).await;

    Ok(())
}
