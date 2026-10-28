
```markdown
<div align="center">

# 🛡️ QCore
### Quantum-Resistant & Hardware-Level Inception Stealth Core
**هسته تونلینگ پنهان‌کار، مقاوم در برابر تحلیل‌های هوش مصنوعی DPI و مجهز به رمزنگاری پسا-کوانتوم**

[![Rust](https://img.shields.io/badge/Language-Rust_2021%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Post-Quantum](https://img.shields.io/badge/NIST_FIPS_203-ML--KEM--768-blue.svg?style=flat-square)](https://csrc.nist.gov/pubs/fips/203/final)
[![Platform](https://img.shields.io/badge/Platform-Linux_|_Windows_|_macOS_|_Android_|_FreeBSD_|_MikroTik-green.svg?style=flat-square)](#ماتریس-پشتیبانی-سیستم‌عامل‌ها--platform-matrix)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blueviolet.svg?style=flat-square)](LICENSE)

[فارسی (Persian)](#بخش-اول-راهنمای-فارسی) • [English](#part-2-english-documentation)

</div>

---

# بخش اول: راهنمای فارسی

## ۱. معرفی و فلسفه بنیادین QCore
پروژه **QCore** یک سامانه اختصاصی و ضدسانسور لایه ترنسپورت است که برای بقای اتصال در سخت‌ترین رژیم‌های ممیزی و سیستم‌های بازرسی عمیق بسته (Stateful Hardware DPI & ML-based Classifiers) طراحی شده است.

### ویژگی‌های کلیدی:
* **رمزنگاری هیبرید پسا-کوانتوم (Post-Quantum Core):** ترکیب استاندارد رسمی **NIST FIPS 203 (ML-KEM-768)** با تبادل کلید سنتی **X25519** از طریق مشتق‌سازی **HKDF-BLAKE3**؛ تضمین‌کننده امنیت در برابر کامپیوترهای کوانتومی و پروژه‌های شنود آینده (Store-Now-Decrypt-Later).
* **سنتز خالص فریم‌های TLS کروم (JA4 Fingerprint):** شبیه‌سازی بایت به بایت هدر دست‌تکانی گوگل کروم با افزونه اختصاصی پسا-کوانتوم کروم (`0x11ec`) و تقسیم فریم‌ها (Record Splitting) جهت عبور از رصد دامنه‌ها (SNI).
* **لینک‌های اشتراک کاملاً رمزشده (`qcore://`):** مخفی‌سازی کامل IP، پورت و مشخصات سرور درون لایه رمزنگاری ChaCha20-Poly1305 به طوری که هیچ دیتای خامی برای تحلیل‌گرهای ممیزی فاش نشود.
* **اینباندهای هم‌زمان لوکال:** راه‌اندازی خودکار پروکسی محلی **SOCKS5** (پورت `10808`) و **HTTP Proxy** (پورت `10809`) در سمت کلاینت.
* **مدیریت پیشرفته چندکاربره (Multi-User Engine):** تخصیص سهمیه حجم به گیگابایت، محدودیت زمانی به روز و ثبت مصرف ترافیک به ازای هر کاربر.

---

## ۲. ماتریس پشتیبانی سیستم‌عامل‌ها

| سیستم‌عامل / پلتفرم | نسخه‌ها و معماری‌های هدف | نوع پکیج ارائه‌شده در Releases |
| :--- | :--- | :--- |
| **ویندوز (Windows)** | ویندوز ۱۰ و ۱۱ (x86_64 و ARM64 Snapdragon) | فایل‌های فشرده مستقل `.zip` |
| **لینوکس دبیان و اوبونتو** | Debian 11/12/13 و Ubuntu 20.04/22.04/24.04 | پکیج رسمی نصبی `.deb` + باینری استاتیک |
| **آرچ لینوکس و مانجارو** | تمامی نسخه‌های رولینگ Arch Linux | پکیج نصبی رسمی پکمین `.pkg.tar.zst` |
| **سایر توزیع‌های لینوکس** | CentOS، RHEL، Rocky Linux، Alpine، Fedora | فایل استاتیک `musl` بدون هیچ وابستگی |
| **مکینتاش (macOS)** | macOS 13 (Ventura)، 14 (Sonoma)، 15 (Sequoia)+ | فایل‌های فشرده `.tar.gz` (Apple Silicon و Intel) |
| **روترهای میکروتیک** | MikroTik RouterOS v7 (معماری‌های ARM64، ARMv7، x86) | باینری‌های استاتیک مستقل و سازگار با Container |
| **اندروید (Android)** | اندروید ۸ به بالا (معماری‌های arm64-v8a و armeabi-v7a) | پوشه `jniLibs` حاوی `.so` + باینری CLI ترموکس |
| **فری‌بی‌اس‌دی (FreeBSD)** | FreeBSD 13 و 14 (x86_64) | فایل فشرده `.tar.gz` |

