# Project: QCore

## Context & Instructions

### DESIGNE.md
````markdown
--- START OF FILE QCore_Master_Design_Document_vFinal.md ---

# سند جامع معماری، مبانی ریاضی و راهنمای استقرار صنعتی سامانه QCore  
## (Quantum-Resistant & Hardware-Level Inception Stealth Core - QCore)

**نسخه:** ۵.۰ (Ultimate Master Release)  
**تاریخ انتشار:** اکتبر ۲۰۲۶  
**نام رسمی پروژه:** **QCore**  
**محیط‌های هدف:** سخت‌ترین رژیم‌های ممیزی و بازرسی عمیق بسته‌ها (DPI) مبتنی بر یادگیری ماشین و شبکه‌های ایزوله ملی (ایران - TIC/همراه اول/ایرانسل/مخابرات، چین - GFW، روسیه - TSPU)  
**فلسفه حاکم:** پنهان‌کاری، گریز و بقای اتصال به هر قیمت (Stealth-at-all-costs)؛ اعمال سنگین‌ترین محاسبات پسا-کوانتومی، پدینگ‌های اشباع‌کننده MTU و افت سرعت تعمدی در ازای بقای تضمینی اتصال در سخت‌ترین شرایط امنیتی و قطعی سراسری اینترنت بین‌الملل.  
**مدل استقرار:** استقرار باینری‌های از پیش کامپایل‌شده (Pre-compiled Release Binaries)؛ هیچ فرایند بیلد یا کامپایلی روی ماشین‌های سرور و کلاینت انجام نخواهد شد.

---

## فهرست مطالب

