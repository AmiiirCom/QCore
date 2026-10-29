//! QCore Client Executable Entrypoint.
//!
//! Runs local RFC 1928 SOCKS5 and RFC 9110 HTTP CONNECT proxy listeners,
//! routing application streams across encrypted quantum-resistant tunnels.

mod ffi;
mod inbound;
mod wizard;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use inbound::http_proxy::HttpProxyServer;
use inbound::socks5::Socks5Server;
use qcore_crypto::share_link::ServerPayload;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use wizard::ClientWizard;

/// Command-line argument structure for the client executable.
#[derive(Parser)]
#[command(name = "qcore-client", about = "QCore Cross-Platform Client & Local Inbounds")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Listening address for SOCKS5 proxy inbound.
    #[arg(short, long, default_value = "127.0.0.1:10808")]
    socks_addr: String,

    /// Listening address for HTTP proxy inbound.
    #[arg(long, default_value = "127.0.0.1:10809")]
    http_addr: String,
}

/// Client subcommands.
#[derive(Subcommand)]
enum Commands {
    /// Run interactive setup wizard to import a share link.
    Setup,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    if let Some(Commands::Setup) = cli.command {
        match ClientWizard::run_interactive() {
            Ok(payload) => {
                info!("Client configured successfully for endpoint: {}", payload.endpoint);
                return Ok(());
            }
            Err(e) => {
                println!("[\x1b[31mx\x1b[0m] Error: {}", e);
                return Ok(());
            }
        }
    }

    let config_data = std::fs::read_to_string("client_config.json").map_err(|_| {
        anyhow!("Configuration file not found. Run 'qcore-client setup' first.")
    })?;
    let payload: ServerPayload = serde_json::from_str(&config_data)?;
    let shared_payload = Arc::new(payload);

    let socks_addr = cli.socks_addr.clone();
    let http_addr = cli.http_addr.clone();

    let socks_payload = shared_payload.clone();
    let socks_handle = tokio::spawn(async move {
        if let Err(e) = Socks5Server::run(&socks_addr, socks_payload).await {
            tracing::error!("SOCKS5 server error: {}", e);
        }
    });

    let http_payload = shared_payload.clone();
    let http_handle = tokio::spawn(async move {
        if let Err(e) = HttpProxyServer::run(&http_addr, http_payload).await {
            tracing::error!("HTTP proxy server error: {}", e);
        }
    });

    info!("QCore Client is operational.");
    info!("SOCKS5 Inbound active on {}", cli.socks_addr);
    info!("HTTP Proxy Inbound active on {}", cli.http_addr);

    let _ = tokio::try_join!(socks_handle, http_handle);
    Ok(())
}