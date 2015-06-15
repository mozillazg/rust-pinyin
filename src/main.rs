extern crate pinyin;

pub fn main() {
    let mut args = pinyin::Args::new();
    args.heteronym = true;
    println!("{:?}",  pinyin::pinyin("中", &args));
    println!("{:?}",  pinyin::pinyin("中国人", &args));
    // args.style = pinyin::Style::Tone2;
    // println!("{:?}",  pinyin::pinyin("中国人", &args));
}