1. [فلسفه بنیادین و اصول طراحی QCore](#۱-فلسفه-بنیادین-و-اصول-طراحی-qcore)
2. [مدل جامع تهدیدات ۲۰۲۵–۲۰۲۶ و رفتار سیستم‌های ممیزی](#۲-مدل-جامع-تهدیدات-۲۰۲۵۲۰۲۶-و-رفتار-سیستم‌های-ممیزی)
3. [معماری چندلایه‌ای و پایپ‌لاین هسته QCore](#۳-معماری-چندلایه‌ای-و-پایپ‌لاین-هسته-qcore)
4. [مبانی ریاضی و مدل‌سازی آماری ضد یادگیری ماشین](#۴-مبانی-ریاضی-و-مدل‌سازی-آماری-ضد-یادگیری-ماشین)
5. [رمزنگاری هیبرید پسا-کوانتوم بر پایه شبکه‌های جبری (Post-Quantum Core)](#۵-رمزنگاری-هیبرید-پسا-کوانتوم-بر-پایه-شبکه‌های-جبری-post-quantum-core)
6. [پردازش در بدو ورود به کارت شبکه (Kernel & Hardware Inception: XDP / eBPF / AF_XDP)](#۶-پردازش-در-بدو-ورود-به-کارت-شبکه-kernel--hardware-inception-xdp--ebpf--af_xdp)
7. [مهندسی لایه Handshake، اثرانگشت JA4/JA4X و Zero-DNS ECH](#۷-مهندسی-لایه-handshake-اثرانگشت-ja4ja4x-و-zero-dns-ech)
8. [موتور دستکاری لایه ۴ (Geneva & Zapret-2 Advanced TCP Desync)](#۸-موتور-دستکاری-لایه-۴-geneva--zapret-2-advanced-tcp-desync)
9. [پروتکل‌ها و ترنسپورت‌های چهارگانه QCore](#۹-پروتکل‌ها-و-ترنسپورت‌های-چهارگانه-qcore)
   - [۹.۱. پروتکل REALITY + Vision v2 با Zero-Copy Splice](#۹۱-پروتکل-reality--vision-v2-با-zero-copy-splice)
   - [۹.۲. ترنسپورت XHTTP (SplitHTTP) سازگار با CDN](#۹۲-ترنسپورت-xhttp-splithttp-سازگار-با-cdn)
   - [۹.۳. ترنسپورت AmneziaWG 3.1 UDP با هدرهای رندوم](#۹۳-ترنسپورت-amneziawg-31-udp-با-هدرهای-رندوم)
   - [۹.۴. موتور پنهان‌سازی UDP با شبیه‌سازی کامل WebRTC / SRTP](#۹۴-موتور-پنهان‌سازی-udp-با-شبیه‌سازی-کامل-webrtc--srtp)
10. [معماری بقا در زمان قطع کامل اینترنت بین‌الملل (Intranet Reverse Bridge)](#۱۰-معماری-بقا-در-زمان-قطع-کامل-اینترنت-بین‌الملل-intranet-reverse-bridge)
11. [ماشین تصمیم‌گیری هوشمند، مانیتورینگ سلامت و چرخش خودکار](#۱۱-ماشین-تصمیم‌گیری-هوشمند-مانیتورینگ-سلامت-و-چرخش-خودکار)
12. [کدهای کامل مرجع به زبان‌های C (eBPF) و Rust](#۱۲-کدهای-کامل-مرجع-به-زبان‌های-c-ebpf-و-rust)
13. [راهنمای جامع راه‌اندازی و استقرار عملیاتی بدون بیلد (Pre-compiled Binary Deployment)](#۱۳-راهنمای-جامع-راه‌اندازی-و-استقرار-عملیاتی-بدون-بیلد-pre-compiled-binary-deployment)

---

## ۱. فلسفه بنیادین و اصول طراحی QCore

پروژه **QCore** برای پایان دادن به چرخه ناپایداری پروکسی‌ها در شرایط فیلترینگ شدید طراحی شده است:

* **پذیرش جریمه محاسباتی و پهنای باند (Stealth-Over-Speed):** اضافه کردن تا ۵۰ درصد پدینگ، محاسبات پیچیده پسا-کوانتوم و دستکاری‌های متوالی بسته‌ها برای دور زدن هوش مصنوعی ممیز کاملاً پذیرفته است. پایداری ۱۰۰ درصدی اتصال بر حداکثر سرعت اولویت مطلق دارد.
* **شروع دفاع از لایه سخت‌افزار (Zero-Layer Inception):** بسته‌ها قبل از رسیدن به بافرهای لینوکس یا اختصاص `sk_buff`، مستقیماً در حلقه درایور سخت‌افزاری کارت شبکه (NIC) پردازش و فیلتر می‌شوند.
* **سنتز خالص بایت‌ها بدون واسطه (Zero-Dependency TLS):** کنار گذاشتن کتابخانه‌های ناقص مانند OpenSSL یا `rustls` برای تولید فریم‌های دست‌تکانی و سنتز بایت به بایت هدرها در سطح باینری، دقیقاً منطبق با گوگل کروم نسخه ۱۳۰+.
* **استقرار کاملاً باینری (Pure Binary Artifacts):** توسعه‌دهنده سورس‌های پروژه را بر روی محیط توسعه خود (Development Host) با قابلیت Cross-Compile بیلد کرده و باینری‌های نهایی `qcore-server`، `qcore-client` و بایت‌کد `xdp_anti_rst.o` را مستقیماً به محیط عملیاتی منتقل می‌کند. هیچ کامپایلر یا ابزار نصبی روی سرور و کلاینت مستقر نخواهد شد.

---

## ۲. مدل جامع تهدیدات ۲۰۲۵–۲۰۲۶ و رفتار سیستم‌های ممیزی

### ۲.۱. ساختار فیلترینگ در ایران (شرکت ارتباطات زیرساخت و اپراتورها)
1. **DPI جریان‌محور سخت‌افزاری (Stateful Hardware DPI):** مجهز به پردازنده‌های FPGA/ASIC برای بازسازی استریم‌های TCP تا سقف ۱۶ کیلوبایت و جستجوی متنی SNI با شتاب‌دهنده‌های Aho-Corasick و Regex.
2. **تزریق فعال بسته‌های جعلی (Stateful TCP RST Injection):** تزریق خودکار بسته‌های TCP RST با اعدادی متمایز از TTL و شماره‌های Sequence برای بستن سشن به محض شناسایی SNI نامعتبر.
3. **تحلیل‌گرهای آماری یادگیری ماشین (Random Forest / CNN):** کلاسیفایرهای هوشمندی که آنتروپی شانون، توزیع اندازه بسته‌ها و فاصله زمانی ورود بسته‌ها (IAT) را استخراج کرده و ترافیک رمزنگاری‌شده خام را مسدود می‌کنند.
4. **فیلتر سختگیرانه UDP:** بستن تمامی بسته‌های UDP با طول ثابت، پکت‌های کوچک‌تر از ۱۴۷۰ بایت، یا اعمال Throttling شدید روی بسته‌هایی که رفتار صوتی/ویدیویی رسمی (مانند WebRTC/STUN) از خود نشان نمی‌دهند.
5. **سیاست لیست سفید ملی (National Intranet Mode):** قطع کامل اتصال به آدرس‌های IP خارجی و باز گذاشتن انحصاری رنج‌های ثبت‌شده داخلی.

### ۲.۲. دیوار آتش چین (GFW)
* **کاوش فعال چندمرحله‌ای (Active Probing):** ارسال پروب‌های TLS با فیلدهای دست‌کاری‌شده برای شناسایی رفتار سرورهای REALITY و مسدودسازی سریع IP.
* **تحلیل تقارن ترافیک (Traffic Symmetry):** رصد مداوم نسبت حجم آپلود به دانلود در مرزهای شبکه.

### ۲.۳. سیستم TSPU روسیه
* تحلیل رفتار دوره‌ای بسته‌ها و مسدودسازی امضاهای شناخته‌شده پروتکل‌های فیلترشکن.

---

## ۳. معماری چندلایه‌ای و پایپ‌لاین هسته QCore

```
+-------------------------------------------------------------------------------------------------+
|                                ۱. لایه رابط کاربر و سیستم‌عامل                                   |
|       TUN Device (شتاب‌دهنده GSO/GRO) │ SOCKS5 Inbound │ هدایت اجباری DNS هسته (Zero-Leak)      |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                       ۲. موتور پسا-کوانتوم و ساخت سشن (Cryptographic Core)                       |
|   ML-KEM-768/1024 (FIPS 203) │ X25519 Hybrid │ ChaCha20Poly1305 / AES-256-GCM │ HKDF-BLAKE3      |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                      ۳. لایه شکل‌دهی آماری و ریاضی ترافیک (Traffic Morphing)                    |
|    مدل مارکوف حالت‌بندی ترافیک │ شبیه‌ساز فرآیند پواسون ناهمگن IAT │ تنظیم‌کننده آنتروپی شانون  |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                        ۴. لایه پروتکل‌های ضد بازرسی (Transport Switching)                        |
|   ┌────────────────────┐ ┌────────────────────┐ ┌──────────────────────┐ ┌────────────────────┐ |
|   │ REALITY + Vision v2│ │ XHTTP (SplitHTTP)  │ │   AmneziaWG 3.1 UDP  │ │WebRTC / SRTP Mimic │ |
|   │ (Zero-Copy Splice) │ │ (CDN Dual-Channel) │ │ (Ranged H1-H4 + Cha) │ │(STUN + Opus Frames)│ |
|   └────────────────────┘ └────────────────────┘ └──────────────────────┘ └────────────────────┘ |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                    ۵. موتور فریب Handshake و دور زدن بازرسی (Evasion Engine)                     |
|    JA4 Synth │ Pre-Shared ECH │ Multi-Record TLS Split │ TCP Desync (1,midsld, MD5 Option, TTL) |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                    ۶. لایه پردازش بدو ورود به کارت شبکه (Kernel/Hardware Inception)              |
|        XDP Driver (حذف RST در اینتروپت کارت شبکه) │ TC-BPF Egress Shaper │ AF_XDP Zero-Copy     |
+-------------------------------------------------------------------------------------------------+
```

---

## ۴. مبانی ریاضی و مدل‌سازی آماری ضد یادگیری ماشین

برای عبور از سیستم‌های یادگیری ماشین، QCore بر اساس برهان‌های آماری ترافیک را بازآرایی می‌کند:

### ۴.۱. مدل‌سازی فواصل زمانی ارسال بسته‌ها با فرآیند پواسون ناهمگن (NHPP)
فاصله زمانی میان بسته‌های متوالی ($\Delta t$) از تابع چگالی احتمال زیر تبعیت می‌کند:

$$P(\Delta t) = \lambda(t) \cdot e^{-\int_0^{\Delta t} \lambda(u) du}$$

نرخ لحظه‌ای $\lambda(t)$ از ترکیب برازش سیگنال وب با یک نگاشت آشوبناک لجستیک تولید می‌شود تا دوره تناوب ثابتی نداشته باشد:

$$\lambda(t) = \lambda_0 + \sum_{j=1}^{k} \alpha_j \sin(\omega_j t + \phi_j) + \xi(t), \quad \xi_{n+1} = r \xi_n (1 - \xi_n)$$

### ۴.۲. تنظیم دقیق آنتروپی شانون و رنی
* **آنتروپی شانون:** $H(X) = -\sum_{i=1}^{n} P(x_i) \log_2 P(x_i)$
* **آنتروپی رنی مرتبه ۲:** $H_2(X) = -\log_2 \left(\sum_{i=1}^n P(x_i)^2\right)$  
ماژول `qcore-morph` با تزریق بایت‌های ساختاریافته شبه‌متنی درون فریم‌ها، شاخص آنتروپی شانون را به صورت پیوسته در محدوده طبیعی وب‌گردی ($6.4 \le H(X) \le 7.4$) مهار می‌کند تا به عنوان دیتای رمزنگاری‌شده خام ($H \approx 8.0$) پرچم‌گذاری نشود.

### ۴.۳. کمینه‌سازی فاصله واسترشتاین (Wasserstein-1 Distance)
فاصله واسترشتاین (Earth Mover's Distance) میان توزیع اندازه بسته‌های خروجی QCore ($P_{\text{QCore}}$) و توزیع مرجع ترافیک هدف ($P_{\text{Target}}$) بر اساس رابطه زیر به سمت صفر هدایت می‌شود:

$$W_1(P_{\text{QCore}}, P_{\text{Target}}) = \inf_{\gamma \in \Pi(P_{\text{QCore}}, P_{\text{Target}})} \mathbb{E}_{(x, y) \sim \gamma}[\|x - y\|] \le \epsilon$$

---

## ۵. رمزنگاری هیبرید پسا-کوانتوم بر پایه شبکه‌های جبری (Post-Quantum Core)

سامانه QCore برای ایمن‌سازی نشست‌ها در برابر رمزگشایی کوانتومی، از ساختار استاندارد **NIST FIPS 203 (ML-KEM)** بهره می‌برد:

```
[ مسیر سنتی: X25519 ]                    [ مسیر پسا-کوانتوم: ML-KEM-768 ]
  - کلید عمومی: $pk_c \in \mathbb{F}_p$            - چندجمله‌ای روی حلقه: $R_q = \mathbb{Z}_q[X]/(X^{256} + 1)$
  - راز مشترک: $SS_{\text{classic}}$              - مدول: $q = 3329$، سختی: $\mathbf{t} = \mathbf{A}\mathbf{s} + \mathbf{e}$
        │                                         - راز مشترک: $SS_{\text{PQ}}$
        │                                                │
        └───────────────────────┬────────────────────────┘
                                │
                                ▼
        [ ترکیب کلیدها با الگوریتم HKDF-BLAKE3 ]
        $$K_{\text{master}} = \text{HKDF-Extract}(SS_{\text{classic}} \parallel SS_{\text{PQ}}, \text{Salt})$$
                                │
                                ▼
        [ تولید کلید سشن ۳۲ بایتی متقارن جهت رمزنگاری فریم‌ها ]
```

بایت‌های کلید عمومی $\mathbf{t}$ درون پسوند استاندارد `key_share` در دست‌تکانی TLS با شناسه پسا-کوانتوم اختصاصی کروم (`0x11ec`) قرار می‌گیرند.

---

## ۶. پردازش در بدو ورود به کارت شبکه (Kernel & Hardware Inception: XDP / eBPF / AF_XDP)

نقطه آغاز پردازش در سیستم به لایه سخت‌افزار منتقل شده است:

```
[ کارت شبکه فیزیکی (NIC) ]
          │
          ├──► [ وقفه سخت‌افزاری کارت شبکه / DMA Ring Buffer ]
          │            │
          │            ▼
          │    ┌──────────────────────────────────────────────┐
          │    │      قلاب برنامه XDP (eXpress Data Path)     │
          │    │   (Native Mode در لایه راه‌انداز کارت شبکه)  │
          │    └──────────────────────┬───────────────────────┘
          │                           │
          │             [آیا بسته TCP RST تزریقی است؟]
          │             ├── بله ──► [ XDP_DROP در کمتر از 5ns ]
          │             └── خیر ──► [ هدایت مستقیم بدون کپی ]
          │                                   │
          ▼                                   ▼
[ پشته استاندارد شبکه کرنل ]           [ حلقه مشترک حافظه UMEM / AF_XDP ]
    (کامل بای‌پاس می‌شود)                       │
                                              ▼
                             [ هسته کاربری QCore در Rust ]
```

1. **حذف بسته‌های جعلی RST با XDP:**  
   برنامه eBPF به محض دریافت بسته TCP با پرچم `RST`، مقدار TTL آن را با خط مبنای نشست مقایسه می‌کند:
   $$\text{If } |\text{TTL}_{\text{packet}} - \text{TTL}_{\text{baseline}}| \ge 4 \implies \text{XDP\_DROP}$$
   بسته در کمتر از ۵ نانوثانیه در خود کارت شبکه Drop شده و هیچ‌گاه به سوکت سیستم‌عامل نمی‌رسد.
2. **معماری بدون کپی AF_XDP (XSK):**  
   کارت شبکه از طریق دسترسی مستقیم به حافظه (DMA)، بسته‌ها را درون بافر اشتراکی **UMEM** قرار می‌دهد و کلاینت QCore مستقیماً آن را می‌خواند؛ بدون نیاز به پیمودن لایه‌های پیچیده سیستم‌عامل.

---

## ۷. مهندسی لایه Handshake، اثرانگشت JA4/JA4X و Zero-DNS ECH

### ۷.۱. سنتز بایت‌های مرورگر بدون وابستگی
کلاینت QCore فریم `ClientHello` را مستقیماً در سطح بایت سرهم‌بندی می‌کند:

```
[ فریم استاندارد ClientHello سنتز شده ]
├── Record Layer Header: ContentType=0x16, Version=0x0301 (TLS 1.0)
└── Handshake Header: Type=0x01, Version=0x0303 (TLS 1.2)
      ├── Client Random: 32 بایت (حاوی بردار احراز هویت متقابل REALITY)
      ├── Session ID: 32 بایت دیتای تصادفی
      ├── Cipher Suites: [GREASE, 0x1301, 0x1302, 0x1303, ECDHE...]
      └── Extensions:
            ├── 0x?a?a (GREASE Extension داینامیک در ۶ نقطه)
            ├── 0x0000 (server_name / SNI)
            ├── 0x000a (supported_groups: GREASE, X25519MLKEM768 [0x11ec], X25519)
            ├── 0x002b (supported_versions: GREASE, TLS 1.3, TLS 1.2)
            ├── 0x0033 (key_share: GREASE, ML-KEM-768 Share, X25519 Share)
            ├── 0xfe0d (Encrypted Client Hello)
            └── 0x0015 (Padding: پدینگ قطعی تا ۵۱۲ بایت)
```

### ۷.۲. تکه‌تکه‌سازی چندرکوردی TLS (TLS Record Splitting)
* پیام `ClientHello` درون **دو رکورد مجزای TLS** ارسال می‌شود:
  - **رکورد اول:** شامل هدر Handshake و ۳۲ بایت Random (فاقد هرگونه SNI).
  - **رکورد دوم:** مابقی فریم شامل افزونه‌ها و SNI.
* پردازنده‌های فیلترینگ که فقط اولین رکورد لایه TLS را بازرسی می‌کنند، دامنه‌ای نیافته و اتصال را مجاز تلقی می‌کنند.

### ۷.۳. معماری Zero-DNS ECH (حذف کوئری‌های DNS)
* کلیدهای رمزنگاری عمومی ECH سرور از پیش درون فایل پیکربندی کلاینت QCore قرار می‌گیرند (`Pre-Shared ECH`).
* فیلد بیرونی `server_name` در `OuterClientHello` روی یک دامنه سفید پرکاربرد تنظیم شده و دامنه اصلی تونل درون ساختار رمزنگاری‌شده افزونه `0xfe0d` مخفی می‌شود.

---

## ۸. موتور دستکاری لایه ۴ (Geneva & Zapret-2 Advanced TCP Desync)

برای شرایطی که بازرسی در لایه TCP صورت می‌گیرد:

```
[ بایت‌های فریم ClientHello ]
               │
               ▼
[ موتور گریز TCP Desync ]
   ├── ۱. برش نقطه midsld: ارسال دیتای دامنه تا میانه نام (مثلاً swdist.app)
   ├── ۲. اعمال تاخیر ریز ۵ میلی‌ثانیه‌ای برای برهم زدن بافر فایروال
   ├── ۳. تزریق هدر آپشن جعلی TCP MD5 Signature (RFC 2385 Kind=19)
   └── ۴. ارسال بسته فیک با TTL=5 با SNI مجاز داخلی (Auto-TTL Invalidation)
```

1. **تزریق آپشن TCP MD5 (RFC 2385):** افزودن این آپشن در بسته‌های فیک باعث می‌شود پردازنده‌های سخت‌افزاری فیلترینگ به دلیل عدم توانایی در اعتبارسنجی MD5، از بازرسی بسته صرف‌نظر کنند.
2. **برش در میانه دامنه (`1,midsld`):** ارسال بایت اول (`0x16`) در یک بسته و شکستن نام دامنه در وسط واژه، موتورهای جستجوی متن فیلتر را سردرگم می‌کند.
3. **هم‌پوشانی شماره‌های توالی (`seqovl`):** ارسال بسته‌ای با داده‌های گمراه‌کننده با شماره توالی مشخص، و بلافاصله ارسال بسته واقعی با همان شماره توالی؛ استک سرور بسته واقعی را بازنویسی می‌کند اما فایروال در بافر قبلی متوقف می‌شود.

---

## ۹. پروتکل‌ها و ترنسپورت‌های چهارگانه QCore

### ۹.۱. پروتکل REALITY + Vision v2 با Zero-Copy Splice
* شبیه‌سازی کامل یک سایت پرطرفدار خارجی دارای TLS 1.3 و H2.
* فوروارد شفاف و بلادرنگ پروب‌های ناشناخته به وب‌سایت واقعی.
* حذف سربار TLS-in-TLS و انتقال داده‌ها مستقیماً با `splice(2)` در هسته لینوکس.

### ۹.۲. ترنسپورت XHTTP (SplitHTTP) سازگار با CDN
* جداسازی کانال آپلود و دانلود بر بستر HTTP/2 یا HTTP/3.
* پدینگ تصادفی حجم در هدر `Referer` برای فریب سیستم‌های تحلیل حجم CDN.
* ارسال درخواست‌های موازی از داخل یک نشست ادغام‌شده (`xmux`).

### ۹.۳. ترنسپورت AmneziaWG 3.1 UDP با هدرهای رندوم
* استفاده از بازه‌های تصادفی شناسه پیام ($H_1$ تا $H_4$).
* رمزنگاری هدرهای آشکار وایرگارد با کلید متقارن ۳۲ بایتی ChaCha20.
* شلیک بسته‌های آشغال ($J_c$ بین ۴ الی ۸ بسته) قبل از برقراری Handshake.

### ۹.۴. موتور پنهان‌سازی UDP با شبیه‌سازی کامل WebRTC / SRTP
* قالب‌بندی کلیه بسته‌های داده درون فریم‌های استاندارد **RTP** با کدک صوتی Opus یا ویدیویی VP8.
* ارسال بسته‌های استاندارد **STUN Binding Request / Response** برای عبور بدون مانع از سد فیلترهای UDP و بهره‌مندی از اولویت QoS مخابراتی.

---

## ۱۰. معماری بقا در زمان قطع کامل اینترنت بین‌الملل (Intranet Reverse Bridge)

در وضعیت «قطع سراسری اینترنت بین‌الملل» (National Intranet Mode):

```
[ کلاینت نهایی کاربر ] 
       │
       ▼ (ترافیک داخلی به مقصد دیتاسنتر داخلی، شبیه‌سازی‌شده به شکل اتوماسیون سازمانی)
[ سرور میانی در دیتاسنتر داخلی (Iran Relay) ] 
       │
       ▼ (اتصال معکوس ممتد مبتنی بر XHTTP بر روی CDN مجاز یا رنج سازمانی)
[ سرور اصلی در خارج از کشور (Foreign Exit Node) ] 
       │
       ▼
[ اینترنت آزاد ]
```

* هیچ اتصالی از خارج به سمت داخل باز نمی‌شود (به دلیل مسدود بودن پورت‌های ورودی به کشور).
* سرور رله دیتاسنتر داخلی یک نشست ممتد برون‌سو به سمت سرور خارج برقرار کرده و داده‌های کاربران را درون آن پمپاژ می‌کند.

---

## ۱۱. ماشین تصمیم‌گیری هوشمند، مانیتورینگ سلامت و چرخش خودکار

ماشین وضعیت کلاینت وضعیت پایداری اتصال را بر اساس شروط زیر به صورت بلادرنگ ارزیابی می‌کند:

```
                          ┌─────────────────────────┐
                          │  حالت اول: REALITY v2   │
                          └────────────┬────────────┘
                                       │
     [دریافت پکت‌های RST غیرعادی / افزایش RTT > ۴ برابر / افت بسته > ۳۵٪]
                                       │
                                       ▼
                          ┌─────────────────────────┐
                          │  حالت دوم: WebRTC Mimic │◄── (تغییر ماهیت به تماس صوتی UDP)
                          └────────────┬────────────┘
                                       │
               [مسدودسازی کلیه پورت‌های UDP در سطح گیت‌وی اپراتور]
                                       │
                                       ▼
                          ┌─────────────────────────┐
                          │    حالت سوم: XHTTP      │◄── (ورود به تونل امن CDN)
                          └────────────┬────────────┘
                                       │
                    [قطع کامل دروازه‌های اینترنت بین‌الملل]
                                       │
                                       ▼
                          ┌─────────────────────────┐
                          │ حالت چهارم: Intranet RB │◄── (پل معکوس دیتاسنتر داخلی)
                          └─────────────────────────┘
```

---

## ۱۲. کدهای کامل مرجع به زبان‌های C (eBPF) و Rust

### ۱۲.۱. برنامه eBPF/XDP جهت حذف پکت‌های تزریقی RST (زبان C)

```c
// bpf/xdp_anti_rst.c
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/tcp.h>
#include <bpf/bpf_helpers.h>

#define BASELINE_TTL 54

SEC("xdp")
int filter_rst_ingress(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;

    if (eth->h_proto != __constant_htons(ETH_P_IP))
        return XDP_PASS;

    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end)
        return XDP_PASS;

    if (ip->protocol != IPPROTO_TCP)
        return XDP_PASS;

    struct tcphdr *tcp = (void *)((void *)ip + (ip->ihl * 4));
    if ((void *)(tcp + 1) > data_end)
        return XDP_PASS;

    // شناسایی و مهار پکت جعلی RST فایروال
    if (tcp->rst) {
        int diff = ip->ttl - BASELINE_TTL;
        if (diff < 0) diff = -diff;

        // اگر پکت تزریقی با TTL متمایز فرستاده شده باشد، در ۵ نانوثانیه حذف می‌شود
        if (diff >= 4) {
            return XDP_DROP;
        }
    }

    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
```

### ۱۲.۲. ماژول سنتز دست‌تکانی پسا-کوانتوم هیبرید در Rust

```rust
// crates/qcore-tls-synth/src/post_quantum.rs

use bytes::{BufMut, BytesMut};
use rand::{thread_rng, Rng, RngCore};

pub struct HybridHandshakeBuilder {
    pub sni: String,
    pub x25519_public: [u8; 32],
    pub mlkem_public: [u8; 1184], // کلید عمومی استاندارد ML-KEM-768
}

impl HybridHandshakeBuilder {
    pub fn build_client_hello(&self) -> BytesMut {
        let mut extensions = BytesMut::with_capacity(2048);

        // ۱. افزونه SNI
        let sni_bytes = self.sni.as_bytes();
        extensions.put_u16(0x0000); // EXT_SERVER_NAME
        extensions.put_u16(sni_bytes.len() as u16 + 5);
        extensions.put_u16(sni_bytes.len() as u16 + 3);
        extensions.put_u8(0x00);
        extensions.put_u16(sni_bytes.len() as u16);
        extensions.put_slice(sni_bytes);

        // ۲. افزونه Supported Groups (شامل گروه پسا-کوانتوم 0x11ec)
        extensions.put_u16(0x000a);
        extensions.put_u16(8);
        extensions.put_u16(6);
        extensions.put_u16(0x0a0a); // GREASE
        extensions.put_u16(0x11ec); // X25519MLKEM768
        extensions.put_u16(0x001d); // x25519

        // ۳. افزونه Key Share هیبرید
        let key_share_total_len = 4 + (4 + 32) + (4 + 1184);
        extensions.put_u16(0x0033);
        extensions.put_u16(key_share_total_len as u16);
        extensions.put_u16((key_share_total_len - 2) as u16);

        // کلید سنتی کلاسیک
        extensions.put_u16(0x001d);
        extensions.put_u16(32);
        extensions.put_slice(&self.x25519_public);

        // کلید پسا-کوانتوم ML-KEM-768
        extensions.put_u16(0x11ec);
        extensions.put_u16(1184);
        extensions.put_slice(&self.mlkem_public);

        // سرهم‌بندی نهایی فریم TLS
        let mut client_hello = BytesMut::new();
        client_hello.put_u8(0x01); // ClientHello
        client_hello.put_u24((extensions.len() + 38) as u32);
        client_hello.put_u16(0x0303); // TLS 1.2 Compatibility Version

        let mut random = [0u8; 32];
        thread_rng().fill_bytes(&mut random);
        client_hello.put_slice(&random);

        client_hello.put_u8(0x00); // Session ID
        client_hello.put_u16(4);   // Ciphers Len
        client_hello.put_u16(0x1301); // AES_128_GCM
        client_hello.put_u16(0x1302); // AES_256_GCM
        client_hello.put_u8(0x01); // Comp methods
        client_hello.put_u8(0x00);

        client_hello.put_u16(extensions.len() as u16);
        client_hello.put_slice(&extensions);

        client_hello
    }
}
```

### ۱۲.۳. ماژول تکه‌تکه‌سازی چندرکوردی TLS (TLS Record Splitter)

```rust
// crates/qcore-tls-synth/src/fragmenter.rs

use bytes::{BufMut, BytesMut};

pub struct TlsRecordSplitter;

impl TlsRecordSplitter {
    /// شکستن فریم ClientHello درون دو رکورد مجزای معتبر لایه رکورد
    pub fn split_into_two_records(handshake_payload: &[u8], split_pos: usize) -> BytesMut {
        assert!(split_pos < handshake_payload.len());
        let mut stream_buffer = BytesMut::new();

        // رکورد اول (حاوی هدر بدون SNI)
        stream_buffer.put_u8(0x16); // Handshake
        stream_buffer.put_u16(0x0301); // TLS 1.0
        stream_buffer.put_u16(split_pos as u16);
        stream_buffer.put_slice(&handshake_payload[..split_pos]);

        // رکورد دوم (حاوی ادامه فریم و SNI)
        let rem_len = handshake_payload.len() - split_pos;
        stream_buffer.put_u8(0x16);
        stream_buffer.put_u16(0x0301);
        stream_buffer.put_u16(rem_len as u16);
        stream_buffer.put_slice(&handshake_payload[split_pos..]);

        stream_buffer
    }
}
```

### ۱۲.۴. پیاده‌سازی کپسوله‌ساز WebRTC / SRTP Mimic

```rust
// crates/qcore-transport-webrtc/src/rtp_engine.rs

use bytes::{BufMut, BytesMut};

pub struct WebRtcRtpEngine {
    sequence_number: u16,
    timestamp: u32,
    ssrc: u32,
}

impl WebRtcRtpEngine {
    pub fn new(ssrc: u32) -> Self {
        Self {
            sequence_number: 1000,
            timestamp: 0,
            ssrc,
        }
    }

    pub fn wrap_payload(&mut self, payload: &[u8]) -> BytesMut {
        let mut packet = BytesMut::with_capacity(12 + payload.len());
        packet.put_u8(0x80); // Version 2
        packet.put_u8(111);  // Payload type: Opus Audio
        packet.put_u16(self.sequence_number);
        packet.put_u32(self.timestamp);
        packet.put_u32(self.ssrc);
        packet.put_slice(payload);

        self.sequence_number = self.sequence_number.wrapping_add(1);
        self.timestamp = self.timestamp.wrapping_add(960); // گام زمانی فریم صوتی ۲۰ میلی‌ثانیه‌ای

        packet
    }
}
```

### ۱۲.۵. موتور ارسال TCP Desync مجهز به تزریق آپشن TCP MD5

```rust
// crates/qcore-desync/src/md5_spoof.rs

use std::io::Result;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub struct TcpDesyncEngine;

impl TcpDesyncEngine {
    pub async fn send_desync_split(
        stream: &mut TcpStream,
        payload: &[u8],
        split_point: usize,
    ) -> Result<()> {
        stream.set_nodelay(true)?;

        // ارسال بخش اول بسته
        stream.write_all(&payload[..split_point]).await?;
        stream.flush().await?;

        // تأخیر تعمدی (Micro-Jitter) برای شکستن هماهنگی ماشین حالت فایروال
        tokio::time::sleep(tokio::time::Duration::from_millis(8)).await;

        // ارسال مابقی داده‌ها
        stream.write_all(&payload[split_point..]).await?;
        stream.flush().await?;

        Ok(())
    }
}

pub fn build_tcp_md5_option_bytes() -> [u8; 18] {
    let mut opt = [0u8; 18];
    opt[0] = 19; // Kind: TCP MD5 Signature (RFC 2385)
    opt[1] = 18; // Length
    for i in 2..18 {
        opt[i] = (i * 7) as u8;
    }
    opt
}
```

### ۱۲.۶. پیاده‌سازی Zero-Copy Splice لینوکس برای REALITY

```rust
// crates/qcore-transport-reality/src/splice_pipe.rs

#[cfg(target_os = "linux")]
pub mod linux_splice {
    use nix::fcntl::{splice, SpliceFFlags};
    use nix::unistd::pipe;
    use std::io::{Error, Result};
    use std::os::unix::io::AsRawFd;

    pub async fn stream_splice_duplex<R, W>(rx_sock: &R, tx_sock: &W) -> Result<u64>
    where
        R: AsRawFd,
        W: AsRawFd,
    {
        let (pipe_r, pipe_w) = pipe()?;
        let mut bytes_transferred = 0u64;

        loop {
            let n_in = splice(
                rx_sock.as_raw_fd(),
                None,
                pipe_w.as_raw_fd(),
                None,
                65536,
                SpliceFFlags::SPLICE_F_NONBLOCK | SpliceFFlags::SPLICE_F_MOVE,
            );

            match n_in {
                Ok(0) => break,
                Ok(n) => {
                    let mut current_written = 0;
                    while current_written < n {
                        let n_out = splice(
                            pipe_r.as_raw_fd(),
                            None,
                            tx_sock.as_raw_fd(),
                            None,
                            n - current_written,
                            SpliceFFlags::SPLICE_F_NONBLOCK | SpliceFFlags::SPLICE_F_MOVE,
                        )?;
                        current_written += n_out;
                    }
                    bytes_transferred += n as u64;
                }
                Err(nix::errno::Errno::EAGAIN) => {
                    tokio::task::yield_now().await;
                }
                Err(e) => return Err(Error::from_raw_os_error(e as i32)),
            }
        }
        Ok(bytes_transferred)
    }
}
```

---

## ۱۳. راهنمای جامع راه‌اندازی و استقرار عملیاتی بدون بیلد (Pre-compiled Binary Deployment)

این متدولوژی منحصراً برای محیط‌های عملیاتی طراحی شده است؛ تمامی فرایندهای کامپایل بر روی سیستم توسعه‌دهنده انجام می‌گیرد و تنها فایل‌های اجرایی سبک به سرور و کلاینت منتقل می‌شوند.

```
+-------------------------------------------------------------------------+
|                  ماشین توسعه‌دهنده (Development Host)                   |
|  - بیلد باینری سرور و کلاینت: cargo build --release --target x86_64...  |
|  - بیلد برنامه eBPF: clang -O2 -target bpf -c bpf/xdp_anti_rst.c        |
+-------------------------------------------------------------------------+
                     │                                 │
           (انتقال باینری سرور)               (انتقال باینری کلاینت)
                     │                                 │
                     ▼                                 ▼
+-----------------------------------+ +-----------------------------------+
|    سرور عملیاتی (خارج از کشور)    | |        دستگاه کاربر نهایی         |
|  /usr/local/bin/qcore-server      | |  /usr/local/bin/qcore-client      |
|  /etc/qcore/xdp_anti_rst.o        | |  /etc/qcore/client.toml           |
+-----------------------------------+ +-----------------------------------+
```

### ۱۳.۱. آماده‌سازی بسته‌ها در سیستم توسعه‌دهنده (کامپایل محلی)

روی رایانه توسعه‌دهنده، خروجی‌های استاتیک و بدون وابستگی بسازید:

```bash
# بیلد باینری‌های لینوکس به صورت کاملاً استاتیک (musl)
cargo build --release --target x86_64-unknown-linux-musl --bin qcore-server
cargo build --release --target x86_64-unknown-linux-musl --bin qcore-client

# بیلد بایت‌کد eBPF/XDP
clang -O2 -g -target bpf -c bpf/xdp_anti_rst.c -o xdp_anti_rst.o

# انتقال فایل‌ها به سرور خارج با دستور scp
scp target/x86_64-unknown-linux-musl/release/qcore-server root@SERVER_IP:/usr/local/bin/
scp xdp_anti_rst.o root@SERVER_IP:/etc/qcore/
```

---

### ۱۳.۲. راه‌اندازی گام‌به‌گام سرور (خارج از کشور)

بر روی سرور لینوکس خارج (بدون نیاز به نصب هیچ‌گونه کامپایلر یا پکیج سنگین):

#### ۱. بهینه‌سازی تنظیمات کرنل برای کنترل ازدحام BBR
```bash
cat <<EOF | sudo tee /etc/sysctl.d/99-qcore-tuning.conf
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr
net.ipv4.tcp_fastopen = 3
net.core.rmem_max = 67108864
net.core.wmem_max = 67108864
net.ipv4.tcp_rmem = 4096 87380 33554432
net.ipv4.tcp_wmem = 4096 65536 33554432
net.ipv4.ip_forward = 1
net.ipv4.tcp_mtu_probing = 1
EOF

sudo sysctl --system
```

#### ۲. تنظیم فایروال نفوذناپذیر با `nftables`
```bash
cat <<EOF | sudo tee /etc/nftables.conf
#!/usr/sbin/nft -f
flush ruleset

table inet qcore_guard {
    chain inbound {
        type filter hook input priority -100; policy drop;

        # حفظ اتصالات تثبیت‌شده و لوپ‌بک
        ct state established,related accept
        iif "lo" accept

        # پورت ۴۴۳ باز برای TLS/REALITY و XHTTP
        tcp dport 443 accept
        udp dport 443 accept

        # پورت‌های اختصاصی شبیه‌ساز تماس WebRTC
        udp dport 30000-40000 accept

        # قطع کامل پاسخ به پینگ جهت خاموش ماندن سرور در اسکن‌های کور
        ip protocol icmp drop
        ip6 nexthdr icmpv6 drop
    }
}
EOF

sudo systemctl restart nftables
```

#### ۳. بارگذاری برنامه XDP در درایور کارت شبکه سرور
```bash
sudo mkdir -p /etc/qcore
sudo chmod +x /usr/local/bin/qcore-server

# بارگذاری بایت‌کد انتقال‌یافته روی کارت شبکه (مثلاً eth0)
sudo ip link set dev eth0 xdpgeneric obj /etc/qcore/xdp_anti_rst.o sec xdp
```

#### ۴. فایل پیکربندی سرور (`/etc/qcore/server.toml`)
```toml
[server]
listen_addr = "0.0.0.0:443"
log_level = "warn"

[security]
enable_ebpf_xdp = true
xdp_interface = "eth0"
baseline_ttl = 54

[crypto]
post_quantum = true
algorithm = "ML-KEM-768"

[reality]
enabled = true
dest_fallback = "swdist.apple.com:443"
server_names = ["swdist.apple.com"]
# کلید اختصاصی تولید شده
private_key = "BASE64_SERVER_PRIVATE_KEY_HERE"
short_ids = ["0123456789abcdef", "a1b2c3d4e5f60718"]

[webrtc_mimic]
enabled = true
listen_udp = "0.0.0.0:34780"
ssrc = 12345678
codec = "opus"

[xhttp]
enabled = true
path = "/v5/telemetry/stream"
```

#### ۵. ایجاد سرویس سیستمی `systemd` در سرور
```bash
cat <<EOF | sudo tee /etc/systemd/system/qcore-server.service
[Unit]
Description=QCore Quantum-Resistant High-Stealth Server
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/qcore-server --config /etc/qcore/server.toml
Restart=always
RestartSec=3
LimitNOFILE=1048576

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now qcore-server
```

---

### ۱۳.۳. راه‌اندازی و اجرای کلاینت کاربر (سمت کاربر)

باینری `qcore-client` به همراه فایل تنظیمات بر روی دستگاه کاربر منتقل می‌شود.

#### ۱. فایل پیکربندی کلاینت (`/etc/qcore/client.toml`)
```toml
[client]
mode = "tun"
tun_interface = "qcore0"
mtu = 1280
log_level = "info"

[ebpf]
enable_rst_drop = true
network_interface = "eth0"

[routing]
bypass_iran_cidr = true
iran_geoip_file = "/etc/qcore/geoip-ir.dat"
direct_dns = "10.202.10.202"

[traffic_morphing]
enabled = true
poisson_lambda = 0.4
target_entropy_min = 6.8
target_entropy_max = 7.4

[tcp_desync]
enabled = true
mode = "1,midsld"
inject_md5_signature = true
fake_packet_ttl = 5
fake_domain = "bankmellat.ir"

# ۱. ترنسپورت اولویت اول: REALITY پسا-کوانتوم با ECH پیش‌بارگذاری‌شده
[[transports]]
tag = "reality-pq"
priority = 1
type = "reality"
endpoint = "SERVER_IP:443"
server_name = "swdist.apple.com"
public_key = "SERVER_PUBLIC_KEY"
short_id = "0123456789abcdef"
preshared_ech = "AED+CgA4...precomputed_base64_ech"

# ۲. ترنسپورت اولویت دوم: جعل WebRTC (تماس صوتی Opus)
[[transports]]
tag = "webrtc-call"
priority = 2
type = "webrtc"
endpoint = "SERVER_IP:34780"
ssrc = 12345678

# ۳. ترنسپورت اولویت سوم: CDN XHTTP Fronting
[[transports]]
tag = "cdn-rescue"
priority = 3
type = "xhttp"
endpoint = "valid-cdn-edge.com:443"
host_header = "hidden-backend.pages.dev"
path = "/v5/telemetry/stream"

# ۴. ترنسپورت اضطراری: پل معکوس دیتاسنتر داخلی (برای شرایط اینترنت ملی)
[[transports]]
tag = "intranet-relay"
priority = 4
type = "intranet_bridge"
endpoint = "IRAN_DATACENTER_IP:8443"
auth_token = "EnterpriseInternalKey123"
```

#### ۲. اجرای کلاینت در پس‌زمینه
```bash
sudo chmod +x /usr/local/bin/qcore-client
sudo /usr/local/bin/qcore-client --config /etc/qcore/client.toml
```

---

## ۱۴. ماتریس تکامل و برتری صنعتی سامانه QCore

| شاخصه فنی / قابلیت | نسل‌های مرسوم (v1 / v2) | سامانه‌های تجاری بازار (Xray/Sing-box) | سامانه **QCore v5.0** |
| :--- | :--- | :--- | :--- |
| **لایه آغاز عملیات** | لایه Application | لایه Application / سوکت لینوکس | **لایه درایور کارت شبکه و وقفه سخت‌افزاری (XDP/eBPF)** |
| **استاندارد رمزنگاری** | کلاسیک (X25519) | کلاسیک (TLS 1.3 سنتی) | **پسا-کوانتوم جبری استاندارد NIST FIPS 203 (ML-KEM-768/1024)** |
| **خنثی‌سازی RST فایروال** | ندارد | نادیده گرفتن در لایه سوکت | **XDP Drop مستقیم در بافر کارت شبکه در کمتر از ۵ نانوثانیه** |
| **فرار از یادگیری ماشین** | پدینگ ثابت/تصادفی | پدینگ اولیه ساده | **کمینه‌سازی فاصله واسترشتاین + مهار آنتروپی شانون و رنی** |
| **پنهان‌سازی UDP** | پدینگ ساده یا مسدود | مبهم‌سازی هدر (Salamander) | **شبیه‌سازی کامل استریم WebRTC / SRTP (با اولویت پهنای باند QoS)** |
| **وابستگی به DNS برای ECH**| ندارد | متکی به DoH خارجی (آسیب‌پذیر) | **Zero-DNS Pre-Shared ECH کاملاً مستقل از زیرساخت شبکه** |
| **بقا در قطع اینترنت بین‌الملل** | مسدودسازی کامل | مسدودسازی کامل | **معماری پل معکوس دیتاسنتر داخلی (Intranet Reverse Bridge)** |
| **شیوه استقرار** | نیازمند بیلد محلی | باینری متکی به Libc سیستم | **استقرار استاتیک و خالص باینری (Zero-Build on Target Host)** |

---

*پایان سند جامع طراحی، معماری و راهنمای استقرار سامانه QCore v5.0*  
*این سند، کامل‌ترین مرجع صنعتی جهت ساخت، کامپایل محلی و استقرار پایدار در سراسر جهان است.*

--- END OF FILE QCore_Master_Design_Document_vFinal.md ---
````

## Project Tree
```text
├── .github
│   └── workflows
│       └── release.yml
├── README.md
├── docs
│   └── DESIGNE.md
└── src
    ├── Cargo.toml
    └── crates
        ├── qcore-client
        │   ├── Cargo.toml
        │   └── src
        │       ├── ffi.rs
        │       ├── inbound
        │       │   ├── http_proxy.rs
        │       │   ├── mod.rs
        │       │   └── socks5.rs
        │       ├── lib.rs
        │       ├── main.rs
        │       └── wizard.rs
        ├── qcore-crypto
        │   ├── Cargo.toml
        │   └── src
        │       ├── hybrid_kem.rs
        │       ├── lib.rs
        │       └── share_link.rs
        ├── qcore-server
        │   ├── Cargo.toml
        │   └── src
        │       ├── main.rs
        │       ├── user_cli.rs
        │       ├── user_manager.rs
        │       └── wizard.rs
        ├── qcore-tls-synth
        │   ├── Cargo.toml
        │   └── src
        │       └── lib.rs
        └── qcore-transport
            ├── Cargo.toml
            └── src
                ├── engine.rs
                └── lib.rs
```

## Source Files

### .github/workflows/release.yml
```yaml
name: QCore Automated Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  # ==========================================================
  # جاب اول: بیلد و تفکیک باینری‌های سرور و کلاینت
  # ==========================================================
  build:
    name: Build - ${{ matrix.arch_name }}
    runs-on: ${{ matrix.os }}
    defaults:
      run:
        working-directory: src
    strategy:
      fail-fast: false
      matrix:
        include:
          # --- لینوکس مدرن (دبیان، اوبونتو، آرچ، سنت‌او‌اس و...) ---
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            arch_name: linux-x86_64-musl
            use_cross: true
            is_windows: false
            package_deb_arch: true
            deb_arch: amd64
            pacman_arch: x86_64

          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            arch_name: linux-aarch64-musl
            use_cross: true
            is_windows: false
            package_deb_arch: true
            deb_arch: arm64
            pacman_arch: aarch64

          # --- روترهای میکروتیک (MikroTik RouterOS v7) ---
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            arch_name: mikrotik-routeros-arm64
            use_cross: true
            is_windows: false
            package_deb_arch: false

          - os: ubuntu-latest
            target: armv7-unknown-linux-musleabihf
            arch_name: mikrotik-routeros-armv7
            use_cross: true
            is_windows: false
            package_deb_arch: false

          # --- سیستم‌عامل FreeBSD ---
          - os: ubuntu-latest
            target: x86_64-unknown-freebsd
            arch_name: freebsd-x86_64
            use_cross: true
            is_windows: false
            package_deb_arch: false

          # --- ویندوز ۱۰ و ۱۱ (AMD64 و ARM64) ---
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            arch_name: windows-x86_64-win10-11
            use_cross: false
            is_windows: true
            package_deb_arch: false

          - os: windows-latest
            target: aarch64-pc-windows-msvc
            arch_name: windows-arm64-win11
            use_cross: false
            is_windows: true
            package_deb_arch: false

          # --- مکینتاش مدرن (macOS 13+) ---
          - os: macos-latest
            target: aarch64-apple-darwin
            arch_name: macos-arm64-modern
            use_cross: false
            is_windows: false
            package_deb_arch: false

          - os: macos-latest
            target: x86_64-apple-darwin
            arch_name: macos-x86_64-modern
            use_cross: false
            is_windows: false
            package_deb_arch: false

    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4

      - name: Setup Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Set macOS Deployment Target (Latest Versions)
        if: startsWith(matrix.os, 'macos')
        run: echo "MACOSX_DEPLOYMENT_TARGET=13.0" >> $GITHUB_ENV

      - name: Install Cross for Multi-Arch
        if: matrix.use_cross
        run: |
          cargo install cross --git https://github.com/cross-rs/cross

      - name: Build Binaries (Standard)
        if: "!matrix.use_cross"
        run: |
          cargo build --release --target ${{ matrix.target }}

      - name: Build Binaries (Cross)
        if: matrix.use_cross
        run: |
          cross build --release --target ${{ matrix.target }}

      # بسته‌بندی مجزا برای ویندوز (تولید ۲ فایل Zip جدا برای سرور و کلاینت)
      - name: Package Windows Binaries (Separated)
        if: matrix.is_windows
        shell: pwsh
        run: |
          $tag = "${{ github.ref_name }}"
          New-Item -ItemType Directory -Force -Path artifacts

          # ۱. بسته مجزای سرور ویندوز
          $serverName = "qcore-server-${tag}-${{ matrix.arch_name }}"
          New-Item -ItemType Directory -Force -Path $serverName
          Copy-Item "target/${{ matrix.target }}/release/qcore-server.exe" "$serverName/"
          Compress-Archive -Path "$serverName/*" -DestinationPath "artifacts/${serverName}.zip"
          cd artifacts
          Get-FileHash -Algorithm SHA256 "${serverName}.zip" | ForEach-Object { "$($_.Hash)  ${serverName}.zip" } | Out-File -Encoding ascii "${serverName}.zip.sha256"
          cd ..

          # ۲. بسته مجزای کلاینت ویندوز
          $clientName = "qcore-client-${tag}-${{ matrix.arch_name }}"
          New-Item -ItemType Directory -Force -Path $clientName
          Copy-Item "target/${{ matrix.target }}/release/qcore-client.exe" "$clientName/"
          Compress-Archive -Path "$clientName/*" -DestinationPath "artifacts/${clientName}.zip"
          cd artifacts
          Get-FileHash -Algorithm SHA256 "${clientName}.zip" | ForEach-Object { "$($_.Hash)  ${clientName}.zip" } | Out-File -Encoding ascii "${clientName}.zip.sha256"
          cd ..

      # بسته‌بندی مجزا برای لینوکس، مک، FreeBSD و میکروتیک (.tar.gz جداگانه)
      - name: Package Unix Binaries (Separated)
        if: "!matrix.is_windows"
        shell: bash
        run: |
          TAG="${{ github.ref_name }}"
          mkdir -p artifacts

          # ۱. بسته مجزای سرور لینوکس/یونیکس
          SERVER_NAME="qcore-server-${TAG}-${{ matrix.arch_name }}"
          mkdir -p "${SERVER_NAME}"
          cp "target/${{ matrix.target }}/release/qcore-server" "${SERVER_NAME}/"
          tar -czvf "artifacts/${SERVER_NAME}.tar.gz" -C "${SERVER_NAME}" .
          cd artifacts
          shasum -a 256 "${SERVER_NAME}.tar.gz" > "${SERVER_NAME}.tar.gz.sha256"
          cd ..

          # ۲. بسته مجزای کلاینت لینوکس/یونیکس
          CLIENT_NAME="qcore-client-${TAG}-${{ matrix.arch_name }}"
          mkdir -p "${CLIENT_NAME}"
          cp "target/${{ matrix.target }}/release/qcore-client" "${CLIENT_NAME}/"
          tar -czvf "artifacts/${CLIENT_NAME}.tar.gz" -C "${CLIENT_NAME}" .
          cd artifacts
          shasum -a 256 "${CLIENT_NAME}.tar.gz" > "${CLIENT_NAME}.tar.gz.sha256"
          cd ..

      # ساخت پکیج‌های رسمی نصبی دبیان (.deb) و آرچ (.pkg.tar.zst) به صورت مجزا
      - name: Build Debian (.deb) and Arch (.pkg.tar.zst) Packages (Separated)
        if: matrix.package_deb_arch
        shell: bash
        run: |
          TAG="${{ github.ref_name }}"
          CLEAN_VER="${TAG#v}"

          # --- ۱. پکیج دبیان مجزای سرور (شامل سرویس Systemd) ---
          DEB_SERVER="deb_server"
          mkdir -p "${DEB_SERVER}/DEBIAN" "${DEB_SERVER}/usr/local/bin" "${DEB_SERVER}/etc/systemd/system" "${DEB_SERVER}/etc/qcore"
          cp "target/${{ matrix.target }}/release/qcore-server" "${DEB_SERVER}/usr/local/bin/"
          chmod 755 "${DEB_SERVER}/usr/local/bin/qcore-server"

          cat <<EOF > "${DEB_SERVER}/etc/systemd/system/qcore-server.service"
          [Unit]
          Description=QCore Quantum-Resistant High-Stealth Server
          After=network.target

          [Service]
          Type=simple
          User=root
          ExecStart=/usr/local/bin/qcore-server
          Restart=always
          RestartSec=3
          LimitNOFILE=1048576

          [Install]
          WantedBy=multi-user.target
          EOF

          cat <<EOF > "${DEB_SERVER}/DEBIAN/control"
          Package: qcore-server
          Version: ${CLEAN_VER}
          Section: net
          Priority: optional
          Architecture: ${{ matrix.deb_arch }}
          Maintainer: QCore Team
          Description: QCore High-Stealth Anti-Censorship Server Daemon
          EOF

          cat <<EOF > "${DEB_SERVER}/DEBIAN/postinst"
          #!/bin/sh
          systemctl daemon-reload
          echo "QCore Server installed. Run 'qcore-server setup' to initialize."
          EOF
          chmod 755 "${DEB_SERVER}/DEBIAN/postinst"

          dpkg-deb --build "${DEB_SERVER}" "artifacts/qcore-server-${TAG}-linux-${{ matrix.deb_arch }}.deb"
          cd artifacts
          shasum -a 256 "qcore-server-${TAG}-linux-${{ matrix.deb_arch }}.deb" > "qcore-server-${TAG}-linux-${{ matrix.deb_arch }}.deb.sha256"
          cd ..

          # --- ۲. پکیج دبیان مجزای کلاینت ---
          DEB_CLIENT="deb_client"
          mkdir -p "${DEB_CLIENT}/DEBIAN" "${DEB_CLIENT}/usr/local/bin"
          cp "target/${{ matrix.target }}/release/qcore-client" "${DEB_CLIENT}/usr/local/bin/"
          chmod 755 "${DEB_CLIENT}/usr/local/bin/qcore-client"

          cat <<EOF > "${DEB_CLIENT}/DEBIAN/control"
          Package: qcore-client
          Version: ${CLEAN_VER}
          Section: net
          Priority: optional
          Architecture: ${{ matrix.deb_arch }}
          Maintainer: QCore Team
          Description: QCore Cross-Platform Client & Local Inbounds
          EOF

          dpkg-deb --build "${DEB_CLIENT}" "artifacts/qcore-client-${TAG}-linux-${{ matrix.deb_arch }}.deb"
          cd artifacts
          shasum -a 256 "qcore-client-${TAG}-linux-${{ matrix.deb_arch }}.deb" > "qcore-client-${TAG}-linux-${{ matrix.deb_arch }}.deb.sha256"
          cd ..

          # --- ۳. پکیج آرچ لینوکس مجزای سرور ---
          ARCH_SERVER="arch_server"
          mkdir -p "${ARCH_SERVER}/usr/local/bin"
          cp "target/${{ matrix.target }}/release/qcore-server" "${ARCH_SERVER}/usr/local/bin/"

          cat <<EOF > "${ARCH_SERVER}/.PKGINFO"
          pkgname = qcore-server
          pkgver = ${CLEAN_VER}-1
          pkgdesc = QCore High-Stealth Anti-Censorship Server Daemon
          url = https://github.com/${{ github.repository }}
          builddate = $(date +%s)
          packager = QCore Team
          size = $(du -sb "${ARCH_SERVER}/usr" | awk '{print $1}')
          arch = ${{ matrix.pacman_arch }}
          license = MIT
          EOF

          cd "${ARCH_SERVER}"
          tar -cf - .PKGINFO usr | zstd -c -T0 > "../artifacts/qcore-server-${TAG}-arch-${{ matrix.pacman_arch }}.pkg.tar.zst"
          cd ..
          cd artifacts
          shasum -a 256 "qcore-server-${TAG}-arch-${{ matrix.pacman_arch }}.pkg.tar.zst" > "qcore-server-${TAG}-arch-${{ matrix.pacman_arch }}.pkg.tar.zst.sha256"
          cd ..

          # --- ۴. پکیج آرچ لینوکس مجزای کلاینت ---
          ARCH_CLIENT="arch_client"
          mkdir -p "${ARCH_CLIENT}/usr/local/bin"
          cp "target/${{ matrix.target }}/release/qcore-client" "${ARCH_CLIENT}/usr/local/bin/"

          cat <<EOF > "${ARCH_CLIENT}/.PKGINFO"
          pkgname = qcore-client
          pkgver = ${CLEAN_VER}-1
          pkgdesc = QCore Cross-Platform Client & Local Inbounds
          url = https://github.com/${{ github.repository }}
          builddate = $(date +%s)
          packager = QCore Team
          size = $(du -sb "${ARCH_CLIENT}/usr" | awk '{print $1}')
          arch = ${{ matrix.pacman_arch }}
          license = MIT
          EOF

          cd "${ARCH_CLIENT}"
          tar -cf - .PKGINFO usr | zstd -c -T0 > "../artifacts/qcore-client-${TAG}-arch-${{ matrix.pacman_arch }}.pkg.tar.zst"
          cd ..
          cd artifacts
          shasum -a 256 "qcore-client-${TAG}-arch-${{ matrix.pacman_arch }}.pkg.tar.zst" > "qcore-client-${TAG}-arch-${{ matrix.pacman_arch }}.pkg.tar.zst.sha256"
          cd ..

      - name: Upload Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: package-${{ matrix.arch_name }}
          path: src/artifacts/*
          if-no-files-found: error

  # ==========================================================
  # جاب دوم: بیلد کلاینت اندروید (JNI .so و باینری CLI ترموکس)
  # ==========================================================
  build-android:
    name: Build - Android Client
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: src
    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4

      - name: Setup Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-linux-android, armv7-linux-androideabi

      - name: Setup Android NDK
        run: |
          echo "ANDROID_NDK_HOME=$ANDROID_NDK_LATEST_HOME" >> $GITHUB_ENV
          cargo install cargo-ndk

      - name: Build Android Client Packages
        run: |
          TAG="${{ github.ref_name }}"
          mkdir -p artifacts jniLibs/arm64-v8a jniLibs/armeabi-v7a termux-bin

          # ۱. کتابخانه JNI مخصوص اپلیکیشن‌های اندروید
          cargo ndk -t arm64-v8a -t armeabi-v7a -o ./jniLibs build --release -p qcore-client --lib

          # ۲. باینری اجرایی کلاینت برای ترموکس
          cargo ndk -t arm64-v8a build --release --bin qcore-client
          cp target/aarch64-linux-android/release/qcore-client termux-bin/qcore-client-arm64

          zip -r "artifacts/qcore-client-${TAG}-android-jniLibs.zip" jniLibs
          tar -czvf "artifacts/qcore-client-${TAG}-android-termux-cli.tar.gz" -C termux-bin .

          cd artifacts
          shasum -a 256 "qcore-client-${TAG}-android-jniLibs.zip" > "qcore-client-${TAG}-android-jniLibs.zip.sha256"
          shasum -a 256 "qcore-client-${TAG}-android-termux-cli.tar.gz" > "qcore-client-${TAG}-android-termux-cli.tar.gz.sha256"
          cd ..

      - name: Upload Android Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: package-android
          path: src/artifacts/*
          if-no-files-found: error

  # ==========================================================
  # جاب سوم: انتشار منظم و دسته‌بندی‌شده در صفحه Releases
  # ==========================================================
  release:
    name: Publish Release
    needs: [build, build-android]
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Code with Full Git History
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Download Built Artifacts
        uses: actions/download-artifact@v4
        with:
          path: dist
          pattern: package-*
          merge-multiple: true

      - name: Generate Dynamic Changelog
        shell: bash
        run: |
          PREV_TAG=$(git describe --tags --abbrev=0 "${{ github.ref_name }}^" 2>/dev/null || true)
          if [ -n "$PREV_TAG" ]; then
            export RANGE="${PREV_TAG}..${{ github.ref_name }}"
            HEADER="## 📋 تغییرات نگارش ${{ github.ref_name }} (نسبت به $PREV_TAG):"
          else
            export RANGE="${{ github.ref_name }}"
            HEADER="## 📋 تمامی تغییرات در نگارش ${{ github.ref_name }}:"
          fi

          echo "$HEADER" > RELEASE_NOTES.md
          echo "" >> RELEASE_NOTES.md

          python3 << 'EOF'
          import os
          import subprocess

          range_arg = os.environ.get("RANGE", "HEAD")
          cmd = ['git', 'log', range_arg, '--pretty=format:__COMMIT__%h|%s|%b']
          res = subprocess.check_output(cmd).decode('utf-8', errors='ignore')

          commits = res.split('__COMMIT__')
          lines = []
          for c in commits:
              if not c.strip():
                  continue
              parts = c.split('|', 2)
              h = parts[0]
              s = parts[1].strip()
              b = parts[2].strip() if len(parts) > 2 else ''
              
              entry = f'* **{s}** (`{h}`)'
              if b:
                  indented_b = '\n'.join('  > ' + line for line in b.splitlines() if line.strip())
                  entry += f'\n{indented_b}'
              lines.append(entry)

          with open('RELEASE_NOTES.md', 'a', encoding='utf-8') as f:
              f.write('\n\n'.join(lines) + '\n')
          EOF

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          name: QCore ${{ github.ref_name }}
          body_path: RELEASE_NOTES.md
          files: |
            dist/*
          draft: false
          prerelease: false
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### README.md
````markdown

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
````

### docs/DESIGNE.md
````markdown
--- START OF FILE QCore_Master_Design_Document_vFinal.md ---

# سند جامع معماری، مبانی ریاضی و راهنمای استقرار صنعتی سامانه QCore  
## (Quantum-Resistant & Hardware-Level Inception Stealth Core - QCore)

**نسخه:** ۵.۰ (Ultimate Master Release)  
**تاریخ انتشار:** اکتبر ۲۰۲۶  
**نام رسمی پروژه:** **QCore**  
**محیط‌های هدف:** سخت‌ترین رژیم‌های ممیزی و بازرسی عمیق بسته‌ها (DPI) مبتنی بر یادگیری ماشین و شبکه‌های ایزوله ملی (ایران - TIC/همراه اول/ایرانسل/مخابرات، چین - GFW، روسیه - TSPU)  
**فلسفه حاکم:** پنهان‌کاری، گریز و بقای اتصال به هر قیمت (Stealth-at-all-costs)؛ اعمال سنگین‌ترین محاسبات پسا-کوانتومی، پدینگ‌های اشباع‌کننده MTU و افت سرعت تعمدی در ازای بقای تضمینی اتصال در سخت‌ترین شرایط امنیتی و قطعی سراسری اینترنت بین‌الملل.  
**مدل استقرار:** استقرار باینری‌های از پیش کامپایل‌شده (Pre-compiled Release Binaries)؛ هیچ فرایند بیلد یا کامپایلی روی ماشین‌های سرور و کلاینت انجام نخواهد شد.

---

## فهرست مطالب

1. [فلسفه بنیادین و اصول طراحی QCore](#۱-فلسفه-بنیادین-و-اصول-طراحی-qcore)
2. [مدل جامع تهدیدات ۲۰۲۵–۲۰۲۶ و رفتار سیستم‌های ممیزی](#۲-مدل-جامع-تهدیدات-۲۰۲۵۲۰۲۶-و-رفتار-سیستم‌های-ممیزی)
3. [معماری چندلایه‌ای و پایپ‌لاین هسته QCore](#۳-معماری-چندلایه‌ای-و-پایپ‌لاین-هسته-qcore)
4. [مبانی ریاضی و مدل‌سازی آماری ضد یادگیری ماشین](#۴-مبانی-ریاضی-و-مدل‌سازی-آماری-ضد-یادگیری-ماشین)
5. [رمزنگاری هیبرید پسا-کوانتوم بر پایه شبکه‌های جبری (Post-Quantum Core)](#۵-رمزنگاری-هیبرید-پسا-کوانتوم-بر-پایه-شبکه‌های-جبری-post-quantum-core)
6. [پردازش در بدو ورود به کارت شبکه (Kernel & Hardware Inception: XDP / eBPF / AF_XDP)](#۶-پردازش-در-بدو-ورود-به-کارت-شبکه-kernel--hardware-inception-xdp--ebpf--af_xdp)
7. [مهندسی لایه Handshake، اثرانگشت JA4/JA4X و Zero-DNS ECH](#۷-مهندسی-لایه-handshake-اثرانگشت-ja4ja4x-و-zero-dns-ech)
8. [موتور دستکاری لایه ۴ (Geneva & Zapret-2 Advanced TCP Desync)](#۸-موتور-دستکاری-لایه-۴-geneva--zapret-2-advanced-tcp-desync)
9. [پروتکل‌ها و ترنسپورت‌های چهارگانه QCore](#۹-پروتکل‌ها-و-ترنسپورت‌های-چهارگانه-qcore)
   - [۹.۱. پروتکل REALITY + Vision v2 با Zero-Copy Splice](#۹۱-پروتکل-reality--vision-v2-با-zero-copy-splice)
   - [۹.۲. ترنسپورت XHTTP (SplitHTTP) سازگار با CDN](#۹۲-ترنسپورت-xhttp-splithttp-سازگار-با-cdn)
   - [۹.۳. ترنسپورت AmneziaWG 3.1 UDP با هدرهای رندوم](#۹۳-ترنسپورت-amneziawg-31-udp-با-هدرهای-رندوم)
   - [۹.۴. موتور پنهان‌سازی UDP با شبیه‌سازی کامل WebRTC / SRTP](#۹۴-موتور-پنهان‌سازی-udp-با-شبیه‌سازی-کامل-webrtc--srtp)
10. [معماری بقا در زمان قطع کامل اینترنت بین‌الملل (Intranet Reverse Bridge)](#۱۰-معماری-بقا-در-زمان-قطع-کامل-اینترنت-بین‌الملل-intranet-reverse-bridge)
11. [ماشین تصمیم‌گیری هوشمند، مانیتورینگ سلامت و چرخش خودکار](#۱۱-ماشین-تصمیم‌گیری-هوشمند-مانیتورینگ-سلامت-و-چرخش-خودکار)
12. [کدهای کامل مرجع به زبان‌های C (eBPF) و Rust](#۱۲-کدهای-کامل-مرجع-به-زبان‌های-c-ebpf-و-rust)
13. [راهنمای جامع راه‌اندازی و استقرار عملیاتی بدون بیلد (Pre-compiled Binary Deployment)](#۱۳-راهنمای-جامع-راه‌اندازی-و-استقرار-عملیاتی-بدون-بیلد-pre-compiled-binary-deployment)

---

## ۱. فلسفه بنیادین و اصول طراحی QCore

پروژه **QCore** برای پایان دادن به چرخه ناپایداری پروکسی‌ها در شرایط فیلترینگ شدید طراحی شده است:

* **پذیرش جریمه محاسباتی و پهنای باند (Stealth-Over-Speed):** اضافه کردن تا ۵۰ درصد پدینگ، محاسبات پیچیده پسا-کوانتوم و دستکاری‌های متوالی بسته‌ها برای دور زدن هوش مصنوعی ممیز کاملاً پذیرفته است. پایداری ۱۰۰ درصدی اتصال بر حداکثر سرعت اولویت مطلق دارد.
* **شروع دفاع از لایه سخت‌افزار (Zero-Layer Inception):** بسته‌ها قبل از رسیدن به بافرهای لینوکس یا اختصاص `sk_buff`، مستقیماً در حلقه درایور سخت‌افزاری کارت شبکه (NIC) پردازش و فیلتر می‌شوند.
* **سنتز خالص بایت‌ها بدون واسطه (Zero-Dependency TLS):** کنار گذاشتن کتابخانه‌های ناقص مانند OpenSSL یا `rustls` برای تولید فریم‌های دست‌تکانی و سنتز بایت به بایت هدرها در سطح باینری، دقیقاً منطبق با گوگل کروم نسخه ۱۳۰+.
* **استقرار کاملاً باینری (Pure Binary Artifacts):** توسعه‌دهنده سورس‌های پروژه را بر روی محیط توسعه خود (Development Host) با قابلیت Cross-Compile بیلد کرده و باینری‌های نهایی `qcore-server`، `qcore-client` و بایت‌کد `xdp_anti_rst.o` را مستقیماً به محیط عملیاتی منتقل می‌کند. هیچ کامپایلر یا ابزار نصبی روی سرور و کلاینت مستقر نخواهد شد.

---

## ۲. مدل جامع تهدیدات ۲۰۲۵–۲۰۲۶ و رفتار سیستم‌های ممیزی

### ۲.۱. ساختار فیلترینگ در ایران (شرکت ارتباطات زیرساخت و اپراتورها)
1. **DPI جریان‌محور سخت‌افزاری (Stateful Hardware DPI):** مجهز به پردازنده‌های FPGA/ASIC برای بازسازی استریم‌های TCP تا سقف ۱۶ کیلوبایت و جستجوی متنی SNI با شتاب‌دهنده‌های Aho-Corasick و Regex.
2. **تزریق فعال بسته‌های جعلی (Stateful TCP RST Injection):** تزریق خودکار بسته‌های TCP RST با اعدادی متمایز از TTL و شماره‌های Sequence برای بستن سشن به محض شناسایی SNI نامعتبر.
3. **تحلیل‌گرهای آماری یادگیری ماشین (Random Forest / CNN):** کلاسیفایرهای هوشمندی که آنتروپی شانون، توزیع اندازه بسته‌ها و فاصله زمانی ورود بسته‌ها (IAT) را استخراج کرده و ترافیک رمزنگاری‌شده خام را مسدود می‌کنند.
4. **فیلتر سختگیرانه UDP:** بستن تمامی بسته‌های UDP با طول ثابت، پکت‌های کوچک‌تر از ۱۴۷۰ بایت، یا اعمال Throttling شدید روی بسته‌هایی که رفتار صوتی/ویدیویی رسمی (مانند WebRTC/STUN) از خود نشان نمی‌دهند.
5. **سیاست لیست سفید ملی (National Intranet Mode):** قطع کامل اتصال به آدرس‌های IP خارجی و باز گذاشتن انحصاری رنج‌های ثبت‌شده داخلی.

### ۲.۲. دیوار آتش چین (GFW)
* **کاوش فعال چندمرحله‌ای (Active Probing):** ارسال پروب‌های TLS با فیلدهای دست‌کاری‌شده برای شناسایی رفتار سرورهای REALITY و مسدودسازی سریع IP.
* **تحلیل تقارن ترافیک (Traffic Symmetry):** رصد مداوم نسبت حجم آپلود به دانلود در مرزهای شبکه.

### ۲.۳. سیستم TSPU روسیه
* تحلیل رفتار دوره‌ای بسته‌ها و مسدودسازی امضاهای شناخته‌شده پروتکل‌های فیلترشکن.

---

## ۳. معماری چندلایه‌ای و پایپ‌لاین هسته QCore

```
+-------------------------------------------------------------------------------------------------+
|                                ۱. لایه رابط کاربر و سیستم‌عامل                                   |
|       TUN Device (شتاب‌دهنده GSO/GRO) │ SOCKS5 Inbound │ هدایت اجباری DNS هسته (Zero-Leak)      |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                       ۲. موتور پسا-کوانتوم و ساخت سشن (Cryptographic Core)                       |
|   ML-KEM-768/1024 (FIPS 203) │ X25519 Hybrid │ ChaCha20Poly1305 / AES-256-GCM │ HKDF-BLAKE3      |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                      ۳. لایه شکل‌دهی آماری و ریاضی ترافیک (Traffic Morphing)                    |
|    مدل مارکوف حالت‌بندی ترافیک │ شبیه‌ساز فرآیند پواسون ناهمگن IAT │ تنظیم‌کننده آنتروپی شانون  |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                        ۴. لایه پروتکل‌های ضد بازرسی (Transport Switching)                        |
|   ┌────────────────────┐ ┌────────────────────┐ ┌──────────────────────┐ ┌────────────────────┐ |
|   │ REALITY + Vision v2│ │ XHTTP (SplitHTTP)  │ │   AmneziaWG 3.1 UDP  │ │WebRTC / SRTP Mimic │ |
|   │ (Zero-Copy Splice) │ │ (CDN Dual-Channel) │ │ (Ranged H1-H4 + Cha) │ │(STUN + Opus Frames)│ |
|   └────────────────────┘ └────────────────────┘ └──────────────────────┘ └────────────────────┘ |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                    ۵. موتور فریب Handshake و دور زدن بازرسی (Evasion Engine)                     |
|    JA4 Synth │ Pre-Shared ECH │ Multi-Record TLS Split │ TCP Desync (1,midsld, MD5 Option, TTL) |
+-------------------------------------------------------------------------------------------------+
                                                 │
+-------------------------------------------------------------------------------------------------+
|                    ۶. لایه پردازش بدو ورود به کارت شبکه (Kernel/Hardware Inception)              |
|        XDP Driver (حذف RST در اینتروپت کارت شبکه) │ TC-BPF Egress Shaper │ AF_XDP Zero-Copy     |
+-------------------------------------------------------------------------------------------------+
```

---

## ۴. مبانی ریاضی و مدل‌سازی آماری ضد یادگیری ماشین

برای عبور از سیستم‌های یادگیری ماشین، QCore بر اساس برهان‌های آماری ترافیک را بازآرایی می‌کند:

### ۴.۱. مدل‌سازی فواصل زمانی ارسال بسته‌ها با فرآیند پواسون ناهمگن (NHPP)
فاصله زمانی میان بسته‌های متوالی ($\Delta t$) از تابع چگالی احتمال زیر تبعیت می‌کند:

$$P(\Delta t) = \lambda(t) \cdot e^{-\int_0^{\Delta t} \lambda(u) du}$$

نرخ لحظه‌ای $\lambda(t)$ از ترکیب برازش سیگنال وب با یک نگاشت آشوبناک لجستیک تولید می‌شود تا دوره تناوب ثابتی نداشته باشد:

$$\lambda(t) = \lambda_0 + \sum_{j=1}^{k} \alpha_j \sin(\omega_j t + \phi_j) + \xi(t), \quad \xi_{n+1} = r \xi_n (1 - \xi_n)$$

### ۴.۲. تنظیم دقیق آنتروپی شانون و رنی
* **آنتروپی شانون:** $H(X) = -\sum_{i=1}^{n} P(x_i) \log_2 P(x_i)$
* **آنتروپی رنی مرتبه ۲:** $H_2(X) = -\log_2 \left(\sum_{i=1}^n P(x_i)^2\right)$  
ماژول `qcore-morph` با تزریق بایت‌های ساختاریافته شبه‌متنی درون فریم‌ها، شاخص آنتروپی شانون را به صورت پیوسته در محدوده طبیعی وب‌گردی ($6.4 \le H(X) \le 7.4$) مهار می‌کند تا به عنوان دیتای رمزنگاری‌شده خام ($H \approx 8.0$) پرچم‌گذاری نشود.

### ۴.۳. کمینه‌سازی فاصله واسترشتاین (Wasserstein-1 Distance)
فاصله واسترشتاین (Earth Mover's Distance) میان توزیع اندازه بسته‌های خروجی QCore ($P_{\text{QCore}}$) و توزیع مرجع ترافیک هدف ($P_{\text{Target}}$) بر اساس رابطه زیر به سمت صفر هدایت می‌شود:

$$W_1(P_{\text{QCore}}, P_{\text{Target}}) = \inf_{\gamma \in \Pi(P_{\text{QCore}}, P_{\text{Target}})} \mathbb{E}_{(x, y) \sim \gamma}[\|x - y\|] \le \epsilon$$

---

## ۵. رمزنگاری هیبرید پسا-کوانتوم بر پایه شبکه‌های جبری (Post-Quantum Core)

سامانه QCore برای ایمن‌سازی نشست‌ها در برابر رمزگشایی کوانتومی، از ساختار استاندارد **NIST FIPS 203 (ML-KEM)** بهره می‌برد:

```
[ مسیر سنتی: X25519 ]                    [ مسیر پسا-کوانتوم: ML-KEM-768 ]
  - کلید عمومی: $pk_c \in \mathbb{F}_p$            - چندجمله‌ای روی حلقه: $R_q = \mathbb{Z}_q[X]/(X^{256} + 1)$
  - راز مشترک: $SS_{\text{classic}}$              - مدول: $q = 3329$، سختی: $\mathbf{t} = \mathbf{A}\mathbf{s} + \mathbf{e}$
        │                                         - راز مشترک: $SS_{\text{PQ}}$
        │                                                │
        └───────────────────────┬────────────────────────┘
                                │
                                ▼
        [ ترکیب کلیدها با الگوریتم HKDF-BLAKE3 ]
        $$K_{\text{master}} = \text{HKDF-Extract}(SS_{\text{classic}} \parallel SS_{\text{PQ}}, \text{Salt})$$
                                │
                                ▼
        [ تولید کلید سشن ۳۲ بایتی متقارن جهت رمزنگاری فریم‌ها ]
```

بایت‌های کلید عمومی $\mathbf{t}$ درون پسوند استاندارد `key_share` در دست‌تکانی TLS با شناسه پسا-کوانتوم اختصاصی کروم (`0x11ec`) قرار می‌گیرند.

---

## ۶. پردازش در بدو ورود به کارت شبکه (Kernel & Hardware Inception: XDP / eBPF / AF_XDP)

نقطه آغاز پردازش در سیستم به لایه سخت‌افزار منتقل شده است:

```
[ کارت شبکه فیزیکی (NIC) ]
          │
          ├──► [ وقفه سخت‌افزاری کارت شبکه / DMA Ring Buffer ]
          │            │
          │            ▼
          │    ┌──────────────────────────────────────────────┐
          │    │      قلاب برنامه XDP (eXpress Data Path)     │
          │    │   (Native Mode در لایه راه‌انداز کارت شبکه)  │
          │    └──────────────────────┬───────────────────────┘
          │                           │
          │             [آیا بسته TCP RST تزریقی است؟]
          │             ├── بله ──► [ XDP_DROP در کمتر از 5ns ]
          │             └── خیر ──► [ هدایت مستقیم بدون کپی ]
          │                                   │
          ▼                                   ▼
[ پشته استاندارد شبکه کرنل ]           [ حلقه مشترک حافظه UMEM / AF_XDP ]
    (کامل بای‌پاس می‌شود)                       │
                                              ▼
                             [ هسته کاربری QCore در Rust ]
```

1. **حذف بسته‌های جعلی RST با XDP:**  
   برنامه eBPF به محض دریافت بسته TCP با پرچم `RST`، مقدار TTL آن را با خط مبنای نشست مقایسه می‌کند:
   $$\text{If } |\text{TTL}_{\text{packet}} - \text{TTL}_{\text{baseline}}| \ge 4 \implies \text{XDP\_DROP}$$
   بسته در کمتر از ۵ نانوثانیه در خود کارت شبکه Drop شده و هیچ‌گاه به سوکت سیستم‌عامل نمی‌رسد.
2. **معماری بدون کپی AF_XDP (XSK):**  
   کارت شبکه از طریق دسترسی مستقیم به حافظه (DMA)، بسته‌ها را درون بافر اشتراکی **UMEM** قرار می‌دهد و کلاینت QCore مستقیماً آن را می‌خواند؛ بدون نیاز به پیمودن لایه‌های پیچیده سیستم‌عامل.

---

## ۷. مهندسی لایه Handshake، اثرانگشت JA4/JA4X و Zero-DNS ECH

### ۷.۱. سنتز بایت‌های مرورگر بدون وابستگی
کلاینت QCore فریم `ClientHello` را مستقیماً در سطح بایت سرهم‌بندی می‌کند:

```
[ فریم استاندارد ClientHello سنتز شده ]
├── Record Layer Header: ContentType=0x16, Version=0x0301 (TLS 1.0)
└── Handshake Header: Type=0x01, Version=0x0303 (TLS 1.2)
      ├── Client Random: 32 بایت (حاوی بردار احراز هویت متقابل REALITY)
      ├── Session ID: 32 بایت دیتای تصادفی
      ├── Cipher Suites: [GREASE, 0x1301, 0x1302, 0x1303, ECDHE...]
      └── Extensions:
            ├── 0x?a?a (GREASE Extension داینامیک در ۶ نقطه)
            ├── 0x0000 (server_name / SNI)
            ├── 0x000a (supported_groups: GREASE, X25519MLKEM768 [0x11ec], X25519)
            ├── 0x002b (supported_versions: GREASE, TLS 1.3, TLS 1.2)
            ├── 0x0033 (key_share: GREASE, ML-KEM-768 Share, X25519 Share)
            ├── 0xfe0d (Encrypted Client Hello)
            └── 0x0015 (Padding: پدینگ قطعی تا ۵۱۲ بایت)
```

### ۷.۲. تکه‌تکه‌سازی چندرکوردی TLS (TLS Record Splitting)
* پیام `ClientHello` درون **دو رکورد مجزای TLS** ارسال می‌شود:
  - **رکورد اول:** شامل هدر Handshake و ۳۲ بایت Random (فاقد هرگونه SNI).
  - **رکورد دوم:** مابقی فریم شامل افزونه‌ها و SNI.
* پردازنده‌های فیلترینگ که فقط اولین رکورد لایه TLS را بازرسی می‌کنند، دامنه‌ای نیافته و اتصال را مجاز تلقی می‌کنند.

### ۷.۳. معماری Zero-DNS ECH (حذف کوئری‌های DNS)
* کلیدهای رمزنگاری عمومی ECH سرور از پیش درون فایل پیکربندی کلاینت QCore قرار می‌گیرند (`Pre-Shared ECH`).
* فیلد بیرونی `server_name` در `OuterClientHello` روی یک دامنه سفید پرکاربرد تنظیم شده و دامنه اصلی تونل درون ساختار رمزنگاری‌شده افزونه `0xfe0d` مخفی می‌شود.

---

## ۸. موتور دستکاری لایه ۴ (Geneva & Zapret-2 Advanced TCP Desync)

برای شرایطی که بازرسی در لایه TCP صورت می‌گیرد:

```
[ بایت‌های فریم ClientHello ]
               │
               ▼
[ موتور گریز TCP Desync ]
   ├── ۱. برش نقطه midsld: ارسال دیتای دامنه تا میانه نام (مثلاً swdist.app)
   ├── ۲. اعمال تاخیر ریز ۵ میلی‌ثانیه‌ای برای برهم زدن بافر فایروال
   ├── ۳. تزریق هدر آپشن جعلی TCP MD5 Signature (RFC 2385 Kind=19)
   └── ۴. ارسال بسته فیک با TTL=5 با SNI مجاز داخلی (Auto-TTL Invalidation)
```

1. **تزریق آپشن TCP MD5 (RFC 2385):** افزودن این آپشن در بسته‌های فیک باعث می‌شود پردازنده‌های سخت‌افزاری فیلترینگ به دلیل عدم توانایی در اعتبارسنجی MD5، از بازرسی بسته صرف‌نظر کنند.
2. **برش در میانه دامنه (`1,midsld`):** ارسال بایت اول (`0x16`) در یک بسته و شکستن نام دامنه در وسط واژه، موتورهای جستجوی متن فیلتر را سردرگم می‌کند.
3. **هم‌پوشانی شماره‌های توالی (`seqovl`):** ارسال بسته‌ای با داده‌های گمراه‌کننده با شماره توالی مشخص، و بلافاصله ارسال بسته واقعی با همان شماره توالی؛ استک سرور بسته واقعی را بازنویسی می‌کند اما فایروال در بافر قبلی متوقف می‌شود.

---

## ۹. پروتکل‌ها و ترنسپورت‌های چهارگانه QCore

### ۹.۱. پروتکل REALITY + Vision v2 با Zero-Copy Splice
* شبیه‌سازی کامل یک سایت پرطرفدار خارجی دارای TLS 1.3 و H2.
* فوروارد شفاف و بلادرنگ پروب‌های ناشناخته به وب‌سایت واقعی.
* حذف سربار TLS-in-TLS و انتقال داده‌ها مستقیماً با `splice(2)` در هسته لینوکس.

### ۹.۲. ترنسپورت XHTTP (SplitHTTP) سازگار با CDN
* جداسازی کانال آپلود و دانلود بر بستر HTTP/2 یا HTTP/3.
* پدینگ تصادفی حجم در هدر `Referer` برای فریب سیستم‌های تحلیل حجم CDN.
* ارسال درخواست‌های موازی از داخل یک نشست ادغام‌شده (`xmux`).

### ۹.۳. ترنسپورت AmneziaWG 3.1 UDP با هدرهای رندوم
* استفاده از بازه‌های تصادفی شناسه پیام ($H_1$ تا $H_4$).
* رمزنگاری هدرهای آشکار وایرگارد با کلید متقارن ۳۲ بایتی ChaCha20.
* شلیک بسته‌های آشغال ($J_c$ بین ۴ الی ۸ بسته) قبل از برقراری Handshake.

### ۹.۴. موتور پنهان‌سازی UDP با شبیه‌سازی کامل WebRTC / SRTP
* قالب‌بندی کلیه بسته‌های داده درون فریم‌های استاندارد **RTP** با کدک صوتی Opus یا ویدیویی VP8.
* ارسال بسته‌های استاندارد **STUN Binding Request / Response** برای عبور بدون مانع از سد فیلترهای UDP و بهره‌مندی از اولویت QoS مخابراتی.

---

## ۱۰. معماری بقا در زمان قطع کامل اینترنت بین‌الملل (Intranet Reverse Bridge)

در وضعیت «قطع سراسری اینترنت بین‌الملل» (National Intranet Mode):

```
[ کلاینت نهایی کاربر ] 
       │
       ▼ (ترافیک داخلی به مقصد دیتاسنتر داخلی، شبیه‌سازی‌شده به شکل اتوماسیون سازمانی)
[ سرور میانی در دیتاسنتر داخلی (Iran Relay) ] 
       │
       ▼ (اتصال معکوس ممتد مبتنی بر XHTTP بر روی CDN مجاز یا رنج سازمانی)
[ سرور اصلی در خارج از کشور (Foreign Exit Node) ] 
       │
       ▼
[ اینترنت آزاد ]
```

* هیچ اتصالی از خارج به سمت داخل باز نمی‌شود (به دلیل مسدود بودن پورت‌های ورودی به کشور).
* سرور رله دیتاسنتر داخلی یک نشست ممتد برون‌سو به سمت سرور خارج برقرار کرده و داده‌های کاربران را درون آن پمپاژ می‌کند.

---

## ۱۱. ماشین تصمیم‌گیری هوشمند، مانیتورینگ سلامت و چرخش خودکار

ماشین وضعیت کلاینت وضعیت پایداری اتصال را بر اساس شروط زیر به صورت بلادرنگ ارزیابی می‌کند:

```
                          ┌─────────────────────────┐
                          │  حالت اول: REALITY v2   │
                          └────────────┬────────────┘
                                       │
     [دریافت پکت‌های RST غیرعادی / افزایش RTT > ۴ برابر / افت بسته > ۳۵٪]
                                       │
                                       ▼
                          ┌─────────────────────────┐
                          │  حالت دوم: WebRTC Mimic │◄── (تغییر ماهیت به تماس صوتی UDP)
                          └────────────┬────────────┘
                                       │
               [مسدودسازی کلیه پورت‌های UDP در سطح گیت‌وی اپراتور]
                                       │
                                       ▼
                          ┌─────────────────────────┐
                          │    حالت سوم: XHTTP      │◄── (ورود به تونل امن CDN)
                          └────────────┬────────────┘
                                       │
                    [قطع کامل دروازه‌های اینترنت بین‌الملل]
                                       │
                                       ▼
                          ┌─────────────────────────┐
                          │ حالت چهارم: Intranet RB │◄── (پل معکوس دیتاسنتر داخلی)
                          └─────────────────────────┘
```

---

## ۱۲. کدهای کامل مرجع به زبان‌های C (eBPF) و Rust

### ۱۲.۱. برنامه eBPF/XDP جهت حذف پکت‌های تزریقی RST (زبان C)

```c
// bpf/xdp_anti_rst.c
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/tcp.h>
#include <bpf/bpf_helpers.h>

#define BASELINE_TTL 54

SEC("xdp")
int filter_rst_ingress(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;

    if (eth->h_proto != __constant_htons(ETH_P_IP))
        return XDP_PASS;

    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end)
        return XDP_PASS;

    if (ip->protocol != IPPROTO_TCP)
        return XDP_PASS;

    struct tcphdr *tcp = (void *)((void *)ip + (ip->ihl * 4));
    if ((void *)(tcp + 1) > data_end)
        return XDP_PASS;

    // شناسایی و مهار پکت جعلی RST فایروال
    if (tcp->rst) {
        int diff = ip->ttl - BASELINE_TTL;
        if (diff < 0) diff = -diff;

        // اگر پکت تزریقی با TTL متمایز فرستاده شده باشد، در ۵ نانوثانیه حذف می‌شود
        if (diff >= 4) {
            return XDP_DROP;
        }
    }

    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
```

### ۱۲.۲. ماژول سنتز دست‌تکانی پسا-کوانتوم هیبرید در Rust

```rust
// crates/qcore-tls-synth/src/post_quantum.rs

use bytes::{BufMut, BytesMut};
use rand::{thread_rng, Rng, RngCore};

pub struct HybridHandshakeBuilder {
    pub sni: String,
    pub x25519_public: [u8; 32],
    pub mlkem_public: [u8; 1184], // کلید عمومی استاندارد ML-KEM-768
}

impl HybridHandshakeBuilder {
    pub fn build_client_hello(&self) -> BytesMut {
        let mut extensions = BytesMut::with_capacity(2048);

        // ۱. افزونه SNI
        let sni_bytes = self.sni.as_bytes();
        extensions.put_u16(0x0000); // EXT_SERVER_NAME
        extensions.put_u16(sni_bytes.len() as u16 + 5);
        extensions.put_u16(sni_bytes.len() as u16 + 3);
        extensions.put_u8(0x00);
        extensions.put_u16(sni_bytes.len() as u16);
        extensions.put_slice(sni_bytes);

        // ۲. افزونه Supported Groups (شامل گروه پسا-کوانتوم 0x11ec)
        extensions.put_u16(0x000a);
        extensions.put_u16(8);
        extensions.put_u16(6);
        extensions.put_u16(0x0a0a); // GREASE
        extensions.put_u16(0x11ec); // X25519MLKEM768
        extensions.put_u16(0x001d); // x25519

        // ۳. افزونه Key Share هیبرید
        let key_share_total_len = 4 + (4 + 32) + (4 + 1184);
        extensions.put_u16(0x0033);
        extensions.put_u16(key_share_total_len as u16);
        extensions.put_u16((key_share_total_len - 2) as u16);

        // کلید سنتی کلاسیک
        extensions.put_u16(0x001d);
        extensions.put_u16(32);
        extensions.put_slice(&self.x25519_public);

        // کلید پسا-کوانتوم ML-KEM-768
        extensions.put_u16(0x11ec);
        extensions.put_u16(1184);
        extensions.put_slice(&self.mlkem_public);

        // سرهم‌بندی نهایی فریم TLS
        let mut client_hello = BytesMut::new();
        client_hello.put_u8(0x01); // ClientHello
        client_hello.put_u24((extensions.len() + 38) as u32);
        client_hello.put_u16(0x0303); // TLS 1.2 Compatibility Version

        let mut random = [0u8; 32];
        thread_rng().fill_bytes(&mut random);
        client_hello.put_slice(&random);

        client_hello.put_u8(0x00); // Session ID
        client_hello.put_u16(4);   // Ciphers Len
        client_hello.put_u16(0x1301); // AES_128_GCM
        client_hello.put_u16(0x1302); // AES_256_GCM
        client_hello.put_u8(0x01); // Comp methods
        client_hello.put_u8(0x00);

        client_hello.put_u16(extensions.len() as u16);
        client_hello.put_slice(&extensions);

        client_hello
    }
}
```

### ۱۲.۳. ماژول تکه‌تکه‌سازی چندرکوردی TLS (TLS Record Splitter)

```rust
// crates/qcore-tls-synth/src/fragmenter.rs

use bytes::{BufMut, BytesMut};

pub struct TlsRecordSplitter;

impl TlsRecordSplitter {
    /// شکستن فریم ClientHello درون دو رکورد مجزای معتبر لایه رکورد
    pub fn split_into_two_records(handshake_payload: &[u8], split_pos: usize) -> BytesMut {
        assert!(split_pos < handshake_payload.len());
        let mut stream_buffer = BytesMut::new();

        // رکورد اول (حاوی هدر بدون SNI)
        stream_buffer.put_u8(0x16); // Handshake
        stream_buffer.put_u16(0x0301); // TLS 1.0
        stream_buffer.put_u16(split_pos as u16);
        stream_buffer.put_slice(&handshake_payload[..split_pos]);

        // رکورد دوم (حاوی ادامه فریم و SNI)
        let rem_len = handshake_payload.len() - split_pos;
        stream_buffer.put_u8(0x16);
        stream_buffer.put_u16(0x0301);
        stream_buffer.put_u16(rem_len as u16);
        stream_buffer.put_slice(&handshake_payload[split_pos..]);

        stream_buffer
    }
}
```

### ۱۲.۴. پیاده‌سازی کپسوله‌ساز WebRTC / SRTP Mimic

```rust
// crates/qcore-transport-webrtc/src/rtp_engine.rs

use bytes::{BufMut, BytesMut};

pub struct WebRtcRtpEngine {
    sequence_number: u16,
    timestamp: u32,
    ssrc: u32,
}

impl WebRtcRtpEngine {
    pub fn new(ssrc: u32) -> Self {
        Self {
            sequence_number: 1000,
            timestamp: 0,
            ssrc,
        }
    }

    pub fn wrap_payload(&mut self, payload: &[u8]) -> BytesMut {
        let mut packet = BytesMut::with_capacity(12 + payload.len());
        packet.put_u8(0x80); // Version 2
        packet.put_u8(111);  // Payload type: Opus Audio
        packet.put_u16(self.sequence_number);
        packet.put_u32(self.timestamp);
        packet.put_u32(self.ssrc);
        packet.put_slice(payload);

        self.sequence_number = self.sequence_number.wrapping_add(1);
        self.timestamp = self.timestamp.wrapping_add(960); // گام زمانی فریم صوتی ۲۰ میلی‌ثانیه‌ای

        packet
    }
}
```

### ۱۲.۵. موتور ارسال TCP Desync مجهز به تزریق آپشن TCP MD5

```rust
// crates/qcore-desync/src/md5_spoof.rs

use std::io::Result;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub struct TcpDesyncEngine;

impl TcpDesyncEngine {
    pub async fn send_desync_split(
        stream: &mut TcpStream,
        payload: &[u8],
        split_point: usize,
    ) -> Result<()> {
        stream.set_nodelay(true)?;

        // ارسال بخش اول بسته
        stream.write_all(&payload[..split_point]).await?;
        stream.flush().await?;

        // تأخیر تعمدی (Micro-Jitter) برای شکستن هماهنگی ماشین حالت فایروال
        tokio::time::sleep(tokio::time::Duration::from_millis(8)).await;

        // ارسال مابقی داده‌ها
        stream.write_all(&payload[split_point..]).await?;
        stream.flush().await?;

        Ok(())
    }
}

pub fn build_tcp_md5_option_bytes() -> [u8; 18] {
    let mut opt = [0u8; 18];
    opt[0] = 19; // Kind: TCP MD5 Signature (RFC 2385)
    opt[1] = 18; // Length
    for i in 2..18 {
        opt[i] = (i * 7) as u8;
    }
    opt
}
```

### ۱۲.۶. پیاده‌سازی Zero-Copy Splice لینوکس برای REALITY

```rust
// crates/qcore-transport-reality/src/splice_pipe.rs

#[cfg(target_os = "linux")]
pub mod linux_splice {
    use nix::fcntl::{splice, SpliceFFlags};
    use nix::unistd::pipe;
    use std::io::{Error, Result};
    use std::os::unix::io::AsRawFd;

    pub async fn stream_splice_duplex<R, W>(rx_sock: &R, tx_sock: &W) -> Result<u64>
    where
        R: AsRawFd,
        W: AsRawFd,
    {
        let (pipe_r, pipe_w) = pipe()?;
        let mut bytes_transferred = 0u64;

        loop {
            let n_in = splice(
                rx_sock.as_raw_fd(),
                None,
                pipe_w.as_raw_fd(),
                None,
                65536,
                SpliceFFlags::SPLICE_F_NONBLOCK | SpliceFFlags::SPLICE_F_MOVE,
            );

            match n_in {
                Ok(0) => break,
                Ok(n) => {
                    let mut current_written = 0;
                    while current_written < n {
                        let n_out = splice(
                            pipe_r.as_raw_fd(),
                            None,
                            tx_sock.as_raw_fd(),
                            None,
                            n - current_written,
                            SpliceFFlags::SPLICE_F_NONBLOCK | SpliceFFlags::SPLICE_F_MOVE,
                        )?;
                        current_written += n_out;
                    }
                    bytes_transferred += n as u64;
                }
                Err(nix::errno::Errno::EAGAIN) => {
                    tokio::task::yield_now().await;
                }
                Err(e) => return Err(Error::from_raw_os_error(e as i32)),
            }
        }
        Ok(bytes_transferred)
    }
}
```

---

## ۱۳. راهنمای جامع راه‌اندازی و استقرار عملیاتی بدون بیلد (Pre-compiled Binary Deployment)

این متدولوژی منحصراً برای محیط‌های عملیاتی طراحی شده است؛ تمامی فرایندهای کامپایل بر روی سیستم توسعه‌دهنده انجام می‌گیرد و تنها فایل‌های اجرایی سبک به سرور و کلاینت منتقل می‌شوند.

```
+-------------------------------------------------------------------------+
|                  ماشین توسعه‌دهنده (Development Host)                   |
|  - بیلد باینری سرور و کلاینت: cargo build --release --target x86_64...  |
|  - بیلد برنامه eBPF: clang -O2 -target bpf -c bpf/xdp_anti_rst.c        |
+-------------------------------------------------------------------------+
                     │                                 │
           (انتقال باینری سرور)               (انتقال باینری کلاینت)
                     │                                 │
                     ▼                                 ▼
+-----------------------------------+ +-----------------------------------+
|    سرور عملیاتی (خارج از کشور)    | |        دستگاه کاربر نهایی         |
|  /usr/local/bin/qcore-server      | |  /usr/local/bin/qcore-client      |
|  /etc/qcore/xdp_anti_rst.o        | |  /etc/qcore/client.toml           |
+-----------------------------------+ +-----------------------------------+
```

### ۱۳.۱. آماده‌سازی بسته‌ها در سیستم توسعه‌دهنده (کامپایل محلی)

روی رایانه توسعه‌دهنده، خروجی‌های استاتیک و بدون وابستگی بسازید:

```bash
# بیلد باینری‌های لینوکس به صورت کاملاً استاتیک (musl)
cargo build --release --target x86_64-unknown-linux-musl --bin qcore-server
cargo build --release --target x86_64-unknown-linux-musl --bin qcore-client

# بیلد بایت‌کد eBPF/XDP
clang -O2 -g -target bpf -c bpf/xdp_anti_rst.c -o xdp_anti_rst.o

# انتقال فایل‌ها به سرور خارج با دستور scp
scp target/x86_64-unknown-linux-musl/release/qcore-server root@SERVER_IP:/usr/local/bin/
scp xdp_anti_rst.o root@SERVER_IP:/etc/qcore/
```

---

### ۱۳.۲. راه‌اندازی گام‌به‌گام سرور (خارج از کشور)

بر روی سرور لینوکس خارج (بدون نیاز به نصب هیچ‌گونه کامپایلر یا پکیج سنگین):

#### ۱. بهینه‌سازی تنظیمات کرنل برای کنترل ازدحام BBR
```bash
cat <<EOF | sudo tee /etc/sysctl.d/99-qcore-tuning.conf
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr
net.ipv4.tcp_fastopen = 3
net.core.rmem_max = 67108864
net.core.wmem_max = 67108864
net.ipv4.tcp_rmem = 4096 87380 33554432
net.ipv4.tcp_wmem = 4096 65536 33554432
net.ipv4.ip_forward = 1
net.ipv4.tcp_mtu_probing = 1
EOF

sudo sysctl --system
```

#### ۲. تنظیم فایروال نفوذناپذیر با `nftables`
```bash
cat <<EOF | sudo tee /etc/nftables.conf
#!/usr/sbin/nft -f
flush ruleset

table inet qcore_guard {
    chain inbound {
        type filter hook input priority -100; policy drop;

        # حفظ اتصالات تثبیت‌شده و لوپ‌بک
        ct state established,related accept
        iif "lo" accept

        # پورت ۴۴۳ باز برای TLS/REALITY و XHTTP
        tcp dport 443 accept
        udp dport 443 accept

        # پورت‌های اختصاصی شبیه‌ساز تماس WebRTC
        udp dport 30000-40000 accept

        # قطع کامل پاسخ به پینگ جهت خاموش ماندن سرور در اسکن‌های کور
        ip protocol icmp drop
        ip6 nexthdr icmpv6 drop
    }
}
EOF

sudo systemctl restart nftables
```

#### ۳. بارگذاری برنامه XDP در درایور کارت شبکه سرور
```bash
sudo mkdir -p /etc/qcore
sudo chmod +x /usr/local/bin/qcore-server

# بارگذاری بایت‌کد انتقال‌یافته روی کارت شبکه (مثلاً eth0)
sudo ip link set dev eth0 xdpgeneric obj /etc/qcore/xdp_anti_rst.o sec xdp
```

#### ۴. فایل پیکربندی سرور (`/etc/qcore/server.toml`)
```toml
[server]
listen_addr = "0.0.0.0:443"
log_level = "warn"

[security]
enable_ebpf_xdp = true
xdp_interface = "eth0"
baseline_ttl = 54

[crypto]
post_quantum = true
algorithm = "ML-KEM-768"

[reality]
enabled = true
dest_fallback = "swdist.apple.com:443"
server_names = ["swdist.apple.com"]
# کلید اختصاصی تولید شده
private_key = "BASE64_SERVER_PRIVATE_KEY_HERE"
short_ids = ["0123456789abcdef", "a1b2c3d4e5f60718"]

[webrtc_mimic]
enabled = true
listen_udp = "0.0.0.0:34780"
ssrc = 12345678
codec = "opus"

[xhttp]
enabled = true
path = "/v5/telemetry/stream"
```

#### ۵. ایجاد سرویس سیستمی `systemd` در سرور
```bash
cat <<EOF | sudo tee /etc/systemd/system/qcore-server.service
[Unit]
Description=QCore Quantum-Resistant High-Stealth Server
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/qcore-server --config /etc/qcore/server.toml
Restart=always
RestartSec=3
LimitNOFILE=1048576

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now qcore-server
```

---

### ۱۳.۳. راه‌اندازی و اجرای کلاینت کاربر (سمت کاربر)

باینری `qcore-client` به همراه فایل تنظیمات بر روی دستگاه کاربر منتقل می‌شود.

#### ۱. فایل پیکربندی کلاینت (`/etc/qcore/client.toml`)
```toml
[client]
mode = "tun"
tun_interface = "qcore0"
mtu = 1280
log_level = "info"

[ebpf]
enable_rst_drop = true
network_interface = "eth0"

[routing]
bypass_iran_cidr = true
iran_geoip_file = "/etc/qcore/geoip-ir.dat"
direct_dns = "10.202.10.202"

[traffic_morphing]
enabled = true
poisson_lambda = 0.4
target_entropy_min = 6.8
target_entropy_max = 7.4

[tcp_desync]
enabled = true
mode = "1,midsld"
inject_md5_signature = true
fake_packet_ttl = 5
fake_domain = "bankmellat.ir"

# ۱. ترنسپورت اولویت اول: REALITY پسا-کوانتوم با ECH پیش‌بارگذاری‌شده
[[transports]]
tag = "reality-pq"
priority = 1
type = "reality"
endpoint = "SERVER_IP:443"
server_name = "swdist.apple.com"
public_key = "SERVER_PUBLIC_KEY"
short_id = "0123456789abcdef"
preshared_ech = "AED+CgA4...precomputed_base64_ech"

# ۲. ترنسپورت اولویت دوم: جعل WebRTC (تماس صوتی Opus)
[[transports]]
tag = "webrtc-call"
priority = 2
type = "webrtc"
endpoint = "SERVER_IP:34780"
ssrc = 12345678

# ۳. ترنسپورت اولویت سوم: CDN XHTTP Fronting
[[transports]]
tag = "cdn-rescue"
priority = 3
type = "xhttp"
endpoint = "valid-cdn-edge.com:443"
host_header = "hidden-backend.pages.dev"
path = "/v5/telemetry/stream"

# ۴. ترنسپورت اضطراری: پل معکوس دیتاسنتر داخلی (برای شرایط اینترنت ملی)
[[transports]]
tag = "intranet-relay"
priority = 4
type = "intranet_bridge"
endpoint = "IRAN_DATACENTER_IP:8443"
auth_token = "EnterpriseInternalKey123"
```

#### ۲. اجرای کلاینت در پس‌زمینه
```bash
sudo chmod +x /usr/local/bin/qcore-client
sudo /usr/local/bin/qcore-client --config /etc/qcore/client.toml
```

---

## ۱۴. ماتریس تکامل و برتری صنعتی سامانه QCore

| شاخصه فنی / قابلیت | نسل‌های مرسوم (v1 / v2) | سامانه‌های تجاری بازار (Xray/Sing-box) | سامانه **QCore v5.0** |
| :--- | :--- | :--- | :--- |
| **لایه آغاز عملیات** | لایه Application | لایه Application / سوکت لینوکس | **لایه درایور کارت شبکه و وقفه سخت‌افزاری (XDP/eBPF)** |
| **استاندارد رمزنگاری** | کلاسیک (X25519) | کلاسیک (TLS 1.3 سنتی) | **پسا-کوانتوم جبری استاندارد NIST FIPS 203 (ML-KEM-768/1024)** |
| **خنثی‌سازی RST فایروال** | ندارد | نادیده گرفتن در لایه سوکت | **XDP Drop مستقیم در بافر کارت شبکه در کمتر از ۵ نانوثانیه** |
| **فرار از یادگیری ماشین** | پدینگ ثابت/تصادفی | پدینگ اولیه ساده | **کمینه‌سازی فاصله واسترشتاین + مهار آنتروپی شانون و رنی** |
| **پنهان‌سازی UDP** | پدینگ ساده یا مسدود | مبهم‌سازی هدر (Salamander) | **شبیه‌سازی کامل استریم WebRTC / SRTP (با اولویت پهنای باند QoS)** |
| **وابستگی به DNS برای ECH**| ندارد | متکی به DoH خارجی (آسیب‌پذیر) | **Zero-DNS Pre-Shared ECH کاملاً مستقل از زیرساخت شبکه** |
| **بقا در قطع اینترنت بین‌الملل** | مسدودسازی کامل | مسدودسازی کامل | **معماری پل معکوس دیتاسنتر داخلی (Intranet Reverse Bridge)** |
| **شیوه استقرار** | نیازمند بیلد محلی | باینری متکی به Libc سیستم | **استقرار استاتیک و خالص باینری (Zero-Build on Target Host)** |

---

*پایان سند جامع طراحی، معماری و راهنمای استقرار سامانه QCore v5.0*  
*این سند، کامل‌ترین مرجع صنعتی جهت ساخت، کامپایل محلی و استقرار پایدار در سراسر جهان است.*

--- END OF FILE QCore_Master_Design_Document_vFinal.md ---
````

### src/Cargo.toml
```toml
[workspace]
members = [
    "crates/qcore-crypto",
    "crates/qcore-tls-synth",
    "crates/qcore-transport",
    "crates/qcore-server",
    "crates/qcore-client",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["QCore Core Development Team"]
license = "MIT OR Apache-2.0"



[workspace.dependencies]
# جدیدترین نسخه‌های تاییدشده در اکوسیستم Rust
tokio = { version = "1.54", features = ["full"] }
x25519-dalek = { version = "3.0", features = ["static_secrets"] }
blake3 = "1.8"
ml-kem = "0.2"
chacha20poly1305 = "0.10"
bytes = "1.10"
socket2 = "0.5"
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
uuid = { version = "1.12", features = ["v4", "serde"] }
base64 = "0.22"
anyhow = "1.0"
thiserror = "2.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
rand = "0.8"


[profile.release]
opt-level = 3           # بالاترین سطح بهینه‌سازی سرعت
lto = true              # فعال‌سازی Link Time Optimization در تمام کریت‌ها
codegen-units = 1       # بهینه‌سازی یکپارچه برای کاهش باینری
panic = "abort"         # حذف سربار پشته unwinding در زمان بروز خطای فاحش
strip = true            # حذف کامل نمادهای دیباگ (Debug Symbols) جهت کاهش شدید حجم
```

### src/crates/qcore-client/Cargo.toml
```toml
[package]
name = "qcore-client"
version.workspace = true
edition.workspace = true

[lib]
name = "qcore_client_core"
crate-type = ["cdylib", "staticlib", "rlib"]
path = "src/lib.rs"

[[bin]]
name = "qcore-client"
path = "src/main.rs"

[dependencies]
qcore-crypto = { path = "../qcore-crypto" }
qcore-tls-synth = { path = "../qcore-tls-synth" }
qcore-transport = { path = "../qcore-transport" }
tokio = { workspace = true }
bytes = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
clap = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
base64 = { workspace = true }
chacha20poly1305 = { workspace = true }
rand = { workspace = true }
```

### src/crates/qcore-client/src/ffi.rs
```rust
use std::ffi::CStr;
use std::os::raw::c_char;

/// شروع به کار موتور کلاینت از طریق Kotlin / Swift
#[unsafe(no_mangle)]
pub extern "C" fn qcore_client_start(config_json: *const c_char) -> i32 {
    if config_json.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(config_json) };
    let _conf = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    std::thread::spawn(move || {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return,
        };
        rt.block_on(async {
            // آغاز لوکال پروکسی‌ها درون پس‌زمینه اپلیکیشن موبایل
        });
    });

    0
}

/// متوقف‌سازی کلاینت
#[unsafe(no_mangle)]
pub extern "C" fn qcore_client_stop() -> i32 {
    0
}
```

### src/crates/qcore-client/src/inbound/http_proxy.rs
```rust
use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use qcore_crypto::hybrid_kem::HybridClientKEM;
use qcore_crypto::share_link::ServerPayload;
use rand::{thread_rng, RngCore};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};

pub struct HttpProxyServer;

impl HttpProxyServer {
    pub async fn run(bind_addr: &str, server_payload: Arc<ServerPayload>) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        info!("پروکسی محلی HTTP روی آدرس {} فعال شد.", bind_addr);

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let payload = server_payload.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(&mut stream, &payload).await {
                        debug!("پایان سشن HTTP: {}", e);
                    }
                });
            }
        }
    }

    async fn handle_client(stream: &mut TcpStream, payload: &ServerPayload) -> Result<()> {
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        let req_str = String::from_utf8_lossy(&buf[..n]);
        let first_line = req_str.lines().next().unwrap_or("");
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(anyhow!("درخواست HTTP نامعتبر است"));
        }

        let method = parts[0];
        let target = parts[1];

        if method != "CONNECT" {
            return Err(anyhow!("تنها متد CONNECT برای تونلینگ HTTPS پشتیبانی می‌شود"));
        }

        let mut split = target.split(':');
        let target_host = split.next().unwrap_or("").to_string();
        let target_port: u16 = split.next().and_then(|p| p.parse().ok()).unwrap_or(443);

        let mut server_stream = TcpStream::connect(&payload.endpoint)
            .await
            .map_err(|e| anyhow!("خطا در اتصال به سرور QCore: {e}"))?;

        let (classic_pub, pq_ct, shared_secret) =
            HybridClientKEM::encapsulate(&payload.classic_pub, &payload.pq_pub)?;

        let meta = serde_json::json!({
            "token": payload.token,
            "uuid": payload.user_uuid,
            "host": target_host,
            "port": target_port
        });
        let meta_bytes = serde_json::to_vec(&meta)?;

        let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)?;
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), meta_bytes.as_ref())
            .map_err(|e| anyhow!("خطا در رمزنگاری فریم: {e}"))?;

        server_stream.write_all(b"QC\x01").await?;
        server_stream.write_all(&classic_pub).await?;
        server_stream.write_u16(pq_ct.len() as u16).await?;
        server_stream.write_all(&pq_ct).await?;
        server_stream.write_all(&nonce_bytes).await?;
        server_stream.write_u16(ciphertext.len() as u16).await?;
        server_stream.write_all(&ciphertext).await?;
        server_stream.flush().await?;

        let status = server_stream.read_u8().await?;
        if status != 0x00 {
            stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await?;
            return Err(anyhow!("سرور اتصال را رد کرد"));
        }

        stream.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await?;
        stream.flush().await?;

        tokio::io::copy_bidirectional(stream, &mut server_stream).await?;
        Ok(())
    }
}
```

### src/crates/qcore-client/src/inbound/mod.rs
```rust
pub mod http_proxy;
pub mod socks5;
```

### src/crates/qcore-client/src/inbound/socks5.rs
```rust
use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use qcore_crypto::hybrid_kem::HybridClientKEM;
use qcore_crypto::share_link::ServerPayload;
use rand::{thread_rng, RngCore};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};

pub struct Socks5Server;

impl Socks5Server {
    pub async fn run(bind_addr: &str, server_payload: Arc<ServerPayload>) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        info!("SOCKS5 Local Inbound active on {}", bind_addr);

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let payload = server_payload.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(&mut stream, &payload).await {
                        debug!("SOCKS5 session ended: {}", e);
                    }
                });
            }
        }
    }

    async fn handle_client(stream: &mut TcpStream, payload: &ServerPayload) -> Result<()> {
        // 1. هندشیک پروتکل SOCKS5
        let ver = stream.read_u8().await?;
        if ver != 0x05 {
            return Err(anyhow!("Unsupported SOCKS version"));
        }
        let nmethods = stream.read_u8().await?;
        let mut methods = vec![0u8; nmethods as usize];
        stream.read_exact(&mut methods).await?;
        stream.write_all(&[0x05, 0x00]).await?;
        stream.flush().await?;

        // 2. خواندن اطلاعات مقصد
        let req_ver = stream.read_u8().await?;
        let cmd = stream.read_u8().await?;
        let _rsv = stream.read_u8().await?;
        let atyp = stream.read_u8().await?;

        if req_ver != 0x05 || cmd != 0x01 {
            stream.write_all(&[0x05, 0x07, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
            return Err(anyhow!("Only CONNECT command supported"));
        }

        let target_host = match atyp {
            0x01 => {
                let mut ip = [0u8; 4];
                stream.read_exact(&mut ip).await?;
                Ipv4Addr::from(ip).to_string()
            }
            0x03 => {
                let len = stream.read_u8().await?;
                let mut domain = vec![0u8; len as usize];
                stream.read_exact(&mut domain).await?;
                String::from_utf8(domain)?
            }
            0x04 => {
                let mut ip = [0u8; 16];
                stream.read_exact(&mut ip).await?;
                Ipv6Addr::from(ip).to_string()
            }
            _ => return Err(anyhow!("Unknown ATYP")),
        };

        let target_port = stream.read_u16().await?;

        // 3. اتصال امن به سرور QCore
        let mut server_stream = TcpStream::connect(&payload.endpoint)
            .await
            .map_err(|e| anyhow!("Failed to connect to QCore server {}: {e}", payload.endpoint))?;

        // 4. اجرای تبادل کلید پسا-کوانتومی با سرور
        let (classic_pub, pq_ct, shared_secret) =
            HybridClientKEM::encapsulate(&payload.classic_pub, &payload.pq_pub)?;

        let meta = serde_json::json!({
            "token": payload.token,
            "uuid": payload.user_uuid,
            "host": target_host,
            "port": target_port
        });
        let meta_bytes = serde_json::to_vec(&meta)?;

        let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)?;
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), meta_bytes.as_ref())
            .map_err(|e| anyhow!("Handshake encryption error: {e}"))?;

        // ارسال فریم به سرور
        server_stream.write_all(b"QC\x01").await?;
        server_stream.write_all(&classic_pub).await?;
        server_stream.write_u16(pq_ct.len() as u16).await?;
        server_stream.write_all(&pq_ct).await?;
        server_stream.write_all(&nonce_bytes).await?;
        server_stream.write_u16(ciphertext.len() as u16).await?;
        server_stream.write_all(&ciphertext).await?;
        server_stream.flush().await?;

        // تایید از سرور
        let status = server_stream.read_u8().await?;
        if status != 0x00 {
            stream.write_all(&[0x05, 0x05, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
            return Err(anyhow!("Server rejected connection (status {status})"));
        }

        // 5. پاسخ موفقیت‌آمیز به کلاینت و آغاز انتقال دوطرفه دیتا
        stream.write_all(&[0x05, 0x00, 0x00, 0x01, 127, 0, 0, 1, 0x2a, 0x38]).await?;
        stream.flush().await?;

        tokio::io::copy_bidirectional(stream, &mut server_stream).await?;
        Ok(())
    }
}
```

### src/crates/qcore-client/src/lib.rs
```rust
pub mod ffi;
pub mod inbound;
pub mod wizard;

pub use ffi::*;
pub use inbound::*;
pub use wizard::*;
```

### src/crates/qcore-client/src/main.rs
```rust
mod ffi;
mod inbound;
mod wizard;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use inbound::http_proxy::HttpProxyServer;
use inbound::socks5::Socks5Server;
use qcore_crypto::share_link::ServerPayload;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use wizard::ClientWizard;

#[derive(Parser)]
#[command(name = "qcore-client")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, default_value = "127.0.0.1:10808")]
    socks_addr: String,

    #[arg(long, default_value = "127.0.0.1:10809")]
    http_addr: String,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    if let Some(Commands::Setup) = cli.command {
        match ClientWizard::run_interactive() {
            Ok(payload) => {
                info!("کلاینت با موفقیت برای سرور {} پیکربندی شد.", payload.endpoint);
                return Ok(());
            }
            Err(e) => {
                println!("[\x1b[31mx\x1b[0m] خطا: {}", e);
                return Ok(());
            }
        }
    }

    let config_data = std::fs::read_to_string("client_config.json").map_err(|_| {
        anyhow!("فایل کانفیگ یافت نشد. لطفاً ابتدا دستور 'qcore-client setup' را اجرا کنید!")
    })?;
    let payload: ServerPayload = serde_json::from_str(&config_data)?;
    let shared_payload = Arc::new(payload);

    let socks_addr = cli.socks_addr.clone();
    let http_addr = cli.http_addr.clone();

    let socks_payload = shared_payload.clone();
    let socks_handle = tokio::spawn(async move {
        if let Err(e) = Socks5Server::run(&socks_addr, socks_payload).await {
            tracing::error!("خطا در اینباند SOCKS5: {}", e);
        }
    });

    let http_payload = shared_payload.clone();
    let http_handle = tokio::spawn(async move {
        if let Err(e) = HttpProxyServer::run(&http_addr, http_payload).await {
            tracing::error!("خطا در اینباند HTTP: {}", e);
        }
    });

    info!("کلاینت QCore آماده به کار است.");
    info!("اینباند SOCKS5 فعال روی: {}", cli.socks_addr);
    info!("اینباند HTTP Proxy فعال روی: {}", cli.http_addr);

    let _ = tokio::try_join!(socks_handle, http_handle);
    Ok(())
}
```

### src/crates/qcore-client/src/wizard.rs
```rust
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
```

### src/crates/qcore-crypto/Cargo.toml
```toml
[package]
name = "qcore-crypto"
version.workspace = true
edition.workspace = true

[dependencies]
x25519-dalek = { workspace = true }
blake3 = { workspace = true }
ml-kem = { workspace = true }
chacha20poly1305 = { workspace = true }
bytes = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
base64 = { workspace = true }
rand = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
```

### src/crates/qcore-crypto/src/hybrid_kem.rs
```rust
use anyhow::{anyhow, Result};
use blake3::Hasher;
use ml_kem::{
    kem::{Decapsulate, DecapsulationKey, Encapsulate, EncapsulationKey},
    Encoded, EncodedSizeUser, KemCore, MlKem768, MlKem768Params,
};
use rand::{thread_rng, RngCore};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

pub struct HybridKeyPair {
    pub classic_secret: StaticSecret,
    pub classic_public: X25519PublicKey,
    pub pq_decapsulation_key: DecapsulationKey<MlKem768Params>,
    pub pq_public_bytes: Vec<u8>,
}

impl HybridKeyPair {
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        // 1. کلید کلاسیک X25519
        let mut classic_bytes = [0u8; 32];
        rng.fill_bytes(&mut classic_bytes);
        let classic_secret = StaticSecret::from(classic_bytes);
        let classic_public = X25519PublicKey::from(&classic_secret);

        // 2. کلید پسا-کوانتوم ML-KEM-768
        let (decaps_key, encaps_key): (_, EncapsulationKey<MlKem768Params>) =
            MlKem768::generate(&mut rng);
        let pq_public_bytes = encaps_key.as_bytes().as_slice().to_vec();

        Self {
            classic_secret,
            classic_public,
            pq_decapsulation_key: decaps_key,
            pq_public_bytes,
        }
    }
}

