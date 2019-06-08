use crate::data::CHAR_BLOCKS;
use std::convert::{identity, TryFrom};

#[cfg(feature = "compat")]
mod compat;
mod data;
mod pinyin;
#[cfg(feature = "heteronym")]
mod pinyin_multi;

#[cfg(feature = "compat")]
pub use crate::compat::*;
pub use crate::pinyin::{Pinyin, PinyinStrIter, ToPinyin};
#[cfg(feature = "heteronym")]
pub use crate::pinyin_multi::{PinyinMulti, PinyinMultiIter, PinyinMultiStrIter, ToPinyinMulti};

/// 将给定输入字符串的拼音通过给定映射函数后存入 `Vec` 中
///
/// 这个函数会跳过任何没有拼音的字符。本函数主要用于测试目的。
pub fn to_pinyin_vec<F>(input: &str, f: F) -> Vec<&'static str>
where
    F: Fn(Pinyin) -> &'static str,
{
    input.to_pinyin().filter_map(identity).map(f).collect()
}

/// 单个字符的拼音数据
struct PinyinData {
    #[cfg(feature = "plain")]
    plain: &'static str,
    #[cfg(feature = "with_tone")]
    with_tone: &'static str,
    #[cfg(feature = "with_tone_num")]
    with_tone_num: &'static str,
    #[cfg(feature = "with_tone_num_end")]
    with_tone_num_end: &'static str,
    #[cfg(feature = "compat")]
    split: usize,
}

/// 在 [start, end) 之间字符的数据块
struct CharBlock {
    /// 本块的第一个字符
    start_code: u32,
    /// 本块字符的数据索引
    /// 零值表示对应字符没有拼音数据，非零值表示对应的拼音数据为 `PINYIN_DATA[i]`。
    data: &'static [u16],
    /// 本块字符对应的多音字数据索引
    /// 对应的多音字数据为 `HETERONYM_TABLE[i]`。
    #[cfg(feature = "heteronym")]
    heteronym: &'static [u16],
}

#[inline]
fn get_block_and_index(ch: char) -> Option<(&'static CharBlock, usize)> {
    let code = u32::from(ch);
    for block in CHAR_BLOCKS.iter() {
        if code < block.start_code {
            return None;
        }
        let idx = usize::try_from(code - block.start_code).unwrap();
        if idx < block.data.len() {
            return Some((block, idx));
        }
    }
    None
}
