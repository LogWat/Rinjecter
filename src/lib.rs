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

    let rb1: [u32; 14] = [
        0x4BEA00A1, 0xEA0005C7, 0xEA001589, 0xEA003D83, 
        0xEA000D8B, 0xEA00158B, 0xEA043589, 0xEA0405C7,
        0xEA04358B, 0xEA04158B, 0x4BEA04A1, 0xEA040D8B,
        0xEA041589, 0xEA083D83
        ];
    let rb2: [u32; 2] = [0x9000, 0x4B];
        
    // rewrite program
    Process::write(&process, 0x41DBD4, rb1[0]).unwrap();     // -> mov eax, [0x4BEA00] ([0x4BEA00] = 0x0)
    Process::write(&process, 0x41DBD8, rb2[0]).unwrap();     // -> nop
    Process::write(&process, 0x41DF21, rb1[0]).unwrap();
    Process::write(&process, 0x41DF25, rb2[0]).unwrap();
    Process::write(&process, 0x41F9E7, rb1[0]).unwrap();
    Process::write(&process, 0x41F9EB, rb2[0]).unwrap();
    Process::write(&process, 0x41FC8D, rb1[1]).unwrap();
    Process::write(&process, 0x41FC91, rb2[1]).unwrap();     // -> mov [0x4BEA00], 0x1
    Process::write(&process, 0x41DF76, rb1[1]).unwrap();
    Process::write(&process, 0x41DF7A, rb2[1]).unwrap();
    Process::write(&process, 0x41FDF3, rb1[1]).unwrap();
    Process::write(&process, 0x41FDF7, rb2[1]).unwrap();
    Process::write(&process, 0x41FF01, rb1[2]).unwrap();
    Process::write(&process, 0x41FF05, rb2[1]).unwrap();     // -> mov [0x4BEA00], edx
    Process::write(&process, 0x42035E, rb1[2]).unwrap();
    Process::write(&process, 0x420362, rb2[1]).unwrap();
    Process::write(&process, 0x420399, rb1[0]).unwrap();
    Process::write(&process, 0x4203A3, rb2[0]).unwrap();
    Process::write(&process, 0x421B93, rb1[0]).unwrap();
    Process::write(&process, 0x421B97, rb2[0]).unwrap();
    Process::write(&process, 0x423EBE, rb1[3]).unwrap();     // -> cmp [0x4BEA00], 0x3
    Process::write(&process, 0x423EC2, rb2[1]).unwrap();
    Process::write(&process, 0x42E1D4, rb1[2]).unwrap();
    Process::write(&process, 0x42E1D8, rb2[1]).unwrap();
    Process::write(&process, 0x42E8CA, rb1[4]).unwrap();     // -> mov ecx, [0x4BEA00]
    Process::write(&process, 0x42E8CE, rb2[1]).unwrap();
    Process::write(&process, 0x434A58, rb1[3]).unwrap();
    Process::write(&process, 0x434A5C, rb2[1]).unwrap();
    Process::write(&process, 0x43A762, rb1[3]).unwrap();
    Process::write(&process, 0x43A766, rb2[1]).unwrap();
    Process::write(&process, 0x440BF7, rb1[0]).unwrap();
    Process::write(&process, 0x440BFB, rb2[0]).unwrap();
    Process::write(&process, 0x440CAB, rb1[3]).unwrap();
    Process::write(&process, 0x440CB1, rb2[1]).unwrap();
    Process::write(&process, 0x440D95, rb1[0]).unwrap();
    Process::write(&process, 0x440D99, rb2[0]).unwrap();
    Process::write(&process, 0x441274, rb1[3]).unwrap();
    Process::write(&process, 0x441278, rb2[1]).unwrap();
    Process::write(&process, 0x47BF1D, rb1[5]).unwrap();    // -> mov edx, [0x4BEA00]
    Process::write(&process, 0x47BF21, rb2[1]).unwrap();
    Process::write(&process, 0x41F8CE, rb1[7]).unwrap();    // -> mov [0x4BEA00], 0x0
    Process::write(&process, 0x41F8D2, rb2[1]).unwrap();
    Process::write(&process, 0x41F8D6, 0x0 as u32).unwrap();
    Process::write(&process, 0x41F8ED, rb1[6]).unwrap();    // -> mov [0x4BEA00], esi
    Process::write(&process, 0x41F8F1, rb2[1]).unwrap();
    Process::write(&process, 0x41F90D, rb1[8]).unwrap();    // -> mov [0x4BEA00], eax
    Process::write(&process, 0x41F911, rb2[0]).unwrap();    // -> nop
    Process::write(&process, 0x41F9BD, rb1[7]).unwrap();
    Process::write(&process, 0x41F9C1, rb2[1]).unwrap();
    Process::write(&process, 0x41F9C5, 0x0 as u32).unwrap();
    Process::write(&process, 0x4204A8, rb1[9]).unwrap();    // -> mov edx, [0x4BEA00]
    Process::write(&process, 0x4204AC, rb2[1]).unwrap();
    Process::write(&process, 0x420518, rb1[10]).unwrap();   // -> mov eax, [0x4BEA00]
    Process::write(&process, 0x420524, rb2[0]).unwrap();    // -> nop
    Process::write(&process, 0x420535, rb1[11]).unwrap();   // -> mov ecx, [0x4BEA00]
    Process::write(&process, 0x420539, rb2[1]).unwrap();
    Process::write(&process, 0x42055F, rb1[8]).unwrap();    // -> mov esi, [0x4BEA00]
    Process::write(&process, 0x420563, rb2[1]).unwrap();
    Process::write(&process, 0x42DAB7, rb1[9]).unwrap();    // -> mov eax, [0x4BEA00]
    Process::write(&process, 0x42DABB, rb2[1]).unwrap();
    Process::write(&process, 0x42E1DF, rb1[12]).unwrap();   // -> mov [0x4BEA00], edx
    Process::write(&process, 0x42E1E3, rb2[1]).unwrap();
    Process::write(&process, 0x42E8B5, rb1[10]).unwrap();
    Process::write(&process, 0x42E8B9, rb2[0]).unwrap();
    Process::write(&process, 0x42E90D, rb1[11]).unwrap();
    Process::write(&process, 0x42E911, rb2[1]).unwrap();
    Process::write(&process, 0x41DD98, rb1[13]).unwrap();   // -> cmp [0x4BEA00], 0x3
    Process::write(&process, 0x41DD9C, rb2[1]).unwrap();
    Process::write(&process, 0x41DD9C + 0x2, 0x3).unwrap();


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