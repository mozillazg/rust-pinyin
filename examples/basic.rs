use pinyin::{ToPinyin, ToPinyinMulti};

fn main() {
    let hans = "中国人";

    // 无声调，输出 zhong guo ren
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{} ", pinyin.plain());
        }
    }
    println!();

    // 包含声调，输出 zhōng guó rén
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{} ", pinyin.with_tone());
        }
    }
    println!();

    // 声调用数字表示，输出 zho1ng guo2 re2n
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{} ", pinyin.with_tone_num());
        }
    }
    println!();

    // 多音字，输出
    // zho1ng zho4ng
    // guo2
    // re2n
    for multi in hans.to_pinyin_multi() {
        if let Some(multi) = multi {
            for pinyin in multi {
                print!("{} ", pinyin.with_tone_num());
            }
            println!();
        }
    }
}
