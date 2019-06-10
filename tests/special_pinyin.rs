use pinyin::ToPinyinMulti;

#[test]
fn special_pinyin() {
    assert_eq!(
        list_all_heteronym('欸'),
        &["ai1", "ai3", "xie4", "ê2", "ei2", "ê3", "ei3", "ê4", "ei4", "ê1", "ei1"],
    );
    assert_eq!(list_all_heteronym('嘸'), &["fu3", "wu3", "m1", "m2"]);
    assert_eq!(list_all_heteronym('呣'), &["m2", "mou2", "m4"]);
}

fn list_all_heteronym(ch: char) -> Vec<&'static str> {
    ch.to_pinyin_multi()
        .unwrap()
        .into_iter()
        .map(|pinyin| pinyin.with_tone_num_end())
        .collect::<Vec<_>>()
}
