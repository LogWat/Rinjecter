#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

mod processlib;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::{winnt::*, memoryapi, libloaderapi};
use winapi::shared::minwindef::*;

use processlib::Process;

use std::{mem};
use std::convert::TryInto;
use rand::Rng;

const DPATH: u32 = 0x4B5B4C;

#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE, 
    reason: DWORD,
    _: LPVOID
) -> i32 {

    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(hinst_dll);
                let mut process = Process::current_process();
                if changedisplayname() == false {
                    let errtext = "Failed to change display name.\0".to_string();
                    err_msgbox(errtext);
                }
            }
            return true as i32;
        },
        DLL_PROCESS_DETACH => {
            return true as i32;
        },
        _ => true as i32,
    }
}

unsafe extern "stdcall" fn rewrite_program() -> Result<(), &'static str> {
    let mut oldp: DWORD = 0;
    if memoryapi::VirtualProtect(
        0x401000 as *mut _,
        0x9E000 as _,
        PAGE_READWRITE,
        &mut oldp as *mut _,
    ) != 0 {
        return Err("Failed to change memory protection.");
    }

    // rewrite program
    *(0x41DBD4 as *mut u32) = 0x4BEA00A1;   // -> mov eax, [0x4BEA00] ([0x4BEA00] = 0x0)
    *(0x41DBD8 as *mut u32) = 0x9000;       // -> nop
    *(0x41DF21 as *mut u32) = 0x4BEA00A1;
    *(0x41DF25 as *mut u32) = 0x9000;
    *(0x41F9E7 as *mut u32) = 0x4BEA00A1;
    *(0x41F9EB as *mut u32) = 0x9000;
    *(0x41FC8D as *mut u32) = 0xEA0005C7;    
    *(0x41FC91 as *mut u32) = 0x4B;         // -> mov [0x4BEA00], 0x1
    *(0x41DF76 as *mut u32) = 0xEA0005C7;
    *(0x41DF7A as *mut u32) = 0x4B;
    *(0x41FDF3 as *mut u32) = 0xEA0005C7;
    *(0x41FDF7 as *mut u32) = 0x4B;
    *(0x41FF01 as *mut u32) = 0xEA001589;   // -> mov [0x4BEA00], edx
    *(0x41FF05 as *mut u32) = 0x4B;
    *(0x42035E as *mut u32) = 0xEA005C7;
    *(0x420362 as *mut u32) = 0x4B;
    *(0x420399 as *mut u32) = 0x4BEA00A1;
    *(0x4203A3 as *mut u32) = 0x9000;
    *(0x421B93 as *mut u32) = 0x4BEA00A1;
    *(0x421B97 as *mut u32) = 0x9000;
    *(0x423EBE as *mut u32) = 0xEA003D83;   // -> cmp [0x4BEA00], 0x3
    *(0x423EC2 as *mut u32) = 0x4B;
    *(0x42E1D4 as *mut u32) = 0xEA001589;
    *(0x42E1D8 as *mut u32) = 0x4B;

    if memoryapi::VirtualProtect(
        0x401000 as *mut _,
        0x9E000 as _,
        oldp,
        &mut oldp as *mut _,
    ) != 0 {
        return Err("Failed to change memory protection.");
    }
    
    Ok(())
}

// 生ポインタの利用 *mut or *const
unsafe extern "stdcall" fn changedisplayname() -> bool {

    let mut addr = *((*(DPATH as *mut i32) + 0x192C) as *mut i32); // [[0x4B5B4C] + 0x64B * 4]
    let num_of_characters = *(((*(DPATH as *mut i32)) + 0xCD4) as *mut i32); // [[0x4B5B4C] + 0x335 * 4]

    let mut last_page: DWORD = 0;
    let names: Vec<&[u8]> = vec![b"Hello, UnderWorld!\0", b"\\(^o^)/\0", b"OXOXOXOXOXOXOXOXOXOXOXOXO\0", b" \0", b"OMFG! Miko!!!\0"];

    for _ in 0..num_of_characters {
        if *((addr + 0x4) as *mut i32) == 0x7473694D && *((addr + 0x8) as *mut i32) == 0x6E656B61 {

            // 書き換えを行うために権限変更
            if memoryapi::VirtualProtect(
                addr as *mut _,
                ((mem::size_of::<i32>() * 4) as u64).try_into().unwrap(),
                PAGE_READWRITE,
                &mut last_page
            ) == 0 {
                return false;
            }

            // Nameを書き換え
            let mut rng = rand::thread_rng();
            let name_index = rng.gen_range(0..(names.len() - 1));
            for i in 0..names[name_index].len() {
                *((addr + 0x4 + i as i32) as *mut u8) = names[name_index][i];
            }

            // 権限を元に戻す
            if memoryapi::VirtualProtect(
                addr as *mut _,
                ((mem::size_of::<i32>() * 4) as u64).try_into().unwrap(),
                last_page,
                &mut last_page
            ) == 0 {
                return false;
            }
            
            return true;
        }
        addr += 0x438;
    }
    false
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