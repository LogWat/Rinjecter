use crate::processlib::{Process, Module, Thread};

use winapi::um::{handleapi, memoryapi, processthreadsapi, tlhelp32, winnt};
use winapi::shared::minwindef;
use winapi::um::winuser::{MB_OK, MessageBoxW};

pub unsafe extern "stdcall" fn Get_Thread_Owner_PID(process: &Process) -> bool {
    let mut thread_list: Vec<Thread> = Vec::new();
    match Process::get_threadlist(process) {
        Ok(list) => thread_list = list,
        Err(_e) => {
            return false;
        }
    };
    let msg = format!("{:?}", thread_list[2].owner_tid);
    err_msgbox(msg);
    let msg2 = format!("{:?}", process.pid);
    err_msgbox(msg2);
    true
}

unsafe extern "stdcall" fn err_msgbox(text: String) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let caption = "⚠Error⚠\0".to_string();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK
    );
}