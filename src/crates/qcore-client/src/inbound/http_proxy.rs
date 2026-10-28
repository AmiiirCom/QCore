use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use qcore_crypto::hybrid_kem::HybridClientKEM;
use qcore_crypto::share_link::ServerPayload;
use rand::{thread_rng, RngCore};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};

pub struct HttpProxyServer;

impl HttpProxyServer {
    pub async fn run(bind_addr: &str, server_payload: Arc<ServerPayload>) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        info!("پروکسی محلی HTTP روی آدرس {} فعال شد.", bind_addr);

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let payload = server_payload.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(&mut stream, &payload).await {
                        debug!("پایان سشن HTTP: {}", e);
                    }
                });
            }
        }
    }

    async fn handle_client(stream: &mut TcpStream, payload: &ServerPayload) -> Result<()> {
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        let req_str = String::from_utf8_lossy(&buf[..n]);
        let first_line = req_str.lines().next().unwrap_or("");
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(anyhow!("درخواست HTTP نامعتبر است"));
        }

        let method = parts[0];
        let target = parts[1];

        if method != "CONNECT" {
            return Err(anyhow!("تنها متد CONNECT برای تونلینگ HTTPS پشتیبانی می‌شود"));
        }

        let mut split = target.split(':');
        let target_host = split.next().unwrap_or("").to_string();
        let target_port: u16 = split.next().and_then(|p| p.parse().ok()).unwrap_or(443);

        let mut server_stream = TcpStream::connect(&payload.endpoint)
            .await
            .map_err(|e| anyhow!("خطا در اتصال به سرور QCore: {e}"))?;

        let (classic_pub, pq_ct, shared_secret) =
            HybridClientKEM::encapsulate(&payload.classic_pub, &payload.pq_pub)?;

        let meta = serde_json::json!({
            "token": payload.token,
            "uuid": payload.user_uuid,
            "host": target_host,
            "port": target_port
        });
        let meta_bytes = serde_json::to_vec(&meta)?;

        let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)?;
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), meta_bytes.as_ref())
            .map_err(|e| anyhow!("خطا در رمزنگاری فریم: {e}"))?;

        server_stream.write_all(b"QC\x01").await?;
        server_stream.write_all(&classic_pub).await?;
        server_stream.write_u16(pq_ct.len() as u16).await?;
        server_stream.write_all(&pq_ct).await?;
        server_stream.write_all(&nonce_bytes).await?;
        server_stream.write_u16(ciphertext.len() as u16).await?;
        server_stream.write_all(&ciphertext).await?;
        server_stream.flush().await?;

        let status = server_stream.read_u8().await?;
        if status != 0x00 {
            stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await?;
            return Err(anyhow!("سرور اتصال را رد کرد"));
        }

        stream.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await?;
        stream.flush().await?;

        tokio::io::copy_bidirectional(stream, &mut server_stream).await?;
        Ok(())
    }
}