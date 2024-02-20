mod sys;

use std::ffi::{c_float, CString};
use napi_derive_ohos::napi;

#[napi]
pub fn add(left: u32, right: u32)  {
    let a = CString::new("/data/storage/el2/base/haps/entry/files/mmkv").unwrap();
    unsafe {
        sys::init(a.as_ptr().cast());
        
        let mmkv = sys::getDefaultMMKV();

        sys::set(&mmkv, (10 as c_float).into(), "float\0".as_ptr().cast());
        sys::get_float(&mmkv, "float\0".as_ptr().cast());
    }
}
