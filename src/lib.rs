#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::*;

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(_: HINSTANCE, reason: u32, _: u32) -> i32 {
    let lp_text: Vec<u16> = "Hello, World!!\0".encode_utf16().collect();
    let caption = "hello\0".to_string();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    MessageBoxW(std::ptr::null_mut(), lp_text.as_ptr(), lp_caption.as_ptr(), MB_OK);
    0
}