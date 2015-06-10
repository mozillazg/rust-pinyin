use std::collections::HashMap;

// 带音标字符。
pub fn init() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    m.insert("ā", "a1");
    m.insert("á", "a2");
    m.insert("ǎ", "a3");
    m.insert("à", "a4");
    m.insert("ē", "e1");
    m.insert("é", "e2");
    m.insert("ě", "e3");
    m.insert("è", "e4");
    m.insert("ō", "o1");
    m.insert("ó", "o2");
    m.insert("ǒ", "o3");
    m.insert("ò", "o4");
    m.insert("ī", "i1");
    m.insert("í", "i2");
    m.insert("ǐ", "i3");
    m.insert("ì", "i4");
    m.insert("ū", "u1");
    m.insert("ú", "u2");
    m.insert("ǔ", "u3");
    m.insert("ù", "u4");
    m.insert("ü", "v0");
    m.insert("ǘ", "v2");
    m.insert("ǚ", "v3");
    m.insert("ǜ", "v4");
    m.insert("ń", "n2");
    m.insert("ň", "n3");
    m.insert("", "m2");

    m
}
