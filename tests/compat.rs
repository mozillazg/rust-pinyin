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
