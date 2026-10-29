//! Server Interactive Setup Wizard with Camouflage Pool Support.

use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ml_kem::EncodedSizeUser;
use qcore_crypto::hybrid_kem::HybridKeyPair;
use qcore_crypto::share_link::{SecureLinkEngine, ServerPayload};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use uuid::Uuid;

use crate::user_manager::UserConfig;

/// Persistent server configuration stored in JSON format.
#[derive(Serialize, Deserialize, Clone)]
pub struct StoredServerConfig {
    pub port: u16,
    pub sni: String,
    pub server_ip: String,
    pub classic_secret_b64: String,
    pub classic_public_b64: String,
    pub pq_decaps_key_b64: String,
    pub pq_public_b64: String,
    pub users: Vec<UserConfig>,
}

pub struct ServerWizard;

impl ServerWizard {
    pub fn run_interactive() -> Result<()> {
        println!("============================================================");
        println!("            QCore Server - Setup Wizard                     ");
        println!("============================================================");

        let stdin = io::stdin();

        print!("[?] Enter listening port [Default: 8443]: ");
        io::stdout().flush()?;
        let mut port_input = String::new();
        stdin.read_line(&mut port_input)?;
        let port_str = port_input.trim();
        let port: u16 = if port_str.is_empty() { 8443 } else { port_str.parse().unwrap_or(8443) };

        print!("[?] Enter fallback SNI target domains (comma-separated rotation pool) [Default: swdist.apple.com, gateway.icloud.com, addons.mozilla.org]: ");
        io::stdout().flush()?;
        let mut sni_input = String::new();
        stdin.read_line(&mut sni_input)?;
        let sni = sni_input.trim();
        let sni = if sni.is_empty() { "swdist.apple.com, gateway.icloud.com, addons.mozilla.org" } else { sni };

        print!("[?] Enter server public IP or hostname [Default: 127.0.0.1]: ");
        io::stdout().flush()?;
        let mut ip_input = String::new();
        stdin.read_line(&mut ip_input)?;
        let server_ip = ip_input.trim();
        let server_ip = if server_ip.is_empty() { "127.0.0.1" } else { server_ip };

        println!("\n[*] Generating NIST FIPS 203 ML-KEM-768 + X25519 hybrid keys with memory sanitization...");
        let keypair = HybridKeyPair::generate();
        let admin_uuid = Uuid::new_v4().to_string();
        let admin_token = Uuid::new_v4().to_string();

        let admin_user = UserConfig {
            uuid: admin_uuid.clone(),
            name: "admin".into(),
            token: admin_token.clone(),
            quota_bytes: 0,
            expiry_timestamp: 0,
            enabled: true,
        };

        let stored = StoredServerConfig {
            port,
            sni: sni.to_string(),
            server_ip: server_ip.to_string(),
            classic_secret_b64: STANDARD.encode(keypair.classic_secret.to_bytes()),
            classic_public_b64: STANDARD.encode(keypair.classic_public.as_bytes()),
            pq_decaps_key_b64: STANDARD.encode(keypair.pq_decapsulation_key.as_bytes().as_slice()),
            pq_public_b64: STANDARD.encode(&keypair.pq_public_bytes),
            users: vec![admin_user],
        };
        std::fs::write("server_config.json", serde_json::to_string_pretty(&stored)?)?;

        print!("[?] Enter password to encrypt the admin share link: ");
        io::stdout().flush()?;
        let mut pass_input = String::new();
        stdin.read_line(&mut pass_input)?;
        let password = pass_input.trim();

        let payload = ServerPayload {
            endpoint: format!("{}:{}", server_ip, port),
            sni: sni.to_string(),
            user_uuid: admin_uuid,
            token: admin_token,
            classic_pub: *keypair.classic_public.as_bytes(),
            pq_pub: keypair.pq_public_bytes.clone(),
            transport: "reality-pq".into(),
        };

        let sym_link = SecureLinkEngine::export_symmetric(&payload, password)?;

        println!("\n------------------------------------------------------------");
        println!("             Configuration Saved Successfully               ");
        println!("------------------------------------------------------------");
        println!("Config file: server_config.json");
        println!("\nAdministrative Share Link (with SNI Rotation Pool):");
        println!("\x1b[36m{}\x1b[0m", sym_link);
        println!("\nNote: Additional users can be provisioned at any time via:");
        println!("  qcore-server user add <NAME> --quota-gb <GB> --days <DAYS>");
        println!("------------------------------------------------------------\n");

        Ok(())
    }
}