#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_value__ {
    _unused: [u8; 0],
}

pub type napi_value = *mut napi_value__;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_env__ {
    _unused: [u8; 0],
}

pub type napi_env = *mut napi_env__;

pub type napi_status = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_callback_info__ {
    _unused: [u8; 0],
}
pub type napi_callback_info = *mut napi_callback_info__;

pub type napi_callback =
    Option<unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value>;

pub type napi_property_attributes = i32;

pub type napi_addon_register_func =
    Option<unsafe extern "C" fn(env: napi_env, exports: napi_value) -> napi_value>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct napi_property_descriptor {
    pub utf8name: *const c_char,
    pub name: napi_value,
    pub method: napi_callback,
    pub getter: napi_callback,
    pub setter: napi_callback,
    pub value: napi_value,
    pub attributes: napi_property_attributes,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_module {
    pub nm_version: c_int,
    pub nm_flags: c_uint,
    pub nm_filename: *const c_char,
    pub nm_register_func: napi_addon_register_func,
    pub nm_modname: *const c_char,
    pub nm_priv: *mut c_void,
    pub reserved: [*mut c_void; 4usize],
}

extern "C" {
    pub fn napi_get_cb_info(
        env: napi_env,
        cbinfo: napi_callback_info,
        argc: *mut usize,
        argv: *mut napi_value,
        this_arg: *mut napi_value,
        data: *mut *mut c_void,
    ) -> napi_status;
    pub fn napi_get_value_double(env: napi_env, value: napi_value, result: *mut f64)
        -> napi_status;
    pub fn napi_create_double(env: napi_env, value: f64, result: *mut napi_value) -> napi_status;
    pub fn napi_module_register(mod_: *mut napi_module);
    pub fn napi_define_properties(
        env: napi_env,
        object: napi_value,
        property_count: usize,
        properties: *const napi_property_descriptor,
    ) -> napi_status;
    pub fn napi_set_named_property(
        env: napi_env,
        object: napi_value,
        utf8name: *const c_char,
        value: napi_value,
    ) -> napi_status;
    pub fn napi_create_function(
        env: napi_env,
        utf8name: *const c_char,
        length: usize,
        cb: napi_callback,
        data: *mut c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
