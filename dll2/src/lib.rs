#![allow(non_snake_case)]
extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::um::{libloaderapi};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use std::thread;
use std::sync::{Arc, Mutex};

mod process;
mod ffi_helpers;
mod dbg;
mod threads;
mod otherwinapi;

use process::Process;

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
            }

            let target_process = match find_target_process("mugen.exe") {
                Ok(process) => process,
                Err(err) => {
                    let msg = format!("Failed to find target process: {}\0", err);
                    let title = "ERROR\0";
                    otherwinapi::MsgBox(&msg, title);
                    return 0;
                }
            };

            let process = Arc::new(Mutex::new(target_process));
            let mut handles = vec![];

            let th0 = thread::spawn(move || {
                match threads::wait_debugevnet(process.clone()) {
                    Ok(_) => {},
                    Err(err) => {
                        let msg = format!("Failed to wait debugevnet: {}\0", err);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, title);
                    }
                }
            });

            handles.push(&th0);
            th0.join().unwrap();

            return true as i32;
        },
        DLL_PROCESS_DETACH => {
            true as i32
        },
        _ => true as i32,
    }
}

fn find_target_process(name: &str) -> Result<Process, u32> {
    let mut process = Process::empty();
    match process.get_process_from_name(name) {
        Ok(process) => Ok(process),
        Err(e) => Err(e),
    }
}