use cc::Build;
use std::{env, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let h = env::current_dir().unwrap().join("mmkv/Core");
    let ndk = env::var("OHOS_NDK_HOME").unwrap();
    let basic_h = PathBuf::from(&ndk).join("native/sysroot/usr/include");
    println!("cargo:rustc-link-search={:?}", &h);
    println!("cargo:rustc-link-search={:?}", &basic_h);

    fn is_cpp_file(entry: &DirEntry) -> bool {
        entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map_or(false, |e| e == "cpp" || e == "S")
    }

    let cpp_files: Vec<PathBuf> = WalkDir::new(&h)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_cpp_file)
        .map(|i| i.into_path())
        .collect();

    Build::new()
        .asm_flag("-x")
        .asm_flag("assembler-with-cpp")
        .asm_flag("-march=armv8-a+crypto")
        .define("MMKV_EMBED_ZLIB", "1")
        .flag(format!("--sysroot={}/native/sysroot", &ndk).as_str())
        .file("wrapper.cpp")
        .files(cpp_files)
        .define("__MUSL__", None)
        .debug(true)
        .cpp(true)
        .include(&h)
        .include(&basic_h)
        .cpp_link_stdlib("c++_shared")
        .target("aarch64-linux-ohos")
        .compile("native_mmkv");

    napi_build_ohos::setup();
}
