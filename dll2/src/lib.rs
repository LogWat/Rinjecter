#![allow(non_snake_case)]
extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::um::{libloaderapi};


mod process;
mod ffi_helpers;
mod dbg;
mod threads;
mod otherwinapi;

use process::Process;

#[no_mangle]
pub extern "stdcall" fn DllMain(
    _hinst_dll: HINSTANCE, 
    reason: DWORD,
    _: LPVOID
) -> i32 {
    match reason {
        _DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(_hinst_dll);
            }

            let msg = "[!] DLL_PROCESS_ATTACH\0";
            let title = "INFO\0";
            unsafe { otherwinapi::MsgBox(msg, title); }

            let mut target_process = match find_target_process("mugen.exe\0") {
                Ok(process) => process,
                Err(err) => {
                    let msg = format!("Failed to find target process: {}\0", err);
                    let title = "ERROR\0";
                    unsafe { otherwinapi::MsgBox(&msg, title); }
                    return 0;
                }
            };


            return true as i32;
        },
        _DLL_PROCESS_DETACH => {
            true as i32
        },
        _ => true as i32,
    }
}

fn find_target_process(name: &str) -> Result<Process, u32> {
    let mut process = Process::empty();
    match process.get_process_from_name(name) {
        Ok(process) => Ok(process),
        Err(err) => Err(err),
    }
}