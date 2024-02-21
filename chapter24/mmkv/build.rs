use cc::Build;
use walkdir::{DirEntry, WalkDir};
use std::{env, path::PathBuf};

fn main() {
    let h = env::current_dir().unwrap().join("mmkv/Core");
    println!("cargo:rustc-link-search={:?}",&h);

    fn is_cpp_file(entry: &DirEntry) -> bool {
        entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "cpp" || e == "S")
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
        .define("MMKV_EMBED_ZLIB","1")
        .file("wrapper.cpp")
        .files(cpp_files)
        .debug(true)
        .cpp(true)
        .include(&h)
        .cpp_link_stdlib("c++_shared")
        .compile("native_mmkv");

    println!("cargo:rerun-if-changed=wrapper.cpp");
    println!("cargo:rustc-link-lib=static=native_mmkv");

    napi_build_ohos::setup();
}
