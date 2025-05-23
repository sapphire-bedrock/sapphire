mod error;

use crate::error::*;
use network::error::NetworkError;
use network::raknet::listener::Listener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Hello, World!");

    let address = "0.0.0.0:19132"
        .parse::<SocketAddr>()
        .map_err(|e| NetworkError::Custom(format!("Invalid address: {}", e)))?;

    let mut listener = Listener::bind(address).await?;

    listener.run().await?;

    Ok(())
}