---

## ۳. راهنمای استقرار و راه‌اندازی سرور (Server Guide)

### الف) نصب سریع روی لینوکس (سرور خارج از کشور)

#### روش اول: استفاده از پکیج رسمی Debian/Ubuntu (.deb)
```bash
# دانلود آخرین نسخه سرور از صفحه Releases گیت‌هاب
wget https://github.com/YOUR_USER/QCore/releases/latest/download/qcore-server-v5.0.3-linux-amd64.deb

# نصب پکیج (به طور خودکار باینری را در مسیر سیستمی قرار داده و سرویس Systemd را ثبت می‌کند)
sudo dpkg -i qcore-server-v5.0.3-linux-amd64.deb
```

#### روش دوم: استفاده از پکیج رسمی Arch Linux (.pkg.tar.zst)
```bash
sudo pacman -U qcore-server-v5.0.3-arch-x86_64.pkg.tar.zst
```

#### روش سوم: استفاده از فایل استاتیک (بدون پیش‌نیاز روی تمام توزیع‌ها)
```bash
tar -xzvf qcore-server-v5.0.3-linux-x86_64-musl.tar.gz
sudo mv qcore-server /usr/local/bin/
sudo chmod +x /usr/local/bin/qcore-server
```

---

### ب) اجرای ویزارد اولیه راه‌اندازی سرور
دستور زیر را در ترمینال سرور اجرا کنید:
```bash
qcore-server setup
```
ویزارد تعاملی سوالات زیر را مطرح می‌کند:
1. **Listening Port:** پورت اتصال سرور (پیش‌فرض: `443`).
2. **Fallback Target Domain:** دامنه استتار ضد پروبینگ (پیش‌فرض: `swdist.apple.com`).
3. **Server Public IP:** آدرس IP پابلیک سرور شما.
4. **Password:** یک کلمه عبور قوی برای رمزنگاری لینک ادمین اولیه.

*خروجی ویزارد:* کلیدهای پسا-کوانتوم تولید شده، در فایل `server_config.json` ذخیره می‌شوند و اولین لینک اشتراک رمزشده با پیشوند `qcore://sym...` صادر می‌شود.

---

### ج) دستورات مدیریت کاربران (User Management CLI)

مدیر سرور می‌تواند در هر زمان بدون نیاز به راه‌اندازی مجدد، کاربر اضافه، حذف و مانیتور کند:

```bash
# ۱. افزودن کاربر جدید با سهمیه ۵۰ گیگابایت و اعتبار ۳۰ روزه:
qcore-server user add ali --quota-gb 50 --days 30

# ۲. مشاهده جدول تمام کاربران، شناسه‌ها، حجم مجاز و وضعیت:
qcore-server user list

# ۳. صدور مجدد لینک اشتراک برای کاربری که قبلاً ساخته شده:
qcore-server user link ali

# ۴. مسدودسازی یا حذف یک کاربر:
qcore-server user remove ali
```

---

### د) بهینه‌سازی شبکه لینوکس و اجرای دائم به عنوان سرویس (Systemd)

۱. **فعال‌سازی کنترل ازدحام BBR در هسته لینوکس:**
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

۲. **روشن نگه‌داشتن سرور با Systemd:**
```bash
sudo systemctl enable --now qcore-server
sudo systemctl status qcore-server
```

---

## ۴. راهنمای راه‌اندازی و استفاده از کلاینت (Client Guide)

### الف) راه‌اندازی در ویندوز (Windows 10 & 11)
۱. بسته `qcore-client-v*-windows-x86_64-win10-11.zip` را دانلود و استخراج کنید.  
۲. در خط فرمان یا PowerShell دستور زیر را اجرا کنید:
```powershell
.\qcore-client.exe setup
```
۳. لینک رمزشده دریافتی از سرور (`qcore://sym...`) و کلمه عبور آن را وارد کنید. تنظیمات با موفقیت رمزگشایی و در فایل `client_config.json` ذخیره می‌شود.  
۴. حالا با دستور زیر کلاینت را روشن کنید:
```powershell
.\qcore-client.exe
```
مشاهده خواهید کرد که هر دو پروکسی محلی روشن می‌شوند:
* **SOCKS5 Inbound:** `127.0.0.1:10808`
* **HTTP Proxy Inbound:** `127.0.0.1:10809`

