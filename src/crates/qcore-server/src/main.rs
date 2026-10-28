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
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use user_cli::{UserCliHandler, UserCommands};
use user_manager::UserManager;
use wizard::{ServerWizard, StoredServerConfig};
use x25519_dalek::StaticSecret;

#[derive(Parser)]
#[command(name = "qcore-server")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// ویزارد اولیه راه‌اندازی سرور
    Setup,
    /// مدیریت کاربران سرور (افزودن، لیست، حذف و صدور لینک)
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
        .map_err(|_| anyhow!("لطفاً ابتدا با دستور 'qcore-server setup' سرور را راه‌اندازی کنید."))?;
    let config: StoredServerConfig = serde_json::from_str(&config_data)?;

    // لود کردن تمامی کاربران ثبت‌شده در فایل تنظیمات
    let user_manager = UserManager::new();
    for u in &config.users {
        user_manager.add_user(u.clone()).await;
    }
    info!("تعداد {} کاربر با موفقیت در سیستم احراز هویت لود شدند.", config.users.len());

    let classic_secret_bytes: [u8; 32] = STANDARD.decode(&config.classic_secret_b64)?
        .try_into().map_err(|_| anyhow!("Invalid classic secret format"))?;
    let classic_secret = StaticSecret::from(classic_secret_bytes);

    let dk_bytes = STANDARD.decode(&config.pq_decaps_key_b64)?;
    let encoded_dk = Encoded::<DecapsulationKey<MlKem768Params>>::try_from(dk_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid ML-KEM key length"))?;
    let pq_decaps_key = Arc::new(DecapsulationKey::<MlKem768Params>::from_bytes(&encoded_dk));

    let bind_addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("سرور QCore آماده دریافت اتصالات روی آدرس {} است.", bind_addr);

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
    let mut magic = [0u8; 3];
    if socket.read_exact(&mut magic).await.is_err() || &magic != b"QC\x01" {
        if let Ok(mut fallback) = TcpStream::connect(format!("{}:443", fallback_domain)).await {
            let _ = tokio::io::copy_bidirectional(&mut socket, &mut fallback).await;
        }
        return Ok(());
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
        .map_err(|_| anyhow!("خطا در رمزگشایی متادیتا"))?;

    let meta: serde_json::Value = serde_json::from_slice(&decrypted)?;
    let token = meta["token"].as_str().unwrap_or("");
    let uuid = meta["uuid"].as_str().unwrap_or("");
    let host = meta["host"].as_str().unwrap_or("");
    let port = meta["port"].as_u64().unwrap_or(0) as u16;

    let user_state = match user_manager.authenticate(uuid, token).await {
        Some(u) => u,
        None => {
            socket.write_u8(0x01).await?;
            return Err(anyhow!("کاربر غیرمجاز یا منقضی شده"));
        }
    };

    let target_addr = format!("{}:{}", host, port);
    let mut target_stream = match TcpStream::connect(&target_addr).await {
        Ok(s) => s,
        Err(_) => {
            let _ = socket.write_u8(0x02).await;
            return Err(anyhow!("مقصد در دسترس نیست"));
        }
    };

    socket.write_u8(0x00).await?;
    socket.flush().await?;

    let (tx, rx) = tokio::io::copy_bidirectional(&mut socket, &mut target_stream).await?;
    UserManager::record_traffic(&user_state, tx + rx);
    info!("پایان نشست کاربر '{}' -> {} (ارسال: {} بایت، دریافت: {} بایت)", user_state.config.name, target_addr, tx, rx);
    Ok(())
}