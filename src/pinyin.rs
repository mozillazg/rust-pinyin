use crate::data::*;
use crate::{get_block_and_index, PinyinData};
use seq_macro::seq;
use std::convert::TryFrom;
use std::slice::Iter;
use std::str::Chars;

/// 单个字符的拼音信息
#[derive(Copy, Clone)]
pub struct Pinyin(pub(crate) &'static PinyinData);

impl Pinyin {
    /// 普通风格，不带声调
    ///
    /// *仅在启用 `plain` 特性时可用*
    /// ```
    /// # use pinyin::*;
    /// assert_eq!(to_pinyin_vec("拼音", Pinyin::plain), vec!["pin", "yin"]);
    /// ```
    #[cfg(feature = "plain")]
    pub fn plain(self) -> &'static str {
        self.0.plain
    }

    /// 带声调的风格
    ///
    /// *仅在启用 `with_tone` 特性时可用*
    /// ```
    /// # use pinyin::*;
    /// assert_eq!(to_pinyin_vec("拼音", Pinyin::with_tone), vec!["pīn", "yīn"]);
    /// ```
    #[cfg(feature = "with_tone")]
    pub fn with_tone(self) -> &'static str {
        self.0.with_tone
    }

    /// 声调在各个拼音之后，使用数字1-4表示的风格
    ///
    /// *仅在启用 `with_tone_num` 特性时可用*
    /// ```
    /// # use pinyin::*;
    /// assert_eq!(to_pinyin_vec("拼音", Pinyin::with_tone_num), vec!["pi1n", "yi1n"]);
    /// ```
    #[cfg(feature = "with_tone_num")]
    pub fn with_tone_num(self) -> &'static str {
        self.0.with_tone_num
    }

    /// 声调在拼音最后，使用数字1-4表示的风格
    ///
    /// *仅在启用 `with_tone_num_end` 特性时可用*
    /// ```
    /// # use pinyin::*;
    /// assert_eq!(to_pinyin_vec("拼音", Pinyin::with_tone_num_end), vec!["pin1", "yin1"]);
    /// ```
    #[cfg(feature = "with_tone_num_end")]
    pub fn with_tone_num_end(self) -> &'static str {
        self.0.with_tone_num_end
    }

    /// 首字母风格
    ///
    /// *仅在启用 `plain` 特性时可用*
    /// ```
    /// # use pinyin::*;
    /// assert_eq!(to_pinyin_vec("拼音", Pinyin::first_letter), vec!["p", "y"]);
    /// assert_eq!(to_pinyin_vec("中国", Pinyin::first_letter), vec!["z", "g"]);
    /// assert_eq!(to_pinyin_vec("安心", Pinyin::first_letter), vec!["a", "x"]);
    /// ```
    #[cfg(feature = "plain")]
    pub fn first_letter(self) -> &'static str {
        let ch = self.0.plain.chars().next().unwrap();
        &self.0.plain[..ch.len_utf8()]
    }

    #[cfg(feature = "compat")]
    pub(crate) fn initials(self) -> &'static str {
        &self.0.plain[..self.0.split]
    }

    #[cfg(feature = "compat")]
    pub(crate) fn finals_plain(self) -> &'static str {
        &self.0.plain[self.0.split..]
    }

    #[cfg(feature = "compat")]
    pub(crate) fn finals_with_tone(self) -> &'static str {
        &self.0.with_tone[self.0.split..]
    }

    #[cfg(feature = "compat")]
    pub(crate) fn finals_with_tone_num(self) -> &'static str {
        &self.0.with_tone_num[self.0.split..]
    }
}

/// 用于获取拼音信息的trait
pub trait ToPinyin {
    type Output;
    fn to_pinyin(&self) -> Self::Output;
}

/// ```
/// # #[cfg(feature = "plain")] {
/// use pinyin::ToPinyin;
/// assert_eq!('拼'.to_pinyin().unwrap().plain(), "pin");
/// # }
/// ```
impl ToPinyin for char {
    type Output = Option<Pinyin>;

    fn to_pinyin(&self) -> Option<Pinyin> {
        get_block_and_index(*self).and_then(|(block, index)| {
            match usize::try_from(block.data[index]).unwrap() {
                0 => None,
                idx => Some(Pinyin(&PINYIN_DATA[idx])),
            }
        })
    }
}

