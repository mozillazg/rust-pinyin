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
        return TestCase {
            hans: hans,
            args: args,
            result: result
                .into_iter()
                .map(|vec| vec.into_iter().map(String::from).collect())
                .collect(),
            lazy_result: lazy_result.into_iter().map(String::from).collect(),
        };
    }
}
