use crate::process::{Process, Thread, Module};
use crate::otherwinapi;
use crate::dbg::Debugger;

use winapi::um::{
    minwinbase::{DEBUG_EVENT, CREATE_THREAD_DEBUG_EVENT, LOAD_DLL_DEBUG_EVENT},
    winnt::{DBG_CONTINUE},
    debugapi, processthreadsapi, libloaderapi, errhandlingapi,
    winbase::{INFINITE, DEBUG_PROCESS},
    winnt::{PROCESS_ALL_ACCESS, HANDLE}
};

use std::{mem, ptr};

pub extern "system" fn thread_entry(_module: *mut libc::c_void) -> u32 {
    let mut process = match find_target_process("mugen.exe") {
        Ok(p) => p,
        Err(e) => {
            println!("[!] Failed to find target process: {}", e);
            return 0x1;
        }
    };

    let mut debugger = match Debugger::new(process.pid) {
        Ok(d) => d,
        Err(e) => {
            println!("[!] Failed to create debugger: {}", e);
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

    // How many calculations are there?
    let calc_list = match num_of_processes("calc.exe") {
        Ok(n) => {
            if n.len() == 0 {
                let msg = "Failed to find calc.exe.\0";
                let title = "ERROR\0";
                otherwinapi::MsgBox(&msg, title);
                return 0x1;
            }
            n
        },
        Err(e) => {
            let msg = format!("[!] Failed to get number of processes.\nError Code: {}\0", e);
            let title = "ERROR\0";
            otherwinapi::MsgBox(&msg, title);
            return 0x1;
        }
    };
    match debugger.attach() {
        Ok(_) => {
            // if there are more than two calc.exe's,
            // kill the one that is not self
            if calc_list.len() > 1 {
                for i in calc_list {
                    if i.pid != Process::get_current_process().pid {
                        match i.kill_process() {
                            Ok(_) => {},
                            Err(e) => {
                                let msg = format!("[!] Failed to kill process.\nError Code: {}\0", e);
                                let title = "ERROR\0";
                                otherwinapi::MsgBox(&msg, title);
                                return 0x1;
                            }
                        }
                    }
                }
            }
        },
        Err(_e) => {
            // if more than three, then we can't do anything
            // if there's one or two, create a new process
            if calc_list.len() > 2 {
                let msg = "Found more than 2 calc.exe processes.\0";
                let title = "ERROR\0";
                otherwinapi::MsgBox(&msg, title);
                return 0x1;
            } else {
                match otherwinapi::CreateProcess(
                    "C:\\Windows\\System32\\calc.exe",
                    "",
                    false,
                    DEBUG_PROCESS,
                    0x1
                ) {
                    Ok(_) => {},
                    Err(e) => {
                        let msg = format!("[!] Failed to create process.\nError Code: {}\0", e);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, title);
                        return 0x1;
                    }
                }
                // if the process is successfully created,
                // inject 2.dll

                // At first, get the path of 2.dll
                let self_modules = match Module::get_module_from_path(&process, "Mistaken") {
                    Ok(m) => {
                        if m.len() == 0 {
                            let msg = "Failed to find 2.dll.\0";
                            let title = "ERROR\0";
                            otherwinapi::MsgBox(&msg, title);
                            return 0x1;
                        }
                        m
                    },
                    Err(e) => {
                        let msg = format!("[!] Failed to get self module.\nError Code: {}\0", e);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, title);
                        return 0x1;
                    }
                };
                let mut dll2_path: String = String::new();
                for module in self_modules {
                    if module.path.contains("Mistaken\\2.dll") {
                        dll2_path = module.path.clone();
                        break;
                    }
                }
                if dll2_path.is_empty() {
                    let msg = "Failed to find 2.dll.\0";
                    let title = "ERROR\0";
                    otherwinapi::MsgBox(&msg, title);
                    return 0x1;
                }

                // Inject 2.dll
                let _ = match dll_inject(&mut process, &dll2_path) {
                    Ok(_) => {},
                    Err(e) => {
                        let msg = format!("[!] Failed to inject 2.dll.\nError Code: {}\0", e);
                        let title = "ERROR\0";
                        otherwinapi::MsgBox(&msg, title);
                        return 0x1;
                    }
                };
            }
        }
    }

    let _ = wait_debugevnet(&mut debugger);

    0x0
}

fn wait_debugevnet(debugger: &Debugger) -> u32 {

    let mut thread_list: Vec<Thread> = match Process::get_threadlist(&debugger.process) {
        Ok(list) => list,
        Err(_e) => return 0x1,
    };
    let mut module_list: Vec<Module> = match Module::get_module_from_path(&debugger.process, "") {
        Ok(list) => list,
        Err(_e) => return 0x1,
    };
    let mut specific_module_list = match Module::get_module_from_path(&debugger.process, "chars") {
        Ok(list) => list,
        Err(_e) => return 0x1,
    };

    match suspend_thread(&debugger.process, &mut thread_list, &mut module_list, &mut specific_module_list) {
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
                    module_list = match Module::get_module_from_path(&debugger.process, "") {
                        Ok(list) => list,
                        Err(_e) => return 0x1,
                    };
                    specific_module_list = match Module::get_module_from_path(&debugger.process, "chars") {
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

            match suspend_thread(&debugger.process, &mut thread_list, &mut module_list, &mut specific_module_list) {
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

fn num_of_processes(proc_path: &str) -> Result<Vec<Process>, u32> {
    let mut process_list: Vec<Process> = vec![];
    match Process::enumerate_process() {
        Ok(list) => {
            for process in list {
                if process.name().contains(proc_path) {
                    process_list.push(process);
                }
            }
            Ok(process_list)
        },
        Err(e) => Err(e),
    }
}

fn dll_inject(process: &mut Process, dll_path: &str) -> Result<HANDLE, String> {
    process.handle = unsafe {
        processthreadsapi::OpenProcess(
            PROCESS_ALL_ACCESS,
            0,
            process.pid
        )
    };
    if process.handle == ptr::null_mut() {
        return Err(format!("Failed to open process. Error Code: {}\0", unsafe { errhandlingapi::GetLastError() }));
    }

    let arg_address = match Process::allocate_memory(process, dll_path.len() as u32) {
        Ok(a) => a,
        Err(e) => {
            return Err(format!("Failed to allocate memory. Error Code: {}\0", e));
        }
    };

    match Process::write_memory(process, arg_address, dll_path) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("Failed to write memory. Error Code: {}\0", e));
        }
    };

    let h_kernel32 = unsafe {
        libloaderapi::GetModuleHandleA(b"Kernel32.dll\0".as_ptr() as *const _)
    };
    if h_kernel32 == ptr::null_mut() {
        return Err(format!("Failed to get kernel32.dll handle. Error Code: {}\0", unsafe { errhandlingapi::GetLastError() }));
    }

    let h_loadlib = unsafe {
        libloaderapi::GetProcAddress(h_kernel32, b"LoadLibraryA\0".as_ptr() as *const _)
    };
    if h_loadlib == ptr::null_mut() {
        return Err(format!("Failed to get LoadLibraryA address. Error Code: {}\0", unsafe { errhandlingapi::GetLastError() }));
    }

    let h_thread = match otherwinapi::CreateRemoteThread(
        process.handle,
        h_loadlib as u32,
        arg_address,
    ) {
        Ok(h) => h,
        Err(e) => {
            return Err(format!("Failed to create remote thread. Error Code: {}\0", e));
        }
    };

    return Ok(h_thread);
}