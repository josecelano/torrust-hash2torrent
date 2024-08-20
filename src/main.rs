use anyhow::Context;
use librqbit::Session;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use torrust_hash2torrent::server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    let output_dir = "./storage";

    // Create the session

    info!("creating BitTorrent client session ...");

    let session = Session::new(output_dir.into())
        .await
        .context("error creating session")?;

    // Start the HTTP server

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    info!("starting server on: http://{server_addr} ..."); // DevSkim: ignore DS137138

    server::start(&server_addr, session).await;

    Ok(())
}