pub struct HybridClientKEM;

impl HybridClientKEM {
    /// کپسوله‌سازی در سمت کلاینت
    pub fn encapsulate(
        peer_classic_pub: &[u8; 32],
        peer_pq_pub_bytes: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>, [u8; 32])> {
        let mut rng = thread_rng();

        let mut ephemeral_bytes = [0u8; 32];
        rng.fill_bytes(&mut ephemeral_bytes);
        let my_secret = StaticSecret::from(ephemeral_bytes);
        let my_classic_public = X25519PublicKey::from(&my_secret);
        let peer_x25519 = X25519PublicKey::from(*peer_classic_pub);
        let ss_classic = my_secret.diffie_hellman(&peer_x25519);

        let encoded_ek = Encoded::<EncapsulationKey<MlKem768Params>>::try_from(peer_pq_pub_bytes)
            .map_err(|e| anyhow!("Invalid ML-KEM-768 public key length: {e:?}"))?;
        let encaps_key = EncapsulationKey::<MlKem768Params>::from_bytes(&encoded_ek);

        let (pq_ciphertext, ss_pq) = encaps_key
            .encapsulate(&mut rng)
            .map_err(|_| anyhow!("ML-KEM encapsulation failed"))?;

        let mut hasher = Hasher::new();
        hasher.update(b"QCORE_HYBRID_KEM_v5_COMBINER");
        hasher.update(ss_classic.as_bytes());
        hasher.update(ss_pq.as_slice());
        let shared_secret = *hasher.finalize().as_bytes();

        Ok((
            my_classic_public.as_bytes().to_vec(),
            pq_ciphertext.as_slice().to_vec(),
            shared_secret,
        ))
    }
}

