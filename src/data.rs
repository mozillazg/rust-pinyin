#![allow(clippy::unreadable_literal)]

use crate::{CharBlock, PinyinData};
use lazy_static::lazy_static;
use seq_macro::seq;
use std::collections::HashMap;

pub(crate) static PINYIN_DATA: &[PinyinData] =
    include!(concat!(env!("OUT_DIR"), "/pinyin_data.rs"));

#[cfg(feature = "heteronym")]
pub(crate) static HETERONYM_TABLE: &[&[u16]] =
    include!(concat!(env!("OUT_DIR"), "/heteronym_table.rs"));

pub(crate) static CHAR_BLOCKS: &[CharBlock] = include!(concat!(env!("OUT_DIR"), "/char_blocks.rs"));

seq!(N in 2..=9 {
    lazy_static! {
        #(pub(crate) static ref PHRASE_TABLE_~N: HashMap<&'static str, &'static [u16; N]> =
            HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_", stringify!(N), ".rs")));)*
    }
});
