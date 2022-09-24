use getset::Getters;
use std::{mem, ffi::OsString, os::windows::ffi::OsStringExt, ptr};

use winapi::{
    um::{
        handleapi, memoryapi, processthreadsapi, tlhelp32, winnt, errhandlingapi,
        winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
        {winuser},
    },
    shared::{
        minwindef::{HMODULE, DWORD},
        windef::{HWND},
    },
};

use crate::ffi_helpers;
use crate::overwrite::AddrSize;

#[derive(Getters)]
#[get = "pub"]
#[repr(C)]
pub struct Process {
    pub pid: u32,
    pub handle: winnt::HANDLE,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

#[derive(Getters)]
#[get = "pub"]
pub struct Module {
    pub handle: HMODULE,
    pub name: OsString,
    pub path: OsString,
    pub base_addr: u32,
    pub size: u32,
}

pub struct Window {
    pub hwnd: HWND,
    pub title: String,
    pub pid: u32,
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

    pub fn from_handle(handle: winnt::HANDLE) -> Self {
        let mut process = Self {
            pid: 0,
            handle,
        };
        process.pid = unsafe { processthreadsapi::GetProcessId(process.handle) };

        process
    }

    // Exにする必要は無いけど一応他のプロセスに対しても使えるようにしておく
    pub fn check_protection(&self, address: u32) -> Result<winnt::MEMORY_BASIC_INFORMATION, u32> {
        let mut meminfo: winnt::MEMORY_BASIC_INFORMATION = unsafe { mem::zeroed() };
        if unsafe {
            memoryapi::VirtualQueryEx(
                self.handle,
                address as _,
                &mut meminfo as *mut _ as _,
                mem::size_of::<winnt::MEMORY_BASIC_INFORMATION>() as _,
            )
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(meminfo)
    }

    // address: BaseAddr, size: RegionSize, protection: Protect
    pub fn change_protection(&self, address: u32, protection: DWORD, size: u32) -> Result<DWORD, u32> {
        let mut oldp: DWORD = 0;
        if unsafe {
            memoryapi::VirtualProtectEx(
                self.handle,
                address as _,
                size as _,
                protection,
                &mut oldp,
            )
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        return Ok(oldp);
    }

    #[allow(dead_code)]
    pub unsafe fn read<T>(&self, address: u32, _size: T) -> &T {
        &*(address as *const T)
    }

    pub unsafe fn write(&self, address: u32, value: AddrSize)  {
        match value {
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

impl Window {
    pub fn get_current_window() -> Result<Self, u32> {
        let hwnd = unsafe { winuser::GetForegroundWindow() };
        if hwnd == ptr::null_mut() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        let mut pid: u32 = 0;
        if unsafe { winuser::GetWindowThreadProcessId(hwnd, &mut pid) } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        if pid == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        let mut title = [0u16; 512];
        let len = unsafe { winuser::GetWindowTextW(hwnd, title.as_mut_ptr(), 512) };
        if len == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        let title = OsString::from_wide(&title[..]).into_string().unwrap();


        Ok(Window {
            hwnd,
            title,
            pid,
        })
    }

    pub fn change_window_title(&mut self, new_title: &str) -> Result<(), u32> {
        let title = ffi_helpers::win32_to_utf16(new_title);
        let ret = unsafe { winuser::SetWindowTextW(self.hwnd, title.as_ptr()) };
        if ret == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
        self.title = new_title.to_string();

        Ok(())
    }
}