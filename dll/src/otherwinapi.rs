use winapi::shared::minwindef::{DWORD, BOOL};
use winapi::um::winnt::{HANDLE};
use winapi::um::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::{processthreadsapi, errhandlingapi};

use std::{mem, ptr};

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