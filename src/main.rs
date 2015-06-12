extern crate pinyin;

pub fn main() {
    for v in  pinyin::pinyin(&"中", &pinyin::Args::new()) {
        println!("{}", v);
    }
}