/// ```
/// # #[cfg(feature = "plain")] {
/// use pinyin::{ToPinyin, Pinyin};
/// let mut iter = "拼音".to_pinyin();
/// let mut next_plain = || iter.next().and_then(|p| p).map(Pinyin::plain);
/// assert_eq!(next_plain(), Some("pin"));
/// assert_eq!(next_plain(), Some("yin"));
/// assert_eq!(next_plain(), None);
/// # }
/// ```
impl<'a> ToPinyin for &'a str {
    type Output = PinyinStrIter<'a>;

    #[inline]
    fn to_pinyin(&self) -> Self::Output {
        PinyinStrIter(self.chars())
    }
}

/// *辅助迭代器*，用于获取字符串的拼音信息
pub struct PinyinStrIter<'a>(Chars<'a>);

impl<'a> Iterator for PinyinStrIter<'a> {
    type Item = Option<Pinyin>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| c.to_pinyin())
    }
}

pub struct PinyinPhraseIter<'a>(Iter<'a, &'a str>);

impl<'a> Iterator for PinyinPhraseIter<'a> {
    type Item = Option<Vec<Pinyin>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|s| {
            // Important: Always stay in sync with MAX_PHRASE_LENGTH in build.rs
            seq!(N in 2..=9 {
                match s.chars().count() {
                    1 => s
                        .chars()
                        .next()
                        .unwrap()
                        .to_pinyin()
                        .map(|pinyin| vec![pinyin]),
                    #(N => match PHRASE_TABLE_~N.get(s) {
                        Some(pinyin_indices) => Some(
                            pinyin_indices
                                .iter()
                                .map(|idx| Pinyin(&PINYIN_DATA[*idx as usize]))
                                .collect(),
                        ),
                        None => {
                            s.to_pinyin().collect()
                        },
                    },)*
                    _ => s.to_pinyin().collect(),
                }
            })
        })
    }
}

impl<'a> ToPinyin for Iter<'a, &'a str> {
    type Output = PinyinPhraseIter<'a>;

    fn to_pinyin(&self) -> Self::Output {
        PinyinPhraseIter(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pinyin, ToPinyin};

    #[test]
    fn special_code_point() {
        assert!('\u{10FFFF}'.to_pinyin().is_none());
    }

    #[test]
    fn phrase_pinyin() {
        [
            (vec!["重新"], vec![Some(vec!["chóng", "xīn"])]),
            (vec!["同行"], vec![Some(vec!["tóng", "háng"])]),
            // tone may change, wait for issue: https://github.com/mozillazg/phrase-pinyin-data/issues/43
            (
                vec!["便宜", "便宜货"],
                vec![Some(vec!["pián", "yí"]), Some(vec!["pián", "yí", "huò"])],
            ),
            (
                vec!["贪便宜", "便宜从事"],
                vec![
                    Some(vec!["tān", "pián", "yí"]),
                    Some(vec!["biàn", "yí", "cóng", "shì"]),
                ],
            ),
            (vec!["打量"], vec![Some(vec!["dǎ", "liàng"])]),
            (
                vec!["薄荷", "薄弱", "衣服", "薄"],
                vec![
                    Some(vec!["bò", "hé"]),
                    Some(vec!["bó", "ruò"]),
                    Some(vec!["yī", "fú"]),
                    Some(vec!["báo"]),
                ],
            ),
            (
                vec!["高血压", "流血"],
                vec![Some(vec!["gāo", "xuè", "yā"]), Some(vec!["liú", "xiě"])],
            ),
            // "大喝一声" is out-of-vocabulary right now,
            // so we comment that out for now
            (
                vec![
                    // "大喝一声",
                    "喝水", "喝彩",
                ],
                vec![
                    // Some(vec!["dà", "hè", "yī", "shēng"]),
                    Some(vec!["hē", "shuǐ"]),
                    Some(vec!["hè", "cǎi"]),
                ],
            ),
            (vec!["\u{10FFFF}"], vec![None]),
            (vec!["\u{10FFFF}你好"], vec![None]),
        ]
        .iter()
        .for_each(|(phrase, pinyin)| {
            assert_eq!(
                &phrase
                    .iter()
                    .to_pinyin()
                    .map(|pinyins| pinyins.map(|pinyins| pinyins
                        .iter()
                        .map(|p| Pinyin::with_tone(*p))
                        .collect::<Vec<_>>()))
                    .collect::<Vec<_>>(),
                pinyin
            )
        })
    }
}
