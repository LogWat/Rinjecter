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

    // Exにする必要は無いけど一応他のプロセスに対しても使えるようにしておく
    #[allow(dead_code)]
    fn check_protection(&self, address: u32) -> Result<minwindef::DWORD, &'static str> {
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

    pub unsafe fn read<T>(&self, address: u32) -> Result<&T, &'static str> {
        match self.check_protection(address) {
            Ok(protect) => {
                match protect {
                    winnt::PAGE_READWRITE | winnt::PAGE_EXECUTE_READWRITE => {
                        return Ok(&*(address as *const T));
                    },
                    _ => {
                        let mut oldp: minwindef::DWORD = 0;

                        if memoryapi::VirtualProtectEx(
                            self.handle,
                            address as *mut _,
                            mem::size_of::<T>() as _,
                            winnt::PAGE_READWRITE,
                            &mut oldp as *mut _,
                        ) == 0 {
                            return Err("Failed to change memory protection.");
                        }

                        let result = &*(address as *const T);

                        if memoryapi::VirtualProtectEx(
                            self.handle,
                            address as *mut _,
                            mem::size_of::<T>() as _,
                            oldp,
                            &mut oldp as *mut _,
                        ) == 0 {
                            return Err("Failed to change memory protection.");
                        }

                        return Ok(result);
                    }
                };
            },
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub unsafe fn write<T>(&self, address: u32, value: T) -> Result<(), &'static str> {
        match self.check_protection(address) {
            Ok(protect) => {
                match protect {
                    winnt::PAGE_READWRITE | winnt::PAGE_EXECUTE_READWRITE => {
                        *(address as *mut T) = value;
                        return Ok(());
                    },
                    _ => {
                        return Err("Failed to write memory.\nMemory is not writable.");
                    }
                };
            },
            Err(err) => {
                return Err(err);
            }
        }
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

    pub unsafe fn read<T>(&self, address: u32) -> &T {
        &*(address as *const T)
    }

    pub unsafe fn write<T>(&self, address: u32, value: T) {
        *(address as *mut T) = value;
    }
}