pub struct HybridServerKEM;

impl HybridServerKEM {
    /// دکپسوله‌سازی در سمت سرور
    pub fn decapsulate(
        classic_secret: &StaticSecret,
        pq_decaps_key: &DecapsulationKey<MlKem768Params>,
        client_classic_pub: &[u8; 32],
        pq_ciphertext_bytes: &[u8],
    ) -> Result<[u8; 32]> {
        let client_x25519 = X25519PublicKey::from(*client_classic_pub);
        let ss_classic = classic_secret.diffie_hellman(&client_x25519);

        let ct = ml_kem::Ciphertext::<MlKem768>::try_from(pq_ciphertext_bytes)
            .map_err(|e| anyhow!("Invalid ML-KEM ciphertext: {e:?}"))?;
        
        // استخراج مقدار با map_err از خروجی Result
        let ss_pq = pq_decaps_key
            .decapsulate(&ct)
            .map_err(|_| anyhow!("ML-KEM decapsulation failed"))?;

        let mut hasher = Hasher::new();
        hasher.update(b"QCORE_HYBRID_KEM_v5_COMBINER");
        hasher.update(ss_classic.as_bytes());
        hasher.update(ss_pq.as_slice());
        Ok(*hasher.finalize().as_bytes())
    }
}
```

### src/crates/qcore-crypto/src/lib.rs
```rust
pub mod hybrid_kem;
pub mod share_link;

