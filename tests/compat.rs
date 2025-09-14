#![allow(deprecated)]

extern crate pinyin;

pub struct TestCase {
    pub hans: String,
    pub args: pinyin::Args,
    pub result: Vec<Vec<String>>,
    pub lazy_result: Vec<String>,
}

impl TestCase {
    pub fn new(
        hans: String,
        args: pinyin::Args,
        result: Vec<Vec<String>>,
        lazy_result: Vec<String>,
    ) -> TestCase {
        TestCase {
            hans,
            args,
            result,
            lazy_result,
        }
    }

    pub fn new2(
        hans: String,
        args: pinyin::Args,
        result: Vec<Vec<&str>>,
        lazy_result: Vec<&str>,
    ) -> TestCase {
        TestCase {
            hans,
            args,
            result: result
                .into_iter()
                .map(|vec| vec.into_iter().map(String::from).collect())
                .collect(),
            lazy_result: lazy_result.into_iter().map(String::from).collect(),
        }
    }
}

#[test]
fn test_pinyin() {
    let test_data = vec![
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args::new(),
            vec![
                vec!["zhong".to_string()],
                vec!["guo".to_string()],
                vec!["ren".to_string()],
            ],
            vec!["zhong".to_string(), "guo".to_string(), "ren".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::Normal,
                heteronym: false,
            },
            vec![
                vec!["zhong".to_string()],
                vec!["guo".to_string()],
                vec!["ren".to_string()],
            ],
            vec!["zhong".to_string(), "guo".to_string(), "ren".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::Tone,
                heteronym: false,
            },
            vec![
                vec!["zhōng".to_string()],
                vec!["guó".to_string()],
                vec!["rén".to_string()],
            ],
            vec!["zhōng".to_string(), "guó".to_string(), "rén".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::Tone2,
                heteronym: false,
            },
            vec![
                vec!["zho1ng".to_string()],
                vec!["guo2".to_string()],
                vec!["re2n".to_string()],
            ],
            vec!["zho1ng".to_string(), "guo2".to_string(), "re2n".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::Initials,
                heteronym: false,
            },
            vec![
                vec!["zh".to_string()],
                vec!["g".to_string()],
                vec!["r".to_string()],
            ],
            vec!["zh".to_string(), "g".to_string(), "r".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::FirstLetter,
                heteronym: false,
            },
            vec![
                vec!["z".to_string()],
                vec!["g".to_string()],
                vec!["r".to_string()],
            ],
            vec!["z".to_string(), "g".to_string(), "r".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::Finals,
                heteronym: false,
            },
            vec![
                vec!["ong".to_string()],
                vec!["uo".to_string()],
                vec!["en".to_string()],
            ],
            vec!["ong".to_string(), "uo".to_string(), "en".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::FinalsTone,
                heteronym: false,
            },
            vec![
                vec!["ōng".to_string()],
                vec!["uó".to_string()],
                vec!["én".to_string()],
            ],
            vec!["ōng".to_string(), "uó".to_string(), "én".to_string()],
        ),
        TestCase::new(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::FinalsTone2,
                heteronym: false,
            },
            vec![
                vec!["o1ng".to_string()],
                vec!["uo2".to_string()],
                vec!["e2n".to_string()],
            ],
            vec!["o1ng".to_string(), "uo2".to_string(), "e2n".to_string()],
        ),
        TestCase::new2(
            "中国人".to_string(),
            pinyin::Args {
                style: pinyin::Style::Normal,
                heteronym: true,
            },
            vec![vec!["zhong"], vec!["guo"], vec!["ren"]],
            vec!["zhong", "guo", "ren"],
        ),
        TestCase::new2(
            "阿拉巴".to_string(),
            pinyin::Args {
                style: pinyin::Style::Normal,
                heteronym: true,
            },
            vec![vec!["a", "e"], vec!["la"], vec!["ba"]],
            vec!["a", "la", "ba"],
        ),
    ];
    for data in &test_data {
        assert_eq!(data.result, pinyin::pinyin(&data.hans, &data.args));
        assert_eq!(
            data.lazy_result,
            pinyin::lazy_pinyin(&data.hans, &data.args)
        );
    }
}

