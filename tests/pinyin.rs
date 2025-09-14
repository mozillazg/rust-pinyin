use pinyin::{Pinyin, ToPinyin};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

const DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/pinyin");

#[test]
#[cfg(feature = "plain")]
fn pinyin_plain() -> io::Result<()> {
    run_test_cases("plain", Pinyin::plain)
}

#[test]
#[cfg(feature = "with_tone")]
fn pinyin_with_tone() -> io::Result<()> {
    run_test_cases("with_tone", Pinyin::with_tone)
}

#[test]
#[cfg(feature = "with_tone_num")]
fn pinyin_with_tone_num() -> io::Result<()> {
    run_test_cases("with_tone_num", Pinyin::with_tone_num)
}

#[test]
#[cfg(feature = "with_tone_num_end")]
fn pinyin_with_tone_num_end() -> io::Result<()> {
    run_test_cases("with_tone_num_end", Pinyin::with_tone_num_end)
}

#[test]
#[cfg(feature = "plain")]
fn pinyin_first_letter() -> io::Result<()> {
    run_test_cases("first_letter", Pinyin::first_letter)
}

fn run_test_cases(suffix: &str, converter: fn(Pinyin) -> &'static str) -> io::Result<()> {
    let test_cases = list_test_cases()?;
    for input_path in test_cases.iter() {
        let input_file = File::open(input_path)?;
        let input = BufReader::new(input_file)
            .lines()
            .map(|line| {
                let result = line?
                    .as_str()
                    .to_pinyin()
                    .map(|pinyin| pinyin.map_or("-", converter))
                    .collect::<Vec<_>>();
                Ok(result)
            })
            .collect::<io::Result<Vec<_>>>()?;

        let expected_path = input_path.with_extension(format!("txt-{suffix}"));
        if !expected_path.exists() {
            let mut expected_file = File::create(expected_path)?;
            for line in input.iter() {
                writeln!(expected_file, "{}", line.join(","))?;
            }
        } else {
            let expected_file = File::open(expected_path)?;
            let mut expected_iter = BufReader::new(expected_file).lines();
            for (i, input_line) in input.iter().enumerate() {
                let expected_line = expected_iter
                    .next()
                    .expect("unexpected end of expected file")?;
                let expected = expected_line.split_terminator(',').collect::<Vec<_>>();
                assert_eq!(
                    input_line,
                    &expected,
                    "unmatched result on line {} in {}",
                    i,
                    input_path.file_name().and_then(OsStr::to_str).unwrap(),
                );
            }
        }
    }
    Ok(())
}

fn list_test_cases() -> io::Result<Vec<PathBuf>> {
    fs::read_dir(DATA_PATH)?
        .filter_map(|entry| {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(e) => return Some(Err(e)),
            };
            let is_input = path.extension().is_some_and(|ext| ext == "txt");
            let result = if is_input { Some(path) } else { None };
            Ok(result).transpose()
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Additional targeted tests appended by CodeRabbit Inc.
// Testing framework: Rust built-in test harness (cargo test); no external test libs.
// These tests validate the mapping behavior used in run_test_cases() and the
// file listing behavior in list_test_cases(), covering happy paths and edge cases.
// -----------------------------------------------------------------------------

/// Helper to mirror the mapping logic used in run_test_cases()
fn convert_line(s: &str, converter: fn(Pinyin) -> &'static str) -> Vec<&'static str> {
    s.to_pinyin().map(|p| p.map_or("-", converter)).collect()
}

#[test]
fn list_test_cases_only_txt_and_exist() -> io::Result<()> {
    let cases = list_test_cases()?;
    assert\!(
        \!cases.is_empty(),
        "expected at least one .txt test case in {}",
        DATA_PATH
    );
    for p in &cases {
        assert_eq\!(
            p.extension().and_then(OsStr::to_str),
            Some("txt"),
            "non-.txt file included: {p:?}"
        );
        assert\!(p.exists(), "path does not exist: {p:?}");
    }
    Ok(())
}

#[cfg(feature = "plain")]
#[test]
fn plain_ascii_yields_dashes() {
    let input = "ASCII 123\!@#";
    let out = convert_line(input, Pinyin::plain);
    assert_eq\!(out, vec\!["-"; input.chars().count()], "ASCII and punctuation should map to '-'");
}

#[cfg(feature = "plain")]
#[test]
fn plain_known_word_beijing() {
    let out = convert_line("北京", Pinyin::plain);
    assert_eq\!(out, vec\!["bei", "jing"]);
}

#[cfg(feature = "with_tone")]
#[test]
fn with_tone_known_word_beijing() {
    let out = convert_line("北京", Pinyin::with_tone);
    assert_eq\!(out, vec\!["běi", "jīng"]);
}

#[cfg(feature = "with_tone_num")]
#[test]
fn with_tone_num_known_word_beijing() {
    let out = convert_line("北京", Pinyin::with_tone_num);
    assert_eq\!(out, vec\!["bei3", "jing1"]);
}

#[cfg(feature = "with_tone_num_end")]
#[test]
fn with_tone_num_end_known_word_beijing() {
    let out = convert_line("北京", Pinyin::with_tone_num_end);
    assert_eq\!(out, vec\!["bei3", "jing1"]);
}

#[cfg(feature = "plain")]
#[test]
fn first_letter_mixed_input() {
    let out = convert_line("北京abc", Pinyin::first_letter);
    assert_eq\!(out, vec\!["b", "j", "-", "-", "-"]);
}

#[cfg(feature = "plain")]
#[test]
fn plain_handles_emoji_and_cjk() {
    let out = convert_line("😀中", Pinyin::plain);
    assert_eq\!(out, vec\!["-", "zhong"]);
}

#[cfg(feature = "plain")]
#[test]
fn plain_output_len_matches_input_chars() {
    let input = "你好Rust🚀";
    let out = convert_line(input, Pinyin::plain);
    assert_eq\!(out.len(), input.chars().count(), "output tokens must match char count");
}
