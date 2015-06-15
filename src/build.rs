extern crate phf_codegen;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(env!("OUT_DIR")).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut builder = phf_codegen::Map::new();
    let data = include!("data");
    for i in 0..data.len() {
        let (k, v) = data[i];
        builder.entry(k, &escape_str(v));
    }
    write!(&mut file, "static PINYIN_MAP: ::phf::Map<u32, &'static str> = ").unwrap();
    builder.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(&mut file, "static PHONETIC_SYMBOL_MAP: ::phf::Map<&'static str, &'static str> = ").unwrap();
    phf_codegen::Map::new()
        .entry("ā", "\"a1\"")
        .entry("á", "\"a2\"")
        .entry("ǎ", "\"a3\"")
        .entry("à", "\"a4\"")
        .entry("ē", "\"e1\"")
        .entry("é", "\"e2\"")
        .entry("ě", "\"e3\"")
        .entry("è", "\"e4\"")
        .entry("ō", "\"o1\"")
        .entry("ó", "\"o2\"")
        .entry("ǒ", "\"o3\"")
        .entry("ò", "\"o4\"")
        .entry("ī", "\"i1\"")
        .entry("í", "\"i2\"")
        .entry("ǐ", "\"i3\"")
        .entry("ì", "\"i4\"")
        .entry("ū", "\"u1\"")
        .entry("ú", "\"u2\"")
        .entry("ǔ", "\"u3\"")
        .entry("ù", "\"u4\"")
        .entry("ü", "\"v0\"")
        .entry("ǘ", "\"v2\"")
        .entry("ǚ", "\"v3\"")
        .entry("ǜ", "\"v4\"")
        .entry("ń", "\"n2\"")
        .entry("ň", "\"n3\"")
        .entry("", "\"m2\"")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();
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
