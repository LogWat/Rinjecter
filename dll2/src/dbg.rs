use winapi::{
    um::{
        winnt::{
            HANDLE, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, TOKEN_PRIVILEGES, SE_PRIVILEGE_ENABLED,
            PROCESS_ALL_ACCESS,
        },
        processthreadsapi, winbase, securitybaseapi, errhandlingapi, debugapi,
    },
};

use std::{mem, ptr};

use crate::process::Process;
use crate::ffi_helpers;

#[repr(C)]
pub struct Debugger {
    pub process: Process,
    pub isDebuggerAttached: bool,
}

impl Debugger {
    // Constructor ( return empty Process )
    pub fn new(pid: u32) -> Result<Self, u32> {
        let handle = unsafe { processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, 0, pid) };
        if handle == ptr::null_mut() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(Debugger {
            process: Process {
                handle,
                pid,
            },
            isDebuggerAttached: false,
        })
    }

    pub fn attach(&mut self) -> Result<(), u32> {
        if unsafe {
            debugapi::DebugActiveProcess(self.process.pid)
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        self.isDebuggerAttached = true;

        Ok(())
    }

    pub fn set_privilege(&self) -> Result<(), u32> {

        let mut token: HANDLE = std::ptr::null_mut();
        // [!] this process handle is "Self" not "Target Process"
        if unsafe {
            processthreadsapi::OpenProcessToken(
                processthreadsapi::GetCurrentProcess(),
                TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                &mut token
            )
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        if token == std::ptr::null_mut() {
            return Err(unsafe { errhandlingapi::GetLastError() });
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
            return Err(unsafe { errhandlingapi::GetLastError() });
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
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(())
    }
}