---

### ب) تنظیم کلاینت روی مرورگر و کل سیستم
* **تنظیم پروکسی کل ویندوز:** به مسیر `Settings > Network & internet > Proxy` بروید. در بخش Manual proxy گزینه Edit را زده، آدرس را `127.0.0.1` و پورت را `10809` قرار داده و Save کنید.
* **تنظیم در افزونه SwitchyOmega مرورگر:** یک پروفایل از نوع **SOCKS5** با سرور `127.0.0.1` و پورت `10808` ایجاد کنید.
* **تست اتصال در خط فرمان:**
  ```powershell
  curl.exe -x socks5h://127.0.0.1:10808 https://httpbin.org/ip
  ```

---

### ج) استفاده در اندروید (Android)
* **اپلیکیشن‌های اختصاصی (Android Studio):** برنامه‌نویسان می‌توانند پوشه `jniLibs` ارائه‌شده در صفحه Release را مستقیماً داخل پروژه اندروید کپی کرده و از توابع C-ABI (`qcore_client_start` و `qcore_client_stop`) برای اتصال استفاده کنند.
* **محیط ترموکس (Termux):** آرشیو `qcore-client-v*-android-termux-cli.tar.gz` را دانلود، استخراج و به صورت باینری خط فرمان اجرا نمایید.

---

### د) استفاده در روترهای میکروتیک (MikroTik RouterOS v7)
برای فعال‌سازی QCore روی روترهای میکروتیک مجهز به پردازنده‌های ARM64 (مانند RB5009 یا سری CCR2004) از پکیج Container میکروتیک استفاده کنید:
```routeros
/container/config/set registry-url=https://registry-1.docker.io tmpdir=disk1/tmp
/container/add file=qcore-client-mikrotik-arm64.tar interface=veth1 root-dir=disk1/qcore
/container/start [find where root-dir="disk1/qcore"]
```

---

<br/>

---

# Part 2: English Documentation

## 1. Overview & Core Philosophy
**QCore** is an industrial-grade, quantum-resistant, and high-stealth transport tunneling framework specifically architected to withstand advanced stateful Deep Packet Inspection (DPI) and Machine-Learning traffic classification regimes.

### Technical Highlights:
* **NIST FIPS 203 Post-Quantum Hybrid Core:** Integrates lattice-based **ML-KEM-768** with classic **X25519** Diffie-Hellman using **HKDF-BLAKE3** key derivation to ensure forward secrecy against quantum-assisted decryptors.
* **Browser-Grade JA4 Fingerprint Synthesis:** Synthesizes Google Chrome 130+ handshake frames byte-by-byte with the Chrome post-quantum extension (`0x11ec`) and implements TLS Record Splitting to blind DPI SNI sniffers.
* **Encrypted Share Link Protocol (`qcore://`):** Zero-leak configuration format. Server IPs, endpoints, and credentials are encrypted using ChaCha20-Poly1305 with unique salts and nonces.
* **Dual Inbound Local Architecture:** Automatically exposes both an RFC 1928-compliant **SOCKS5** server on `127.0.0.1:10808` and an HTTP CONNECT proxy on `127.0.0.1:10809`.
* **Enterprise Multi-Tenancy Engine:** Native user management with data quotas (GB), account expiration timelines (days), and atomic traffic accounting.

---

## 2. Platform Matrix

| Platform | Supported Architecture / Versions | Artifact Type in Releases |
| :--- | :--- | :--- |
| **Windows** | Windows 10 & 11 (x86_64 and ARM64 / Snapdragon X) | Standalone `.zip` archives |
| **Debian / Ubuntu** | Debian 11/12/13, Ubuntu 20.04/22.04/24.04 | Native `.deb` packages + static binaries |
| **Arch Linux / Manjaro** | All rolling releases of Arch Linux | Native Pacman packages (`.pkg.tar.zst`) |
| **Universal Linux** | CentOS, RHEL, Rocky, AlmaLinux, Alpine, Fedora | Fully static `musl` binaries (Zero-dependency) |
| **macOS** | macOS 13 (Ventura), 14 (Sonoma), 15 (Sequoia)+ | `.tar.gz` for Apple Silicon (M1-M4) & Intel |
| **MikroTik RouterOS** | RouterOS v7 (ARM64, ARMv7, x86/CHR) | Standalone static binaries & Container rootfs |
| **Android** | Android 8.0+ (arm64-v8a & armeabi-v7a) | Pre-compiled `jniLibs` (`.so`) & Termux CLI |
| **FreeBSD** | FreeBSD 13 & 14 (x86_64) | Standalone `.tar.gz` |

