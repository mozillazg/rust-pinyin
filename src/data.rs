#![allow(clippy::unreadable_literal)]

use crate::{CharBlock, PinyinData};
use std::collections::HashMap;

pub(crate) static PINYIN_DATA: &[PinyinData] =
    include!(concat!(env!("OUT_DIR"), "/pinyin_data.rs"));

#[cfg(feature = "heteronym")]
pub(crate) static HETERONYM_TABLE: &[&[u16]] =
    include!(concat!(env!("OUT_DIR"), "/heteronym_table.rs"));

pub(crate) static CHAR_BLOCKS: &[CharBlock] = include!(concat!(env!("OUT_DIR"), "/char_blocks.rs"));

pub(crate) static PHRASE_TABLE: &[(&'static str, &[&[usize]])] =
    include!(concat!(env!("OUT_DIR"), "/phrase_table.rs"));
