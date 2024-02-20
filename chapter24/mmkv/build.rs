use cc::Build;
use walkdir::{DirEntry, WalkDir};
use std::{env, path::PathBuf};

fn main() {
    let h = env::current_dir().unwrap().join("mmkv/Core");
    println!("cargo:rustc-link-search={:?}",&h);

    fn is_cpp_file(entry: &DirEntry) -> bool {
        entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "cpp")
    }

    let cpp_files: Vec<PathBuf> = WalkDir::new(&h)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_cpp_file)
        .map(|i| i.into_path())
        .collect();


    Build::new()
        .file("wrapper.cpp")
        .files(cpp_files)
        .debug(true)
        .cpp(true)
        .include(&h)
        .cpp_link_stdlib("c++_static")
        .compile("native_mmkv");

    println!("cargo:rerun-if-changed=wraper.cpp");
    println!("cargo:rustc-link-lib=static=native_mmkv");

    napi_build_ohos::setup();
}