---

## 3. Server Deployment Guide

### A) Installation on Linux (VPS)

#### Option 1: Debian / Ubuntu (`.deb`)
```bash
# Download latest server deb package
wget https://github.com/YOUR_USER/QCore/releases/latest/download/qcore-server-v5.0.3-linux-amd64.deb

# Install package (installs binaries to /usr/local/bin and registers systemd unit)
sudo dpkg -i qcore-server-v5.0.3-linux-amd64.deb
```

#### Option 2: Arch Linux (`.pkg.tar.zst`)
```bash
sudo pacman -U qcore-server-v5.0.3-arch-x86_64.pkg.tar.zst
```

#### Option 3: Universal Static Binary
```bash
tar -xzvf qcore-server-v5.0.3-linux-x86_64-musl.tar.gz
sudo mv qcore-server /usr/local/bin/
sudo chmod +x /usr/local/bin/qcore-server
```

---

### B) Interactive Server Setup Wizard
Run the setup wizard:
```bash
qcore-server setup
```
Provide the requested parameters:
* **Port:** Default is `443` (or any custom port like `8443`).
* **Fallback Domain:** Stealth SNI impersonation target (default: `swdist.apple.com`).
* **Server Public IP:** Your VPS public IPv4 or domain name.
* **Password:** Master password used to encrypt the administrative share link.

---

### C) Multi-User Management CLI
The server allows dynamic user provisioning directly via CLI:

```bash
# Add user with 50 GB traffic quota and 30-day validity:
qcore-server user add alice --quota-gb 50 --days 30

# List all users, quotas, and expiration states:
qcore-server user list

# Re-export encrypted share link for an existing user:
qcore-server user link alice

# Disable or delete a user:
qcore-server user remove alice
```

---

### D) System Tuning & Daemonizing (Systemd)

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

2. **Start and Enable Service:**
```bash
sudo systemctl enable --now qcore-server
sudo systemctl status qcore-server
```

---

## 4. Client Deployment Guide

### A) Windows (10 & 11)
1. Download and extract `qcore-client-v*-windows-x86_64-win10-11.zip`.
2. Open PowerShell or Command Prompt in the extracted directory and run:
   ```powershell
   .\qcore-client.exe setup
   ```
3. Paste the `qcore://sym...` link and enter the decryption password.
4. Start the client:
   ```powershell
   .\qcore-client.exe
   ```
   Both local inbounds will be activated:
   * **SOCKS5:** `127.0.0.1:10808`
   * **HTTP Proxy:** `127.0.0.1:10809`

---

### B) Proxy Verification & Browser Configuration
* **System-wide Proxy (Windows):** Navigate to `Settings > Network & internet > Proxy > Manual proxy setup`, specify `127.0.0.1` and Port `10809`.
* **Browser Extension (SwitchyOmega / FoxyProxy):** Add a SOCKS5 profile pointing to `127.0.0.1:10808`.
* **Terminal Test:**
  ```powershell
  curl.exe -x socks5h://127.0.0.1:10808 https://httpbin.org/ip
  ```

---

### C) Android Integration
* **Mobile App Developers:** Import `jniLibs/` containing `libqcore_client_core.so` directly into your Android Studio project. Call the exported C-ABI functions `qcore_client_start` and `qcore_client_stop`.
* **Termux Users:** Extract `qcore-client-v*-android-termux-cli.tar.gz` and execute `./qcore-client-arm64`.

---

## 5. Building from Source

### Prerequisites
* Rust Stable toolchain (Rust 1.82+ with Edition 2021/2024 support)
* Clang / LLVM

```bash
# Clone the repository
git clone https://github.com/YOUR_USER/QCore.git
cd QCore/src

# Build release binaries for host platform
cargo build --release

# Binaries will be outputted to:
# src/target/release/qcore-server
# src/target/release/qcore-client
```

---

## 6. Security & Cryptographic Disclosure
QCore implements hybrid key exchanges strictly compliant with **NIST Special Publication 800-208** and **FIPS 203**. Cryptographic primitives are executed via constant-time algorithms to mitigate timing and side-channel vulnerabilities.

## 7. License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.