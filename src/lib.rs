#[macro_use]
extern crate lazy_static;

use std::ascii::AsciiExt;
use std::collections::HashMap;

// pub use pinyin_dict;
mod pinyin_dict;
mod phonetic_symbol;

lazy_static! {
    static ref PINYINMAP: HashMap<&'static str, &'static str> = {
        let m = pinyin_dict::init();
        m
    };
    static ref PHONETICSYMBOL: HashMap<&'static str, &'static str> = {
        let m = phonetic_symbol::init();
        m
    };
}

enum Style {
    // 普通风格，不带声调（默认风格）。如： pin yin
    Normal,
    // 声调风格1，拼音声调在韵母第一个字母上。如： pīn yīn
    Tone,
    // 声调风格2，即拼音声调在各个拼音之后，用数字 [0-4] 进行表示。如： pi1n yi1n
    Tone2,
    // 声母风格，只返回各个拼音的声母部分。如： 中国 的拼音 zh g
    Initials,
    // 首字母风格，只返回拼音的首字母部分。如： p y
    FirstLetter,
    // 韵母风格1，只返回各个拼音的韵母部分，不带声调。如： ong uo
    Finals,
    // 韵母风格2，带声调，声调在韵母第一个字母上。如： ōng uó
    FinalsTone,
    // 韵母风格2，带声调，声调在各个拼音之后，用数字 [0-4] 进行表示。如： o1ng uo2
    FinalsTone2,
}

// 声母表
const initials: [&'static str; 24] = [
    "zh", "ch", "sh", "b", "p", "m", "f", "d", "t", "n", "l", "g",
    "k", "h", "j", "q", "x", "r", "z", "c", "s", "yu", "y", "w",
];

// 所有带声调的字符
const rePhoneticSymbol: &'static str = (
    "āáǎàēéěèōóǒòīíǐìūúǔùüǘǚǜńň"
);

// // 匹配带声调字符的正则表达式
// var rePhoneticSymbol = regexp.MustCompile("[" + rePhoneticSymbolSource + "]")
//
// // 匹配使用数字标识声调的字符的正则表达式
// var reTone2 = regexp.MustCompile("([aeoiuvnm])([0-4])$")
//
// Args 配置信息
pub struct Args {
    style:     Style,    // 拼音风格（默认： NORMAL)
    heteronym: bool,   // 是否启用多音字模式（默认：禁用）
    separator: String, // Slug 中使用的分隔符（默认：-)
}

impl Args {
    fn New() -> Args {
        Args{
            style: Style::Normal,
            heteronym: false,
            separator: "-".to_string(),
        }
    }
}

// 获取单个拼音中的声母
fn initial(p: &str) -> &str {
    let mut s = "";
    for v in initials.iter() {
        if p.starts_with(v) {
            s = v;
            break;
        }
    }
    s
}

// 获取单个拼音中的韵母
fn _final(p: &str) -> String {
    let i = initial(p);
    if i == "" {
        return p.to_string();
    }
    let s: Vec<&str> = p.splitn(2, i).collect();
    s.concat()
}

// func toFixed(p string, a Args) string {
//     if a.Style == Initials {
//         return initial(p)
//     }
//
//     // 替换拼音中的带声调字符
//     py := rePhoneticSymbol.ReplaceAllStringFunc(p, func(m string) string {
//         symbol, _ := phoneticSymbol[m]
//         switch a.Style {
//         // 不包含声调
//         case Normal, FirstLetter, Finals:
//             // 去掉声调: a1 -> a
//             m = reTone2.ReplaceAllString(symbol, "$1")
//         case Tone2, FinalsTone2:
//             // 返回使用数字标识声调的字符
//             m = symbol
//         default:
//             //  // 声调在头上
//         }
//         return m
//     })
//
//     switch a.Style {
//     // 首字母
//     case FirstLetter:
//         py = string([]byte(py)[0])
//     // 韵母
//     case Finals, FinalsTone, FinalsTone2:
//         py = final(py)
//     }
//     return py
// }
//
// func applyStyle(p []string, a Args) []string {
//     newP := []string{}
//     for _, v := range p {
//         newP = append(newP, toFixed(v, a))
//     }
//     return newP
// }
//
// // SinglePinyin 把单个 `rune` 类型的汉字转换为拼音.
// func SinglePinyin(r rune, a Args) []string {
//     value, ok := PinyinDict[int(r)]
//     pys := []string{}
//     if ok {
//         if len(value) < 1 || a.Heteronym {
//             pys = strings.Split(value, ",")
//         } else {
//             pys = strings.Split(value, ",")[:1]
//         }
//     }
//     return applyStyle(pys, a)
// }

// Pinyin 汉字转拼音，支持多音字模式.
pub fn pinyin(s: &str) -> Vec<&str> {
    let mut ret = vec![""];
    let v: Vec<char> = s.chars().collect();
    for n in v {
        let m1: String = n.escape_unicode().collect();
        // let m2: String = m1.trim_matches(' ').to_ascii_uppercase();
        // let m: &str = m2.trim_matches(' ');
        let m: &str = m1.trim_matches(' ');
        match PINYINMAP.get(m) {
            Some(&pys) => {
                ret = pys.split(',').collect();
            },
            None => {
            }
        }
    }
    return ret
}
