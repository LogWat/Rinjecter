use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;

pub fn win32_to_utf16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}