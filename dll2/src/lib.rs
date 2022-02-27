#![allow(non_snake_case)]
extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::um::winuser::{MB_OK, MessageBoxW};

#[no_mangle]
pub extern "stdcall" fn DllMain(
    _hinst_dll: HINSTANCE, 
    reason: DWORD,
    _: LPVOID
) -> i32 {
    match reason {
        _DLL_PROCESS_ATTACH => {
            let msg = "[!] DLL_PROCESS_ATTACH\0";
            let title = "INFO\0";
            unsafe { MsgBox(msg, title); }


            return true as i32;
        },
        _DLL_PROCESS_DETACH => {
            return true as i32;
        },
        _ => true as i32,
    }
}

unsafe fn MsgBox(text: &str, title: &str) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let lp_caption: Vec<u16> = title.encode_utf16().collect();

    MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK
    );
}