use cc::Build;
use napi_build_ohos;

fn main() {
    std::env::set_var("CXX","/Volumes/PSSD/code/harmony/harmony-rs/chapter24/mmkv/scripts/aarch64-unknown-linux-ohos-clang++.sh");
    Build::new()
        .file("mmkv/Core/MMKV.cpp")
        .debug(true)
        .cpp(true)
        .cpp_link_stdlib("c++_shared")
        .compile("mmkv");
    napi_build_ohos::setup();
}
