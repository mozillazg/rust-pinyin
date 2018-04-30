extern crate pinyin;

pub fn main() {
    let hans = "中国人";
    let mut args = pinyin::Args::new();

    // 默认输出 [["zhong"] ["guo"] ["ren"]]
    println!("{:?}", pinyin::pinyin(hans, &args));

    // 包含声调 [["zh\u{14d}ng"], ["gu\u{f3}"], ["r\u{e9}n"]]
    args.style = pinyin::Style::Tone;
    println!("{:?}", pinyin::pinyin(hans, &args));

    // 声调用数字表示 [["zho1ng"] ["guo2"] ["re2n"]]
    args.style = pinyin::Style::Tone2;
    println!("{:?}", pinyin::pinyin(hans, &args));

    // 开启多音字模式
    args = pinyin::Args::new();
    args.heteronym = true;
    // [["zhong", "zhong"] ["guo"] ["ren"]]
    println!("{:?}", pinyin::pinyin(hans, &args));

    // [["zho1ng", "zho4ng"] ["guo2"] ["re2n"]]
    args.style = pinyin::Style::Tone2;
    println!("{:?}", pinyin::pinyin(hans, &args));

    // ["zho1ng", "guo2", "re2n"]
    println!("{:?}", pinyin::lazy_pinyin(hans, &args));
}
