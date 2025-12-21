#![allow(deprecated)]

use crate::{Pinyin, ToPinyin, ToPinyinMulti};
use std::collections::HashSet;

/// 拼音风格
#[deprecated = "请使用 `Pinyin` 的方法代替"]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Style {
    /// 普通风格，不带声调（默认风格）。如： `pin yin`
    Normal,
    /// 声调风格 1，拼音声调在韵母第一个字母上。如： `pīn yīn`
    Tone,
    /// 声调风格 2，即拼音声调在各个拼音之后，用数字 [0-4] 进行表示。如： `pi1n yi1n`
    Tone2,
    /// 声母风格，只返回各个拼音的声母部分。如：中国 的拼音 `zh g`
    Initials,
    /// 首字母风格，只返回拼音的首字母部分。如： `p y`
    FirstLetter,
    /// 韵母风格 1，只返回各个拼音的韵母部分，不带声调。如： `ong uo`
    Finals,
    /// 韵母风格 2，带声调，声调在韵母第一个字母上。如： `ōng uó`
    FinalsTone,
    /// 韵母风格 2，带声调，声调在各个拼音之后，用数字 [0-4] 进行表示。如： `o1ng uo2`
    FinalsTone2,
}

/// 参数
#[deprecated]
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

fn apply_style(py: Pinyin, style: &Style) -> &'static str {
    match style {
        Style::Normal => py.plain(),
        Style::Tone => py.with_tone(),
        Style::Tone2 => py.with_tone_num(),
        Style::Initials => py.initials(),
        Style::FirstLetter => py.first_letter(),
        Style::Finals => py.finals_plain(),
        Style::FinalsTone => py.finals_with_tone(),
        Style::FinalsTone2 => py.finals_with_tone_num(),
    }
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
#[deprecated = "请使用 `ToPinyin` 或 `ToPinyinMulti` 代替"]
pub fn pinyin(s: &str, a: &Args) -> Vec<Vec<String>> {
    if a.heteronym {
        s.to_pinyin_multi()
            .map(|multi| match multi {
                Some(multi) => {
                    let mut set = HashSet::new();
                    multi
                        .into_iter()
                        .map(|pinyin| apply_style(pinyin, &a.style))
                        .filter(|s| set.insert(*s))
                        .map(str::to_string)
                        .collect()
                }
                None => vec![],
            })
            .collect()
    } else {
        s.to_pinyin()
            .map(|pinyin| match pinyin {
                Some(pinyin) => vec![apply_style(pinyin, &a.style).to_string()],
                None => vec![],
            })
            .collect()
    }
}

/// 汉字转拼音，与 ``pinyin`` 的区别是返回值不同，每个汉字只取一个音
///
/// ```
/// let hans = "中国人";
/// let args = pinyin::Args::new();
///
/// // 默认输出 ["zhong", "guo", "ren"]
/// println!("{:?}",  pinyin::lazy_pinyin(hans, &args));
/// ```
#[deprecated = "请使用 `ToPinyin` 代替"]
pub fn lazy_pinyin(s: &str, a: &Args) -> Vec<String> {
    s.to_pinyin()
        .flatten()
        .map(|pinyin| apply_style(pinyin, &a.style).to_string())
        .collect()
}
