#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

mod processlib;
mod overwrite;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::{winnt::*, libloaderapi};
use winapi::shared::minwindef::*;

use processlib::Process;
use overwrite::{OverWrite, AddrSize};

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
                let process = Process::current_process();
                match overwrite::OverWrite(&process) {
                    Ok(_) => {},
                    Err(e) => {
                        let msg = format!("Failed to overwrite.\n{}", e);
                        err_msgbox(msg);
                    }
                };
                if changedisplayname(&process) == false {
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

// 生ポインタの利用 *mut or *const
unsafe extern "stdcall" fn changedisplayname(process: &Process) -> bool {

    let mut addr = *((*(DPATH as *mut i32) + 0x192C) as *mut i32);
    let num_of_characters = *(((*(DPATH as *mut i32)) + 0xCD4) as *mut i32);

    let names: Vec<&[u8]> = vec![
        b"Hello, World!\0", 
        b"\\(^o^)/\0", 
        b"OXOXOXOXOXOXOXOXOXOXOXOXO\0", 
        b"OMFG! Miko!!!\0",
        b":D\0",
        b":P\0",
        b"\0",
        b"42\0",
        b"<!> MISTAKEN <!>\0",
    ];

    for _ in 0..num_of_characters {
        if *((addr + 0x4) as *mut i32) == 0x7473694D && *((addr + 0x8) as *mut i32) == 0x6E656B61 {

            // Nameを書き換え
            let mut rng = rand::thread_rng();
            let name_index = rng.gen_range(0..(names.len()));
            let mut byte_list: Vec<OverWrite> = Vec::new();

            for i in 0..names[name_index].len() {
                byte_list.push(
                    OverWrite {
                        addr: (addr + 0x4 + i as i32) as u32,
                        byte: AddrSize::Byte(names[name_index][i]),
                    }
                );
            }

            match overwrite::overwrite_process_list(&byte_list, process) {
                Ok(_) => {},
                Err(_e) => { return false; }
            };

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