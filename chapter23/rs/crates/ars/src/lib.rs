use jni::objects::JObject;
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_rust_MainActivity_HelloRust<'local>(
    env: JNIEnv<'local>,
    _class: JObject<'local>,
) -> jstring {
    let s = CString::new("Hello Rust").unwrap();
    let out = env.new_string(s.to_str().unwrap()).unwrap();
    out.into_raw()
}
