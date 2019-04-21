use dict::PINYIN_MAP;
use integer_hasher::IntegerHashMap;

lazy_static! {
    pub static ref PINYIN_HASHMAP: IntegerHashMap<char, &'static str> =
        PINYIN_MAP.iter().cloned().collect();
}