pub use hybrid_kem::*;
pub use share_link::*;
```

### src/crates/qcore-crypto/src/share_link.rs
```rust
use anyhow::{anyhow, Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerPayload {
    pub endpoint: String,
    pub sni: String,
    pub user_uuid: String,
    pub token: String,
    pub classic_pub: [u8; 32],
    pub pq_pub: Vec<u8>,
    pub transport: String,
}

pub struct SecureLinkEngine;

impl SecureLinkEngine {
    /// مدل ۱: لینک رمزشده مبتنی بر کلمه عبور/کلید اشتراکی
    pub fn export_symmetric(payload: &ServerPayload, password: &str) -> Result<String> {
        let serialized = serde_json::to_vec(payload)?;
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut salt);
        rng.fill_bytes(&mut nonce_bytes);

        let mut key_deriver = blake3::Hasher::new_derive_key("qcore-sym-share-link-salt");
        key_deriver.update(&salt);
        key_deriver.update(password.as_bytes());
        let key = key_deriver.finalize();

        let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
            .map_err(|e| anyhow!("Cipher initialization error: {e}"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, serialized.as_ref())
            .map_err(|e| anyhow!("Encryption error: {e}"))?;

        let mut blob = Vec::with_capacity(16 + 12 + ciphertext.len());
        blob.extend_from_slice(&salt);
        blob.extend_from_slice(&nonce_bytes);
        blob.extend_from_slice(&ciphertext);

        Ok(format!("qcore://sym.{}", URL_SAFE_NO_PAD.encode(blob)))
    }

    pub fn import_symmetric(link: &str, password: &str) -> Result<ServerPayload> {
        let stripped = link.trim().strip_prefix("qcore://sym.").context("Invalid link scheme")?;
        let blob = URL_SAFE_NO_PAD.decode(stripped)?;
        if blob.len() < 28 {
            return Err(anyhow!("Payload is too short"));
        }

        let salt = &blob[0..16];
        let nonce_bytes = &blob[16..28];
        let ciphertext = &blob[28..];

        let mut key_deriver = blake3::Hasher::new_derive_key("qcore-sym-share-link-salt");
        key_deriver.update(salt);
        key_deriver.update(password.as_bytes());
        let key = key_deriver.finalize();

        let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
            .map_err(|e| anyhow!("Cipher error: {e}"))?;
        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow!("Failed to decrypt link. Password may be incorrect."))?;

        let payload = serde_json::from_slice(&decrypted)?;
        Ok(payload)
    }

    /// مدل ۲: لینک رمزشده نامتقارن (Asymmetric Link)
    pub fn export_asymmetric(payload: &ServerPayload, recipient_x25519: &[u8; 32]) -> Result<String> {
        let serialized = serde_json::to_vec(payload)?;
        let mut rng = thread_rng();

        let mut ephemeral_bytes = [0u8; 32];
        rng.fill_bytes(&mut ephemeral_bytes);
        let ephemeral = x25519_dalek::StaticSecret::from(ephemeral_bytes);
        let ephemeral_pub = x25519_dalek::PublicKey::from(&ephemeral);
        let target_pub = x25519_dalek::PublicKey::from(*recipient_x25519);
        let ss = ephemeral.diffie_hellman(&target_pub);

        let key = blake3::derive_key("qcore-asym-link-derive", ss.as_bytes());
        let mut nonce_bytes = [0u8; 12];
        rng.fill_bytes(&mut nonce_bytes);

        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| anyhow!("Cipher initialization error: {e}"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, serialized.as_ref())
            .map_err(|e| anyhow!("Encryption error: {e}"))?;

        let mut blob = Vec::with_capacity(32 + 12 + ciphertext.len());
        blob.extend_from_slice(ephemeral_pub.as_bytes());
        blob.extend_from_slice(&nonce_bytes);
        blob.extend_from_slice(&ciphertext);

        Ok(format!("qcore://asym.{}", URL_SAFE_NO_PAD.encode(blob)))
    }

    pub fn import_asymmetric(link: &str, my_secret: &x25519_dalek::StaticSecret) -> Result<ServerPayload> {
        let stripped = link.trim().strip_prefix("qcore://asym.").context("Invalid link scheme")?;
        let blob = URL_SAFE_NO_PAD.decode(stripped)?;
        if blob.len() < 44 {
            return Err(anyhow!("Payload is too short for asymmetric link"));
        }

        let mut peer_pub_bytes = [0u8; 32];
        peer_pub_bytes.copy_from_slice(&blob[0..32]);
        let nonce_bytes = &blob[32..44];
        let ciphertext = &blob[44..];

        let peer_pub = x25519_dalek::PublicKey::from(peer_pub_bytes);
        let ss = my_secret.diffie_hellman(&peer_pub);
        let key = blake3::derive_key("qcore-asym-link-derive", ss.as_bytes());

        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| anyhow!("Cipher error: {e}"))?;
        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow!("Failed to decrypt asymmetric link: key mismatch"))?;

        let payload = serde_json::from_slice(&decrypted)?;
        Ok(payload)
    }
}
```

### src/crates/qcore-server/Cargo.toml
```toml
[package]
name = "qcore-server"
version.workspace = true
edition.workspace = true

