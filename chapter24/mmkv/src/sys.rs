use std::ffi::{c_char, c_double, c_float, c_void};

#[repr(C)]
pub struct MMKV {
    _unused: [u8; 0],
}

extern "C" {
    pub fn getDefaultMMKV() -> MMKV;
    pub fn initializeMMKV(dir: *const c_char) -> c_void;
    pub fn set(mmkv: &MMKV,v: c_float,k: *const c_char) -> c_void;
    pub fn get_float(mmkv: &MMKV,k: *const c_char) -> c_float;
}