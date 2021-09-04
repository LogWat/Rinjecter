#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::*;

#[no_mangle]
pub unsafe extern "system" fn DllMain(_: HINSTANCE, reason: u32, _: u32) -> BOOL {
    match reason {
        1 => {
            let s = "DLL_PROCESS_ATTACH\0".to_string();
            msg(&s);
        }
        _ => {
            let s = "DEFAULT\0".to_string();
            msg(&s);
        }
    }
    TRUE
}

#[no_mangle]
pub extern "C" fn hello() {
    let s = "hello\0".to_string();
    msg(&s);
}

fn msg(caption: &str) {
    let lp_text: Vec<u16> = "Hello World! \u{1F60E}\0".encode_utf16().collect();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK
        );
    }
}