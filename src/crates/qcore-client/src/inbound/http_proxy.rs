//! Local HTTP Proxy Inbound.
//!
//! Implements RFC 9110 HTTP/1.1 CONNECT tunneling, translating local application
//! requests into post-quantum encrypted QCore session frames.

use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use qcore_crypto::hybrid_kem::HybridClientKEM;
use qcore_crypto::share_link::ServerPayload;
use qcore_transport::TransportEngine;
use rand::{thread_rng, RngCore};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};

const SESSION_IDLE_TIMEOUT: Duration = Duration::from_secs(180);
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);

/// Inbound HTTP proxy listener.
pub struct HttpProxyServer;

impl HttpProxyServer {
    /// Binds the HTTP proxy listener to the specified address.
    pub async fn run(bind_addr: &str, server_payload: Arc<ServerPayload>) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        info!("HTTP Proxy listening on {}", bind_addr);

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let payload = server_payload.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(&mut stream, &payload).await {
                        debug!("HTTP proxy session ended: {}", e);
                    }
                });
            }
        }
    }

    /// Handles an individual HTTP CONNECT proxy request.
    async fn handle_client(stream: &mut TcpStream, payload: &ServerPayload) -> Result<()> {
        let server_stream = tokio::time::timeout(HANDSHAKE_TIMEOUT, async {
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                return Err(anyhow!("Empty HTTP request"));
            }

            let req_str = String::from_utf8_lossy(&buf[..n]);
            let first_line = req_str.lines().next().unwrap_or("");
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(anyhow!("Invalid HTTP request format"));
            }

            let method = parts[0];
            let target = parts[1];

            if method != "CONNECT" {
                return Err(anyhow!("Only HTTPS CONNECT tunneling is supported"));
            }

            let mut split = target.split(':');
            let target_host = split.next().unwrap_or("").to_string();
            let target_port: u16 = split.next().and_then(|p| p.parse().ok()).unwrap_or(443);

            let mut server_stream = TcpStream::connect(&payload.endpoint)
                .await
                .map_err(|e| anyhow!("Failed to connect to QCore server {}: {e}", payload.endpoint))?;

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
                .map_err(|e| anyhow!("Handshake encryption error: {e}"))?;

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
                return Err(anyhow!("Server rejected connection (status {status})"));
            }

            stream.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await?;
            stream.flush().await?;

            Ok::<TcpStream, anyhow::Error>(server_stream)
        }).await.map_err(|_| anyhow!("HTTP proxy negotiation timed out"))??;

        let client_stream = stream.try_clone()?;
        TransportEngine::bridge_connections(client_stream, server_stream, SESSION_IDLE_TIMEOUT).await?;
        Ok(())
    }
}