use crate::processlib::{Window};
use crate::otherwinapi;
use rand::Rng;

use winapi::um::synchapi;

pub extern "system" fn thread_entry(_module: *mut libc::c_void) -> u32 {
    let mut window = match Window::get_current_window() {
        Ok(w) => w,
        Err(_e) => {
            let msg = "Failed to get current window.\0";
            let title = "ERROR\0";
            otherwinapi::MsgBox(&msg, &title);
            return 0x1;
        }
    };

    let window_title_list = [
        "Windows\0",
        "AIUEO\0",
        "OMG\0",
        "WTF\0",
        "Di.Gi.Charat\0"
    ];

    loop {
        let mut rng = rand::thread_rng();
        let window_title = window_title_list[rng.gen_range(0..window_title_list.len())];
        match window.change_window_title(&window_title) {
            Ok(_) => {},
            Err(e) => {
                let msg = format!("[!] Failed to change window title.\nError Code: {}\0", e);
                let title = "ERROR\0";
                otherwinapi::MsgBox(&msg, &title);
                continue;
            }
        }
        unsafe { synchapi::Sleep(5000); }
    }
}
