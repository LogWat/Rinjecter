use winapi::um::winuser::{MB_OK, MessageBoxW};

pub fn MsgBox(text: &str, title: &str) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let lp_caption: Vec<u16> = title.encode_utf16().collect();

    unsafe { MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK
    ); }
}