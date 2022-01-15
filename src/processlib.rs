use getset::Getters;
use std::{mem, ffi::OsString, os::windows::ffi::OsStringExt};
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

#[derive(Getters)]
#[get = "pub"]
pub struct Thread {
    pub handle: winnt::HANDLE,
    pub tid: u32,
}

impl Process {
    #[no_mangle]
    pub fn current_process() -> Self {
        let mut process = Self {
            pid: 0,
            handle: unsafe { processthreadsapi::GetCurrentProcess() },
        };
        process.pid = unsafe { processthreadsapi::GetProcessId(process.handle) };
        process
    }

    #[no_mangle]
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

    #[no_mangle]
    pub fn get_module(&self, module_name: &str) -> Result<Module, &'static str> {
        let module = unsafe { 
            tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPMODULE, self.pid) 
        };
        if module == handleapi::INVALID_HANDLE_VALUE {
            return Err("Failed to create snapshot.");
        }

        let mut module_entry: tlhelp32::MODULEENTRY32W = unsafe { mem::zeroed() };
        module_entry.dwSize = mem::size_of::<tlhelp32::MODULEENTRY32>() as _;

        while unsafe { tlhelp32::Module32NextW(module, &mut module_entry) } != 0 {
            let name = OsString::from_wide(&module_entry.szModule[..]).into_string();
            let name = match name {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Failed to convert OsString to String.");
                    continue;
                },
            };
            if name.contains(module_name) {
                unsafe { handleapi::CloseHandle(module) };
                return Ok(Module {
                    base_address: module_entry.modBaseAddr as _,
                    size: module_entry.modBaseSize as _,
                });
            }
        }
        Err("Failed to find module.")
    }
}

impl Module {
    pub fn read<T>(&self, offset: u32) -> Result<T, &'static str> {
        let mut read = unsafe { mem::zeroed() };
        let mut amount_read: libc::size_t = 0;

        unsafe {
            if memoryapi::ReadProcessMemory(
                Process::current_process().handle,
                (self.base_address + offset) as *mut _,
                &mut read as *mut _ as *mut _,
                mem::size_of::<T>() as _,
                &mut amount_read as *mut _,
            ) != (true as _) || amount_read == 0 {
                return Err("Failed to read memory.");
            }
        }

        Ok(read)
    }

    pub fn write<T>(&self, offset: u32, value: T) -> Result<(), &'static str> {
        let mut written: libc::size_t = 0;
        unsafe {
            if memoryapi::WriteProcessMemory(
                Process::current_process().handle,
                (self.base_address + offset) as *mut _,
                &value as *const _ as *mut _,
                mem::size_of::<T>() as _,
                &mut written as *mut _,
            ) != mem::size_of_val(&value) as i32 {
                return Err("Failed to write memory.");
            }
        }

        Ok(())
    }
}