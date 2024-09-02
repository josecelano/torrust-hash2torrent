use anyhow::Context;
use librqbit::Session;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use torrust_hash2torrent::{server, AppState, Config};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    let config = Config {
        session_output_dir: "./storage/session".into(),
        cache_dir: "./storage/torrents".into(),
    };

    // Create the session

    info!("creating BitTorrent client session ...");

    let session = Session::new(config.session_output_dir.clone().into())
        .await
        .context("error creating session")?;

    // Start the HTTP server

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    info!("starting server on: http://{server_addr} ..."); // DevSkim: ignore DS137138

    let app_state = AppState {
        session,
        config: Arc::new(config),
    };

    server::start(&server_addr, app_state).await;

    Ok(())
}
