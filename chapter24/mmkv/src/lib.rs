mod sys;

use napi_derive_ohos::napi;
use std::ffi::c_float;

#[napi]
pub fn add(left: u32, right: u32) -> u32 {
    unsafe {
        sys::initializeMMKV(
            "/data/storage/el2/base/haps/entry/files/mmkv\0"
                .as_ptr()
                .cast(),
        );

        let mmkv = sys::getDefaultMMKV();

        sys::set(&mmkv, (10 as c_float).into(), "float\0".as_ptr().cast());
        sys::get_float(&mmkv, "float\0".as_ptr().cast());
    }
    left + right
}
