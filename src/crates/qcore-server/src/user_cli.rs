use anyhow::{anyhow, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use clap::Subcommand;
use qcore_crypto::share_link::{SecureLinkEngine, ServerPayload};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::user_manager::UserConfig;
use crate::wizard::StoredServerConfig;

#[derive(Subcommand)]
pub enum UserCommands {
    /// افزودن کاربر جدید همراه با سهمیه حجم و زمان و صدور لینک
    Add {
        /// نام کاربر
        name: String,
        /// سهمیه حجم به گیگابایت (0 = نامحدود)
        #[arg(short, long, default_value = "0")]
        quota_gb: u64,
        /// مدت اعتبار به روز (0 = نامحدود)
        #[arg(short, long, default_value = "0")]
        days: u64,
        /// رمز عبور لینک اشتراک (در صورت خالی بودن پرسیده می‌شود)
        #[arg(short, long)]
        password: Option<String>,
    },
    /// نمایش لیست کاربران سرور، سهمیه و وضعیت
    List,
    /// حذف یک کاربر بر اساس نام یا UUID
    Remove {
        id_or_name: String,
    },
    /// صدور مجدد لینک اشتراک رمزشده برای کاربر موجود
    Link {
        id_or_name: String,
        #[arg(short, long)]
        password: Option<String>,
    },
}

pub struct UserCliHandler;

impl UserCliHandler {
    pub fn handle(cmd: UserCommands) -> Result<()> {
        let config_data = std::fs::read_to_string("server_config.json")
            .map_err(|_| anyhow!("ابتدا باید سرور را با دستور 'qcore-server setup' راه‌اندازی کنید."))?;
        let mut config: StoredServerConfig = serde_json::from_str(&config_data)?;

        match cmd {
            UserCommands::Add { name, quota_gb, days, password } => {
                Self::add_user(&mut config, name, quota_gb, days, password)?;
            }
            UserCommands::List => {
                Self::list_users(&config);
            }
            UserCommands::Remove { id_or_name } => {
                Self::remove_user(&mut config, &id_or_name)?;
            }
            UserCommands::Link { id_or_name, password } => {
                Self::generate_link(&config, &id_or_name, password)?;
            }
        }

        Ok(())
    }

    fn add_user(
        config: &mut StoredServerConfig,
        name: String,
        quota_gb: u64,
        days: u64,
        password: Option<String>,
    ) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expiry = if days > 0 { now + (days * 86400) } else { 0 };
        let quota_bytes = quota_gb * 1024 * 1024 * 1024;

        let user_uuid = Uuid::new_v4().to_string();
        let token = Uuid::new_v4().to_string();

        let pass = match password {
            Some(p) => p,
            None => {
                print!("[?] رمز عبور برای لینک رمزشده کاربر '{}' را وارد کنید: ", name);
                io::stdout().flush()?;
                let mut p = String::new();
                io::stdin().read_line(&mut p)?;
                p.trim().to_string()
            }
        };

        let new_user = UserConfig {
            uuid: user_uuid.clone(),
            name: name.clone(),
            token: token.clone(),
            quota_bytes,
            expiry_timestamp: expiry,
            enabled: true,
        };

        config.users.push(new_user);
        std::fs::write("server_config.json", serde_json::to_string_pretty(config)?)?;

        // ساخت لینک اختصاصی کاربر
        let classic_pub_bytes: [u8; 32] = STANDARD.decode(&config.classic_public_b64)?
            .try_into().map_err(|_| anyhow!("Invalid classic public key"))?;
        let pq_pub = STANDARD.decode(&config.pq_public_b64)?;

        let payload = ServerPayload {
            endpoint: format!("{}:{}", config.server_ip, config.port),
            sni: config.sni.clone(),
            user_uuid,
            token,
            classic_pub: classic_pub_bytes,
            pq_pub,
            transport: "reality-pq".into(),
        };

        let link = SecureLinkEngine::export_symmetric(&payload, &pass)?;

        println!("\n============================================================");
        println!("             کاربر با موفقیت اضافه شد                      ");
        println!("============================================================");
        println!("نام کاربر  : {}", name);
        println!("سهمیه حجم : {}", if quota_gb > 0 { format!("{} گیگابایت", quota_gb) } else { "نامحدود".into() });
        println!("اعتبار    : {}", if days > 0 { format!("{} روز", days) } else { "نامحدود".into() });
        println!("\nلینک اشتراک رمزشده کاربر:");
        println!("\x1b[36m{}\x1b[0m", link);
        println!("(این لینک را به همراه رمزی که وارد کردید به کاربر تحویل دهید)");
        println!("============================================================\n");

        Ok(())
    }

    fn list_users(config: &StoredServerConfig) {
        println!("\n{:<20} {:<38} {:<15} {:<10}", "نام کاربر", "شناسه UUID", "سهمیه حجم", "وضعیت");
        println!("{:-<88}", "");
        for u in &config.users {
            let quota = if u.quota_bytes > 0 {
                format!("{:.1} GB", u.quota_bytes as f64 / (1024.0 * 1024.0 * 1024.0))
            } else {
                "نامحدود".to_string()
            };
            let status = if u.enabled { "فعال" } else { "مسدود" };
            println!("{:<20} {:<38} {:<15} {:<10}", u.name, u.uuid, quota, status);
        }
        println!("{:-<88}\n", "");
    }

    fn remove_user(config: &mut StoredServerConfig, id_or_name: &str) -> Result<()> {
        let initial_len = config.users.len();
        config.users.retain(|u| u.name != id_or_name && u.uuid != id_or_name);

        if config.users.len() < initial_len {
            std::fs::write("server_config.json", serde_json::to_string_pretty(config)?)?;
            println!("[\x1b[32m+\x1b[0m] کاربر '{}' با موفقیت حذف شد.", id_or_name);
        } else {
            println!("[\x1b[31mx\x1b[0m] کاربری با این مشخصات یافت نشد.");
        }
        Ok(())
    }

    fn generate_link(config: &StoredServerConfig, id_or_name: &str, password: Option<String>) -> Result<()> {
        let user = config.users.iter().find(|u| u.name == id_or_name || u.uuid == id_or_name)
            .ok_or_else(|| anyhow!("کاربر مورد نظر یافت نشد."))?;

        let pass = match password {
            Some(p) => p,
            None => {
                print!("[?] رمز عبور برای لینک کاربر '{}': ", user.name);
                io::stdout().flush()?;
                let mut p = String::new();
                io::stdin().read_line(&mut p)?;
                p.trim().to_string()
            }
        };

        let classic_pub_bytes: [u8; 32] = STANDARD.decode(&config.classic_public_b64)?
            .try_into().map_err(|_| anyhow!("Invalid classic public key"))?;
        let pq_pub = STANDARD.decode(&config.pq_public_b64)?;

        let payload = ServerPayload {
            endpoint: format!("{}:{}", config.server_ip, config.port),
            sni: config.sni.clone(),
            user_uuid: user.uuid.clone(),
            token: user.token.clone(),
            classic_pub: classic_pub_bytes,
            pq_pub,
            transport: "reality-pq".into(),
        };

        let link = SecureLinkEngine::export_symmetric(&payload, &pass)?;
        println!("\nلینک اشتراک جدید برای {}:", user.name);
        println!("\x1b[36m{}\x1b[0m\n", link);
        Ok(())
    }
}