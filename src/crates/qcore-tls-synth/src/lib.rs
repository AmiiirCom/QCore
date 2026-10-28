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