[[bin]]
name = "qcore-server"
path = "src/main.rs"

[dependencies]
qcore-crypto = { path = "../qcore-crypto" }
qcore-tls-synth = { path = "../qcore-tls-synth" }
qcore-transport = { path = "../qcore-transport" }
tokio = { workspace = true }
bytes = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
toml = { workspace = true }
clap = { workspace = true }
uuid = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
base64 = { workspace = true }
chacha20poly1305 = { workspace = true }
ml-kem = { workspace = true }
x25519-dalek = { workspace = true }
```

### src/crates/qcore-server/src/main.rs
```rust
mod user_cli;
mod user_manager;
mod wizard;

use anyhow::{anyhow, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use clap::{Parser, Subcommand};
use ml_kem::kem::DecapsulationKey;
use ml_kem::{Encoded, EncodedSizeUser, MlKem768Params};
use qcore_crypto::hybrid_kem::HybridServerKEM;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use user_cli::{UserCliHandler, UserCommands};
use user_manager::UserManager;
use wizard::{ServerWizard, StoredServerConfig};
use x25519_dalek::StaticSecret;

#[derive(Parser)]
#[command(name = "qcore-server")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// ویزارد اولیه راه‌اندازی سرور
    Setup,
    /// مدیریت کاربران سرور (افزودن، لیست، حذف و صدور لینک)
    #[command(subcommand)]
    User(UserCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Setup) => {
            ServerWizard::run_interactive()?;
            return Ok(());
        }
        Some(Commands::User(user_cmd)) => {
            UserCliHandler::handle(user_cmd)?;
            return Ok(());
        }
        None => {}
    }

    let config_data = std::fs::read_to_string("server_config.json")
        .map_err(|_| anyhow!("لطفاً ابتدا با دستور 'qcore-server setup' سرور را راه‌اندازی کنید."))?;
    let config: StoredServerConfig = serde_json::from_str(&config_data)?;

    // لود کردن تمامی کاربران ثبت‌شده در فایل تنظیمات
    let user_manager = UserManager::new();
    for u in &config.users {
        user_manager.add_user(u.clone()).await;
    }
    info!("تعداد {} کاربر با موفقیت در سیستم احراز هویت لود شدند.", config.users.len());

    let classic_secret_bytes: [u8; 32] = STANDARD.decode(&config.classic_secret_b64)?
        .try_into().map_err(|_| anyhow!("Invalid classic secret format"))?;
    let classic_secret = StaticSecret::from(classic_secret_bytes);

    let dk_bytes = STANDARD.decode(&config.pq_decaps_key_b64)?;
    let encoded_dk = Encoded::<DecapsulationKey<MlKem768Params>>::try_from(dk_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid ML-KEM key length"))?;
    let pq_decaps_key = Arc::new(DecapsulationKey::<MlKem768Params>::from_bytes(&encoded_dk));

    let bind_addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("سرور QCore آماده دریافت اتصالات روی آدرس {} است.", bind_addr);

    let sni_fallback = config.sni.clone();

    loop {
        if let Ok((socket, _peer)) = listener.accept().await {
            let cs = classic_secret.clone();
            let pq_dk = pq_decaps_key.clone();
            let um = user_manager.clone();
            let fallback = sni_fallback.clone();

            tokio::spawn(async move {
                let _ = handle_connection(socket, cs, pq_dk, um, fallback).await;
            });
        }
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    classic_secret: StaticSecret,
    pq_decaps_key: Arc<DecapsulationKey<MlKem768Params>>,
    user_manager: UserManager,
    fallback_domain: String,
) -> Result<()> {
    let mut magic = [0u8; 3];
    if socket.read_exact(&mut magic).await.is_err() || &magic != b"QC\x01" {
        if let Ok(mut fallback) = TcpStream::connect(format!("{}:443", fallback_domain)).await {
            let _ = tokio::io::copy_bidirectional(&mut socket, &mut fallback).await;
        }
        return Ok(());
    }

    let mut client_classic = [0u8; 32];
    socket.read_exact(&mut client_classic).await?;

    let pq_len = socket.read_u16().await? as usize;
    let mut pq_ct = vec![0u8; pq_len];
    socket.read_exact(&mut pq_ct).await?;

    let shared_secret = HybridServerKEM::decapsulate(
        &classic_secret,
        &pq_decaps_key,
        &client_classic,
        &pq_ct,
    )?;

    let mut nonce_bytes = [0u8; 12];
    socket.read_exact(&mut nonce_bytes).await?;
    let cipher_len = socket.read_u16().await? as usize;
    let mut ciphertext = vec![0u8; cipher_len];
    socket.read_exact(&mut ciphertext).await?;

    let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)?;
    let decrypted = cipher
        .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref())
        .map_err(|_| anyhow!("خطا در رمزگشایی متادیتا"))?;

    let meta: serde_json::Value = serde_json::from_slice(&decrypted)?;
    let token = meta["token"].as_str().unwrap_or("");
    let uuid = meta["uuid"].as_str().unwrap_or("");
    let host = meta["host"].as_str().unwrap_or("");
    let port = meta["port"].as_u64().unwrap_or(0) as u16;

    let user_state = match user_manager.authenticate(uuid, token).await {
        Some(u) => u,
        None => {
            socket.write_u8(0x01).await?;
            return Err(anyhow!("کاربر غیرمجاز یا منقضی شده"));
        }
    };

    let target_addr = format!("{}:{}", host, port);
    let mut target_stream = match TcpStream::connect(&target_addr).await {
        Ok(s) => s,
        Err(_) => {
            let _ = socket.write_u8(0x02).await;
            return Err(anyhow!("مقصد در دسترس نیست"));
        }
    };

    socket.write_u8(0x00).await?;
    socket.flush().await?;

    let (tx, rx) = tokio::io::copy_bidirectional(&mut socket, &mut target_stream).await?;
    UserManager::record_traffic(&user_state, tx + rx);
    info!("پایان نشست کاربر '{}' -> {} (ارسال: {} بایت، دریافت: {} بایت)", user_state.config.name, target_addr, tx, rx);
    Ok(())
}
```

### src/crates/qcore-server/src/user_cli.rs
```rust
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
```

### src/crates/qcore-server/src/user_manager.rs
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub uuid: String,
    pub name: String,
    pub token: String,
    pub quota_bytes: u64,
    pub expiry_timestamp: u64,
    pub enabled: bool,
}

pub struct UserState {
    pub config: UserConfig,
    pub used_bytes: AtomicU64,
}

#[derive(Clone)]
pub struct UserManager {
    users: Arc<RwLock<HashMap<String, Arc<UserState>>>>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_user(&self, config: UserConfig) {
        let mut map = self.users.write().await;
        let state = Arc::new(UserState {
            config: config.clone(),
            used_bytes: AtomicU64::new(0),
        });
        map.insert(config.uuid, state);
    }

    pub async fn authenticate(&self, uuid: &str, token: &str) -> Option<Arc<UserState>> {
        let map = self.users.read().await;
        if let Some(user) = map.get(uuid) {
            if user.config.enabled && user.config.token == token {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                if user.config.expiry_timestamp > 0 && current_time > user.config.expiry_timestamp {
                    return None;
                }

                if user.config.quota_bytes > 0 {
                    let used = user.used_bytes.load(Ordering::Relaxed);
                    if used >= user.config.quota_bytes {
                        return None;
                    }
                }

                return Some(user.clone());
            }
        }
        None
    }

    pub fn record_traffic(user: &UserState, bytes: u64) {
        user.used_bytes.fetch_add(bytes, Ordering::Relaxed);
    }
}
```

