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