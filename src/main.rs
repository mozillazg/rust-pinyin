extern crate pinyin;

pub fn main() {
    for v in  pinyin::pinyin(&"丁") {
        println!("{}", v);
    }
}
