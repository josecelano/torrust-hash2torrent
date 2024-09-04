//! Minimal `curl` or `wget` to be used for container health checks.
//!
//! It's convenient to avoid using third-party libraries because:
//!
//! - They are harder to maintain.
//! - They introduce new attack vectors.
use std::time::Duration;
use std::{env, process};

use reqwest::Client;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage:   cargo run --bin http_health_check <HEALTH_URL>");
        eprintln!("Example: cargo run --bin http_health_check http://127.0.0.1:3000/health_check");
        std::process::exit(1);
    }

    println!("Health check ...");

    let url = &args[1].clone();

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("STATUS: {}", response.status());
                process::exit(0);
            } else {
                println!("Non-success status received.");
                process::exit(1);
            }
        }
        Err(err) => {
            println!("ERROR: {err}");
            process::exit(1);
        }
    }
}
