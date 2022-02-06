use crate::processlib::{Process, Module, Thread};

use winapi::um::{handleapi, memoryapi, processthreadsapi, tlhelp32, winnt};
use winapi::shared::minwindef;
use winapi::um::winuser::{MB_OK, MessageBoxW};

pub unsafe extern "stdcall" fn Get_Thread_Owner_PID(process: &Process) -> Result<(), &'static str> {
    let thread_list: Vec<Thread> =  match Process::get_threadlist(process) {
        Ok(list) => list,
        Err(e) => {
            return Err(e);
        }
    };

    let module_list: Vec<Module> = match Process::get_module_from_path(process, "chars") {
        Ok(list) => list,
        Err(e) => {
            return Err(e);
        }
    };
    
    let mut msg = String::new();
    for thread in thread_list {
        let mut unknown_flag: u32 = 0x0;
        let thread_entry_point = match Thread::base_addr(&thread) {
            Ok(addr) => addr,
            Err(e) => {
                return Err(e);
            }
        };
        for module in &module_list {
            if thread_entry_point >= module.base_addr && thread_entry_point < module.base_addr + module.size {
                unknown_flag = 0x1;
                break;
            }
        }
        if unknown_flag == 0x0 {
            msg.push_str(&format!("{:x}\n", thread.tid));
        }
    }
    if msg.len() == 0 {
        msg.push_str("None");
    }
    msg.push_str("\0");
    err_msgbox(msg);

    Ok(())
}

pub unsafe extern "stdcall" fn err_msgbox(text: String) {
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