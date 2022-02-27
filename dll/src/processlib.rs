use getset::Getters;
use std::{mem, ffi::OsString, os::windows::ffi::OsStringExt};
use winapi::um::{handleapi, memoryapi, processthreadsapi, tlhelp32, winnt, errhandlingapi, psapi};
use winapi::shared::minwindef::{HMODULE, DWORD, MAX_PATH};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
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
    pub handle: HMODULE,
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
    pub fn change_protection(&self, address: u32, protection: DWORD, size: u32) -> Result<DWORD, &'static str> {
        let mut oldp: DWORD = 0;
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

    #[allow(dead_code)]
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
            if path.contains(path_name) || (path == "" && self.pid == module_entry.th32ProcessID) {
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
                threads.push(Thread {
                    handle,
                    tid: thread_entry.th32ThreadID,
                });
            }
        }
        unsafe { handleapi::CloseHandle(thread_list) };
        Ok(threads)
    }

    pub fn get_current_thread_id(&self) -> Result<u32, &'static str> {
        let thread_id = unsafe { processthreadsapi::GetCurrentThreadId() };
        if thread_id == 0 {
            return Err("Failed to get current thread id.");
        }
        Ok(thread_id)
    }

    pub fn get_process_path(&self) -> Result<String, DWORD> {
        let mut process_path = [0u16; MAX_PATH];
        let mut process_path_len = 0;
        let ret = unsafe {
            psapi::GetModuleFileNameExW(
                self.handle,
                0 as _,
                process_path.as_mut_ptr() as _,
                MAX_PATH as _,
            )
        };
        if ret == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        process_path_len = ret;
        let process_path = OsString::from_wide(&process_path[..process_path_len as usize]);
        let process_path = match process_path.into_string() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed to convert OsString to String.");
                return Err(unsafe { errhandlingapi::GetLastError() });
            },
        };
        Ok(process_path)
    }

    pub fn allocate_memory(&self, len: u32) -> Result<u32, u32> {
        let addr = unsafe {
            memoryapi::VirtualAllocEx(
                self.handle,
                0 as _,
                len as _,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
        if addr == 0 as _ {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(addr as u32)
    }

    pub fn write_memory(&self, addr: u32, data: &str) -> Result<(), u32> {
        let data = data.as_bytes();
        let ret = unsafe {
            memoryapi::WriteProcessMemory(
                self.handle,
                addr as _,
                data.as_ptr() as _,
                data.len() as _,
                0 as _,
            )
        };
        if ret == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(())
    }
}

impl Module {
}

impl Thread {
    pub fn open_thread(tid: u32) -> Result<Self, &'static str> {
        let handle = unsafe { processthreadsapi::OpenThread(winnt::THREAD_ALL_ACCESS, 0, tid) };
        if handle == handleapi::INVALID_HANDLE_VALUE {
            return Err("Failed to open thread.");
        }
        Ok(Thread {
            handle,
            tid,
        })
    }

    #[allow(dead_code)]
    pub fn terminate(&self) -> Result<(), &'static str> {
        if unsafe { processthreadsapi::TerminateThread(self.handle, 0) } == 0 {
            return Err("Failed to terminate thread.");
        }
        Ok(())
    }

    pub fn suspend(&self) -> Result<(), &'static str> {
        if unsafe { processthreadsapi::SuspendThread(self.handle) } == 0 {
            return Err("Failed to suspend thread.");
        }
        Ok(())
    }

    pub fn base_addr(&self) -> Result<u32, &'static str> {
        let mut dw_start_addr: DWORD = 0;
        if unsafe {
            ntpsapi::NtQueryInformationThread(
                self.handle,
                ntpsapi::ThreadQuerySetWin32StartAddress,
                &mut dw_start_addr as *mut _ as _,
                mem::size_of::<DWORD>() as _,
                &mut 0 as *mut _ as _,
            )
        } != 0 {
            return Err("Failed to get thread entry point.");
        }
        Ok(dw_start_addr as u32)
    }
}