use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;

pub fn win32_to_utf16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

#[allow(dead_code)]
pub fn win32_to_i8(s: &str) -> Vec<i8> {
    OsStr::new(s).encode_wide().chain(once(0)).map(|x| x as i8).collect()
}

pub fn vector_to_addr(v: &Vec<u8>) -> Result<u32, &'static str> {
    if v.len() != 4 {
        return Err("Vector must be 4 bytes long.");
    }
    let mut addr: u32 = 0x0;
    for i in 0..4 {
        addr |= v[3 - i] as u32; // Little endian
        if i < 3 {
            addr <<= 8;
        }
    }
    Ok(addr)
}