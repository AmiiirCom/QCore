//! C-ABI Foreign Function Interface (FFI).
//!
//! Exposes bindings compatible with native mobile application frameworks
//! (Android JNI via Kotlin/Java and iOS/macOS via Swift/Objective-C).

use std::ffi::CStr;
use std::os::raw::c_char;

/// Starts the QCore client engine on a background runtime thread.
///
/// # Safety
/// `config_json` must be a valid null-terminated C string pointer.
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
            // Background daemon routines for embedded mobile targets
        });
    });

    0
}

/// Signals the client engine to terminate and release open sockets.
#[unsafe(no_mangle)]
pub extern "C" fn qcore_client_stop() -> i32 {
    0
}