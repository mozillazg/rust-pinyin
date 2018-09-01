use std::collections::HashSet;

fn main() {
    let dict = include_str!("../../pinyin-data/pinyin.txt");

    // 带声调字符 PHONETIC_SYMBOL_MAP
    let mut phonetic_symbol_map = vec![
        ('ā', "a1"),
        ('á', "a2"),
        ('ǎ', "a3"),
        ('à', "a4"),
        ('ē', "e1"),
        ('é', "e2"),
        ('ě', "e3"),
        ('è', "e4"),
        ('ō', "o1"),
        ('ó', "o2"),
        ('ǒ', "o3"),
        ('ò', "o4"),
        ('ī', "i1"),
        ('í', "i2"),
        ('ǐ', "i3"),
        ('ì', "i4"),
        ('ū', "u1"),
        ('ú', "u2"),
        ('ǔ', "u3"),
        ('ù', "u4"),
        ('ü', "v0"),
        ('ǘ', "v2"),
        ('ǚ', "v3"),
        ('ǜ', "v4"),
        ('ń', "n2"),
        ('ň', "n3"),
        ('', "m2"),
    ];
    let phonetic_symbol_set = phonetic_symbol_map
        .iter()
        .map(|item| item.0)
        .collect::<HashSet<char>>();
    assert_eq!(phonetic_symbol_map.len(), phonetic_symbol_set.len());

    // 拼音库    PINYIN_MAP
    let mut pinyin_map: Vec<(char, String)> = Vec::new();
    let mut pinyin_set: HashSet<char> = HashSet::new();

    for line in dict.lines() {
        let line = line.trim();

        if !line.starts_with('#') {
            let kv = line.split(':').collect::<Vec<&str>>();
            assert_eq!(kv.len() >= 2, true);

            let k = kv[0].trim();
            let v = kv[1].trim();

            let c: char = {
                assert_eq!(k.starts_with("U+"), true);
                let tmp = k.split("U+").collect::<Vec<&str>>();
                assert_eq!(tmp.len(), 2);

                let code_point = u32::from_str_radix(tmp[1], 16).unwrap();
                let c = ::std::char::from_u32(code_point).unwrap();
                c
            };

            let pinyin_list: Vec<String> = {
                let tmp = v.split('#').collect::<Vec<&str>>();
                assert_eq!(tmp.len(), 2);
                let pinyin_list = tmp[0]
                    .split(',')
                    .map(|item| item.trim().to_string())
                    .collect::<Vec<String>>();

                let comment = tmp[1].trim().chars().collect::<Vec<char>>()[0];
                assert_eq!(comment, c);

                pinyin_list
            };

            if pinyin_set.insert(c) {
                let item = (c, pinyin_list.join(","));
                pinyin_map.push(item);
            } else {
                println!("[DEBUG] 重复的数据: {}", line);
            }
        }
    }
    assert_eq!(pinyin_map.len(), pinyin_set.len());

    // NOTE: 使用稳定版的 sort
    phonetic_symbol_map.sort_by_key(|&(k, _)| k);
    pinyin_map.sort_by_key(|&(k, _)| k);

    let template = format!(
        "

pub static PINYIN_MAP: [(char, &str); {}] = {:?};
pub static PHONETIC_SYMBOL_MAP: [(char, &str); {}] = {:?};

    ",
        pinyin_map.len(),
        pinyin_map,
        phonetic_symbol_map.len(),
        phonetic_symbol_map
    );

    println!("{}", template);
}
