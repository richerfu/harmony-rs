mod sys;

use napi_derive_ohos::napi;

#[napi]
pub fn add(value: i32) -> i32 {
    unsafe {
        sys::init_mmkv();
        let mmkv = sys::get_mmkv_instance();
        sys::set_float(mmkv, value, "float\0".as_ptr().cast());
        let result = sys::get_float(mmkv, "float\0".as_ptr().cast());
        result.into()
    }
}
