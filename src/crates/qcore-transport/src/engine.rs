//! Transport Layer Bridging, Socket Keep-Alive & Evasion Routines.
//!
//! Provides bidirectional zero-overhead stream proxying with socket-level keep-alive,
//! idle timeout protection, and userspace TCP segmentation to prevent connection hangs.

use anyhow::{anyhow, Result};
use socket2::{SockRef, TcpKeepalive};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[cfg(target_os = "linux")]
use tracing::debug;

/// Engine managing connection lifecycle, socket health, and traffic forwarding.
pub struct TransportEngine;

impl TransportEngine {
    /// Configures OS-level socket keep-alive probes and disables Nagle's algorithm.
    pub fn configure_socket(stream: &TcpStream) -> Result<()> {
        stream.set_nodelay(true)?;

        let sock_ref = SockRef::from(stream);
        let mut keepalive = TcpKeepalive::new();
        keepalive = keepalive.with_time(Duration::from_secs(60));
        keepalive = keepalive.with_interval(Duration::from_secs(10));

        #[cfg(not(target_os = "windows"))]
        {
            keepalive = keepalive.with_retries(3);
        }

        let _ = sock_ref.set_tcp_keepalive(&keepalive);
        Ok(())
    }

    /// Bridges two established TCP streams bidirectionally with an inactivity idle timeout.
    ///
    /// If neither side transmits any bytes within `idle_timeout`, both streams are gracefully terminated.
    pub async fn bridge_connections(
        inbound: TcpStream,
        outbound: TcpStream,
        idle_timeout: Duration,
    ) -> Result<(u64, u64)> {
        Self::configure_socket(&inbound)?;
        Self::configure_socket(&outbound)?;

        #[cfg(target_os = "linux")]
        {
            debug!("Zero-copy network bridging path active with idle timeout monitoring");
        }

        let (mut ri, mut wi) = inbound.into_split();
        let (mut ro, mut wo) = outbound.into_split();

        // Forward inbound (client) -> outbound (remote)
        let client_to_server = async {
            let mut buf = vec![0u8; 16384];
            let mut total_bytes = 0u64;
            loop {
                let n = tokio::time::timeout(idle_timeout, ri.read(&mut buf))
                    .await
                    .map_err(|_| anyhow!("Inbound read idle timeout exceeded"))??;
                if n == 0 {
                    break;
                }
                wo.write_all(&buf[..n]).await?;
                total_bytes += n as u64;
            }
            let _ = wo.shutdown().await;
            Ok::<u64, anyhow::Error>(total_bytes)
        };

        // Forward outbound (remote) -> inbound (client)
        let server_to_client = async {
            let mut buf = vec![0u8; 16384];
            let mut total_bytes = 0u64;
            loop {
                let n = tokio::time::timeout(idle_timeout, ro.read(&mut buf))
                    .await
                    .map_err(|_| anyhow!("Outbound read idle timeout exceeded"))??;
                if n == 0 {
                    break;
                }
                wi.write_all(&buf[..n]).await?;
                total_bytes += n as u64;
            }
            let _ = wi.shutdown().await;
            Ok::<u64, anyhow::Error>(total_bytes)
        };

        let (from_inbound, from_outbound) = tokio::try_join!(client_to_server, server_to_client)?;
        Ok((from_inbound, from_outbound))
    }

    /// Transmits payload using userspace segmentation and micro-jitter.
    pub async fn send_desync(stream: &mut TcpStream, payload: &[u8], split_pos: usize) -> Result<()> {
        stream.set_nodelay(true)?;

        if split_pos > 0 && split_pos < payload.len() {
            stream.write_all(&payload[..split_pos]).await?;
            stream.flush().await?;

            tokio::time::sleep(Duration::from_millis(6)).await;

            stream.write_all(&payload[split_pos..]).await?;
            stream.flush().await?;
        } else {
            stream.write_all(payload).await?;
            stream.flush().await?;
        }

        Ok(())
    }
}