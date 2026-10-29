//! QCore Server Application Entrypoint.
//!
//! Accepts incoming client connections, verifies post-quantum hybrid handshakes,
//! authenticates credentials, dynamically synchronizes user state, and proxies traffic.

mod user_cli;
mod user_manager;
mod wizard;

use anyhow::{anyhow, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use clap::{Parser, Subcommand};
use ml_kem::kem::DecapsulationKey;
use ml_kem::{Encoded, EncodedSizeUser, MlKem768Params};
use qcore_crypto::hybrid_kem::HybridServerKEM;
use qcore_transport::TransportEngine;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use user_cli::{UserCliHandler, UserCommands};
use user_manager::UserManager;
use wizard::{ServerWizard, StoredServerConfig};
use x25519_dalek::StaticSecret;

/// Inactivity timeout for active sessions (3 minutes).
const SESSION_IDLE_TIMEOUT: Duration = Duration::from_secs(180);

/// Handshake completion timeout (15 seconds).
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Parser)]
#[command(name = "qcore-server", about = "QCore High-Stealth Anti-Censorship Server")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run initial interactive server setup wizard.
    Setup,
    /// Manage user accounts and credentials.
    #[command(subcommand)]
    User(UserCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Setup) => {
            ServerWizard::run_interactive()?;
            return Ok(());
        }
        Some(Commands::User(user_cmd)) => {
            UserCliHandler::handle(user_cmd)?;
            return Ok(());
        }
        None => {}
    }

    let config_data = std::fs::read_to_string("server_config.json")
        .map_err(|_| anyhow!("Server configuration missing. Run 'qcore-server setup' first."))?;
    let config: StoredServerConfig = serde_json::from_str(&config_data)?;

    let user_manager = UserManager::new();
    for u in &config.users {
        user_manager.add_user(u.clone()).await;
    }
    info!("Loaded {} user account(s) into memory.", config.users.len());

    // Spawn background task for live config hot-reloading
    let watcher_user_manager = user_manager.clone();
    tokio::spawn(async move {
        let mut last_modified = std::fs::metadata("server_config.json")
            .and_then(|m| m.modified())
            .ok();

        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;

            if let Ok(metadata) = std::fs::metadata("server_config.json") {
                if let Ok(modified) = metadata.modified() {
                    if last_modified.map_or(true, |last| modified > last) {
                        last_modified = Some(modified);
                        if let Ok(data) = std::fs::read_to_string("server_config.json") {
                            if let Ok(reloaded) = serde_json::from_str::<StoredServerConfig>(&data) {
                                watcher_user_manager.sync_users(reloaded.users).await;
                                info!("Live reload: active user configurations updated from server_config.json");
                            }
                        }
                    }
                }
            }
        }
    });

    let classic_secret_bytes: [u8; 32] = STANDARD.decode(&config.classic_secret_b64)?
        .try_into().map_err(|_| anyhow!("Invalid classic secret format"))?;
    let classic_secret = StaticSecret::from(classic_secret_bytes);

    let dk_bytes = STANDARD.decode(&config.pq_decaps_key_b64)?;
    let encoded_dk = Encoded::<DecapsulationKey<MlKem768Params>>::try_from(dk_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid post-quantum decapsulation key format"))?;
    let pq_decaps_key = Arc::new(DecapsulationKey::<MlKem768Params>::from_bytes(&encoded_dk));

    let bind_addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("QCore Server listening on {}", bind_addr);

    let sni_fallback = config.sni.clone();

    loop {
        if let Ok((socket, _peer)) = listener.accept().await {
            let cs = classic_secret.clone();
            let pq_dk = pq_decaps_key.clone();
            let um = user_manager.clone();
            let fallback = sni_fallback.clone();

            tokio::spawn(async move {
                let _ = handle_connection(socket, cs, pq_dk, um, fallback).await;
            });
        }
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    classic_secret: StaticSecret,
    pq_decaps_key: Arc<DecapsulationKey<MlKem768Params>>,
    user_manager: UserManager,
    fallback_domain: String,
) -> Result<()> {
    // Enforce handshake completion timeout
    let (target_stream, user_state) = tokio::time::timeout(HANDSHAKE_TIMEOUT, async {
        let mut magic = [0u8; 3];
        if socket.read_exact(&mut magic).await.is_err() || &magic != b"QC\x01" {
            if let Ok(fallback) = TcpStream::connect(format!("{}:443", fallback_domain)).await {
                let _ = TransportEngine::bridge_connections(socket, fallback, SESSION_IDLE_TIMEOUT).await;
            }
            return Err(anyhow!("Fallback redirection"));
        }

        let mut client_classic = [0u8; 32];
        socket.read_exact(&mut client_classic).await?;

        let pq_len = socket.read_u16().await? as usize;
        let mut pq_ct = vec![0u8; pq_len];
        socket.read_exact(&mut pq_ct).await?;

        let shared_secret = HybridServerKEM::decapsulate(
            &classic_secret,
            &pq_decaps_key,
            &client_classic,
            &pq_ct,
        )?;

        let mut nonce_bytes = [0u8; 12];
        socket.read_exact(&mut nonce_bytes).await?;
        let cipher_len = socket.read_u16().await? as usize;
        let mut ciphertext = vec![0u8; cipher_len];
        socket.read_exact(&mut ciphertext).await?;

        let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)?;
        let decrypted = cipher
            .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref())
            .map_err(|_| anyhow!("Failed to decrypt handshake metadata"))?;

        let meta: serde_json::Value = serde_json::from_slice(&decrypted)?;
        let token = meta["token"].as_str().unwrap_or("");
        let uuid = meta["uuid"].as_str().unwrap_or("");
        let host = meta["host"].as_str().unwrap_or("");
        let port = meta["port"].as_u64().unwrap_or(0) as u16;

        let user_state = match user_manager.authenticate(uuid, token).await {
            Some(u) => u,
            None => {
                socket.write_u8(0x01).await?;
                return Err(anyhow!("Unauthorized or expired user"));
            }
        };

        let target_addr = format!("{}:{}", host, port);
        let target_stream = match TcpStream::connect(&target_addr).await {
            Ok(s) => s,
            Err(_) => {
                let _ = socket.write_u8(0x02).await;
                return Err(anyhow!("Target unreachable"));
            }
        };

        socket.write_u8(0x00).await?;
        socket.flush().await?;

        Ok::<(TcpStream, Arc<user_manager::UserState>), anyhow::Error>((target_stream, user_state))
    }).await.map_err(|_| anyhow!("Handshake negotiation timed out"))??;

    // Full-duplex forwarding with idle timeout protection
    let (tx, rx) = TransportEngine::bridge_connections(socket, target_stream, SESSION_IDLE_TIMEOUT).await?;
    UserManager::record_traffic(&user_state, tx + rx);
    info!(
        "Session closed for user '{}' (tx: {} bytes, rx: {} bytes)",
        user_state.config.name, tx, rx
    );
    Ok(())
}