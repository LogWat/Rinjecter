#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

mod processlib;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::{winnt::*, libloaderapi};
use winapi::shared::minwindef::*;

use processlib::Process;

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
                if changedisplayname(process) == false {
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

unsafe extern "stdcall" fn overwrite(process: Process) -> Result<(), &'static str> {
        
    // rewrite program
    Process::write(&process, 0x41DBD4, 0x4BEA00A1).unwrap();     // -> mov eax, [0x4BEA00] ([0x4BEA00] = 0x0)
    Process::write(&process, 0x41DBD8, 0x9000).unwrap();         // -> nop
    Process::write(&process, 0x41DF21, 0x4BEA00A1 as u32).unwrap();
    Process::write(&process, 0x41DF25, 0x9000 as u32).unwrap();
    Process::write(&process, 0x41F9E7, 0x4BEA00A1 as u32).unwrap();
    Process::write(&process, 0x41F9EB, 0x9000 as u32).unwrap();
    Process::write(&process, 0x41FC8D, 0xEA0005C7 as u32).unwrap();
    Process::write(&process, 0x41FC91, 0x4B as u32).unwrap();           // -> mov [0x4BEA00], 0x1
    Process::write(&process, 0x41DF76, 0xEA0005C7 as u32).unwrap();
    Process::write(&process, 0x41DF7A, 0x4B as u32).unwrap();
    Process::write(&process, 0x41FDF3, 0xEA0005C7 as u32).unwrap();
    Process::write(&process, 0x41FDF7, 0x4B as u32).unwrap();
    Process::write(&process, 0x41FF01, 0xEA001589 as u32).unwrap();
    Process::write(&process, 0x41FF05, 0x4B as u32).unwrap();           // -> mov [0x4BEA00], edx
    Process::write(&process, 0x42035E, 0xEA005C7 as u32).unwrap();
    Process::write(&process, 0x420362, 0x4B as u32).unwrap();
    Process::write(&process, 0x420399, 0x4BEA00A1 as u32).unwrap();
    Process::write(&process, 0x4203A3, 0x9000 as u32).unwrap();
    Process::write(&process, 0x421B93, 0x4BEA00A1 as u32).unwrap();
    Process::write(&process, 0x421B97, 0x9000 as u32).unwrap();
    Process::write(&process, 0x423EBE, 0xEA003D83 as u32).unwrap();     // -> cmp [0x4BEA00], 0x3
    Process::write(&process, 0x423EC2, 0x4B as u32).unwrap();
    Process::write(&process, 0x42E1D4, 0xEA001589 as u32).unwrap();
    Process::write(&process, 0x42E1D8, 0x4B as u32).unwrap();
    Process::write(&process, 0x42E8CA, 0xEA000D8B as u32).unwrap();
    Process::write(&process, 0x42E8CE, 0x4B as u32).unwrap();

    Ok(())
}

// 生ポインタの利用 *mut or *const
unsafe extern "stdcall" fn changedisplayname(process: Process) -> bool {

    let mut addr = *((*(DPATH as *mut i32) + 0x192C) as *mut i32); // [[0x4B5B4C] + 0x64B * 4]
    let num_of_characters = *(((*(DPATH as *mut i32)) + 0xCD4) as *mut i32); // [[0x4B5B4C] + 0x335 * 4]

    let names: Vec<&[u8]> = vec![b"Hello, UnderWorld!\0", b"\\(^o^)/\0", b"OXOXOXOXOXOXOXOXOXOXOXOXO\0", b" \0", b"OMFG! Miko!!!\0"];

    for _ in 0..num_of_characters {
        if *((addr + 0x4) as *mut i32) == 0x7473694D && *((addr + 0x8) as *mut i32) == 0x6E656B61 {

            // Nameを書き換え
            let mut rng = rand::thread_rng();
            let name_index = rng.gen_range(0..(names.len() - 1));

            for i in 0..names[name_index].len() {
                match Process::write(&process, (addr + 0x4 + i as i32) as u32, names[name_index][i]) {
                    Ok(_) => {},
                    Err(_) => {
                        return false;
                    }
                }
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