fn main() {
    let mut rust_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rust_path.pop();
    let xml2_path: String = String::from(rust_path.to_string_lossy());
    println!("cargo:rustc-link-lib=static=xml2");
    println!("cargo:rustc-link-search=native={}", xml2_path);
}