#[test]
fn test_non_chinese_pinyin() {
    let hans = "中国人abc你好";
    let expect = vec![
        vec!["zhong".to_string()],
        vec!["guo".to_string()],
        vec!["ren".to_string()],
        vec![],
        vec![],
        vec![],
        vec!["ni".to_string()],
        vec!["hao".to_string()],
    ];
    let result = pinyin::pinyin(hans, &pinyin::Args::new());
    assert_eq!(expect, result);
}

#[test]
fn test_non_chinese_lazy_pinyin() {
    let hans = "中国人abc你好";
    let expect = vec![
        "zhong".to_string(),
        "guo".to_string(),
        "ren".to_string(),
        "ni".to_string(),
        "hao".to_string(),
    ];
    let result = pinyin::lazy_pinyin(hans, &pinyin::Args::new());
    assert_eq!(expect, result);
}

#[test]
fn test_new_args() {
    let args = pinyin::Args::new();
    assert_eq!(pinyin::Style::Normal, args.style);
    assert!(!args.heteronym);

    let expected = pinyin::Args {
        style: pinyin::Style::Normal,
        heteronym: false,
    };
    assert_eq!(expected, args);
}

#[test]
fn test_default_args() {
    let args: pinyin::Args = Default::default();
    assert_eq!(pinyin::Style::Normal, args.style);
    assert!(!args.heteronym);

    let args = pinyin::Args::default();
    assert_eq!(pinyin::Style::Normal, args.style);
    assert!(!args.heteronym);

    let expected = pinyin::Args {
        style: pinyin::Style::Normal,
        heteronym: false,
    };
    assert_eq!(expected, args);
}

#[test]
fn test_no_initial() {
    let hans = "安";
    let mut expect = vec!["an".to_string()];
    let mut result = pinyin::lazy_pinyin(hans, &pinyin::Args::new());
    assert_eq!(expect, result);

    expect = vec!["an".to_string()];
    result = pinyin::lazy_pinyin(
        hans,
        &pinyin::Args {
            style: pinyin::Style::Finals,
            heteronym: false,
        },
    );
    assert_eq!(expect, result);
}

#[test]
fn test_no_phonetic_symbol() {
    let hans = "啊";
    let mut expect = vec!["a".to_string()];
    let mut result = pinyin::lazy_pinyin(hans, &pinyin::Args::new());
    assert_eq!(expect, result);

    expect = vec!["a".to_string()];
    result = pinyin::lazy_pinyin(
        hans,
        &pinyin::Args {
            style: pinyin::Style::Finals,
            heteronym: false,
        },
    );
    assert_eq!(expect, result);
}

// Additional tests appended by PR automation.
// Testing framework: Rust built-in test harness (cargo test). No external test libraries used.

#[test]
fn test_empty_input_behaviour() {
    // Empty input should yield empty outputs for both APIs.
    let hans = "";
    let args = pinyin::Args::new();

    let result_full = pinyin::pinyin(hans, &args);
    let result_lazy = pinyin::lazy_pinyin(hans, &args);

    assert\!(result_full.is_empty(), "pinyin(\"\") should be empty, got {:?}", result_full);
    assert\!(result_lazy.is_empty(), "lazy_pinyin(\"\") should be empty, got {:?}", result_lazy);
}

#[test]
fn test_whitespace_and_punctuation_handling() {
    // Non-Chinese characters, including ASCII space and CJK punctuation, should map to empty entries in pinyin(),
    // and be skipped in lazy_pinyin().
    let hans = "中 国，！人";
    // Expected per-character mapping for pinyin():
    // '中' -> ["zhong"], ' ' -> [], '国' -> ["guo"], '，' -> [], '！' -> [], '人' -> ["ren"]
    let expect_full = vec\![
        vec\!["zhong".to_string()],
        vec\![],
        vec\!["guo".to_string()],
        vec\![],
        vec\![],
        vec\!["ren".to_string()],
    ];
    let result_full = pinyin::pinyin(hans, &pinyin::Args::new());
    assert_eq\!(expect_full, result_full, "pinyin() should preserve position with empties for non-Chinese");

    // lazy_pinyin() should skip non-Chinese and return only Chinese pinyin in order.
    let expect_lazy = vec\!["zhong".to_string(), "guo".to_string(), "ren".to_string()];
    let result_lazy = pinyin::lazy_pinyin(hans, &pinyin::Args::new());
    assert_eq\!(expect_lazy, result_lazy, "lazy_pinyin() should skip non-Chinese characters");
}

