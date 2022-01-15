use getset::Getters;
use std::{mem};
use winapi::um::{handleapi, memoryapi, processthreadsapi, tlhelp32, winnt};

#[derive(Getters)]
#[get = "pub"]
pub struct Process {
    pub pid: u32,
    pub handle: winnt::HANDLE,
}

#[derive(Getters)]
#[get = "pub"]
pub struct Module {
    pub base_address: u32,
    pub size: u32,
}


impl Process {
    pub fn current_process() -> Self {
        let mut process = Self {
            pid: 0,
            handle: unsafe { handleapi::GetCurrentProcess() },
        };
        process.pid = unsafe { processthreadsapi::GetProcessId(process.handle) };
        process
    }
}