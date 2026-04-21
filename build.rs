use std::path::Path;

fn main() {
    let src = Path::new("src");
    let parser = src.join("parser.c");
    let scanner = src.join("scanner.c");

    let mut c_compiler_config = cc::Build::new();
    c_compiler_config.std("c11").include(src);
    c_compiler_config.file(&parser);
    c_compiler_config.file(&scanner);
    c_compiler_config.compile("tree-sitter-latex");

    println!("cargo:rerun-if-changed={}", parser.display());
    println!("cargo:rerun-if-changed={}", scanner.display());
    println!("cargo:rerun-if-changed=src/tree_sitter/parser.h");
}
