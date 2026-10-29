

<div align="center">

# 🛡️ QCore
### Quantum-Resistant & Hardware-Level Inception Stealth Core

[![Language](https://img.shields.io/badge/Language-Rust_2021%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Post-Quantum Standard](https://img.shields.io/badge/NIST_FIPS_203-ML--KEM--768-blue.svg?style=flat-square)](https://csrc.nist.gov/pubs/fips/203/final)
[![Platform Matrix](https://img.shields.io/badge/Platform-Linux_|_Windows_|_macOS_|_Android_|_FreeBSD_|_MikroTik-green.svg?style=flat-square)](#platform-compatibility-matrix)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blueviolet.svg?style=flat-square)](LICENSE)

</div>

---

## 1. Overview & Core Philosophy

**QCore** is an industrial-grade, quantum-resistant, and high-stealth transport tunneling framework architected to survive aggressive stateful Deep Packet Inspection (DPI) and Machine-Learning traffic classification regimes.

### Key Architectural Highlights:
* **NIST FIPS 203 Hybrid Post-Quantum Cryptography:** Integrates lattice-based **ML-KEM-768** with classic **X25519** Diffie-Hellman via a constant-time **HKDF-BLAKE3** combiner, securing communications against current interceptors and future quantum decryptors.
* **Browser-Grade Handshake Synthesis (JA4 Fingerprint):** Emulates Google Chrome ClientHello frames byte-for-byte, incorporating the post-quantum group ID `0x11ec` and TLS Record Splitting to blind SNI inspection engines.
* **Encrypted Share Link Protocol (`qcore://`):** Guarantees zero leak of server endpoints, IPs, and authentication tokens through authenticated ChaCha20-Poly1305 symmetric and asymmetric envelopes.
* **Dual Inbound Local Architecture:** Automatically exposes an RFC 1928-compliant **SOCKS5** server on `127.0.0.1:10808` alongside an RFC 9110 **HTTP CONNECT** proxy on `127.0.0.1:10809`.
* **Multi-Tenant Administration Engine:** Native user provisioning with bandwidth quotas (GB), account expiration timelines (days), and atomic traffic accounting.

---

## 2. Platform Compatibility Matrix

| Platform | Supported Targets | Artifact Types |
| :--- | :--- | :--- |
| **Windows** | Windows 10 & 11 (x86_64, ARM64 Snapdragon X) | Standalone `.zip` packages |
| **Debian / Ubuntu** | Debian 11/12/13, Ubuntu 20.04/22.04/24.04 | Native `.deb` packages + static binaries |
| **Arch Linux / Manjaro** | Rolling releases of Arch Linux | Native Pacman packages (`.pkg.tar.zst`) |
| **Universal Linux** | CentOS, RHEL, Rocky, AlmaLinux, Alpine, Fedora | Fully static `musl` binaries |
| **macOS** | macOS 13 (Ventura), 14 (Sonoma), 15 (Sequoia)+ | `.tar.gz` for Apple Silicon & Intel |
| **MikroTik RouterOS** | RouterOS v7 (ARM64, ARMv7, x86/CHR) | Static musl binaries & Container rootfs |
| **Android** | Android 8.0+ (arm64-v8a, armeabi-v7a) | Native `jniLibs` (`.so`) & Termux CLI |
| **FreeBSD** | FreeBSD 13 & 14 (x86_64) | Standalone `.tar.gz` |

---

## 3. Server Operations Guide

### A) Installation on Linux (VPS)

#### Debian / Ubuntu (`.deb`)
```bash
wget https://github.com/YOUR_REPO/releases/latest/download/qcore-server-v0.1.0-linux-amd64.deb
sudo dpkg -i qcore-server-v0.1.0-linux-amd64.deb
```

#### Arch Linux (`.pkg.tar.zst`)
```bash
sudo pacman -U qcore-server-v0.1.0-arch-x86_64.pkg.tar.zst
```

#### Universal Static Binary
```bash
tar -xzvf qcore-server-v0.1.0-linux-x86_64-musl.tar.gz
sudo mv qcore-server /usr/local/bin/
sudo chmod +x /usr/local/bin/qcore-server
```

---

### B) Interactive Setup Wizard
Run the bootstrap wizard:
```bash
qcore-server setup
```
1. **Listening Port:** TCP port (Default: `8443` or `443`).
2. **Fallback Domain:** SNI camouflage domain (Default: `swdist.apple.com`).
3. **Server Public IP:** Host IPv4 or domain name.
4. **Password:** Secret password used to encrypt the administrative share link.

---

### C) Multi-User Management CLI

```bash
# Provision a user with a 50 GB quota valid for 30 days:
qcore-server user add alice --quota-gb 50 --days 30

# List all users, quotas, and expiration status:
qcore-server user list

# Regenerate an encrypted share link for an existing user:
qcore-server user link alice

# Revoke an existing user:
qcore-server user remove alice
```

---

### D) System Tuning & Daemonization (Systemd)

1. **Enable Google BBR Congestion Control:**
```bash
cat <<EOF | sudo tee /etc/sysctl.d/99-qcore.conf
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr
net.core.rmem_max = 67108864
net.core.wmem_max = 67108864
net.ipv4.ip_forward = 1
EOF

sudo sysctl --system
```

2. **Start Systemd Service:**
```bash
sudo systemctl enable --now qcore-server
sudo systemctl status qcore-server
```

---

## 4. Client Operations Guide

### A) Windows (10 & 11)
1. Extract `qcore-client-*-windows-x86_64-win10-11.zip`.
2. Run the interactive setup wizard:
   ```powershell
   .\qcore-client.exe setup
   ```
3. Paste the `qcore://sym...` link and enter the link password.
4. Launch the client:
   ```powershell
   .\qcore-client.exe
   ```
   Active inbounds:
   * **SOCKS5:** `127.0.0.1:10808`
   * **HTTP Proxy:** `127.0.0.1:10809`

---

### B) Verification & Proxy Settings
* **System-wide Proxy:** Navigate to `Settings > Network & internet > Proxy > Manual proxy setup`, specify `127.0.0.1` and Port `10809`.
* **Terminal Test:**
  ```powershell
  curl.exe -x socks5h://127.0.0.1:10808 https://httpbin.org/ip
  ```

---

### C) Android & MikroTik Deployment
* **Android Studio:** Import `jniLibs/` containing `libqcore_client_core.so` and interface via the exported C-ABI functions `qcore_client_start` and `qcore_client_stop`.
* **Termux:** Extract `qcore-client-*-android-termux-cli.tar.gz` and run `./qcore-client-arm64`.
* **MikroTik RouterOS v7:** Deploy static binaries directly or import the container image:
  ```routeros
  /container/add file=qcore-client-mikrotik.tar interface=veth1 root-dir=disk1/qcore
  /container/start [find where root-dir="disk1/qcore"]
  ```

---

## 5. Security & Cryptographic Disclosure

QCore implements hybrid key exchanges strictly compliant with **NIST FIPS 203** and **RFC 7748**. Cryptographic primitives are executed via constant-time algorithms to mitigate timing and side-channel vulnerabilities.

## 6. License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.