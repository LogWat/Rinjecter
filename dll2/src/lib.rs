#![allow(non_snake_case)]
extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::um::{libloaderapi, processthreadsapi, debugapi};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

mod process;
mod ffi_helpers;
mod dbg;
mod threads;
mod otherwinapi;

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

            /*
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
                let msg = "test\0";
                let title = "test\0";
                otherwinapi::MsgBox(&msg, &title);
            });
            
            let th0 = tokio::spawn(async move {
                match threads::wait_debugevnet(process.clone()).await {
                    Ok(_) => {},
                    Err(err) => {
                        let msg = format!("Failed to wait debugevnet: {}\0", err);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, title);
                    }
                }
            });
            handles.push(th0);
            */

            if unsafe { debugapi::IsDebuggerPresent() } != 0 {
                let msg = "OMFG! You are debugging this Process!\0";
                let title = "ERROR\0";
                otherwinapi::MsgBox(&msg, title);
            }

            unsafe {
                processthreadsapi::CreateThread(
                    0 as *mut _,
                    0,
                    Some(threads::thread_entry),
                    0 as *mut _,
                    0,
                    0 as *mut _
                );
            }

            return true as i32;
        },
        DLL_PROCESS_DETACH => {
            true as i32
        },
        _ => true as i32,
    }
}