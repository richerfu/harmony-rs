mod sys;

use std::ffi::CString;
use napi_derive_ohos::napi;

#[napi]
pub fn init(dir: String) {
    let s = CString::new(dir).unwrap();
    unsafe {
        sys::init_mmkv(s.as_ptr().cast())
    };
}

#[napi]
pub fn set_int(value: i32, key: String) {
    let s = CString::new(key).unwrap();
    unsafe {
        let mmkv = sys::get_mmkv_instance();
        sys::set_i32(mmkv, value, s.as_ptr().cast());
    }
}

#[napi]
pub fn get_int(key: String) -> i32 {
    let s = CString::new(key).unwrap();
    unsafe {
        let mmkv = sys::get_mmkv_instance();
        let result = sys::get_i32(mmkv, s.as_ptr().cast());
        result.into()
    }
}

#[napi]
pub fn set_float(value: f64, key: String) {
    let s = CString::new(key).unwrap();
    unsafe {
        let mmkv = sys::get_mmkv_instance();
        sys::set_f64(mmkv, value, s.as_ptr().cast());
    }
}

#[napi]
pub fn get_float(key: String) -> f64 {
    let s = CString::new(key).unwrap();
    unsafe {
        let mmkv = sys::get_mmkv_instance();
        let result = sys::get_f64(mmkv, s.as_ptr().cast());
        result.into()
    }
}