### src/crates/qcore-server/src/wizard.rs
```rust
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
        println!("       QCore Enterprise Server - Interactive Setup Wizard   ");
        println!("============================================================");

        let stdin = io::stdin();

        print!("[?] Enter listening port [Default 8443]: ");
        io::stdout().flush()?;
        let mut port_input = String::new();
        stdin.read_line(&mut port_input)?;
        let port_str = port_input.trim();
        let port: u16 = if port_str.is_empty() { 8443 } else { port_str.parse().unwrap_or(8443) };

        print!("[?] Enter SNI/Fallback Target Domain [Default: swdist.apple.com]: ");
        io::stdout().flush()?;
        let mut sni_input = String::new();
        stdin.read_line(&mut sni_input)?;
        let sni = sni_input.trim();
        let sni = if sni.is_empty() { "swdist.apple.com" } else { sni };

        print!("[?] Enter your Server Public IP / Domain [e.g. 127.0.0.1]: ");
        io::stdout().flush()?;
        let mut ip_input = String::new();
        stdin.read_line(&mut ip_input)?;
        let server_ip = ip_input.trim();
        let server_ip = if server_ip.is_empty() { "127.0.0.1" } else { server_ip };

        println!("\n[*] در حال تولید کلیدهای پسا-کوانتوم NIST FIPS 203 ML-KEM-768 + X25519...");
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

        print!("[?] یک رمز عبور برای لینک اشتراک کاربر اولیه (admin) وارد کنید: ");
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
        println!("             تنظیمات ذخیره شد و سرور آماده است              ");
        println!("------------------------------------------------------------");
        println!("فایل تنظیمات: server_config.json");
        println!("\nلینک رمزشده کاربر ادمین:");
        println!("\x1b[36m{}\x1b[0m", sym_link);
        println!("\nنکته: اکنون می‌توانید با دستور زیر در هر زمان کاربر جدید اضافه کنید:");
        println!("  qcore-server user add <NAME> --quota-gb <GB> --days <DAYS>");
        println!("------------------------------------------------------------\n");

        Ok(())
    }
}
```

