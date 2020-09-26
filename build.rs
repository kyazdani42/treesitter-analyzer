use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=stdc++");

    let dir: PathBuf = ["tree-sitter-rust", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-rust");

    let dir: PathBuf = ["tree-sitter-lua", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.cc"))
        .compile("tree-sitter-lua");
}
