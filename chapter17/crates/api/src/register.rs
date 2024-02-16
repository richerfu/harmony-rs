use once_cell::sync::Lazy;
use std::ptr;
use std::sync::RwLock;
use sys::{napi_callback, napi_property_descriptor};

static REGISTER_FN: Lazy<RwLock<Vec<(&'static str, napi_callback)>>> = Lazy::new(Default::default);

pub fn register_fn(js_name: &'static str, cb: napi_callback) {
    REGISTER_FN.write().unwrap().push((js_name, cb));
}

pub fn gen_fn() -> Vec<napi_property_descriptor> {
    REGISTER_FN
        .write()
        .unwrap()
        .iter()
        .map(|(name, cb)| napi_property_descriptor {
            utf8name: format!("{}\0", *name).as_ptr().cast(),
            name: ptr::null_mut(),
            getter: None,
            setter: None,
            method: *cb,
            attributes: 0,
            value: ptr::null_mut(),
            data: ptr::null_mut(),
        })
        .collect()
}
