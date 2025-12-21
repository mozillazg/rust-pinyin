use pinyin::{ToPinyin, ToPinyinMulti};

fn main() {
    let hans = "中国人";

    // 无声调，输出 zhong guo ren
    for pinyin in hans.to_pinyin().flatten() {
        print!("{} ", pinyin.plain());
    }
    println!();

    // 包含声调，输出 zhōng guó rén
    for pinyin in hans.to_pinyin().flatten() {
        print!("{} ", pinyin.with_tone());
    }
    println!();

    // 声调用数字表示，输出 zho1ng guo2 re2n
    for pinyin in hans.to_pinyin().flatten() {
        print!("{} ", pinyin.with_tone_num());
    }
    println!();

    // 声调用数字在末尾表示，输出 zhong1 guo2 ren2
    for pinyin in hans.to_pinyin().flatten() {
        print!("{} ", pinyin.with_tone_num_end());
    }
    println!();

    // 多音字，输出
    // zho1ng zho4ng
    // guo2
    // re2n
    for multi in hans.to_pinyin_multi().flatten() {
        for pinyin in multi {
            print!("{} ", pinyin.with_tone_num());
        }
        println!();
    }
}
