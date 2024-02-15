#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_env__ {
    _unused: [u8; 0],
}

type StatusCode = i32;

fn test() {
    let a: StatusCode = 1;
}