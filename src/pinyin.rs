use crate::data::PINYIN_DATA;
use crate::{get_block_and_index, PinyinData};
use std::convert::TryFrom;
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

    /// 声调在各个拼音之后，使用数字0-4表示的风格
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
