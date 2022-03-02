use crate::process::{Process, Thread, Module};
use crate::ffi_helpers;
use crate::dbg::Debugger;

use winapi::um::{
    minwinbase::{DEBUG_EVENT, CREATE_THREAD_DEBUG_EVENT, LOAD_DLL_DEBUG_EVENT, EXIT_THREAD_DEBUG_EVENT, UNLOAD_DLL_DEBUG_EVENT},
    winnt::{DBG_CONTINUE},
    debugapi, winbase::INFINITE, errhandlingapi,
};

use std::{mem};
use std::sync::{Arc, Mutex};

pub fn wait_debugevnet(process: Arc<Mutex<Process>>) -> Result<(), u32> {
    let mut debugger = Debugger::new();
    let process = process.lock().unwrap();
    
    match debugger.attach(process.pid) {
        Ok(_) => {},
        Err(e) => return Err(e),
    }

    match debugger.set_privilege() {
        Ok(_) => {},
        Err(e) => return Err(e),
    }

    let mut thread_list: Vec<Thread> = match Process::get_threadlist(&process) {
        Ok(list) => list,
        Err(_e) => return Err(0x1),
    };
    let mut module_list: Vec<Module> = match Module::get_module_from_path(&process, "") {
        Ok(list) => list,
        Err(_e) => return Err(0x1),
    };
    let mut specific_module_list = match Module::get_module_from_path(&process, "chars") {
        Ok(list) => list,
        Err(_e) => return Err(0x1),
    };

    match suspend_thread(&process, &mut thread_list, &mut module_list, &mut specific_module_list) {
        Ok(_) => {},
        Err(_e) => return Err(0x1),
    };

    let mut debug_event: DEBUG_EVENT = unsafe { mem::zeroed() };
    let cnt_flag: u32 = DBG_CONTINUE;
    loop {
        if unsafe { debugapi::WaitForDebugEvent(&mut debug_event, INFINITE) } != 0 {
            match debug_event.dwDebugEventCode {
                CREATE_THREAD_DEBUG_EVENT => {
                    match Thread::open_thread(debug_event.dwThreadId) {
                        Ok(thread) => {
                            thread_list.push(thread);
                        },
                        Err(_e) => return Err(0x1),
                    }
                },
                LOAD_DLL_DEBUG_EVENT => {
                    module_list = match Module::get_module_from_path(&process, "") {
                        Ok(list) => list,
                        Err(_e) => return Err(0x1),
                    };
                    specific_module_list = match Module::get_module_from_path(&process, "chars") {
                        Ok(list) => list,
                        Err(_e) => return Err(0x1),
                    };
                },
                _ => {
                    unsafe { debugapi::ContinueDebugEvent(
                        debug_event.dwProcessId,
                        debug_event.dwThreadId,
                        cnt_flag
                    ) };
                }
            }

            match suspend_thread(&process, &mut thread_list, &mut module_list, &mut specific_module_list) {
                Ok(_) => {},
                Err(_e) => return Err(0x1),
            };

            unsafe { debugapi::ContinueDebugEvent(
                debug_event.dwProcessId,
                debug_event.dwThreadId,
                cnt_flag
            ) };
            
        } else {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }
    }

    Ok(())
}

fn suspend_thread(
    process: &Process,
    thread_list: &Vec<Thread>,
    module_list: &Vec<Module>,
    specific_module_list: &Vec<Module>
) -> Result<(), u32> {
    let mut to_suspend: Vec<&Thread> = Vec::new();
    
    // Find the thread that is not in any of the module ranges
    for thread in thread_list {
        let mut unk_flag = false;
        let thread_entry_point = match Thread::base_addr(thread) {
            Ok(addr) => addr,
            Err(_e) => {
                continue;
            }
        };
        for module in module_list {
            if thread_entry_point >= module.base_addr && thread_entry_point < module.base_addr + module.size {
                unk_flag = true;
                break;
            }
        }
        if !unk_flag {
            to_suspend.push(thread);
        }
    }

    // Find the thread that is in the specific module ranges
    for module in specific_module_list {
        for thread in thread_list {
            let thread_entry_point = match Thread::base_addr(thread) {
                Ok(addr) => addr,
                Err(_e) => {
                    continue;
                }
            };
            if thread_entry_point >= module.base_addr && thread_entry_point < module.base_addr + module.size {
                to_suspend.push(thread);
            }
        }
    }

    for thread in to_suspend {
        match thread.suspend() {
            Ok(_) => {},
            Err(_e) => return Err(0x1),
        }
    }

    Ok(())
}