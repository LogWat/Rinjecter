use crate::processlib::{Process};
use crate::ffi_helpers;
use winapi::um::{winnt, processthreadsapi, winbase, securitybaseapi, debugapi, errhandlingapi};
use winapi::shared::minwindef::{DWORD};
use std::{mem};

pub struct Debugger {
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

        Ok(())
    }

    pub fn set_privilege(&self) -> Result<(), &'static str> {
        let luid_and_attributes = winnt::LUID_AND_ATTRIBUTES {
            Luid: self.luid,
            Attributes: winnt::SE_PRIVILEGE_ENABLED,
        };
        let mut token_privileges: winnt::TOKEN_PRIVILEGES = unsafe { mem::zeroed() };
        token_privileges.PrivilegeCount = 1;
        token_privileges.Privileges[0] = luid_and_attributes;
        if unsafe {
            securitybaseapi::AdjustTokenPrivileges(
                self.token,
                0,
                &mut token_privileges,
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