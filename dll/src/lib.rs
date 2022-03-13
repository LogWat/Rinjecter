#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

mod processlib;
mod overwrite;
mod threadpool;
mod ffi_helpers;
mod otherwinapi;

use winapi::um::{winnt::*, libloaderapi, processthreadsapi, errhandlingapi, debugapi};
use winapi::um::winbase::{CREATE_NEW_PROCESS_GROUP, CREATE_SUSPENDED};
use winapi::shared::minwindef::*;

use processlib::{Process};
use overwrite::{OverWrite, AddrSize};

use rand::Rng;
use std::ptr;

const DPATH: u32 = 0x4B5B4C;

#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE, 
    reason: DWORD,
    _: LPVOID
) -> i32 {
    let mut child_process_handle: HANDLE = ptr::null_mut();
    let mut remote_thread_handle: HANDLE = ptr::null_mut();

    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(hinst_dll);

                if debugapi::IsDebuggerPresent() != 0 {
                    let msg = "OMFG! You are debugging this Process!\0";
                    let title = "WOW!\0";
                    otherwinapi::MsgBox(&msg, &title);
                }

                let process = Process::current_process();

                // create process
                let process_handle = match otherwinapi::CreateProcess(
                    "C:\\Windows\\System32\\calc.exe",
                    "",
                    false,
                    CREATE_NEW_PROCESS_GROUP | CREATE_SUSPENDED,
                    0x1
                ) {
                    Ok(h) => h,
                    Err(e) => {
                        let msg = format!("[!] Failed to create process.\nError Code: {}\0", e);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, &title);
                        return 0x1;
                    }
                };
                child_process_handle = process_handle;
                let mut target_process: Process = Process::from_handle(process_handle);

                // get self module
                let self_modules = match Process::get_module_from_path(&process, "Mistaken") {
                    Ok(m) => m,
                    Err(e) => {
                        let msg = format!("[!] Failed to get self module.\nError Code: {}\0", e);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, &title);
                        return 0x1;
                    }
                };
                let mut dll2_path: String = String::new();
                for module in self_modules {
                    let len = module.path.len();
                    dll2_path = module.path.clone().into_string().unwrap();
                    if !dll2_path.contains("Mistaken\\.dll") {
                        dll2_path.truncate(len);
                        continue;
                    }
                    // replace .dll with 2.dll
                    dll2_path = dll2_path.replace(".dll", "2.dll");
                }
                if dll2_path.is_empty() {
                    let msg = format!("[!] Failed to get self module.\0");
                    let title = "ERROR\0";
                    otherwinapi::MsgBox(&msg, &title);
                    return 0x1;
                }

                // inject dll into calc.exe
                remote_thread_handle = match dll_inject(&mut target_process, &dll2_path) {
                    Ok(h) => h,
                    Err(e) => {
                        let msg = format!("[!] Failed to inject dll.\nError: {}\0", e);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, &title);
                        return 0x1;
                    }
                };
                
                /*
                processthreadsapi::CreateThread(
                    0 as *mut _,
                    0,
                    Some(threadpool::Thread_Checker),
                    0 as *mut _,
                    0,
                    0 as *mut _
                );
                */

                match overwrite::OverWrite(&process) {
                    Ok(_) => {},
                    Err(e) => {
                        let msg = format!("Failed to overwrite.\n{}", e);
                        let title = "ERROR!\0";
                        otherwinapi::MsgBox(&msg, &title);
                    }
                };
                if changedisplayname(&process) == false {
                    let msg = "Failed to change display name.\0".to_string();
                    let title = "ERROR!\0";
                        otherwinapi::MsgBox(&msg, &title);
                }
            }
            return true as i32;
        },
        DLL_PROCESS_DETACH => {
            unsafe {
                if remote_thread_handle != ptr::null_mut() {
                    processthreadsapi::ResumeThread(remote_thread_handle);
                }
                if child_process_handle != ptr::null_mut() {
                    processthreadsapi::ResumeThread(child_process_handle);
                    processthreadsapi::TerminateProcess(child_process_handle, 0);
                }
            }
            return true as i32;
        },
        _ => true as i32,
    }
}


