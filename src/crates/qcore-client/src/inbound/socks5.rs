//! Local SOCKS5 Proxy Inbound.
//!
//! Implements RFC 1928 SOCKS Protocol Version 5 (CONNECT command, IPv4, IPv6,
//! and domain name resolution), forwarding sessions over post-quantum tunnels.

use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use qcore_crypto::hybrid_kem::HybridClientKEM;
use qcore_crypto::share_link::ServerPayload;
use qcore_transport::TransportEngine;
use rand::{thread_rng, RngCore};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};

const SESSION_IDLE_TIMEOUT: Duration = Duration::from_secs(180);
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);

/// Inbound SOCKS5 proxy listener.
pub struct Socks5Server;

impl Socks5Server {
    /// Binds the SOCKS5 proxy listener to the specified address.
    pub async fn run(bind_addr: &str, server_payload: Arc<ServerPayload>) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        info!("SOCKS5 Proxy listening on {}", bind_addr);

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let payload = server_payload.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(&mut stream, &payload).await {
                        debug!("SOCKS5 session ended: {}", e);
                    }
                });
            }
        }
    }

    /// Handles an individual SOCKS5 client session according to RFC 1928.
    async fn handle_client(stream: &mut TcpStream, payload: &ServerPayload) -> Result<()> {
        let server_stream = tokio::time::timeout(HANDSHAKE_TIMEOUT, async {
            // Step 1: Version negotiation
            let ver = stream.read_u8().await?;
            if ver != 0x05 {
                return Err(anyhow!("Unsupported SOCKS version: {ver}"));
            }
            let nmethods = stream.read_u8().await?;
            let mut methods = vec![0u8; nmethods as usize];
            stream.read_exact(&mut methods).await?;
            stream.write_all(&[0x05, 0x00]).await?;
            stream.flush().await?;

            // Step 2: Request evaluation
            let req_ver = stream.read_u8().await?;
            let cmd = stream.read_u8().await?;
            let _rsv = stream.read_u8().await?;
            let atyp = stream.read_u8().await?;

            if req_ver != 0x05 || cmd != 0x01 {
                stream.write_all(&[0x05, 0x07, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
                return Err(anyhow!("Only SOCKS5 CONNECT command is supported"));
            }

            let target_host = match atyp {
                0x01 => {
                    let mut ip = [0u8; 4];
                    stream.read_exact(&mut ip).await?;
                    Ipv4Addr::from(ip).to_string()
                }
                0x03 => {
                    let len = stream.read_u8().await?;
                    let mut domain = vec![0u8; len as usize];
                    stream.read_exact(&mut domain).await?;
                    String::from_utf8(domain)?
                }
                0x04 => {
                    let mut ip = [0u8; 16];
                    stream.read_exact(&mut ip).await?;
                    Ipv6Addr::from(ip).to_string()
                }
                _ => return Err(anyhow!("Unsupported address type: {atyp}")),
            };

            let target_port = stream.read_u16().await?;

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
                stream.write_all(&[0x05, 0x05, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
                return Err(anyhow!("Server rejected connection (status {status})"));
            }

            stream.write_all(&[0x05, 0x00, 0x00, 0x01, 127, 0, 0, 1, 0x2a, 0x38]).await?;
            stream.flush().await?;

            Ok::<TcpStream, anyhow::Error>(server_stream)
        }).await.map_err(|_| anyhow!("SOCKS5 negotiation timed out"))??;

        // Take ownership of stream and forward through TransportEngine
        let client_stream = stream.try_clone()?;
        TransportEngine::bridge_connections(client_stream, server_stream, SESSION_IDLE_TIMEOUT).await?;
        Ok(())
    }
}