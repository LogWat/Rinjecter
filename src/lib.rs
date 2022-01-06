#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;
extern crate kernel32;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::winnt::*;
use winapi::shared::minwindef::*;
use kernel32::*;

use std::mem::size_of;

use std::convert::TryInto;

const DPATH: u32 = 0x4B5B4C;

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(_: HINSTANCE, reason: u32, _: u32) -> BOOL {

    match reason {
        1 => {
            if changedisplayname() == false {
                let errtext = "Failed to change display name.\0".to_string();
                err_msgbox(errtext);
            }
        },
        _ => {}
    }

    TRUE
}

// 生ポインタの利用 *mut or *const
// この場合，アスタリスクは参照外しではなく型の一部である
unsafe extern "stdcall" fn changedisplayname() -> bool {
    let mut last_page: DWORD = 0;

    let address2 = (*((*(DPATH as *mut i32)) as *mut i32) + 0x64B) as *mut i32; // [[[0x4B5B4C]] + 0x64B]

    let num_of_characters = *(((*(DPATH as *mut i32)) + 0x335) as *mut i32); // [[0x4B5B4C] + 0x335]
    let message = format!("{}", num_of_characters);
    err_msgbox(message);
    let c = address2;
    for _ in 0..num_of_characters {
        if *((c as i32 + 0x4) as *mut i32) == 0x7473694D && *((c as i32 + 0x8) as *mut i32) == 0x6E656B61 {
            if VirtualProtect(
                c as *mut _,
                ((size_of::<i32>() * 4) as u64).try_into().unwrap(),
                PAGE_READWRITE,
                &mut last_page
            ) == 0 {
                return false;
            }
            *((c as i32 + 0x4) as *mut i32) = 0x45544154;
            *((c as i32 + 0x8) as *mut i32) = 0x4157414B;
            *((c as i32 + 0xC) as *mut i32) = 0x44414E53;
            *((c as i32 + 0x10) as *mut i32) = 0x4948;
            break;
        }
        *c += 0x10E;
    }
    true
}

unsafe extern "stdcall" fn err_msgbox(text: String) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let caption = "⚠Error⚠\0".to_string();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK
    );
}