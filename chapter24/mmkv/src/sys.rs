use std::ffi::{c_char, c_double, c_int, c_void};

#[repr(C)]
pub struct MMKV {
    _unused: [u8; 0],
}

#[link(name = "native_mmkv", kind = "static")]
extern "C" {
    pub fn get_mmkv_instance() -> *const MMKV;
    pub fn init_mmkv(dir: *const c_char) -> c_void;
    pub fn set_i32(mmkv: *const MMKV, v: c_int, k: *const c_char) -> c_void;
    pub fn get_i32(mmkv: *const MMKV, k: *const c_char) -> c_int;
    pub fn set_f64(mmkv: *const MMKV, v: c_double, k: *const c_char) -> c_void;
    pub fn get_f64(mmkv: *const MMKV, k: *const c_char) -> c_double;
}
