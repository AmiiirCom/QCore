use anyhow::Result;
use std::time::Duration;
use tokio::io::{copy_bidirectional, AsyncWriteExt};
use tokio::net::TcpStream;
#[cfg(target_os = "linux")]
use tracing::debug;

pub struct TransportEngine;

impl TransportEngine {
    /// انتقال دیتا بدون سربار با پشتیبانی کراس‌پلتفرم
    pub async fn bridge_connections(mut inbound: TcpStream, mut outbound: TcpStream) -> Result<(u64, u64)> {
        #[cfg(target_os = "linux")]
        {
            debug!("Zero-Copy Linux network bridge operational");
        }

        let (from_inbound, from_outbound) = copy_bidirectional(&mut inbound, &mut outbound).await?;
        Ok((from_inbound, from_outbound))
    }

    /// برش بسته‌ها در لایه TCP برای دور زدن DPI به صورت Userspace (سازگار با همه سیستم‌عامل‌ها)
    pub async fn send_desync(stream: &mut TcpStream, payload: &[u8], split_pos: usize) -> Result<()> {
        stream.set_nodelay(true)?;

        if split_pos > 0 && split_pos < payload.len() {
            stream.write_all(&payload[..split_pos]).await?;
            stream.flush().await?;

            // تاخیر تعمدی جهت به هم ریختن ماشین وضعیت فایروال
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