#[test]
fn test_emoji_and_mixed_symbols() {
    // Emoji and symbols are non-Chinese; ensure they produce empty entries in pinyin() and are skipped in lazy_pinyin().
    let hans = "中😊国#人";
    let expect_full = vec\![
        vec\!["zhong".to_string()],
        vec\![],
        vec\!["guo".to_string()],
        vec\![],
        vec\!["ren".to_string()],
    ];
    let expect_lazy = vec\!["zhong".to_string(), "guo".to_string(), "ren".to_string()];

    let result_full = pinyin::pinyin(hans, &pinyin::Args::new());
    let result_lazy = pinyin::lazy_pinyin(hans, &pinyin::Args::new());

    assert_eq\!(expect_full, result_full);
    assert_eq\!(expect_lazy, result_lazy);
}

#[test]
fn test_single_char_heteronym_multiple_pronunciations_normal_style() {
    // Validate heteronym behavior for a known multi-pronunciation character "阿".
    // In Normal style with heteronym=true, expect multiple readings in one slot (order as provided by the library).
    let hans = "阿";
    let args = pinyin::Args { style: pinyin::Style::Normal, heteronym: true };

    let result_full = pinyin::pinyin(hans, &args);
    assert_eq\!(1, result_full.len(), "Single character should produce one slot");
    let slot = &result_full[0];

    // We assert set inclusion to avoid order brittleness while still ensuring expected variants are present.
    // Known normal-form readings commonly include "a" and "e".
    assert\!(slot.contains(&"a".to_string()), "Expected heteronym readings to include \"a\", got {:?}", slot);
    assert\!(slot.contains(&"e".to_string()), "Expected heteronym readings to include \"e\", got {:?}", slot);

    // lazy_pinyin with heteronym=true should still pick a deterministic reading (the library's default first).
    let result_lazy = pinyin::lazy_pinyin(hans, &args);
    assert_eq\!(1, result_lazy.len(), "Single character lazy_pinyin should produce one item");
    assert\!(
        result_lazy[0] == "a" || result_lazy[0] == "e",
        "lazy_pinyin should pick a plausible default heteronym reading, got {:?}", result_lazy[0]
    );
}

#[test]
fn test_consistency_between_styles_on_simple_input() {
    // Cross-check that style transformations line up on a simple, unambiguous phrase.
    let hans = "中国人";

    // Normal
    let normal = pinyin::lazy_pinyin(hans, &pinyin::Args { style: pinyin::Style::Normal, heteronym: false });
    assert_eq\!(vec\!["zhong", "guo", "ren"], normal);

    // Initials and Finals should partition the syllables as expected.
    let initials = pinyin::lazy_pinyin(hans, &pinyin::Args { style: pinyin::Style::Initials, heteronym: false });
    let finals   = pinyin::lazy_pinyin(hans, &pinyin::Args { style: pinyin::Style::Finals, heteronym: false });
    assert_eq\!(vec\!["zh", "g", "r"], initials, "Initials should extract initial consonants");
    assert_eq\!(vec\!["ong", "uo", "en"], finals, "Finals should extract vowel/nasal components");

    // FirstLetter should match the first letter of Normal readings.
    let first_letter = pinyin::lazy_pinyin(hans, &pinyin::Args { style: pinyin::Style::FirstLetter, heteronym: false });
    let derived_firsts: Vec<String> = normal.iter().map(|s| s.chars().next().unwrap().to_string()).collect();
    assert_eq\!(derived_firsts, first_letter, "FirstLetter should equal first char of Normal");
}

#[test]
fn test_args_new_matches_default_and_is_copy_independent() {
    // Ensure Args::new and Default::default are consistent; modifying a clone should not affect the original.
    let mut a = pinyin::Args::new();
    let b = pinyin::Args::default();
    assert_eq\!(a, b, "Args::new should equal Args::default");

    // Mutate a copy and ensure they diverge.
    a.heteronym = true;
    assert_ne\!(a, b, "Changing one Args instance should not affect the other");
}

