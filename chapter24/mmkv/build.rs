use cc::Build;
use napi_build_ohos;

fn main() {
    Build::new()
        .file("mmkv/Core/MMKV.cpp")
        .debug(true)
        .cpp(true)
        .cpp_link_stdlib("c++_shared")
        .compile("mmkv");
    napi_build_ohos::setup();
}
