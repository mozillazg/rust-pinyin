use pinyin::ToPinyin;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        for opt_pinyin in line.as_str().to_pinyin() {
            if let Some(pinyin) = opt_pinyin {
                chars.extend(pinyin.with_tone().chars());
            }
        }
    }
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for line in CHARS.iter() {
        for ch in line.iter() {
            let color = if chars.contains(ch) {
                Color::Green
            } else {
                Color::Red
            };
            stdout.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
            write!(&mut stdout, "{}\t", ch)?;
        }
        writeln!(&mut stdout)?;
    }
    Ok(())
}
