use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
};

use tree_sitter_generate::generate_parser_for_grammar;

fn main() {
    let src = Path::new("src");
    let parser = src.join("parser.c");
    if !parser.exists() {
        let grammar_json: _ =
            read_to_string(src.join("grammar.json")).expect("grammar.json not found");
        let (_, parser_c) = generate_parser_for_grammar(&grammar_json, Some((0, 26, 0)))
            .expect("Failed to generate parser grammar");
        write(&parser, parser_c).expect("Failed to commit the generated parser.c file to disk");
    }
    let header_dir = src.join("tree_sitter");
    if !header_dir.exists() {
        create_dir_all(&header_dir).expect("Failed to create tree_sitter header dir");
        write(
            header_dir.join("parser.h"),
            tree_sitter_generate::PARSER_HEADER,
        )
        .expect("Failed to write parser.h");
        write(
            header_dir.join("alloc.h"),
            tree_sitter_generate::ALLOC_HEADER,
        )
        .expect("Failed to write alloc.h");
        write(
            header_dir.join("array.h"),
            tree_sitter_generate::ARRAY_HEADER,
        )
        .expect("Failed to write array.h");
    }
    let mut c_compiler_config = cc::Build::new();
    c_compiler_config.std("c11").include(src);
    c_compiler_config.file(&parser);
    println!(
        "cargo:rerun-if-changed={}",
        parser.to_str().expect("Parser path not a path")
    );
    c_compiler_config.file(src.join("scanner.c"));
    println!(
        "cargo:rerun-if-changed={}",
        src.join("grammar.json").to_str().unwrap()
    );
    c_compiler_config.compile("tree-sitter-latex");
}
