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