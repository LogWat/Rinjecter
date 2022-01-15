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
            handle: unsafe { processthreadsapi::GetCurrentProcess() },
        };
        process.pid = unsafe { processthreadsapi::GetProcessId(process.handle) };
        process
    }

    pub fn read_memory(&self, address: u32) -> Result<u32, &'static str> {
        let mut buffer = unsafe { mem::zeroed() }; 
        let mut bytes_read: libc::size_t = 0;
        unsafe {
            if memoryapi::ReadProcessMemory(
                self.handle,
                address as *mut _,
                &mut buffer as *mut _ as *mut _,
                mem::size_of::<u32>() as _,
                &mut bytes_read as *mut _,
            ) != (true as _) || bytes_read == 0 {
                mem::forget(buffer);
                return Err("Failed to read memory.");
            }
        }

        Ok(buffer)
    }

    pub fn get_module(&self, module_name: &str) -> Result<Module, &'static str> {
        let module = Module {
            unsafe { tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPMODULE, self.pid) },
        };
        if module == handleapi::INVALID_HANDLE_VALUE {
            return Err("Failed to create snapshot.");
        }

        let mut module_entry: htlhelp32::MODULEENTRY32 = unsafe { mem::zeroed() };
    }
}