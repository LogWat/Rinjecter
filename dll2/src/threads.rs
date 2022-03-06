use crate::process::{Process, Thread, Module};
use crate::otherwinapi;
use crate::dbg::Debugger;

use winapi::um::{
    minwinbase::{DEBUG_EVENT, CREATE_THREAD_DEBUG_EVENT, LOAD_DLL_DEBUG_EVENT},
    winnt::{DBG_CONTINUE},
    debugapi, winbase::INFINITE,
};

use std::{mem};

pub extern "system" fn wait_debugevnet(_module: *mut libc::c_void) -> u32 {
    let process = match find_target_process("mugen.exe") {
        Ok(process) => process,
        Err(err) => {
            let msg = format!("Failed to find target process: {}\0", err);
            let title = "ERROR\0";
            otherwinapi::MsgBox(&msg, title);
            return 0x1;
        }
    };

    let mut debugger = match Debugger::new(process.pid) {
        Ok(d) => d,
        Err(err) => {
            let msg = format!("Failed to create debugger: {}\0", err);
            let title = "ERROR\0";
            otherwinapi::MsgBox(&msg, title);
            return 0x1;
        }
    };

    match debugger.set_privilege() {
        Ok(_) => {},
        Err(e) => {
            let msg = format!("Failed to set privilege.\nError Code: {}\0", e);
            let title = "ERROR\0";
            otherwinapi::MsgBox(&msg, title);
            return 0x1;
        }
    }
    
    match debugger.attach() {
        Ok(_) => {},
        Err(e) => {
            let msg = format!("[!!!] Failed to attach.\nError Code: {}\0", e);
            let title = "ERROR\0";
            otherwinapi::MsgBox(&msg, title);
            return 0x1;
        }
    }

    let mut thread_list: Vec<Thread> = match Process::get_threadlist(&process) {
        Ok(list) => list,
        Err(_e) => return 0x1,
    };
    let mut module_list: Vec<Module> = match Module::get_module_from_path(&process, "") {
        Ok(list) => list,
        Err(_e) => return 0x1,
    };
    let mut specific_module_list = match Module::get_module_from_path(&process, "chars") {
        Ok(list) => list,
        Err(_e) => return 0x1,
    };

    match suspend_thread(&process, &mut thread_list, &mut module_list, &mut specific_module_list) {
        Ok(_) => {},
        Err(_e) => return 0x1,
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
                        Err(_e) => return 0x1,
                    }
                },
                LOAD_DLL_DEBUG_EVENT => {
                    module_list = match Module::get_module_from_path(&process, "") {
                        Ok(list) => list,
                        Err(_e) => return 0x1,
                    };
                    specific_module_list = match Module::get_module_from_path(&process, "chars") {
                        Ok(list) => list,
                        Err(_e) => return 0x1,
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
                Err(_e) => return 0x1,
            };

            unsafe { debugapi::ContinueDebugEvent(
                debug_event.dwProcessId,
                debug_event.dwThreadId,
                cnt_flag
            ) };
            
        } else {
            return 0x1;
        }
    }
}

fn suspend_thread(
    _process: &Process,
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

    let mut msg: String = String::new();
    for thread in &to_suspend {
        msg.push_str(&format!("{}", thread.tid));
    }
    if msg.len() == 0 {
        msg.push_str("None");
    }
    msg.push_str("\0");
    let title = "Threads to suspend\0";
    otherwinapi::MsgBox(&msg, &title);


    for thread in &to_suspend {
        match thread.suspend() {
            Ok(_) => {},
            Err(_e) => return Err(0x1),
        }
    }

    Ok(())
}

fn find_target_process(name: &str) -> Result<Process, u32> {
    let mut process = Process::empty();
    match process.get_process_from_name(name) {
        Ok(process) => Ok(process),
        Err(e) => Err(e),
    }
}