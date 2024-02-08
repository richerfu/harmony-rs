fn main() {
    println!("cargo:rustc-link-search=./third_c");
    println!("cargo:rustc-link-lib=add");
}