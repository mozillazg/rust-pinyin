#![allow(clippy::unreadable_literal)]

use crate::{CharBlock, PinyinData};
use std::collections::HashMap;
use lazy_static::lazy_static;

pub(crate) static PINYIN_DATA: &[PinyinData] =
    include!(concat!(env!("OUT_DIR"), "/pinyin_data.rs"));

#[cfg(feature = "heteronym")]
pub(crate) static HETERONYM_TABLE: &[&[u16]] =
    include!(concat!(env!("OUT_DIR"), "/heteronym_table.rs"));

pub(crate) static CHAR_BLOCKS: &[CharBlock] = include!(concat!(env!("OUT_DIR"), "/char_blocks.rs"));

lazy_static! {
    pub(crate) static ref PHRASE_TABLE: HashMap<&'static str, &'static [u16; 19]> = {
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table.rs")))
    };
}
