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

    pub fn read_memory(&self, address: u32) -> Result<u32, &'static str> {
        let mut buffer: Vec<u8> = vec![0; 0x1000];
        let mut bytes_read: u32 = 0;
        unsafe {
            if memoryapi::ReadProcessMemory(
                self.handle,
                address as *mut _,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                &mut bytes_read,
            ) != (true as _) || bytes_read == 0 {
                mem::forget(buffer);
                return Err("Failed to read memory.");
            }
        }
        Ok(buffer)
    }
}