use std::ffi::CString;
use std::ptr;
use sys::{napi_callback_info, napi_env, napi_value};

pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

unsafe extern "C" fn add_unsafe_code(env: napi_env, callback: napi_callback_info) -> napi_value {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    unsafe {
        let mut args = [ptr::null_mut(); 2];
        sys::napi_get_cb_info(
            env,
            callback,
            &mut 2,
            args.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        sys::napi_get_value_double(env, args[0], &mut a);
        sys::napi_get_value_double(env, args[1], &mut b);
    };
    let v = add(a, b);
    let mut res = ptr::null_mut();
    unsafe {
        sys::napi_create_double(env, v, &mut res);
    };
    res
}

unsafe extern "C" fn register_fn(env: napi_env, exports: napi_value) -> napi_value {
    let name = CString::new("add").unwrap();
    let desc = [sys::napi_property_descriptor {
        utf8name: name.as_ptr().cast(),
        name: ptr::null_mut(),
        getter: None,
        setter: None,
        method: Some(add_unsafe_code),
        attributes: 0,
        value: ptr::null_mut(),
        data: ptr::null_mut(),
    }];
    sys::napi_define_properties(env, exports, desc.len(), desc.as_ptr());
    exports
}

#[ctor::ctor]
fn export_module() {
    let name = CString::new("api").unwrap();
    let mut modules = sys::napi_module {
        nm_version: 1,
        nm_filename: ptr::null_mut(),
        nm_flags: 0,
        nm_modname: name.as_ptr().cast(),
        nm_priv: ptr::null_mut() as *mut _,
        nm_register_func: Some(register_fn),
        reserved: [ptr::null_mut() as *mut _; 4],
    };
    unsafe {
        sys::napi_module_register(&mut modules);
    }
}
