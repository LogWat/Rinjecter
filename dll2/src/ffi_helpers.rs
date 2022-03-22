use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;

pub fn win32_to_utf16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

pub fn win32_to_i8(s: &str) -> Vec<i8> {
    OsStr::new(s).encode_wide().chain(once(0)).map(|x| x as i8).collect()
}