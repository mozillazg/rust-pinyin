extern crate phf_codegen;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file = BufWriter::new(File::create(&path).unwrap());

    let mut pinyin_data = String::new();
    File::open("./pinyin-data/pinyin.txt")
        .unwrap()
        .read_to_string(&mut pinyin_data)
        .unwrap();
    let re_line = regex::Regex::new(r"^U\+([a-zA-Z\d]+):\s*([^\s]+)\s*#").unwrap();

    let mut builder = phf_codegen::Map::new();
    for line in pinyin_data.lines() {
        match re_line.captures(line) {
            Some(group) => {
                let hex = &group[1];
                let code_point = u32::from_str_radix(hex, 16).unwrap();
                let pinyin = &group[2];
                builder.entry(code_point, &format!("\"{}\"", pinyin));
            }
            None => continue,
        }
    }
    // 拼音库
    write!(
        &mut out_file,
        "static PINYIN_MAP: ::phf::Map<u32, &'static str> = "
    ).unwrap();
    builder.build(&mut out_file).unwrap();
    write!(&mut out_file, ";\n").unwrap();

    // 带声调字符
    write!(
        &mut out_file,
        "static PHONETIC_SYMBOL_MAP: ::phf::Map<&'static str, &'static str> = "
    ).unwrap();
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
        .build(&mut out_file)
        .unwrap();
    write!(&mut out_file, ";\n").unwrap();
}
