#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::{winnt::*, processthreadsapi, memoryapi, libloaderapi};
use winapi::shared::minwindef::*;

use std::{mem};
use failure::Error;
use std::convert::TryInto;
use rand::Rng;
use getset::Getters;

const DPATH: u32 = 0x4B5B4C;

#[derive(Getters)]
#[get = "pub"]
pub struct Process {
    handle: HANDLE,
    pid: DWORD,
}

impl Process {
    pub fn current_process() -> Self {
        let handle = unsafe { processthreadsapi::GetCurrentProcess() };
        let pid = unsafe { processthreadsapi::GetProcessId(handle) };
        Process { handle, pid }
    }

    pub fn read_memory(&self, address:u32) -> Result<u32> {
        let mut buffer: u32 = 0;
        let mut bytes_read: libc::size_t = 0;
        let result = unsafe { 
            memoryapi::ReadProcessMemory(
                self.handle, 
                address as *const _, 
                &mut buffer as *mut _ as *mut _,
                mem::size_of::<u32>(),
                &mut bytes_read as *mut _,
            )
        };

        if result == 0 || bytes_read != 0 {
        }

        Ok(buffer)
    }
}

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