extern crate pinyin;

pub fn main() {
    let mut args = pinyin::Args::new();
    args.heteronym = true;
    for v in  pinyin::pinyin("中", &args) {
        println!("{}", v);
    }
}
