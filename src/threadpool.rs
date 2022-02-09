use crate::processlib::{Process, Module, Thread};
use crate::dbg::{Debugger};

use std::{mem};
use winapi::um::{minwinbase, winnt, debugapi, winbase};

use winapi::um::winuser::{MB_OK, MessageBoxW};

pub unsafe extern "system" fn Thread_Checker(_module: *mut libc::c_void) -> u32 {
    let debugger: Debugger = match Debugger::new() {
        Ok(d) => d,
        Err(_e) => {
            return 0x1;
        }
    };
    match debugger.set_privilege() {
        Ok(_) => {},
        Err(_e) => {
            return 0x1;
        }
    };


    let mut thread_list: Vec<Thread> = match Process::get_threadlist(&debugger.process) {
        Ok(list) => list,
        Err(_e) => {
            return 0x1;
        }
    };
    let mut module_list: Vec<Module> = match Process::get_module_from_path(&debugger.process, "") {
        Ok(list) => list,
        Err(_e) => {
            return 0x1;
        }
    };
    let mut specific_module_list = match Process::get_module_from_path(&debugger.process, "chars") {
        Ok(list) => list,
        Err(_e) => {
            return 0x1;
        }
    };
    
    match suspend_thread(&debugger.process, &mut thread_list, &mut module_list, &mut specific_module_list) {
        Ok(_) => {
        },
        Err(_e) => {
            return 0x1;
        }
    };

    match debugger.attach() {
        Ok(_) => {
        },
        Err(_e) => {
            return 0x1;
        }
    };

    let mut debug_event: minwinbase::DEBUG_EVENT = mem::zeroed();
    let continue_flag: u32 = winnt::DBG_CONTINUE;

    // Detection of loading DLL or create Thread
    loop {
        if debugapi::WaitForDebugEvent(&mut debug_event, winbase::INFINITE) != 0 {
            match debug_event.dwDebugEventCode {
                minwinbase::CREATE_THREAD_DEBUG_EVENT => {
                    match Thread::open_thread(debug_event.dwThreadId) {
                        Ok(thread) => {
                            thread_list.push(thread);
                        },
                        Err(_e) => {
                            return 0x1;
                        }
                    };
                },
                minwinbase::LOAD_DLL_DEBUG_EVENT => {
                    module_list = match Process::get_module_from_path(&debugger.process, "") {
                        Ok(list) => list,
                        Err(_e) => {
                            return 0x1;
                        }
                    };
                    specific_module_list = match Process::get_module_from_path(&debugger.process, "chars") {
                        Ok(list) => list,
                        Err(_e) => {
                            return 0x1;
                        }
                    };
                },
                _ => {
                    continue;
                }
            }
    
            match suspend_thread(&debugger.process, &mut thread_list, &mut module_list, &mut specific_module_list) {
                Ok(_) => {},
                Err(_e) => {
                    return 0x1;
                }
            };
    
            debugapi::ContinueDebugEvent(debug_event.dwProcessId, debug_event.dwThreadId, continue_flag);
        }
    }
}

unsafe fn suspend_thread(process: &Process, thread_list: &Vec<Thread>, module_list: &Vec<Module>, specific_module_list: &Vec<Module>) -> Result<(), &'static str> {

    let mut evil_thread_list: Vec<&Thread> = Vec::new();
    let current_tid = match Process::get_current_thread_id(process) {
        Ok(tid) => tid,
        Err(e) => return Err(e),
    };

    // Detection of threads that are not in any module address range
    for thread in thread_list {
        let mut unknown_flag: u32 = 0x0;
        let thread_entry_point = match Thread::base_addr(&thread) {
            Ok(addr) => addr,
            Err(e) => {
                return Err(e);
            }
        };
        for module in module_list {
            if thread_entry_point >= module.base_addr && thread_entry_point < module.base_addr + module.size {
                unknown_flag = 0x1;
                break;
            }
        }
        if unknown_flag == 0x0 {
            if thread.tid != current_tid {
                evil_thread_list.push(&thread);
            }
        }
    }

    for module in specific_module_list {
        for thread in thread_list {
            let thread_entry_point = match Thread::base_addr(&thread) {
                Ok(addr) => addr,
                Err(e) => {
                    return Err(e);
                }
            };
            if thread_entry_point >= module.base_addr && thread_entry_point < module.base_addr + module.size {
                if thread.tid != current_tid {
                    evil_thread_list.push(&thread);
                }
            }
        }
    }

    let mut msg: String = String::new();
    for thread in &evil_thread_list {
        msg.push_str(&format!("{:x}\n", thread.tid));
    }
    if msg.len() == 0 {
        msg.push_str("None");
    }
    msg.push_str("\0");
    err_msgbox(msg);

    // suspension of threads
    for thread in evil_thread_list {
        match thread.suspend() {
            Ok(_) => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

unsafe fn err_msgbox(text: String) {
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