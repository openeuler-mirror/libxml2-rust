use std::path::PathBuf;

extern crate rust_ffi;

fn main() {
    let mut rust_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rust_path.pop();
    rust_path.pop();
    let archive_path: String = String::from(rust_path.to_string_lossy());
    println!("cargo:rustc-link-lib=static=xml2");
    println!("cargo:rustc-link-search=native={}", archive_path);

    rust_ffi::add_cfg();
}
