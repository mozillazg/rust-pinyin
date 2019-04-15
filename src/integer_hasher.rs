use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

pub struct IntegerHasher {
    hash: u64,
}

impl Default for IntegerHasher {
    #[inline]
    fn default() -> IntegerHasher {
        IntegerHasher { hash: 0 }
    }
}

impl Hasher for IntegerHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let _len = bytes.len();
        let mut hash: u64 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            hash += u64::from(*byte) << (8 * i);
        }

        self.hash = hash;
    }
}

pub type IntegerHashMap<K, V> = HashMap<K, V, BuildHasherDefault<IntegerHasher>>;
