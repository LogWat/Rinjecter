#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;
extern crate kernel32;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::*;
use kernel32::*;

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(_: HINSTANCE, reason: u32, _: u32) -> BOOL {
    let lp_text: Vec<u16> = "Oh, boy. You've made huge mistake.\0".encode_utf16().collect();
    let caption = "⚠Error⚠\0".to_string();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    MessageBoxW(std::ptr::null_mut(), lp_text.as_ptr(), lp_caption.as_ptr(), MB_OK);
    TRUE
}