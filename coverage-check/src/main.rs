use colored::{Color, Colorize};
use pinyin::ToPinyin;
use std::collections::HashSet;
use std::io::{self, stdin, BufRead};

const CHARS: &[&[char]] = &[
    &['a', 'ā', 'á', 'ǎ', 'à'],
    &['o', 'ō', 'ó', 'ǒ', 'ò'],
    &['e', 'ē', 'é', 'ě', 'è'],
    &['i', 'ī', 'í', 'ǐ', 'ì'],
    &['u', 'ū', 'ú', 'ǔ', 'ù'],
    &['ü', 'ǖ', 'ǘ', 'ǚ', 'ǜ'],
];

fn main() -> io::Result<()> {
    let mut chars = HashSet::new();
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        for opt_pinyin in line.as_str().to_pinyin() {
            if let Some(pinyin) = opt_pinyin {
                chars.extend(pinyin.with_tone().chars());
            }
        }
    }
    for line in CHARS.iter() {
        for ch in line.iter() {
            let color = if chars.contains(ch) {
                Color::Green
            } else {
                Color::Red
            };
            print!("{}\t", ch.to_string().color(color).bold());
        }
        println!();
    }
    Ok(())
}
