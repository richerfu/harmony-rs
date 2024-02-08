use std::ffi::c_int;

extern "C" {
    pub fn add(a: c_int,b: c_int) -> c_int;
}

fn main() {
    let a = unsafe {
        add(1,2)
    };
    println!("{a}");
}
