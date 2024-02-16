use std::ptr;
use sys::{napi_create_double, napi_env, napi_get_value_double, napi_value};

pub trait NapiValue {
    fn get_value_from_raw(env: napi_env, value: napi_value) -> Self;
    fn try_into_raw(env: napi_env, value: Self) -> napi_value;
}

impl NapiValue for f64 {
    fn get_value_from_raw(env: napi_env, value: napi_value) -> f64 {
        let mut res: f64 = 0.0;
        unsafe {
            napi_get_value_double(env, value, &mut res);
        };
        res
    }

    fn try_into_raw(env: napi_env, value: f64) -> napi_value {
        let mut res = ptr::null_mut();
        unsafe {
            napi_create_double(env, value, &mut res);
        };
        res
    }
}

