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

#[derive(Parser)]
#[command(name = "qcore-client")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, default_value = "127.0.0.1:10808")]
    socks_addr: String,

    #[arg(long, default_value = "127.0.0.1:10809")]
    http_addr: String,
}

#[derive(Subcommand)]
enum Commands {
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
                info!("کلاینت با موفقیت برای سرور {} پیکربندی شد.", payload.endpoint);
                return Ok(());
            }
            Err(e) => {
                println!("[\x1b[31mx\x1b[0m] خطا: {}", e);
                return Ok(());
            }
        }
    }

    let config_data = std::fs::read_to_string("client_config.json").map_err(|_| {
        anyhow!("فایل کانفیگ یافت نشد. لطفاً ابتدا دستور 'qcore-client setup' را اجرا کنید!")
    })?;
    let payload: ServerPayload = serde_json::from_str(&config_data)?;
    let shared_payload = Arc::new(payload);

    let socks_addr = cli.socks_addr.clone();
    let http_addr = cli.http_addr.clone();

    let socks_payload = shared_payload.clone();
    let socks_handle = tokio::spawn(async move {
        if let Err(e) = Socks5Server::run(&socks_addr, socks_payload).await {
            tracing::error!("خطا در اینباند SOCKS5: {}", e);
        }
    });

    let http_payload = shared_payload.clone();
    let http_handle = tokio::spawn(async move {
        if let Err(e) = HttpProxyServer::run(&http_addr, http_payload).await {
            tracing::error!("خطا در اینباند HTTP: {}", e);
        }
    });

    info!("کلاینت QCore آماده به کار است.");
    info!("اینباند SOCKS5 فعال روی: {}", cli.socks_addr);
    info!("اینباند HTTP Proxy فعال روی: {}", cli.http_addr);

    let _ = tokio::try_join!(socks_handle, http_handle);
    Ok(())
}