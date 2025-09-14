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

// -----------------------------------------------------------------------------
// Additional tests to increase coverage on edge cases and style behaviors
// -----------------------------------------------------------------------------

#[test]
fn test_empty_input_returns_empty_collections() {
    // Empty string should yield empty vectors for both APIs.
    let hans = "";
    let p = pinyin::pinyin(hans, &pinyin::Args::new());
    let lp = pinyin::lazy_pinyin(hans, &pinyin::Args::new());
    assert\!(p.is_empty(), "pinyin() on empty input should be empty, got {:?}", p);
    assert\!(lp.is_empty(), "lazy_pinyin() on empty input should be empty, got {:?}", lp);
}

#[test]
fn test_only_non_chinese_characters_produce_empty_inner_vectors() {
    // For non-Chinese chars, pinyin() returns empty Vec per character; lazy_pinyin() skips them.
    let hans = "abc123\!? ";
    // Expect one empty Vec per input char
    let expect: Vec<Vec<String>> = hans.chars().map(|_| Vec::<String>::new()).collect();
    let result = pinyin::pinyin(hans, &pinyin::Args::new());
    assert_eq\!(expect, result, "Each non-Chinese char should map to empty possible readings.");

    // lazy_pinyin drops non-Chinese, thus produces empty output
    let lazy = pinyin::lazy_pinyin(hans, &pinyin::Args::new());
    assert\!(lazy.is_empty(), "lazy_pinyin should skip non-Chinese characters entirely.");
}

#[test]
fn test_whitespace_and_punctuation_handling() {
    // Mixture including ideographic comma and ASCII punctuation.
    let hans = "你，好！";
    // pinyin() should return entries for Chinese chars and empty for punctuation
    let args = pinyin::Args::new();
    let result = pinyin::pinyin(hans, &args);
    // Expected: "你" -> ["ni"], "，" -> [], "好" -> ["hao"], "！" -> []
    let expect = vec\![
        vec\!["ni".to_string()],
        vec\![],
        vec\!["hao".to_string()],
        vec\![],
    ];
    assert_eq\!(expect, result);

    // lazy_pinyin should drop punctuation
    let lazy = pinyin::lazy_pinyin(hans, &args);
    let expect_lazy = vec\!["ni".to_string(), "hao".to_string()];
    assert_eq\!(expect_lazy, lazy);
}

#[test]
fn test_styles_for_nihao_across_common_variants() {
    let hans = "你好";

    // Normal
    let args = pinyin::Args { style: pinyin::Style::Normal, heteronym: false };
    assert_eq\!(
        vec\![vec\!["ni".into()], vec\!["hao".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["ni".into(), "hao".into()], pinyin::lazy_pinyin(hans, &args));

    // Tone
    let args = pinyin::Args { style: pinyin::Style::Tone, heteronym: false };
    assert_eq\!(
        vec\![vec\!["nǐ".into()], vec\!["hǎo".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["nǐ".into(), "hǎo".into()], pinyin::lazy_pinyin(hans, &args));

    // Tone2 (numeric)
    let args = pinyin::Args { style: pinyin::Style::Tone2, heteronym: false };
    // Follow crate's Tone2 formatting seen in existing tests (e.g., "zho1ng")
    assert_eq\!(
        vec\![vec\!["ni3".into()], vec\!["ha3o".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["ni3".into(), "ha3o".into()], pinyin::lazy_pinyin(hans, &args));

    // Initials
    let args = pinyin::Args { style: pinyin::Style::Initials, heteronym: false };
    assert_eq\!(
        vec\![vec\!["n".into()], vec\!["h".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["n".into(), "h".into()], pinyin::lazy_pinyin(hans, &args));

    // Finals
    let args = pinyin::Args { style: pinyin::Style::Finals, heteronym: false };
    assert_eq\!(
        vec\![vec\!["i".into()], vec\!["ao".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["i".into(), "ao".into()], pinyin::lazy_pinyin(hans, &args));

    // FinalsTone
    let args = pinyin::Args { style: pinyin::Style::FinalsTone, heteronym: false };
    assert_eq\!(
        vec\![vec\!["ǐ".into()], vec\!["ǎo".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["ǐ".into(), "ǎo".into()], pinyin::lazy_pinyin(hans, &args));

    // FinalsTone2 (numeric)
    let args = pinyin::Args { style: pinyin::Style::FinalsTone2, heteronym: false };
    assert_eq\!(
        vec\![vec\!["i3".into()], vec\!["ao3".into()]],
        pinyin::pinyin(hans, &args)
    );
    assert_eq\!(vec\!["i3".into(), "ao3".into()], pinyin::lazy_pinyin(hans, &args));
}

#[test]
fn test_heteronym_false_returns_single_reading_even_when_multiple_exist() {
    // "阿" has multiple readings in heteronym mode in this crate (see existing tests).
    // With heteronym=false we should get exactly one reading.
    let hans = "阿";
    let args = pinyin::Args { style: pinyin::Style::Normal, heteronym: false };
    let result = pinyin::pinyin(hans, &args);
    assert_eq\!(1, result.len());
    assert_eq\!(1, result[0].len(), "heteronym=false should yield a single reading");
    // lazy_pinyin mirrors that single reading
    let lazy = pinyin::lazy_pinyin(hans, &args);
    assert_eq\!(vec\![result[0][0].clone()], lazy);
}

#[test]
fn test_mixed_content_variants_and_length_invariants() {
    // Ensure length invariants:
    // - pinyin(): length equals number of codepoints in input (including empty Vec for non-Chinese)
    // - lazy_pinyin(): length equals number of Chinese chars only
    let hans = "中A文👋B字9";
    let args = pinyin::Args::new();
    let all = pinyin::pinyin(hans, &args);
    assert_eq\!(hans.chars().count(), all.len(), "pinyin length mismatch");

    let only_chinese_lazy = pinyin::lazy_pinyin(hans, &args);
    // Count Chinese chars manually (basic heuristic: result entries in pinyin() with non-empty vectors)
    let chinese_count = all.iter().filter(|v| \!v.is_empty()).count();
    assert_eq\!(chinese_count, only_chinese_lazy.len(), "lazy_pinyin should include only Chinese chars");
}

#[test]
fn test_args_construction_with_heteronym_true() {
    // Validate public interface for Args with heteronym true.
    let args = pinyin::Args { style: pinyin::Style::Normal, heteronym: true };
    assert_eq\!(pinyin::Style::Normal, args.style);
    assert\!(args.heteronym);

    // Sanity check on heteronym behavior using known input from existing tests.
    let res = pinyin::pinyin("阿", &args);
    assert_eq\!(1, res.len());
    assert\!(res[0].len() >= 1, "heteronym=true should allow multiple readings when present");
}

#[test]
fn test_surrogate_and_emoji_are_skipped_in_lazy_but_counted_as_empty_in_pinyin() {
    // Include an emoji and a surrogate-like high codepoint alongside Chinese characters.
    let hans = "你👀好";
    let args = pinyin::Args::new();

    let full = pinyin::pinyin(hans, &args);
    // Expect: ["ni"], [], ["hao"]
    assert_eq\!(
        vec\![
            vec\!["ni".to_string()],
            vec\![],
            vec\!["hao".to_string()],
        ],
        full
    );

    let lazy = pinyin::lazy_pinyin(hans, &args);
    assert_eq\!(vec\!["ni".to_string(), "hao".to_string()], lazy);
}
