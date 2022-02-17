use crate::processlib::{Process};
use crate::ffi_helpers;
use winapi::um::{winnt, processthreadsapi, winbase, securitybaseapi, debugapi, errhandlingapi};
use winapi::shared::minwindef::{DWORD};
use std::{mem};

pub struct Debugger {
    pub process: Process,
    pub isDebuggerAttached: bool,
}

impl Debugger {
    // Constructor
    pub fn new() -> Result<Debugger, String> {
        let process = Process::current_process();
        Ok(Debugger {
            process,
            isDebuggerAttached: false,
        })
    }

    pub fn set_privilege(&self) -> Result<(), &'static str> {

        let mut token: winnt::HANDLE = std::ptr::null_mut();
        if unsafe {
            processthreadsapi::OpenProcessToken(
                self.process.handle,
                winnt::TOKEN_ADJUST_PRIVILEGES | winnt::TOKEN_QUERY,
                &mut token
            )
        } == 0 {
            return Err("Failed to open process token.");
        }

        if token == std::ptr::null_mut() {
            return Err("Failed to open process token.");
        }

        let mut tkp: winnt::TOKEN_PRIVILEGES = unsafe { mem::zeroed() };
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
        tkp.Privileges[0].Attributes = winnt::SE_PRIVILEGE_ENABLED;

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

    pub fn attach(&mut self) -> Result<(), DWORD> {
        self.process.handle = unsafe { 
            processthreadsapi::OpenProcess(
                winnt::PROCESS_ALL_ACCESS,
                0,
                self.process.pid
            ) 
        };
        if self.process.handle == std::ptr::null_mut() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        if unsafe {
            debugapi::DebugActiveProcess(self.process.pid)
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        self.isDebuggerAttached = true;

        Ok(())
    }
}