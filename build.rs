use std::borrow::Cow;
use std::char;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

const RAW_DATA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/pinyin-data/pinyin.txt"
));

#[cfg(any(
    feature = "plain",
    feature = "with_tone_num",
    feature = "with_tone_num_end"
))]
const PHONETIC_SYMBOL_MAP: &[(char, char, u8)] = &[
    ('ā', 'a', 1),
    ('á', 'a', 2),
    ('ǎ', 'a', 3),
    ('à', 'a', 4),
    ('ē', 'e', 1),
    ('é', 'e', 2),
    ('ě', 'e', 3),
    ('è', 'e', 4),
    ('ế', 'ê', 2),
    ('ề', 'ê', 4),
    ('ō', 'o', 1),
    ('ó', 'o', 2),
    ('ǒ', 'o', 3),
    ('ò', 'o', 4),
    ('ī', 'i', 1),
    ('í', 'i', 2),
    ('ǐ', 'i', 3),
    ('ì', 'i', 4),
    ('ū', 'u', 1),
    ('ú', 'u', 2),
    ('ǔ', 'u', 3),
    ('ù', 'u', 4),
    ('ǘ', 'ü', 2),
    ('ǚ', 'ü', 3),
    ('ǜ', 'ü', 4),
    ('ń', 'n', 2),
    ('ň', 'n', 3),
    ('ǹ', 'n', 4),
    ('ḿ', 'm', 2),
];

#[rustfmt::skip]
const LETTER_TABLE: &[char] = &[
    'b', 'p', 'm', 'f', 'd',
    't', 'n', 'l', 'g', 'k',
    'h', 'j', 'q', 'x', 'r',
    'z', 'c', 's',
                   'w', 'y',
    // 因为数据源里面不会使用 `v` 以及其它的 简写字母，所以这里注释掉
    // 'v', 'ẑ', 'ĉ', 'ŝ', 'ŋ',

    '\u{0304}', '\u{030C}', '\u{0300}', // Unicode 声调连字符
    'a', 'ā', 'á', 'ǎ', 'à',
    'e', 'ē', 'é', 'ě', 'è',
    'i', 'ī', 'í', 'ǐ', 'ì',
    //   "m̄"       "m̌"  "m̀"
    'm',      'ḿ',
    //   "n̄"
    'n',      'ń', 'ň', 'ǹ',
    'o', 'ō', 'ó', 'ǒ', 'ò',
    'u', 'ū', 'ú', 'ǔ', 'ù',
    //   "ê̄"       "ê̌"
    'ê',      'ế',      'ề',
    //   'ǖ'
    'ü',      'ǘ', 'ǚ', 'ǜ',
];

#[cfg(any(feature = "with_tone_num", feature = "with_tone_num_end"))]
const TONE_NUMS: &[char] = &['0', '1', '2', '3', '4'];

