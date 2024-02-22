use std::ffi::{c_char, c_double, c_float, c_void};

#[repr(C)]
pub struct MMKV {
    _unused: [u8; 0],
}

#[link(name = "native_mmkv", kind = "static")]
extern "C" {
    pub fn get_mmkv_instance() -> *const MMKV;
    pub fn init_mmkv() -> c_void;
    pub fn set_float(mmkv: *const MMKV, v: c_float, k: *const c_char) -> c_void;
    pub fn get_float(mmkv: *const MMKV, k: *const c_char) -> c_float;
}
