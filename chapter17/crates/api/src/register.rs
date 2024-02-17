use once_cell::sync::Lazy;
use std::sync::RwLock;
use sys::{napi_callback, napi_env, napi_value};

pub(crate) static REGISTER_FN: Lazy<RwLock<Vec<(&'static str, napi_callback)>>> =
    Lazy::new(Default::default);

pub fn register_fn(js_name: &'static str, cb: napi_callback) {
    REGISTER_FN.write().unwrap().push((js_name, cb));
}

pub fn gen_fn(env: napi_env, exports: napi_value) {
    let register = REGISTER_FN.write().unwrap();
    register.iter().for_each(|(name, cb)| {
        let mut fn_ptr = std::ptr::null_mut();
        unsafe {
            let n = format!("{}\0", *name).as_ptr().cast();
            sys::napi_create_function(env, n, name.len(), *cb, std::ptr::null_mut(), &mut fn_ptr);
            sys::napi_set_named_property(env, exports, n, fn_ptr);
        };
    })
}
