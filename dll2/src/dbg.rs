use winapi::{
    um::{
        winnt::{
            HANDLE, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, TOKEN_PRIVILEGES, SE_PRIVILEGE_ENABLED
        },
        processthreadsapi, winbase, securitybaseapi,
    },
    shared::minwindef::{
        DWORD,
    },
};

use std::{mem};
use crate::process::Process;
use crate::ffi_helpers;

#[repr(C)]
pub struct Debugger {
    pub process: Process,
    pub isDebuggerAttached: bool,
}

impl Debugger {
    // Constructor ( return empty Process )
    pub fn new() -> Debugger {
        Debugger {
            process: Process::empty(),
            isDebuggerAttached: false,
        }
    }

    pub fn set_privilege(&self) -> Result<(), &'static str> {

        let mut token: HANDLE = std::ptr::null_mut();
        if unsafe {
            processthreadsapi::OpenProcessToken(
                self.process.handle,
                TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                &mut token
            )
        } == 0 {
            return Err("Failed to open process token.");
        }

        if token == std::ptr::null_mut() {
            return Err("Failed to open process token.");
        }

        let mut tkp: TOKEN_PRIVILEGES = unsafe { mem::zeroed() };
        let privilege = ffi_helpers::win32_to_utf16("seDebugPrivilege");
        if unsafe {
            winbase::LookupPrivilegeValueW(
                0 as *mut _,
                privilege.as_ptr(),
                &mut tkp.Privileges[0].Luid
            )
        } == 0 {
            return Err("Failed to lookup privilege value.");
        }

        tkp.PrivilegeCount = 1;
        tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

        if unsafe {
            securitybaseapi::AdjustTokenPrivileges(
                token,
                0,
                &mut tkp,
                0,
                0 as *mut _,
                0 as *mut _
            )
        } == 0 {
            return Err("Failed to adjust token privileges.");
        }

        Ok(())
    }
}