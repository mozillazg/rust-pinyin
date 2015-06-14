extern crate phf_codegen;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// Uses the phf_codegen crate to create a compile-time static hash map
// containing the pinyin mappings. The map is created in the
// $OUT_DIR/codegen.rs file, which is included in the main library.
fn main() {
    let mut builder = phf_codegen::Map::new();
    let data = include!("data.rs");
    for i in 0..data.len() {
        let (k, v) = data[i];
        builder.entry(k, &escape_str(v));
    }

    let path = Path::new(env!("OUT_DIR")).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    builder.build(&mut file).unwrap();
    write!(&mut file, "\n").unwrap();
}

fn escape_str(s: &str) -> String {
    let mut res = String::new();
    res.push('"');
    for ch in s.chars() {
        res.push_str(&format!("\\u{{{:x}}}", ch as u32));
    }
    res.push('"');
    res
}
