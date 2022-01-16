use getset::Getters;
use std::{mem, ffi::OsString, os::windows::ffi::OsStringExt};
use winapi::um::{handleapi, memoryapi, processthreadsapi, tlhelp32, winnt};
use winapi::shared::minwindef;

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
    pub fn current_process() -> Self {
        let mut process = Self {
            pid: 0,
            handle: unsafe { processthreadsapi::GetCurrentProcess() },
        };
        process.pid = unsafe { processthreadsapi::GetProcessId(process.handle) };
        process
    }

    // 別にExにする必要は無いけど一応汎用性を持たせるために
    // 他のプロセスに対しても使えるようにしておく
    pub fn check_protection(&self, address: u32) -> Result<minwindef::DWORD, &'static str> {
        let mut meminfo = winnt::MEMORY_BASIC_INFORMATION {
            BaseAddress: address as *mut _,
            AllocationBase: address as *mut _,
            AllocationProtect: 0,
            RegionSize: 0,
            State: 0,
            Protect: 0,
            Type: 0,
        };
        if unsafe {
            memoryapi::VirtualQueryEx(
                self.handle,
                address as _,
                &mut meminfo as *mut _ as _,
                mem::size_of::<winnt::MEMORY_BASIC_INFORMATION>() as _,
            )
        } == 0 {
            return Err("Failed to get memory info.");
        }

        Ok(meminfo.Protect)
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
    #[no_mangle]
    pub fn fix_offset(&self, offset: u32) -> u32 {
        self.base_address + offset
    }

    pub unsafe fn read<T>(&self, offset: u32) -> &T {
        &*(self.fix_offset(offset) as *const T)
    }

    pub unsafe fn write<T>(&self, offset: u32, value: T) {
        *(self.fix_offset(offset) as *mut T) = value;
    }
}