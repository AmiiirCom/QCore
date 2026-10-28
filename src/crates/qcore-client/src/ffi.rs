use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
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
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // شروع لوکال پروکسی‌ها درون پس‌زمینه اپلیکیشن موبایل
        });
    });

    0
}

#[no_mangle]
pub extern "C" fn qcore_client_stop() -> i32 {
    0
}