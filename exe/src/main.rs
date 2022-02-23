use winapi::um::winuser::{MB_OK, MessageBoxW};

fn main() {
    unsafe {
        let msg = format!("[!!!] Hello from SubProcess!!!\0");
        err_msgbox(msg);
    }
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