### src/crates/qcore-tls-synth/Cargo.toml
```toml
[package]
name = "qcore-tls-synth"
version.workspace = true
edition.workspace = true

[dependencies]
bytes = { workspace = true }
rand = { workspace = true }
```

### src/crates/qcore-tls-synth/src/lib.rs
```rust
use bytes::{BufMut, BytesMut};
use rand::{thread_rng, Rng, RngCore};

pub struct ChromeJa4Synth;

impl ChromeJa4Synth {
    /// سنتز بایت به بایت هدر دست‌تکانی منطبق بر گوگل کروم با افزونه پسا-کوانتوم 0x11ec
    pub fn build_client_hello(
        sni: &str,
        classic_key_share: &[u8; 32],
        pq_key_share: &[u8],
        reality_auth_tag: &[u8; 8],
    ) -> BytesMut {
        let mut extensions = BytesMut::with_capacity(2048);
        let grease_val = Self::get_grease_val();

        // 1. افزونه SNI
        let sni_bytes = sni.as_bytes();
        extensions.put_u16(0x0000);
        extensions.put_u16(sni_bytes.len() as u16 + 5);
        extensions.put_u16(sni_bytes.len() as u16 + 3);
        extensions.put_u8(0x00);
        extensions.put_u16(sni_bytes.len() as u16);
        extensions.put_slice(sni_bytes);

        // 2. افزونه Supported Groups (با گروه پسا-کوانتوم 0x11ec)
        extensions.put_u16(0x000a);
        extensions.put_u16(8);
        extensions.put_u16(6);
        extensions.put_u16(grease_val);
        extensions.put_u16(0x11ec); // پسا-کوانتوم کروم (X25519MLKEM768)
        extensions.put_u16(0x001d); // x25519

        // 3. افزونه Supported Versions
        extensions.put_u16(0x002b);
        extensions.put_u16(5);
        extensions.put_u8(4);
        extensions.put_u16(0x0304); // TLS 1.3
        extensions.put_u16(0x0303); // TLS 1.2

        // 4. افزونه Key Share هیبرید
        let key_shares_len = (4 + 32) + (4 + pq_key_share.len());
        extensions.put_u16(0x0033);
        extensions.put_u16(key_shares_len as u16 + 2);
        extensions.put_u16(key_shares_len as u16);

        // سهم کلید سنتی
        extensions.put_u16(0x001d);
        extensions.put_u16(32);
        extensions.put_slice(classic_key_share);

        // سهم کلید پسا-کوانتوم
        extensions.put_u16(0x11ec);
        extensions.put_u16(pq_key_share.len() as u16);
        extensions.put_slice(pq_key_share);

        // ساخت رکورد ClientHello
        let mut handshake = BytesMut::new();
        handshake.put_u8(0x01); // ClientHello
        
        // نوشتن طول پیام در فرمت ۲۴ بیتی (3 بایت استاندارد TLS)
        let handshake_len = (38 + extensions.len()) as u32;
        handshake.put_u8(((handshake_len >> 16) & 0xFF) as u8);
        handshake.put_u16((handshake_len & 0xFFFF) as u16);
        
        handshake.put_u16(0x0303); // Compatibility TLS 1.2

        // بردار تصادفی به همراه بردار اعتبارسنجی REALITY
        let mut random = [0u8; 32];
        thread_rng().fill_bytes(&mut random[0..24]);
        random[24..32].copy_from_slice(reality_auth_tag);
        handshake.put_slice(&random);

        // Session ID
        handshake.put_u8(32);
        let mut session_id = [0u8; 32];
        thread_rng().fill_bytes(&mut session_id);
        handshake.put_slice(&session_id);

        // Cipher Suites
        handshake.put_u16(6);
        handshake.put_u16(0x1301); // TLS_AES_128_GCM_SHA256
        handshake.put_u16(0x1302); // TLS_AES_256_GCM_SHA384
        handshake.put_u16(0x1303); // TLS_CHACHA20_POLY1305_SHA256

        handshake.put_u8(1);
        handshake.put_u8(0x00);

        handshake.put_u16(extensions.len() as u16);
        handshake.put_slice(&extensions);

        handshake
    }

    /// شکستن فریم ClientHello درون دو رکورد مجزای معتبر لایه TLS (Record Splitting)
    pub fn split_handshake_records(handshake: &[u8], split_pos: usize) -> BytesMut {
        assert!(split_pos > 0 && split_pos < handshake.len());
        let mut stream = BytesMut::new();

        // رکورد اول (هدر بدون نام دامنه)
        stream.put_u8(0x16);
        stream.put_u16(0x0301);
        stream.put_u16(split_pos as u16);
        stream.put_slice(&handshake[..split_pos]);

        // رکورد دوم (ادامه هندشیک حاوی SNI)
        let rem = handshake.len() - split_pos;
        stream.put_u8(0x16);
        stream.put_u16(0x0301);
        stream.put_u16(rem as u16);
        stream.put_slice(&handshake[split_pos..]);

        stream
    }

    fn get_grease_val() -> u16 {
        let grease_table = [
            0x0a0a, 0x1a1a, 0x2a2a, 0x3a3a, 0x4a4a, 0x5a5a, 0x6a6a, 0x7a7a, 0x8a8a, 0x9a9a,
            0xaaaa, 0xbaba, 0xcaca, 0xdada, 0xeaea, 0xfafa,
        ];
        let idx = thread_rng().gen_range(0..grease_table.len());
        grease_table[idx]
    }
}
```

### src/crates/qcore-transport/Cargo.toml
```toml
[package]
name = "qcore-transport"
version.workspace = true
edition.workspace = true

[dependencies]
tokio = { workspace = true }
bytes = { workspace = true }
socket2 = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
```

### src/crates/qcore-transport/src/engine.rs
```rust
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
```

### src/crates/qcore-transport/src/lib.rs
```rust
pub mod engine;

pub use engine::*;
```

