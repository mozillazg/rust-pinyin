use pinyin::ToPinyinMulti;

#[test]
fn special_pinyin() {
    assert_eq!(
        list_all_heteronym('欸'),
        &["ai1", "ai3", "ê1", "ê2", "ê3", "ê4", "xie4", "ei2", "ei3", "ei4", "ei1"],
    );
    assert_eq!(list_all_heteronym('嘸'), &["fu3", "wu3", "m1", "m2"]);
    assert_eq!(list_all_heteronym('呣'), &["m2", "m4", "mou2"]);
}

fn list_all_heteronym(ch: char) -> Vec<&'static str> {
    ch.to_pinyin_multi()
        .unwrap()
        .into_iter()
        .map(|pinyin| pinyin.with_tone_num_end())
        .collect::<Vec<_>>()
}
