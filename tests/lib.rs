extern crate pinyin;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct TestCase {
    args: pinyin::Args,
    result: Vec<Vec<String>>,
}

impl TestCase {
    pub fn new(args: pinyin::Args, result: Vec<Vec<String>>) -> TestCase {
        TestCase{args: args, result: result}
    }
}


#[test]
fn test_pinyin() {
    let hans = "中国人";
    let mut testDate = HashMap::new();

    testDate.insert(TestCase::new(pinyin::Args::new(),
                                  vec![vec!["zhong".to_string()],
                                       vec!["guo".to_string()],
                                       vec!["ren".to_string()]]
                                  ));
    for (args, result) in &testDate {
         assert_eq!(result, pinyin::pinyin(hans, args));
    }
    // let testData = map[Args][][]string{
    //     Args{}: [][]string{
    //         []string{"zhong"},
    //         []string{"guo"},
    //         []string{"ren"},
    //     },
    //     Args{Style: Normal}: [][]string{
    //         []string{"zhong"},
    //         []string{"guo"},
    //         []string{"ren"},
    //     },
    //     Args{Style: Tone}: [][]string{
    //         []string{"zhōng"},
    //         []string{"guó"},
    //         []string{"rén"},
    //     },
    //     Args{Style: Tone2}: [][]string{
    //         []string{"zho1ng"},
    //         []string{"guo2"},
    //         []string{"re2n"},
    //     },
    //     Args{Style: Initials}: [][]string{
    //         []string{"zh"},
    //         []string{"g"},
    //         []string{"r"},
    //     },
    //     Args{Style: FirstLetter}: [][]string{
    //         []string{"z"},
    //         []string{"g"},
    //         []string{"r"},
    //     },
    //     Args{Style: Finals}: [][]string{
    //         []string{"ong"},
    //         []string{"uo"},
    //         []string{"en"},
    //     },
    //     Args{Style: FinalsTone}: [][]string{
    //         []string{"ōng"},
    //         []string{"uó"},
    //         []string{"én"},
    //     },
    //     Args{Style: FinalsTone2}: [][]string{
    //         []string{"o1ng"},
    //         []string{"uo2"},
    //         []string{"e2n"},
    //     },
    //     Args{Heteronym: true}: [][]string{
    //         []string{"zhong", "zhong"},
    //         []string{"guo"},
    //         []string{"ren"},
    //     },
    // }
}
