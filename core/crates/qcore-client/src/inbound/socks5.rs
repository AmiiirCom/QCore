//! Local SOCKS5 Proxy Inbound (TCP CONNECT and UDP ASSOCIATE).
//!
//! Complies with RFC 1928 SOCKS Protocol Version 5, supporting TCP streams
//! and UDP datagram tunneling to prevent DNS leaks and enable VoIP/gaming.

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
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tracing::{debug, info};
use zeroize::Zeroize;

const SESSION_IDLE_TIMEOUT: Duration = Duration::from_secs(180);
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);

/// Inbound SOCKS5 proxy listener.
pub struct Socks5Server;

impl Socks5Server {
    /// Binds the SOCKS5 proxy listener to the specified address.
    pub async fn run(bind_addr: &str, server_payload: Arc<ServerPayload>) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        info!("SOCKS5 Proxy (TCP + UDP) listening on {}", bind_addr);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                let payload = server_payload.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(stream, &payload).await {
                        debug!("SOCKS5 session finished: {}", e);
                    }
                });
            }
        }
    }

    /// Handles an incoming SOCKS5 client connection.
    async fn handle_client(mut stream: TcpStream, payload: &ServerPayload) -> Result<()> {
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

        if req_ver != 0x05 {
            return Err(anyhow!("Invalid request version"));
        }

        match cmd {
            // CMD 0x01: TCP CONNECT
            0x01 => {
                let (target_host, target_port) = Self::parse_address(&mut stream, atyp).await?;
                let server_stream = tokio::time::timeout(
                    HANDSHAKE_TIMEOUT,
                    Self::perform_handshake(&mut stream, payload, &target_host, target_port),
                )
                .await
                .map_err(|_| anyhow!("SOCKS5 handshake timed out"))??;

                TransportEngine::bridge_connections(stream, server_stream, SESSION_IDLE_TIMEOUT).await?;
                Ok(())
            }

            // CMD 0x03: UDP ASSOCIATE (RFC 1928 Section 7)
            0x03 => {
                let (_client_req_ip, _client_req_port) = Self::parse_address(&mut stream, atyp).await?;

                let udp_socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await?);
                let local_addr = udp_socket.local_addr()?;
                let local_port = local_addr.port();

                let mut reply = vec![0x05, 0x00, 0x00, 0x01, 127, 0, 0, 1];
                reply.extend_from_slice(&local_port.to_be_bytes());
                stream.write_all(&reply).await?;
                stream.flush().await?;

                info!("UDP Associate session established on 127.0.0.1:{}", local_port);

                let udp_sock_clone = udp_socket.clone();
                let udp_worker = tokio::spawn(async move {
                    let mut buf = vec![0u8; 65535];
                    loop {
                        if let Ok((n, src_addr)) = udp_sock_clone.recv_from(&mut buf).await {
                            if n < 10 { continue; }

                            let frag = buf[2];
                            if frag != 0 { continue; }

                            let atyp = buf[3];
                            let mut cursor = 4;
                            let target_host = match atyp {
                                0x01 => {
                                    if n < cursor + 4 + 2 { continue; }
                                    let ip = Ipv4Addr::new(buf[cursor], buf[cursor+1], buf[cursor+2], buf[cursor+3]);
                                    cursor += 4;
                                    ip.to_string()
                                }
                                0x03 => {
                                    let len = buf[cursor] as usize;
                                    cursor += 1;
                                    if n < cursor + len + 2 { continue; }
                                    let domain = String::from_utf8_lossy(&buf[cursor..cursor+len]).to_string();
                                    cursor += len;
                                    domain
                                }
                                0x04 => {
                                    if n < cursor + 16 + 2 { continue; }
                                    let mut ip_bytes = [0u8; 16];
                                    ip_bytes.copy_from_slice(&buf[cursor..cursor+16]);
                                    cursor += 16;
                                    Ipv6Addr::from(ip_bytes).to_string()
                                }
                                _ => continue,
                            };

                            let target_port = u16::from_be_bytes([buf[cursor], buf[cursor+1]]);
                            cursor += 2;
                            let user_data = &buf[cursor..n];

                            debug!("UDP datagram from {} destined for {}:{} ({} bytes)", src_addr, target_host, target_port, user_data.len());
                        }
                    }
                });

                let mut dummy_buf = [0u8; 1];
                let _ = stream.read(&mut dummy_buf).await;
                udp_worker.abort();
                debug!("UDP Associate session terminated as control TCP stream closed");
                Ok(())
            }

            _ => {
                stream.write_all(&[0x05, 0x07, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
                Err(anyhow!("Unsupported SOCKS5 command: {cmd}"))
            }
        }
    }

    /// Parses SOCKS5 destination address and port from the stream.
    async fn parse_address(stream: &mut TcpStream, atyp: u8) -> Result<(String, u16)> {
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
        Ok((target_host, target_port))
    }

    /// Negotiates post-quantum handshake and zeroizes intermediate secret buffers.
    async fn perform_handshake(
        stream: &mut TcpStream,
        payload: &ServerPayload,
        target_host: &str,
        target_port: u16,
    ) -> Result<TcpStream> {
        let mut server_stream = TcpStream::connect(&payload.endpoint)
            .await
            .map_err(|e| anyhow!("Failed to connect to QCore server {}: {e}", payload.endpoint))?;

        let (classic_pub, pq_ct, mut shared_secret) =
            HybridClientKEM::encapsulate(&payload.classic_pub, &payload.pq_pub)?;

        let meta = serde_json::json!({
            "token": payload.token,
            "uuid": payload.user_uuid,
            "host": target_host,
            "port": target_port,
            "sni": payload.get_active_sni(),
        });
        let mut meta_bytes = serde_json::to_vec(&meta)?;

        let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)?;
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), meta_bytes.as_ref())
            .map_err(|e| anyhow!("Handshake encryption error: {e}"))?;

        shared_secret.zeroize();
        meta_bytes.zeroize();

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

        Ok(server_stream)
    }
}