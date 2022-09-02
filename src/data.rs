#![allow(clippy::unreadable_literal)]

use crate::{CharBlock, PinyinData};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) static PINYIN_DATA: &[PinyinData] =
    include!(concat!(env!("OUT_DIR"), "/pinyin_data.rs"));

#[cfg(feature = "heteronym")]
pub(crate) static HETERONYM_TABLE: &[&[u16]] =
    include!(concat!(env!("OUT_DIR"), "/heteronym_table.rs"));

pub(crate) static CHAR_BLOCKS: &[CharBlock] = include!(concat!(env!("OUT_DIR"), "/char_blocks.rs"));

pub(crate) static PHRASE_TABLE_HETERONYMS: &[(&'static str, &[&[u16]])] =
    include!(concat!(env!("OUT_DIR"), "/phrase_table_heteronyms.rs"));

lazy_static! {
    pub(crate) static ref PHRASE_TABLE_2: HashMap<&'static str, &'static [u16; 2]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_2.rs")));
    pub(crate) static ref PHRASE_TABLE_3: HashMap<&'static str, &'static [u16; 3]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_3.rs")));
    pub(crate) static ref PHRASE_TABLE_4: HashMap<&'static str, &'static [u16; 4]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_4.rs")));
    pub(crate) static ref PHRASE_TABLE_5: HashMap<&'static str, &'static [u16; 5]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_5.rs")));
    pub(crate) static ref PHRASE_TABLE_6: HashMap<&'static str, &'static [u16; 6]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_6.rs")));
    pub(crate) static ref PHRASE_TABLE_7: HashMap<&'static str, &'static [u16; 7]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_7.rs")));
    pub(crate) static ref PHRASE_TABLE_8: HashMap<&'static str, &'static [u16; 8]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_8.rs")));
    pub(crate) static ref PHRASE_TABLE_9: HashMap<&'static str, &'static [u16; 9]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_9.rs")));
    pub(crate) static ref PHRASE_TABLE_10: HashMap<&'static str, &'static [u16; 10]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_10.rs")));
    pub(crate) static ref PHRASE_TABLE_11: HashMap<&'static str, &'static [u16; 11]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_11.rs")));
    pub(crate) static ref PHRASE_TABLE_12: HashMap<&'static str, &'static [u16; 12]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_12.rs")));
    pub(crate) static ref PHRASE_TABLE_13: HashMap<&'static str, &'static [u16; 13]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_13.rs")));
    pub(crate) static ref PHRASE_TABLE_14: HashMap<&'static str, &'static [u16; 14]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_14.rs")));
    pub(crate) static ref PHRASE_TABLE_15: HashMap<&'static str, &'static [u16; 15]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_15.rs")));
    pub(crate) static ref PHRASE_TABLE_16: HashMap<&'static str, &'static [u16; 16]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_16.rs")));
    pub(crate) static ref PHRASE_TABLE_17: HashMap<&'static str, &'static [u16; 17]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_17.rs")));
    pub(crate) static ref PHRASE_TABLE_18: HashMap<&'static str, &'static [u16; 18]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_18.rs")));
    pub(crate) static ref PHRASE_TABLE_19: HashMap<&'static str, &'static [u16; 19]> =
        HashMap::from(include!(concat!(env!("OUT_DIR"), "/phrase_table_19.rs")));
}
