#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn main() {
    const HELLO: &'static str = "Hello, world!\n\0";
    unsafe {
        libc::printf(HELLO.as_ptr() as *const _);
    }
}

#[panic_handler]
fn handle (_info: &core::panic::PanicInfo) -> ! {
    loop {}
}