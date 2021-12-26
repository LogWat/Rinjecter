#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;
extern crate kernel32;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::*;
use kernel32::*;

const DPATH: u32 = 0x4B5B4C;

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(_: HINSTANCE, reason: u32, _: u32) -> BOOL {

    match reason {
        DLL_PROCESSS_ATTACH => {
            if changedisplayname() {
                let errtext = "Failed to change display name.\0".to_string();
                errMsgBox(errtext);
            }
        },
        _ => {}
    }

    TRUE
}

unsafe extern "stdcall" fn changedisplayname() -> bool {
    let oldp: DWORD = 0;
    let oldp2: DWORD = 0;

    let address1 = DPATH as *mut i32;
    let address2 = *(*((i32 ***) (DPATH as *const i32)) as *const i32) as *const i32;

    true
}

unsafe extern "stdcall" fn errMsgBox(text: String) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let caption = "⚠Error⚠\0".to_string();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    MessageBoxW(std::ptr::null_mut(),
                lp_text.as_ptr(),
                lp_caption.as_ptr(),
                MB_OK
            );
}