type Style = (&'static str, fn(&str) -> Cow<'_, str>);
type InputData = Vec<(u32, Vec<&'static str>)>;
type PinyinDataIndex = HashMap<&'static str, usize>;
type HeteronymDataIndex = HashMap<u32, usize>;

fn main() -> io::Result<()> {
    let data = build_data();
    let pinyin_index = generate_pinyin_data(&data)?;
    let heteronym_index = generate_heteronym_table(&data, &pinyin_index)?;
    generate_char_table(&data, &pinyin_index, &heteronym_index)?;
    // 输出这行以保证改动项目的其他文件不会触发编译脚本重新执行
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn build_data() -> InputData {
    let mut input_data = RAW_DATA
        .lines()
        .enumerate()
        // 移除注释和空格
        .map(|(i, mut line)| {
            if let Some(hash_pos) = line.find('#') {
                line = &line[..hash_pos];
            }
            (i, line.trim())
        })
        // 移除空行
        .filter(|(_, line)| !line.is_empty())
        .map(|(i, line)| {
            // Split the line by colon
            let colon_pos = match line.find(':') {
                Some(pos) => pos,
                None => unreachable!("no colon found in line {}", i),
            };
            let code_point = line[..colon_pos].trim();
            let pinyin_list: Vec<_> = line[colon_pos + 1..].trim().split(',').collect();

            // 确保输入数据的字符全部在我们预料之中。
            // 同时也可以提前知道一些被遗弃的码位，如: U+E7C8 和 U+E7C7
            for pinyin in pinyin_list.iter() {
                for ch in pinyin.chars() {
                    let is_known = LETTER_TABLE.contains(&ch);
                    assert!(
                        is_known,
                        "unknown character {:?} at line {}: {}",
                        ch, i, line,
                    );
                }
            }

            // 解析码位
            const CODE_POINT_PREFIX: &str = "U+";
            assert!(code_point.starts_with(CODE_POINT_PREFIX));
            let code = &code_point[CODE_POINT_PREFIX.len()..];
            let code = match u32::from_str_radix(code, 16) {
                Ok(code) => code,
                Err(_) => unreachable!("invalid code point {} at line {}", code, i),
            };
            (code, pinyin_list)
        })
        .collect::<Vec<_>>();
    input_data.sort_by_key(|(code, _)| *code);
    input_data
}

const STYLES: &[Style] = &[
    #[cfg(feature = "plain")]
    ("plain", |input| {
        input.chars().filter_map(|c| get_char_info(c).0).collect()
    }),
    #[cfg(feature = "with_tone")]
    ("with_tone", |input| Cow::from(input)),
    #[cfg(feature = "with_tone_num")]
    ("with_tone_num", |input| {
        let mut result = String::new();
        for ch in input.chars() {
            let (ch, tone) = get_char_info(ch);
            if let Some(ch) = ch {
                result.push(ch);
            }
            if tone > 0 {
                result.push(TONE_NUMS[usize::try_from(tone).unwrap()]);
            }
        }
        result.into()
    }),
    #[cfg(feature = "with_tone_num_end")]
    ("with_tone_num_end", |input| {
        let mut result = String::new();
        let mut output_tone = None;
        for ch in input.chars() {
            let (ch, tone) = get_char_info(ch);
            if let Some(ch) = ch {
                result.push(ch);
            }
            if tone > 0 {
                assert!(output_tone.is_none());
                output_tone = Some(TONE_NUMS[usize::try_from(tone).unwrap()]);
            }
        }
        if let Some(tone) = output_tone {
            result.push(tone);
        }
        result.into()
    }),
];

fn generate_pinyin_data(data: &InputData) -> io::Result<PinyinDataIndex> {
    let mut output = create_out_file("pinyin_data.rs")?;
    let mut pinyin_data = HashMap::new();
    writeln!(output, "&[")?;
    let mut process_pinyin = |pinyin| {
        let index = pinyin_data.len();
        match pinyin_data.entry(pinyin) {
            Entry::Occupied(_) => return Ok(()),
            Entry::Vacant(entry) => {
                entry.insert(index);
            }
        }
        write!(output, "    PinyinData {{ ")?;
        for (field, converter) in STYLES.iter() {
            write!(output, r#"{}: "{}", "#, field, converter(pinyin))?;
        }
        #[cfg(feature = "compat")]
        {
            // 计算切分声母和韵母的位置
            const INITIALS: &[&str] = &[
                "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "j", "q", "x", "r", "zh",
                "ch", "sh", "z", "c", "s",
            ];
            let split = INITIALS
                .iter()
                .find(|initial| pinyin.starts_with(*initial))
                .map_or(0, |initial| initial.len());
            write!(output, "split: {}, ", split)?;
        }
        writeln!(output, "}},")?;
        Ok(())
    };
    // 插入一个空的拼音数据作为零位
    process_pinyin("")?;
    data.iter()
        .flat_map(|(_, list)| list.iter().map(|s| *s))
        .map(process_pinyin)
        .collect::<io::Result<()>>()?;
    writeln!(output, "]")?;
    Ok(pinyin_data)
}

#[cfg(feature = "heteronym")]
fn generate_heteronym_table(
    data: &InputData,
    index: &PinyinDataIndex,
) -> io::Result<HeteronymDataIndex> {
    let mut heteronym_list_index = HashMap::new();
    let mut heteronym_index = HashMap::new();
    let mut output = create_out_file("heteronym_table.rs")?;
    writeln!(output, "&[")?;
    writeln!(output, "    &[],")?;
    heteronym_list_index.insert(vec![].into_boxed_slice(), 0);
    data.iter()
        .map(|(code, list)| {
            let list = list[1..]
                .iter()
                .map(|pinyin| *index.get(pinyin).unwrap())
                .collect::<Box<[_]>>();
            let new_idx = heteronym_list_index.len();
            let idx = match heteronym_list_index.entry(list) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    write!(output, "    &[")?;
                    for i in entry.key().iter() {
                        write!(output, "{}, ", i)?;
                    }
                    writeln!(output, "],")?;
                    entry.insert(new_idx);
                    new_idx
                }
            };
            heteronym_index.insert(*code, idx);
            Ok(())
        })
        .collect::<io::Result<()>>()?;
    writeln!(output, "]")?;
    Ok(heteronym_index)
}

#[cfg(not(feature = "heteronym"))]
fn generate_heteronym_table(
    _data: &InputData,
    _index: &PinyinDataIndex,
) -> io::Result<HeteronymDataIndex> {
    Ok(HashMap::new())
}

fn generate_char_table(
    data: &InputData,
    pinyin_index: &PinyinDataIndex,
    heteronym_index: &HeteronymDataIndex,
) -> io::Result<()> {
    // 将码位分入数据块
    const GAP_THRESHOLD: u32 = 2048;
    let mut block_ranges = vec![];
    data.iter()
        .for_each(|(code, _)| match block_ranges.last_mut() {
            Some((_, end)) if *end + GAP_THRESHOLD > *code => *end = *code + 1,
            _ => block_ranges.push((*code, *code + 1)),
        });
    // 当我们允许最大2048个空位时，我们目前会切出3个块。如果这个数字在未来增加了，我们也许会希望调整策略。
    assert_eq!(block_ranges.len(), 3);

    // 输出字符表
    let mut output = create_out_file("char_blocks.rs")?;
    writeln!(output, "&[")?;
    let mut data_iter = data.iter().peekable();
    for (start, end) in block_ranges {
        let len = usize::try_from(end - start).unwrap();
        let mut block = vec![0; len];
        let mut heteronym = vec![0; len];
        while let Some((code, list)) = data_iter.peek() {
            let idx = usize::try_from(*code - start).unwrap();
            if idx >= len {
                break;
            }
            block[idx] = *pinyin_index.get(list[0]).unwrap();
            if cfg!(feature = "heteronym") {
                heteronym[idx] = *heteronym_index.get(code).unwrap();
            }
            data_iter.next();
        }
        write!(output, "    CharBlock {{ start_code: {}, data: &[", start)?;
        for idx in block {
            write!(output, "{}, ", idx)?;
        }
        write!(output, "], ")?;
        if cfg!(feature = "heteronym") {
            write!(output, "heteronym: &[")?;
            for idx in heteronym {
                write!(output, "{}, ", idx)?;
            }
            write!(output, "], ")?;
        }
        writeln!(output, "}},")?;
    }
    writeln!(output, "]")?;
    Ok(())
}

fn create_out_file(name: &str) -> io::Result<impl Write> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(name);
    Ok(BufWriter::new(File::create(&path)?))
}

#[cfg(any(
    feature = "plain",
    feature = "with_tone_num",
    feature = "with_tone_num_end"
))]
fn get_char_info(ch: char) -> (Option<char>, u8) {
    if let Some((_, base, tone)) = PHONETIC_SYMBOL_MAP.iter().find(|(c, _, _)| *c == ch) {
        return (Some(*base), *tone);
    }
    const TONE_MAP: &[(char, u8)] = &[('\u{304}', 1), ('\u{30c}', 3), ('\u{300}', 4)];
    if let Some((_, tone)) = TONE_MAP.iter().find(|(c, _)| *c == ch) {
        return (None, *tone);
    }
    (Some(ch), 0)
}