fn dll_inject(process: &mut Process, dll_path: &str) -> Result<HANDLE, String> {
    process.handle = unsafe {
        processthreadsapi::OpenProcess(
            PROCESS_ALL_ACCESS,
            0,
            process.pid
        )
    };
    if process.handle == ptr::null_mut() {
        return Err(format!("Failed to open process. Error Code: {}\0", unsafe { errhandlingapi::GetLastError() }));
    }

    let arg_address = match Process::allocate_memory(process, dll_path.len() as u32) {
        Ok(a) => a,
        Err(e) => {
            return Err(format!("Failed to allocate memory. Error Code: {}\0", e));
        }
    };

    match Process::write_memory(process, arg_address, dll_path) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("Failed to write memory. Error Code: {}\0", e));
        }
    };

    let h_kernel32 = unsafe {
        libloaderapi::GetModuleHandleA(b"Kernel32.dll\0".as_ptr() as *const _)
    };
    if h_kernel32 == ptr::null_mut() {
        return Err(format!("Failed to get kernel32.dll handle. Error Code: {}\0", unsafe { errhandlingapi::GetLastError() }));
    }

    let h_loadlib = unsafe {
        libloaderapi::GetProcAddress(h_kernel32, b"LoadLibraryA\0".as_ptr() as *const _)
    };
    if h_loadlib == ptr::null_mut() {
        return Err(format!("Failed to get LoadLibraryA address. Error Code: {}\0", unsafe { errhandlingapi::GetLastError() }));
    }

    let h_thread = match otherwinapi::CreateRemoteThread(
        process.handle,
        h_loadlib as u32,
        arg_address,
    ) {
        Ok(h) => h,
        Err(e) => {
            return Err(format!("Failed to create remote thread. Error Code: {}\0", e));
        }
    };

    return Ok(h_thread);
}

// 生ポインタの利用 *mut or *const
unsafe extern "stdcall" fn changedisplayname(process: &Process) -> bool {

    let mut addr = *((*(DPATH as *mut i32) + 0x192C) as *mut i32);
    let num_of_characters = *(((*(DPATH as *mut i32)) + 0xCD4) as *mut i32);

    let names: Vec<&[u8]> = vec![
        b"]pyyz95Bzgyq4\0", 
        b"I=KzK<:\0", 
        b"GQwrrpg\0", 
        b"ZXSR45X|~z444\0",
        b"/Q\0",
        b"/E\0",
        b"\0",
        b"!'!'!'!'!'!'!'!'!'!'!'!'!'\0",
        b")4+5X\\FAT^P[5)4+\0",
        b"A}pl2gp5tyy5vgtol95t{q5lz`5a}|{~5lz`2gp5a}p5z{yl5z{p5b}z2f5{za*\0",
        b"XF5qzv`xp{ata|z{5|f5gptyyl5f}|aal95qz{2a5lz`5a}|{~*\0",
        b"\\5q|q{2a5~{zb5|a5btf5ezff|wyp5az5fatga5t5{pb5y|{p5|{5t5{txp\\2cp5{pcpg5}ptgq5zs5|a5wpszgp\\5btf5fz5f`geg|fpq4W`a5a}p{5trt|{95\\5qz{2a5gptyyl5}tcp5az5bg|ap5z{5x`ya|eyp5y|{pfFz5\\2yy5`fa5bg|ap5qzb{5azqtl2f5q|{{pgBtapg95x|fz5fz`e95vg|v~paf;;;\0",
    ];

    for _ in 0..num_of_characters {
        if *((addr + 0x4) as *mut i32) == 0x7473694D && *((addr + 0x8) as *mut i32) == 0x6E656B61 {

            // Nameを書き換え
            let mut rng = rand::thread_rng();
            let name_index = rng.gen_range(0..(names.len()));
            let mut byte_list: Vec<OverWrite> = Vec::new();

            for i in 0..names[name_index].len() {
                let mut d_byte: u8 = '\0' as u8;
                if names[name_index][i] != '\0' as u8 {
                    d_byte = names[name_index][i] ^ 21;
                }
                byte_list.push(
                    OverWrite {
                        addr: (addr + 0x4 + i as i32) as u32,
                        byte: AddrSize::Byte(d_byte),
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