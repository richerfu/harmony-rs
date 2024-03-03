use std::ffi::{c_char, CString};

#[no_mangle]
pub extern fn hello_rust() -> *mut c_char {
    CString::new("Hello Rust").unwrap().into_raw()
}

#[no_mangle]
pub extern fn free_hello(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
