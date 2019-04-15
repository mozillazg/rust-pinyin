extern crate pinyin;
use pinyin::integer_hasher::IntegerHasher;
use std::hash::Hasher;

#[test]
fn test_integer_hasher_1() {
    test_integer_hasher_inner( 20464028);
}

#[test]
fn test_integer_hasher_2() {
    test_integer_hasher_inner( 0);
}

#[test]
fn test_integer_hasher_3() {
    test_integer_hasher_inner( 2046);
}

fn test_integer_hasher_inner(i:u64) {
    let mut _hasher = IntegerHasher::default();
    let expected: u64 = i;
    _hasher.write_u64(expected);
    let ret = _hasher.finish();
    assert_eq!(expected, ret);
}
