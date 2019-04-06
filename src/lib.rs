//! 汉语拼音转换工具 Rust 版。
//! [![Build Status](https://img.shields.io/travis/mozillazg/rust-pinyin/master.svg)](https://travis-ci.org/mozillazg/rust-pinyin)
//! [![Coverage Status](https://img.shields.io/coveralls/mozillazg/rust-pinyin/master.svg)](https://coveralls.io/github/mozillazg/rust-pinyin)
//! [![Crates.io Version](https://img.shields.io/crates/v/pinyin.svg)](https://crates.io/crates/pinyin)
//! [![GitHub stars](https://img.shields.io/github/stars/mozillazg/rust-pinyin.svg?style=social&label=Star)](https://github.com/mozillazg/rust-pinyin)
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/pinyin) and can be
//! used by adding `pinyin` to your dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! pinyin = "0.4"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate pinyin;
//! ```
//!
//! # 示例
//!
//! ```
//! extern crate pinyin;
//!
//! pub fn main() {
//!     let hans = "中国人";
//!     let mut args = pinyin::Args::new();
//!
//!     // 默认输出 [["zhong"] ["guo"] ["ren"]]
//!     println!("{:?}",  pinyin::pinyin(hans, &args));
//!
//!     // ["zhong", "guo", "ren"]
//!     println!("{:?}",  pinyin::lazy_pinyin(hans, &args));
//!
//!     // 包含声调 [["zhōng"], ["guó"], ["rén"]]
//!     args.style = pinyin::Style::Tone;
//!     println!("{:?}",  pinyin::pinyin(hans, &args));
//!
//!     // 声调用数字表示 [["zho1ng"] ["guo2"] ["re2n"]]
//!     args.style = pinyin::Style::Tone2;
//!     println!("{:?}",  pinyin::pinyin(hans, &args));
//!
//!     // 开启多音字模式
//!     args = pinyin::Args::new();
//!     args.heteronym = true;
//!     // [["zhong", "zhong"] ["guo"] ["ren"]]
//!     println!("{:?}",  pinyin::pinyin(hans, &args));
//!
//!     // [["zho1ng", "zho4ng"] ["guo2"] ["re2n"]]
//!     args.style = pinyin::Style::Tone2;
//!     println!("{:?}",  pinyin::pinyin(hans, &args));
//! }
//! ```

mod dict;

pub use dict::{PHONETIC_SYMBOL_MAP, PINYIN_MAP};

// 声母表
const _INITIALS: [&str; 21] = [
    "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "j", "q", "x", "r", "zh", "ch", "sh",
    "z", "c", "s",
];

/// 拼音风格
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Style {
    /// 普通风格，不带声调（默认风格）。如： `pin yin`
    Normal,
    /// 声调风格1，拼音声调在韵母第一个字母上。如： `pīn yīn`
    Tone,
    /// 声调风格2，即拼音声调在各个拼音之后，用数字 [0-4] 进行表示。如： `pi1n yi1n`
    Tone2,
    /// 声母风格，只返回各个拼音的声母部分。如： 中国 的拼音 `zh g`
    Initials,
    /// 首字母风格，只返回拼音的首字母部分。如： `p y`
    FirstLetter,
    /// 韵母风格1，只返回各个拼音的韵母部分，不带声调。如： `ong uo`
    Finals,
    /// 韵母风格2，带声调，声调在韵母第一个字母上。如： `ōng uó`
    FinalsTone,
    /// 韵母风格2，带声调，声调在各个拼音之后，用数字 [0-4] 进行表示。如： `o1ng uo2`
    FinalsTone2,
}

/// 参数
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Args {
    /// 拼音风格
    pub style: Style,
    /// 是否启用多音字模式
    pub heteronym: bool,
}

