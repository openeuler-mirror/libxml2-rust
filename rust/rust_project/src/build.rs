use std::path::PathBuf;

fn main() {
    let mut rust_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rust_path.pop();
    rust_path.pop();
    let archive_path: String = String::from(rust_path.to_string_lossy());
    println!("cargo:rustc-link-lib=static=xml2_static");
    println!("cargo:rustc-link-search=native={}", archive_path);
    rust_condition::condition_defined::defined_param::add_cfg();
}
