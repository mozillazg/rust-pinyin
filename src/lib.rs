#[macro_use]
extern crate log;
extern crate phf;
extern crate regex;

use regex::Captures;
use regex::Regex;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Debug)]
pub enum Style {
    // 普通风格，不带声调（默认风格）。如： `pin yin`
    Normal,
    // 声调风格1，拼音声调在韵母第一个字母上。如： `pīn yīn`
    Tone,
    // 声调风格2，即拼音声调在各个拼音之后，用数字 [0-4] 进行表示。如： `pi1n yi1n`
    Tone2,
    // 声母风格，只返回各个拼音的声母部分。如： 中国 的拼音 `zh g`
    Initials,
    // 首字母风格，只返回拼音的首字母部分。如： `p y`
    FirstLetter,
    // 韵母风格1，只返回各个拼音的韵母部分，不带声调。如： `ong uo`
    Finals,
    // 韵母风格2，带声调，声调在韵母第一个字母上。如： `ōng uó`
    FinalsTone,
    // 韵母风格2，带声调，声调在各个拼音之后，用数字 [0-4] 进行表示。如： `o1ng uo2`
    FinalsTone2,
}

// 声母表
const _INITIALS: [&'static str; 24] = [
    "zh", "ch", "sh", "b", "p", "m", "f", "d", "t", "n", "l", "g",
    "k", "h", "j", "q", "x", "r", "z", "c", "s", "yu", "y", "w",
];

// 带声调字符
// const phonetic_symbol: [&'static str; 27] = [
//     "ā", "á", "ǎ", "à", "ē", "é", "ě", "è", "ō", "ó", "ǒ", "ò",
//     "ī", "í", "ǐ", "ì", "ū", "ú", "ǔ", "ù", "ü", "ǘ", "ǚ", "ǜ",
//     "ń", "ň", "",
// ];
// 匹配带声调字符的正则表达式
// const re_phonetic_symbol: Regex = Regex::new(
//     r"[āáǎàēéěèōóǒòīíǐìūúǔùüǘǚǜńň]"
// ).unwrap();

// 匹配使用数字标识声调的字符的正则表达式
// const re_tone2:Regex = Regex::new(r"([aeoiuvnm])([0-4])$").unwrap();

//
pub struct Args {
    pub style:     Style,    // 拼音风格（默认： NORMAL)
    pub heteronym: bool,   // 是否启用多音字模式（默认：禁用）
    pub separator: String, // Slug 中使用的分隔符（默认：-)
}

impl Args {
    pub fn new() -> Args {
        Args {
            style: Style::Normal,
            heteronym: false,
            separator: "-".to_string(),
        }
    }
}

// 获取单个拼音中的声母
fn initial(p: String) -> String {
    let mut s = "".to_string();
    for v in _INITIALS.iter() {
        if p.starts_with(v) {
            s = v.to_string();
            break;
        }
    }
    s
}

// 获取单个拼音中的韵母
fn _final(p: &str) -> String {
    let i = initial(p.to_string());
    if i == "" {
        return p.to_string();
    }
    let s: Vec<&str> = p.splitn(2, &i).collect();
    s.concat()
}

fn to_fixed<'a>(p: String, a: &'a Args) -> String {
    match a.style {
        Style::Initials => {
            return initial(p).to_string();
        },
        _ => {},
    };

    let re_phonetic_symbol = Regex::new(
        r"(?i)[āáǎàēéěèōóǒòīíǐìūúǔùüǘǚǜńň]"
    ).unwrap();

    // 匹配使用数字标识声调的字符的正则表达式
    let re_tone2 = Regex::new(r"([aeoiuvnm])([0-4])$").unwrap();

    // 替换拼音中的带声调字符
    let py = re_phonetic_symbol.replace_all(&p, |caps: &Captures| {
        let cap = caps.at(0).unwrap();
        println!("{}", cap);

        let symbol = match PHONETIC_SYMBOL_MAP.get(&cap) {
            Some(&v) => v,
            None => "",
        };
        println!("{}", symbol);

        let m: String;
        match a.style {
            // 不包含声调
            Style::Normal | Style::FirstLetter | Style::Finals => {
                // 去掉声调: a1 -> a
                m = re_tone2.replace_all(symbol, "$1");
            },
            Style::Tone2 | Style::FinalsTone2 => {
                // 返回使用数字标识声调的字符
                m = symbol.to_string();
            },
            _ => {
                // 声调在头上
                m = cap.to_string();
            },
        }
        m
    });
    println!("{}", py);

    let ret = match a.style {
        // 首字母
        Style::FirstLetter => {
            py.chars().nth(0).unwrap().to_string()
        },
        // 韵母
        Style::Finals | Style::Tone | Style::Tone2 => {
            _final(&py)
        },
        _ => py,
    };

    println!("{}", ret);
    ret
}

fn apply_style<'a>(pys: Vec<String>, a: &'a Args) -> Vec<String> {
    let mut new_pys: Vec<String> = vec![];
    for v in pys {
        let s = to_fixed(v, a);
        new_pys.push(s);
    }
    new_pys
}

fn single_pinyin<'a>(c: char, a: &'a Args) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let n: u32 = c as u32;

    match PINYIN_MAP.get(&n) {
        Some(&pys) => {
            let x: Vec<&str> = pys.split(',').collect();
            if x.len() == 0 || a.heteronym {
                for s in x {
                    ret.push(s.to_string());
                };
            } else {
                ret = vec![x[0].to_string()];
            }
        },
        None => {
            ret = vec![];
        }
    };

    println!("{:?}", ret);
    apply_style(ret, a)
}

pub fn pinyin<'a>(s: &'a str, a: &'a Args) -> Vec<Vec<String>> {
    println!("{}, {:?}, {}, {}", s, a.style, a.heteronym, a.separator);

    let mut ret: Vec<Vec<String>> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        ret.push(single_pinyin(c, a));
    }

    return ret
}
