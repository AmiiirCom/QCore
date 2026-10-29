//! Command-Line User Provisioning Interface.
//!
//! Implements subcommands for adding users, listing quotas, generating share
//! links, and removing revoked credentials.

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

/// User management CLI subcommands.
#[derive(Subcommand)]
pub enum UserCommands {
    /// Provision a new user account and output an encrypted share link.
    Add {
        /// Unique username.
        name: String,
        /// Bandwidth limit in gigabytes (0 = unlimited).
        #[arg(short, long, default_value = "0")]
        quota_gb: u64,
        /// Validity period in days (0 = unlimited).
        #[arg(short, long, default_value = "0")]
        days: u64,
        /// Password to encrypt the share link.
        #[arg(short, long)]
        password: Option<String>,
    },
    /// List all registered users and their quota status.
    List,
    /// Remove an existing user by name or UUID.
    Remove {
        /// Target username or UUID.
        id_or_name: String,
    },
    /// Regenerate an encrypted share link for an existing user.
    Link {
        /// Target username or UUID.
        id_or_name: String,
        /// Password for link encryption.
        #[arg(short, long)]
        password: Option<String>,
    },
}

/// Handler executing user management actions.
pub struct UserCliHandler;

impl UserCliHandler {
    /// Dispatches execution based on the chosen subcommand.
    pub fn handle(cmd: UserCommands) -> Result<()> {
        let config_data = std::fs::read_to_string("server_config.json")
            .map_err(|_| anyhow!("Server configuration not found. Run 'qcore-server setup' first."))?;
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
                print!("[?] Enter password to encrypt share link for user '{}': ", name);
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

        let classic_pub_bytes: [u8; 32] = STANDARD.decode(&config.classic_public_b64)?
            .try_into().map_err(|_| anyhow!("Invalid classic public key stored"))?;
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
        println!("             User Provisioned Successfully                  ");
        println!("============================================================");
        println!("Username : {}", name);
        println!("Quota    : {}", if quota_gb > 0 { format!("{} GB", quota_gb) } else { "Unlimited".into() });
        println!("Validity : {}", if days > 0 { format!("{} days", days) } else { "Indefinite".into() });
        println!("\nEncrypted Share Link:");
        println!("\x1b[36m{}\x1b[0m", link);
        println!("(Distribute this link alongside the chosen password to the client)");
        println!("============================================================\n");

        Ok(())
    }

    fn list_users(config: &StoredServerConfig) {
        println!("\n{:<20} {:<38} {:<15} {:<10}", "Username", "UUID", "Quota", "Status");
        println!("{:-<88}", "");
        for u in &config.users {
            let quota = if u.quota_bytes > 0 {
                format!("{:.1} GB", u.quota_bytes as f64 / (1024.0 * 1024.0 * 1024.0))
            } else {
                "Unlimited".to_string()
            };
            let status = if u.enabled { "Active" } else { "Revoked" };
            println!("{:<20} {:<38} {:<15} {:<10}", u.name, u.uuid, quota, status);
        }
        println!("{:-<88}\n", "");
    }

    fn remove_user(config: &mut StoredServerConfig, id_or_name: &str) -> Result<()> {
        let initial_len = config.users.len();
        config.users.retain(|u| u.name != id_or_name && u.uuid != id_or_name);

        if config.users.len() < initial_len {
            std::fs::write("server_config.json", serde_json::to_string_pretty(config)?)?;
            println!("[\x1b[32m+\x1b[0m] User '{}' removed successfully.", id_or_name);
        } else {
            println!("[\x1b[31mx\x1b[0m] User not found.");
        }
        Ok(())
    }

    fn generate_link(config: &StoredServerConfig, id_or_name: &str, password: Option<String>) -> Result<()> {
        let user = config.users.iter().find(|u| u.name == id_or_name || u.uuid == id_or_name)
            .ok_or_else(|| anyhow!("User not found."))?;

        let pass = match password {
            Some(p) => p,
            None => {
                print!("[?] Enter password to encrypt share link for user '{}': ", user.name);
                io::stdout().flush()?;
                let mut p = String::new();
                io::stdin().read_line(&mut p)?;
                p.trim().to_string()
            }
        };

        let classic_pub_bytes: [u8; 32] = STANDARD.decode(&config.classic_public_b64)?
            .try_into().map_err(|_| anyhow!("Invalid classic public key stored"))?;
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
        println!("\nGenerated Share Link for {}:", user.name);
        println!("\x1b[36m{}\x1b[0m\n", link);
        Ok(())
    }
}