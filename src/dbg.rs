use crate::processlib::{Process, Module, Thread};
use crate::ffi_helpers;
use winapi::um::{winnt, processthreadsapi, winbase};
use std::{mem};

struct Debugger {
    pub process: Process,
    pub token: winnt::HANDLE,
    pub luid: winnt::LUID
}

impl Debugger {
    pub fn new() -> Result<Self, &'static str> {
        let process = Process::current_process();

        let mut token: winnt::HANDLE = std::ptr::null_mut();
        if unsafe {
            processthreadsapi::OpenProcessToken(
                process.handle,
                winnt::TOKEN_ALL_ACCESS,
                &mut token
            )
        } == 0 {
            return Err("Failed to open process token.");
        }

        let mut luid: winnt::LUID = unsafe { mem::zeroed() };
        let privilege = ffi_helpers::win32_to_utf16("seDebugPrivilege");
        if unsafe {
            winbase::LookupPrivilegeValueW(
                0 as *mut _,
                privilege.as_ptr(),
                &mut luid
            )
        } == 0 {
            return Err("Failed to lookup privilege value.");
        }

        Ok(Self { process, token, luid })
    }
}