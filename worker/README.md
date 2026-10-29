
# QCore Edge Worker

A standalone, post-quantum protected transport proxy running directly on Cloudflare's global edge network using WebAssembly (Rust) and the `cloudflare:sockets` TCP API.

---

## 1. Zero-Server / Direct Exit Architecture

In **Direct Exit Mode**, you do **not** need a VPS or any physical server. Cloudflare Workers act as a free, 24/7 serverless exit node, connecting directly to unrestricted internet targets.

```text
[ Client Device ]
       |
       v (Post-Quantum QC\x01 WebSocket Stream)
[ Cloudflare Global Edge ]
       |
       +--> Embedded WebAssembly (Rust Core: NIST FIPS 203 ML-KEM-768 & X25519)
       |    - Verifies handshake magic header
       |    - Decapsulates session keys
       |    - Authenticates client token
       |    - Extracts target destination (e.g. youtube.com:443)
       |
       +--> cloudflare:sockets connect()
       |
       v
[ Unrestricted Internet Target ]
```

---

## 2. Generating Keys & Client Link Without a Server

To generate matching cryptographic keys and a client subscription link without owning a server, run the `qcore-server` binary **once on your local PC** as a key generator:

### Step 1: Run local key generator wizard
On your local machine (Windows / macOS / Linux), run:
```powershell
.\core\target\release\qcore-server.exe setup
```

Answer the interactive prompts:
* **Listening Port:** `443`
* **Fallback Domain:** Press `Enter` (default: `swdist.apple.com`)
* **Server Public IP / Hostname:** Enter your Cloudflare Worker URL (e.g. `my-qcore.subdomain.workers.dev`)
* **Password:** Choose an encryption password for the share link (e.g. `pass123`)

**Output:**
1. A **`server_config.json`** file is generated on your PC.
2. An encrypted **`qcore://sym...`** link is printed in the terminal. **Save this link!**

*(You can now close the server application on your PC. It does not need to run).*

---

## 3. Configuring Cloudflare Worker Environment Variables

Open the generated `server_config.json` on your PC. It will look like this:

```json
{
  "port": 443,
  "sni": "swdist.apple.com",
  "server_ip": "my-qcore.subdomain.workers.dev",
  "classic_secret_b64": "MC4CAQAwBQYDK2VuBCIEI...",
  "classic_public_b64": "XYZ123...",
  "pq_decaps_key_b64": "A1B2C3D4...",
  "pq_public_b64": "E5F6G7H8...",
  "users": [
    {
      "uuid": "4f3b1e2c-...",
      "name": "admin",
      "token": "a8c7e6d5-...",
      "quota_bytes": 0,
      "expiry_timestamp": 0,
      "enabled": true
    }
  ]
}
```

In the **Cloudflare Dashboard** -> **Workers & Pages** -> select your Worker -> **Settings** -> **Variables and Secrets**, add the following variables:

| Variable Name | Value Source in `server_config.json` | Description |
| :--- | :--- | :--- |
| **`MODE`** | `DIRECT` | Enables direct exit proxying mode |
| **`CLASSIC_SECRET_B64`** | Value of `classic_secret_b64` | Server X25519 private key |
| **`PQ_DECAPS_KEY_B64`** | Value of `pq_decaps_key_b64` | Server ML-KEM-768 decapsulation key |
| **`AUTH_TOKEN`** | Value of `token` (inside `users` array) | Master session authentication token |
| **`FALLBACK_DOMAIN`** | `swdist.apple.com` | Camouflage target domain for active probes |

---

## 4. Building the Standalone Obfuscated `worker.js`

The build pipeline compiles the Rust core into WebAssembly, inlines the bytecode directly into a single JavaScript file, and runs deep obfuscation.

```bash
# 1. Compile Rust core to WebAssembly
cd core/crates/qcore-wasm
wasm-pack build --target web --release

# 2. Bundle and obfuscate into a single standalone file
cd ../../../worker
npm install
node build.js
```

**Output:**
A single file is generated at **`worker/dist/worker.js`**.  
This file contains the entire WebAssembly engine inlined as bytes and is heavily obfuscated.

---

## 5. Deployment

### Option A: 1-Click Web Dashboard Deploy (Recommended)
1. Open [Cloudflare Dashboard](https://dash.cloudflare.com/) -> **Workers & Pages**.
2. Click on your Worker and choose **Edit Code**.
3. Select all text, delete it, and paste the entire contents of **`worker/dist/worker.js`**.
4. Click **Deploy**.

### Option B: Wrangler CLI
```bash
cd worker
wrangler deploy dist/worker.js
```

---

## 6. Client Connection

On your end-user device (Windows, Android, macOS, or Linux):
1. Import the configuration:
   ```powershell
   .\qcore-client.exe setup
   ```
2. Paste the `qcore://sym...` link obtained in **Step 2** and enter your password.
3. Start the client:
   ```powershell
   .\qcore-client.exe
   ```
4. Configure your browser or OS proxy to `127.0.0.1:10808` (SOCKS5) or `127.0.0.1:10809` (HTTP). All traffic now routes through the serverless Cloudflare Worker 24/7 with zero VPS costs.