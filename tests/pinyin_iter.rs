use pinyin::ToPinyin;

#[test]
fn test_pinyin_str_iter_clone() {
    let mut iter = "你好".to_pinyin();
    let first = iter.next();
    assert!(first.is_some());

    let mut cloned_iter = iter.clone();
    assert_eq!(iter.next(), cloned_iter.next());
    assert_eq!(iter.next(), cloned_iter.next());
}

#[test]
#[cfg(feature = "heteronym")]
fn test_pinyin_multi_str_iter_clone() {
    use pinyin::ToPinyinMulti;
    let mut iter = "还没".to_pinyin_multi();
    let first = iter.next();
    assert!(first.is_some());

    let mut cloned_iter = iter.clone();
    assert_eq!(iter.next(), cloned_iter.next());
    assert_eq!(iter.next(), cloned_iter.next());
}
