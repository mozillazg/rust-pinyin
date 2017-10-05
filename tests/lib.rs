extern crate pinyin;

struct TestCase {
    args: pinyin::Args,
    result: Vec<Vec<String>>,
}

impl TestCase {
    pub fn new(args: pinyin::Args, result: Vec<Vec<String>>) -> TestCase {
        TestCase {
            args: args,
            result: result,
        }
    }
}


#[test]
fn test_pinyin() {
    let hans = "中国人";
    let test_data = vec![
        TestCase::new(
            pinyin::Args::new(),
            vec![
                vec!["zhong".to_string()],
                vec!["guo".to_string()],
                vec!["ren".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::Normal,
                heteronym: false,
            },
            vec![
                vec!["zhong".to_string()],
                vec!["guo".to_string()],
                vec!["ren".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::Tone,
                heteronym: false,
            },
            vec![
                vec!["zhōng".to_string()],
                vec!["guó".to_string()],
                vec!["rén".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::Tone2,
                heteronym: false,
            },
            vec![
                vec!["zho1ng".to_string()],
                vec!["guo2".to_string()],
                vec!["re2n".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::Initials,
                heteronym: false,
            },
            vec![
                vec!["zh".to_string()],
                vec!["g".to_string()],
                vec!["r".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::FirstLetter,
                heteronym: false,
            },
            vec![
                vec!["z".to_string()],
                vec!["g".to_string()],
                vec!["r".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::Finals,
                heteronym: false,
            },
            vec![
                vec!["ong".to_string()],
                vec!["uo".to_string()],
                vec!["en".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::FinalsTone,
                heteronym: false,
            },
            vec![
                vec!["ōng".to_string()],
                vec!["uó".to_string()],
                vec!["én".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::FinalsTone2,
                heteronym: false,
            },
            vec![
                vec!["o1ng".to_string()],
                vec!["uo2".to_string()],
                vec!["e2n".to_string()],
            ],
        ),
        TestCase::new(
            pinyin::Args {
                style: pinyin::Style::Normal,
                heteronym: true,
            },
            vec![
                vec!["zhong".to_string(), "zhong".to_string()],
                vec!["guo".to_string()],
                vec!["ren".to_string()],
            ],
        ),
    ];
    for data in &test_data {
        assert_eq!(&data.result, &pinyin::pinyin(hans, &data.args));

        let mut lazy_result: Vec<String> = Vec::new();
        for ret in &data.result {
            for pinyin in ret {
                lazy_result.push(pinyin.to_string());
                break;
            }
        }
        assert_eq!(lazy_result, pinyin::lazy_pinyin(hans, &data.args));
    }
}

#[test]
fn test_new_args() {
    let args = pinyin::Args::new();
    assert_eq!(pinyin::Style::Normal, args.style);
    assert_eq!(false, args.heteronym);
}
