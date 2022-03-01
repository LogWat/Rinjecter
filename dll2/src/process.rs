use winapi::{
    um::{
        winnt::{
            HANDLE, 
            PROCESS_ALL_ACCESS,
        },
        tlhelp32, handleapi, psapi,
        tlhelp32::{
            PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
    },
    shared::minwindef::{
        DWORD,
        MAX_PATH,
    },
};
use winapi::um::{processthreadsapi, errhandlingapi};

use crate::otherwinapi;

use std::{mem, ptr, str, ffi::OsString, os::windows::ffi::OsStringExt};


#[repr(C)]
pub struct Process {
    pub pid: u32,
    pub handle: HANDLE,
}

impl Process {
    pub fn empty() -> Self {
        Process {
            pid: 0,
            handle: ptr::null_mut(),
        }
    }

    pub fn open_process(&mut self) -> Result<(), u32> {
        let handle = unsafe { 
            processthreadsapi::OpenProcess(
                PROCESS_ALL_ACCESS,
                0,
                self.pid
            )
        };
        if handle == ptr::null_mut() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        self.handle = handle;

        Ok(())
    }

    pub fn get_process_from_name(&mut self, name: &str) -> Result<Process, u32> {
        let processes: Vec<Process> = match Process::enumerate_process() {
            Ok(processes) => processes,
            Err(err) => return Err(err),
        };

        if processes.len() == 0 {
            return Err(0);
        }

        for process in processes {
            if process.name().contains(name) {
                self.pid = process.pid;
                self.handle = process.handle;
                return Ok(process);
            }
        }

        Err(0)
    }


    fn name(&self) -> String {
        let mut name = [0u16; MAX_PATH];
        unsafe {
            psapi::GetProcessImageFileNameW(
                self.handle,
                name.as_mut_ptr(),
                MAX_PATH as _,
            );
        }

        OsString::from_wide(&name[..]).into_string().unwrap()
    }


    fn enumerate_process() -> Result<Vec<Process>, u32> {
        let mut processes: Vec<Process> = Vec::new();
        let mut process_entry: PROCESSENTRY32W = unsafe { mem::zeroed() };
        process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;
    
        let snapshot = unsafe { tlhelp32::CreateToolhelp32Snapshot(
            TH32CS_SNAPPROCESS,
            0
        ) };
        if snapshot == ptr::null_mut() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
    
        let mut success = unsafe { tlhelp32::Process32FirstW(snapshot, &mut process_entry) };
        while success != 0 {
            let mut process = Process {
                pid: process_entry.th32ProcessID,
                handle: ptr::null_mut(),
            };
            match process.open_process() {
                Ok(_) => processes.push(process),
                Err(_) => {}
            }
            success = unsafe { tlhelp32::Process32NextW(snapshot, &mut process_entry) };
        }
    
        unsafe { handleapi::CloseHandle(snapshot) };
    
        Ok(processes)
    }
}