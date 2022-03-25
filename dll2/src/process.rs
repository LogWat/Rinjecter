use winapi::{
    um::{
        winnt::{
            HANDLE, 
            PROCESS_ALL_ACCESS,
            THREAD_ALL_ACCESS,
            MEM_COMMIT,
            MEM_RESERVE,
            PAGE_READWRITE,
            PAGE_GUARD,
            MEMORY_BASIC_INFORMATION,
        },
        tlhelp32, handleapi, psapi,
        tlhelp32::{
            PROCESSENTRY32W,
            THREADENTRY32,
            MODULEENTRY32W,
            TH32CS_SNAPPROCESS,
            TH32CS_SNAPTHREAD,
            TH32CS_SNAPMODULE
        },
        processthreadsapi, errhandlingapi, memoryapi,
        handleapi::{INVALID_HANDLE_VALUE},
    },
    shared::{
        minwindef::{HMODULE, MAX_PATH,},
    },
};
use ntapi::ntpsapi;

use std::{mem, ptr, str, ffi::OsString, os::windows::ffi::OsStringExt};

#[repr(C)]
pub struct Process {
    pub pid: u32,
    pub handle: HANDLE,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

pub struct MemAttr {
    pub base_addr: u32,
    pub size: u32,
    pub attr: u32,
}

pub struct MemoeryBreakPoint {
    pub old_mem_attr: MemAttr,
    pub new_mem_attr: MemAttr,
}

pub struct Thread {
    pub handle: HANDLE,
    pub tid: u32,
}

pub struct Module {
    pub handle: HMODULE,
    pub name: String,
    pub path: String,
    pub base_addr: u32,
    pub size: u32,
}

impl Process {
    pub fn empty() -> Self {
        Process {
            pid: 0,
            handle: ptr::null_mut(),
        }
    }

    pub fn get_current_process() -> Self {
        let mut process = Self::empty();
        process.handle = unsafe { processthreadsapi::GetCurrentProcess() };
        process.pid = unsafe { processthreadsapi::GetProcessId(process.handle) };
        process
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


    pub fn name(&self) -> String {
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


    pub fn enumerate_process() -> Result<Vec<Process>, u32> {
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

    pub fn kill_process(&self) -> Result<(), u32> {
        let ret = unsafe { processthreadsapi::TerminateProcess(self.handle, 0) };
        if ret == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(())
    }

    pub fn query_map(&self) -> Result<Vec<MemAttr>, u32> {
        let mut mem_attrs: Vec<MemAttr> = Vec::new();
        let mut mem_entry: MEMORY_BASIC_INFORMATION = unsafe { mem::zeroed() };
        let mut addr = 0x0;
        loop {
            let ret = unsafe {
                memoryapi::VirtualQueryEx(
                    self.handle,
                    addr as _,
                    &mut mem_entry,
                    mem::size_of::<MEMORY_BASIC_INFORMATION>() as _,
                )
            };
            if ret == 0 {
                return Err(unsafe { errhandlingapi::GetLastError() });
            }

            let base_addr = mem_entry.BaseAddress as u32;
            let size = mem_entry.RegionSize as u32;
            mem_attrs.push(MemAttr {
                base_addr: base_addr,
                size: size,
                attr: mem_entry.State,
            });
            addr = base_addr + size;

            // User memory location ( 0xC0000000 ~ 0x7FFFFFFF )
            if addr >= 0x7FFF_FFFF {
                break;
            }
        }
        Ok(mem_attrs)
    }

    pub fn bp_set_mem(&self, old_mem_attr: MemAttr) -> Result<MemoeryBreakPoint, u32> {
        let mut bp = MemoeryBreakPoint {
            old_mem_attr: old_mem_attr,
            new_mem_attr: MemAttr {
                base_addr: old_mem_attr.base_addr,
                size: old_mem_attr.size,
                attr: 0,
            },
        };

        if let Ok(mem_attr) = self.change_memory_protection(
            old_mem_attr.base_addr,
            old_mem_attr.size,
            old_mem_attr.attr | PAGE_GUARD,
        ) {
            bp.new_mem_attr.attr = mem_attr;
        } else {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(bp)
    }

    fn change_memory_protection(&self, addr: u32, size: u32, attr: u32) -> Result<u32, u32> {
        let mut oldp: u32 = 0;
        let ret = unsafe {
            memoryapi::VirtualProtectEx(
                self.handle,
                addr as _,
                size as _,
                attr,
                &mut oldp as _,
            )
        };
        if ret == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(oldp)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe { handleapi::CloseHandle(self.handle) };
    }
}


impl Thread {
    pub fn open_thread(tid: u32) -> Result<Self, u32> {
        let handle = unsafe { processthreadsapi::OpenThread(
            THREAD_ALL_ACCESS,
            0,
            tid
        ) };
        if handle == INVALID_HANDLE_VALUE {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(Thread {
            handle,
            tid,
        })
    }

    #[allow(dead_code)]
    pub fn get_current_thread_id() -> Result<u32, u32> {
        let tid = unsafe { processthreadsapi::GetCurrentThreadId() };
        if tid == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(tid)
    }

    #[allow(dead_code)]
    pub fn suspend(&self) -> Result<(), u32> {
        let success = unsafe { processthreadsapi::SuspendThread(self.handle) };
        if success == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(())
    }

    pub fn terminate(&self) -> Result<(), u32> {
        let success = unsafe { processthreadsapi::TerminateThread(self.handle, 0) };
        if success == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        Ok(())
    }

    pub fn base_addr(&self) -> Result<u32, u32> {
        let mut dw_start_addr: u32 = 0;
        if unsafe {
            ntpsapi::NtQueryInformationThread(
                self.handle,
                ntpsapi::ThreadQuerySetWin32StartAddress,
                &mut dw_start_addr as *mut _ as _,
                mem::size_of::<u32>() as _,
                ptr::null_mut(),
            )
        } != 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(dw_start_addr)
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { handleapi::CloseHandle(self.handle) };
    }
}

impl Module {
    pub fn get_module_from_path(process: &Process, path_name: &str) -> Result<Vec<Module>, u32> {
        let module = unsafe {
            tlhelp32::CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process.pid)
        };
        if module == INVALID_HANDLE_VALUE {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        let mut module_entry: MODULEENTRY32W = unsafe { mem::zeroed() };
        module_entry.dwSize = mem::size_of::<MODULEENTRY32W>() as _;
        let mut module_list: Vec<Module> = Vec::new();

        while unsafe { tlhelp32::Module32NextW(module, &mut module_entry) } != 0 {
            let name = match OsString::from_wide(&module_entry.szModule[..]).into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };
            let path = match OsString::from_wide(&module_entry.szExePath[..]).into_string() {
                Ok(path) => path,
                Err(_) => continue,
            };
            if path.contains(path_name) || (path == "" && process.pid == module_entry.th32ProcessID) {
                module_list.push(Module {
                    handle: module_entry.hModule,
                    name,
                    path,
                    base_addr: module_entry.modBaseAddr as u32,
                    size: module_entry.modBaseSize as u32,
                });
            }
        }

        unsafe { handleapi::CloseHandle(module) };
        Ok(module_list)
    }
}