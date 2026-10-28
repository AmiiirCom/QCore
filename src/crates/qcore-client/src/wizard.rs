use anyhow::{anyhow, Result};
use qcore_crypto::share_link::{SecureLinkEngine, ServerPayload};
use std::io::{self, Write};

pub struct ClientWizard;

impl ClientWizard {
    pub fn run_interactive() -> Result<ServerPayload> {
        println!("============================================================");
        println!("       QCore Client Setup - Import Encrypted Subscription    ");
        println!("============================================================");

        print!("[?] Paste your encrypted share link (qcore://...): ");
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
            
            // ذخیره خودکار کانفیگ برای اجراهای بعدی
            std::fs::write("client_config.json", serde_json::to_string_pretty(&payload)?)?;
            
            println!("[\x1b[32m+\x1b[0m] Decryption successful & saved to client_config.json!");
            println!("Target Server: {}", payload.endpoint);
            println!("Impersonated Domain (SNI): {}", payload.sni);
            println!("Quantum Transport: {}", payload.transport);
            return Ok(payload);
        } else if link.starts_with("qcore://asym.") {
            return Err(anyhow!("Asymmetric link requires private key file import."));
        }

        Err(anyhow!("Invalid link format."))
    }
}