use getset::Getters;
use std::{mem, ffi::OsString, os::windows::ffi::OsStringExt};
use winapi::um::{handleapi, memoryapi, processthreadsapi, tlhelp32, winnt};
use winapi::shared::minwindef;
use ntapi::ntpsapi;

use crate::overwrite::AddrSize;

#[derive(Getters)]
#[get = "pub"]
#[repr(C)]
pub struct Process {
    pub pid: u32,
    pub handle: winnt::HANDLE,
}

#[derive(Getters)]
#[get = "pub"]
pub struct Module {
    pub handle: minwindef::HMODULE,
    pub name: OsString,
    pub path: OsString,
    pub base_addr: u32,
    pub size: u32,
}

#[derive(Getters)]
#[get = "pub"]
pub struct Thread {
    pub handle: winnt::HANDLE,
    pub tid: u32,
    pub entry_point: u32,
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
    pub fn check_protection(&self, address: u32) -> Result<winnt::MEMORY_BASIC_INFORMATION, &'static str> {
        let mut meminfo: winnt::MEMORY_BASIC_INFORMATION = unsafe { mem::zeroed() };
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

        Ok(meminfo)
    }

    // address: BaseAddr, size: RegionSize, protection: Protect
    pub fn change_protection(&self, address: u32, protection: minwindef::DWORD, size: u32) -> Result<minwindef::DWORD, &'static str> {
        let mut oldp: minwindef::DWORD = 0;
        if unsafe {
            memoryapi::VirtualProtectEx(
                self.handle,
                address as *mut _,
                size as usize,
                protection,
                &mut oldp as *mut _ as _,
            )
        } == 0 {
            return Err("Failed to change memory protection.");
        }

        return Ok(oldp);
    }

    pub unsafe fn read<T>(&self, address: u32, _size: T) -> &T {
        &*(address as *const T)
    }

    pub unsafe fn write(&self, address: u32, value: AddrSize)  {
        match value {
            AddrSize::Qword(v) => {
                *(address as *mut u64) = v;
            },
            AddrSize::Dword(v) => {
                *(address as *mut u32) = v;
            },
            AddrSize::Word(v) => {
                *(address as *mut u16) = v;
            },
            AddrSize::Byte(v) => {
                *(address as *mut u8) = v;
            },
        }
    }

    pub fn get_module_from_path(&self, path_name: &str) -> Result<Vec<Module>, &'static str> {
        let module = unsafe { 
            tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPMODULE, self.pid) 
        };
        if module == handleapi::INVALID_HANDLE_VALUE {
            return Err("Failed to create snapshot.");
        }

        let mut module_entry: tlhelp32::MODULEENTRY32W = unsafe { mem::zeroed() };
        module_entry.dwSize = mem::size_of::<tlhelp32::MODULEENTRY32W>() as _;
        let mut module_list: Vec<Module> = Vec::new();

        while unsafe { tlhelp32::Module32NextW(module, &mut module_entry) } != 0 {
            let name = OsString::from_wide(&module_entry.szModule[..]).into_string();
            let name = match name {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Failed to convert OsString to String.");
                    continue;
                },
            };
            let path = OsString::from_wide(&module_entry.szExePath[..]).into_string();
            let path = match path {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Failed to convert OsString to String.");
                    continue;
                },
            };
            if path.contains(path_name) {
                unsafe { handleapi::CloseHandle(module) };
                module_list.push(
                    Module {
                        handle: module_entry.hModule,
                        name: OsString::from(name),
                        path: OsString::from(path),
                        base_addr: module_entry.modBaseAddr as u32,
                        size: module_entry.modBaseSize as u32,
                    }
                );
            }
        }
        unsafe { handleapi::CloseHandle(module) };
        Ok(module_list)
    }

    pub fn get_threadlist(&self) -> Result<Vec<Thread>, &'static str> {
        let mut threads = Vec::new();
        let mut thread_entry: tlhelp32::THREADENTRY32 = unsafe { mem::zeroed() };
        thread_entry.dwSize = mem::size_of::<tlhelp32::THREADENTRY32>() as _;

        let thread_list = unsafe { 
            tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPTHREAD, self.pid) 
        };
        if thread_list == handleapi::INVALID_HANDLE_VALUE {
            return Err("Failed to create snapshot.");
        }

        while unsafe { tlhelp32::Thread32Next(thread_list, &mut thread_entry) } != 0 {
            if thread_entry.th32OwnerProcessID == self.pid {
                let handle = unsafe { processthreadsapi::OpenThread(winnt::THREAD_ALL_ACCESS, 0, thread_entry.th32ThreadID) };
                if handle == handleapi::INVALID_HANDLE_VALUE {
                    continue;
                }
                // Get thread entry point
                let mut dw_start_addr: minwindef::DWORD = 0;
                if unsafe {
                    ntpsapi::NtQueryInformationThread(
                        handle,
                        ntpsapi::ThreadQuerySetWin32StartAddress,
                        &mut dw_start_addr as *mut _ as _,
                        mem::size_of::<minwindef::DWORD>() as _,
                        &mut 0 as *mut _ as _,
                    )
                } != 0 {
                    continue;
                }
                
                threads.push(Thread {
                    handle,
                    tid: thread_entry.th32ThreadID,
                    entry_point: dw_start_addr as u32,
                });
            }
        }
        unsafe { handleapi::CloseHandle(thread_list) };
        Ok(threads)
    }
}

impl Module {
}

impl Thread {
}