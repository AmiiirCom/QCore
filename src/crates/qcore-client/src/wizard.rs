//! Client Interactive Setup Wizard.
//!
//! Decrypts incoming `qcore://` subscription share links and persists
//! connection configurations into `client_config.json`.

use anyhow::{anyhow, Result};
use qcore_crypto::share_link::{SecureLinkEngine, ServerPayload};
use std::io::{self, Write};

/// Wizard driver for client profile import.
pub struct ClientWizard;

impl ClientWizard {
    /// Prompts user for link and decryption credentials.
    pub fn run_interactive() -> Result<ServerPayload> {
        println!("============================================================");
        println!("       QCore Client - Import Subscription Link              ");
        println!("============================================================");

        print!("[?] Paste encrypted share link (qcore://...): ");
        io::stdout().flush()?;
        let mut link_input = String::new();
        io::stdin().read_line(&mut link_input)?;
        let link = link_input.trim();

        if link.starts_with("qcore://sym.") {
            print!("[?] Enter decryption password: ");
            io::stdout().flush()?;
            let mut pass_input = String::new();
            io::stdin().read_line(&mut pass_input)?;
            let password = pass_input.trim();

            println!("\n[*] Decrypting configuration payload...");
            let payload = SecureLinkEngine::import_symmetric(link, password)?;

            std::fs::write("client_config.json", serde_json::to_string_pretty(&payload)?)?;

            println!("[\x1b[32m+\x1b[0m] Decryption successful & saved to client_config.json!");
            println!("Endpoint  : {}", payload.endpoint);
            println!("SNI Target: {}", payload.sni);
            println!("Transport : {}", payload.transport);
            return Ok(payload);
        } else if link.starts_with("qcore://asym.") {
            return Err(anyhow!("Asymmetric links require private key file import"));
        }

        Err(anyhow!("Unrecognized link format"))
    }
}