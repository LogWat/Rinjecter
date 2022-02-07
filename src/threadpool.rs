use crate::processlib::{Process, Module, Thread};

use std::{mem};
use winapi::um::{minwinbase, winnt, debugapi, winbase};

pub unsafe extern "stdcall" fn Thread_Checker() -> Result<(), &'static str> {
    let process = Process::current_process();

    let mut thread_list: Vec<Thread> = match Process::get_threadlist(&process) {
        Ok(list) => list,
        Err(e) => {
            return Err(e);
        }
    };
    let mut module_list: Vec<Module> = match Process::get_module_from_path(&process, "") {
        Ok(list) => list,
        Err(e) => {
            return Err(e);
        }
    };
    let mut specific_module_list = match Process::get_module_from_path(&process, "chars") {
        Ok(list) => list,
        Err(e) => {
            return Err(e);
        }
    };
    
    match suspend_thread(&mut thread_list, &mut module_list, &mut specific_module_list) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    };

    let mut debug_event: minwinbase::DEBUG_EVENT = mem::zeroed();
    let continue_flag: u32 = winnt::DBG_CONTINUE;

    if debugapi::WaitForDebugEvent(&mut debug_event, winbase::INFINITE) != 0 {
        if debug_event.dwDebugEventCode == minwinbase::LOAD_DLL_DEBUG_EVENT {
            module_list = match Process::get_module_from_path(&process, "") {
                Ok(list) => list,
                Err(e) => {
                    return Err(e);
                }
            };
            specific_module_list = match Process::get_module_from_path(&process, "chars") {
                Ok(list) => list,
                Err(e) => {
                    return Err(e);
                }
            };
        }

        if debug_event.dwDebugEventCode == minwinbase::CREATE_THREAD_DEBUG_EVENT {
            match Thread::open_thread(debug_event.dwThreadId) {
                Ok(thread) => thread_list.push(thread),
                Err(e) => {
                    return Err(e);
                }
            };
        }

        match suspend_thread(&mut thread_list, &mut module_list, &mut specific_module_list) {
            Ok(_) => {},
            Err(e) => {
                return Err(e);
            }
        };

        debugapi::ContinueDebugEvent(debug_event.dwProcessId, debug_event.dwThreadId, continue_flag);
    }

    Ok(())
}

unsafe extern "stdcall" fn suspend_thread(thread_list: &Vec<Thread>, module_list: &Vec<Module>, specific_module_list: &Vec<Module>) -> Result<(), &'static str> {

    let mut evil_thread_list: Vec<&Thread> = Vec::new();

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
            evil_thread_list.push(thread);
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
                evil_thread_list.push(thread);
            }
        }
    }

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