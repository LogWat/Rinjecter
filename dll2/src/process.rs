use winapi::{
    um::{
        winnt::{
            HANDLE, 
            PROCESS_ALL_ACCESS,
            THREAD_ALL_ACCESS,
        },
        tlhelp32, handleapi, psapi,
        tlhelp32::{
            PROCESSENTRY32W,
            THREADENTRY32,
            TH32CS_SNAPPROCESS,
            TH32CS_SNAPTHREAD,
        },
        processthreadsapi, errhandlingapi,
        handleapi::{INVALID_HANDLE_VALUE},
    },
    shared::minwindef::{
        DWORD,
        MAX_PATH,
    },
};

use crate::otherwinapi;

use std::{mem, ptr, str, ffi::OsString, os::windows::ffi::OsStringExt};


#[repr(C)]
pub struct Process {
    pub pid: u32,
    pub handle: HANDLE,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}


#[repr(C)]
pub struct Thread {
    pub handle: HANDLE,
    pub tid: u32,
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

    pub fn get_threadlist(&self) -> Result<Vec<Thread>, u32> {
        let mut threads: Vec<Thread> = Vec::new();
        let mut thread_entry: THREADENTRY32 = unsafe { mem::zeroed() };
        thread_entry.dwSize = mem::size_of::<THREADENTRY32>() as _;

        let thread_list = unsafe {
            tlhelp32::CreateToolhelp32Snapshot(
                TH32CS_SNAPTHREAD,
                self.pid
            )
        };
        if thread_list == INVALID_HANDLE_VALUE {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        while unsafe { tlhelp32::Thread32Next(thread_list, &mut thread_entry) } != 0 {
            if thread_entry.th32OwnerProcessID == self.pid {
                let handle = unsafe { processthreadsapi::OpenThread(
                    THREAD_ALL_ACCESS,
                    0,
                    thread_entry.th32ThreadID
                )};
                if handle == ptr::null_mut() {
                    continue;
                }
                threads.push(Thread {
                    handle,
                    tid: thread_entry.th32ThreadID,
                });
            }
        }
        unsafe { handleapi::CloseHandle(thread_list) };

        Ok(threads)
    }
}

impl Thread {
    pub fn get_current_thread_id() -> Result<u32, u32> {
        let tid = unsafe { processthreadsapi::GetCurrentThreadId() };
        if tid == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(tid)
    }
}