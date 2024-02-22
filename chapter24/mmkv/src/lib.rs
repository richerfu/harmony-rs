mod sys;

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::pre_init;
use napi_ohos::module_init;

#[napi]
pub fn add(_left: u32, _right: u32) -> f32 {
    unsafe {
        sys::init_mmkv();
        let mmkv = sys::get_mmkv_instance();
        sys::set_float(mmkv,3.12,"float\0".as_ptr().cast());
        let result = sys::get_float(mmkv,"float\0".as_ptr().cast());
        result.into()
    }
}

#[module_init]
fn init() {
    pre_init();
}