impl Args {
    /// 返回一个默认参数
    ///
    /// ```ignore
    /// Args {
    ///    style: Style::Normal,
    ///    heteronym: false,
    /// }
    /// ```
    pub fn new() -> Args {
        Args {
            style: Style::Normal,
            heteronym: false,
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

// 获取单个拼音中的声母
fn initial(p: &str) -> String {
    let mut s = "".to_string();
    for v in &_INITIALS {
        if p.starts_with(v) {
            s = v.to_string();
            break;
        }
    }
    s
}

// 获取单个拼音中的韵母
fn _final(p: &str) -> String {
    let i = initial(p);
    if i == "" {
        return p.to_string();
    }
    let s: Vec<&str> = p.splitn(2, &i).collect();
    s.concat()
}

fn to_fixed(p: &str, a: &Args) -> String {
    if let Style::Initials = a.style {
        return initial(p).to_string();
    };

    // 替换拼音中的带声调字符
    let py = p
        .chars()
        .map(|c| {
            match PHONETIC_SYMBOL_MAP.binary_search_by_key(&c, |&(k, _)| k) {
                Ok(index) => {
                    let symbol = PHONETIC_SYMBOL_MAP[index].1;
                    match a.style {
                        // 不包含声调
                        Style::Normal | Style::FirstLetter | Style::Finals => {
                            // 去掉声调: a1 -> a
                            symbol
                                .chars()
                                .filter(|c: &char| {
                                    // NOTE: 该方法在 rustc 1.17.0 (56124baa9 2017-04-24) 版本当中需要引入 `use std::ascii::AsciiExt;`
                                    // !c.is_ascii_digit()
                                    *c != '0' && *c != '1' && *c != '2' && *c != '3' && *c != '4'
                                })
                                .collect::<String>()
                        }
                        Style::Tone2 | Style::FinalsTone2 => {
                            // 返回使用数字标识声调的字符
                            symbol.to_string()
                        }
                        _ => {
                            // 声调在头上
                            c.to_string()
                        }
                    }
                }
                Err(_) => c.to_string(),
            }
        })
        .collect::<String>();

    match a.style {
        // 首字母
        Style::FirstLetter => py.chars().nth(0).unwrap().to_string(),
        // 韵母
        Style::Finals | Style::FinalsTone | Style::FinalsTone2 => _final(&py),
        _ => py,
    }
}

fn apply_style(pys: Vec<String>, a: &Args) -> Vec<String> {
    let mut new_pys: Vec<String> = vec![];
    for v in pys {
        let s = to_fixed(&v, a);
        new_pys.push(s);
    }
    new_pys
}

fn single_pinyin(c: char, a: &Args) -> Vec<String> {
    let ret: Vec<String> = PINYIN_MAP
        .binary_search_by_key(&c, |&(k, _)| k)
        .map(|index| {
            let pinyin_list = PINYIN_MAP[index].1.split(',').collect::<Vec<&str>>();
            if pinyin_list.is_empty() || a.heteronym {
                pinyin_list
                    .iter()
                    .map(|pinyin| pinyin.to_string())
                    .collect::<Vec<String>>()
            } else {
                vec![pinyin_list[0].to_string()]
            }
        })
        .unwrap_or_default();
    apply_style(ret, a)
}

/// 汉字转拼音
///
/// ```
/// let hans = "中国人";
/// let args = pinyin::Args::new();
///
/// // 默认输出 [["zhong"] ["guo"] ["ren"]]
/// println!("{:?}",  pinyin::pinyin(hans, &args));
/// ```
pub fn pinyin(s: &str, a: &Args) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        ret.push(single_pinyin(c, a));
    }

    ret
}

/// 汉字转拼音, 与 ``pinyin`` 的区别是返回值不同，每个汉字只取一个音
///
/// ```
/// let hans = "中国人";
/// let args = pinyin::Args::new();
///
/// // 默认输出 ["zhong", "guo", "ren"]
/// println!("{:?}",  pinyin::lazy_pinyin(hans, &args));
/// ```
pub fn lazy_pinyin(s: &str, a: &Args) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for pinyin_vc in pinyin(s, a) {
        if !pinyin_vc.is_empty() {
            ret.push(pinyin_vc[0].to_string());
        }
    }

    ret
}
