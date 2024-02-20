use cc::Build;
use std::env;

fn main() {
    let h = env::current_dir().unwrap().join("mmkv/Core");
    println!("cargo:rustc-link-search={:?}",&h);

    Build::new()
        .file("wraper.cpp")
        .debug(true)
        .cpp(true)
        .include(h)
        .cpp_link_stdlib("c++_static")
        .compile("native_mmkv");

    napi_build_ohos::setup();
}
