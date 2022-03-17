#![allow(non_snake_case)]
extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::um::{libloaderapi, debugapi};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use std::{ptr, thread};

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

            if unsafe { debugapi::IsDebuggerPresent() } != 0 {
                let msg = "OMFG! You are debugging this Process!\0";
                let title = "ERROR\0";
                otherwinapi::MsgBox(&msg, title);
            }

            thread::spawn(move || {
                threads::thread_entry(ptr::null_mut());
            });

            return true as i32;
        },
        DLL_PROCESS_DETACH => {
            true as i32
        },
        _ => true as i32,
    }
}