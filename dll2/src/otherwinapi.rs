use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::{DWORD, BOOL};
use winapi::um::winnt::{HANDLE};
use winapi::um::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::{processthreadsapi, errhandlingapi};

use std::{mem, ptr};

pub fn MsgBox(text: &str, title: &str) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let lp_caption: Vec<u16> = title.encode_utf16().collect();

    unsafe { MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK
    ); }
}

pub fn CreateProcess(
    appname: &str,
    cmdline: &str,
    inherit_handles: bool,
    creation_flags: DWORD,
    sinfo_flags: DWORD,
) -> Result<HANDLE, u32> {
    let mut si: STARTUPINFOW = unsafe { mem::zeroed() };
    si.cb = mem::size_of::<STARTUPINFOW>() as DWORD;
    si.dwFlags = sinfo_flags;
    si.wShowWindow = 0x0;
    let mut pi: PROCESS_INFORMATION = unsafe { mem::zeroed() };
    let wc_appname = appname.encode_utf16().collect::<Vec<u16>>();
    let wc_cmdline = cmdline.as_bytes();

    if unsafe {
        processthreadsapi::CreateProcessW(
            wc_appname.as_ptr() as _,
            wc_cmdline.as_ptr() as _,
            ptr::null_mut(),
            ptr::null_mut(),
            inherit_handles as BOOL,
            creation_flags,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut si as *mut _ as _,
            &mut pi as *mut _ as _,
        )
    } == 0 {
        return Err(unsafe { errhandlingapi::GetLastError() });
    }

    Ok(pi.hProcess)
}

pub fn CreateRemoteThread(
    process: HANDLE,
    thread_func: u32,
    thread_func_arg: u32,
) -> Result<HANDLE, u32> {
    let thread_handle = unsafe {
        processthreadsapi::CreateRemoteThread(
            process,
            ptr::null_mut(),
            0,
            Some(mem::transmute(thread_func as usize)),
            thread_func_arg as _,
            0,
            ptr::null_mut(),
        )
    };

    if thread_handle == ptr::null_mut() {
        return Err(unsafe { errhandlingapi::GetLastError() });
    }

    Ok(thread_handle)
}