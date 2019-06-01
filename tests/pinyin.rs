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

        let expected_path = input_path.with_extension(format!("txt-{}", suffix));
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
            let is_input = path.extension().map_or(false, |ext| ext == "txt");
            let result = if is_input { Some(path) } else { None };
            Ok(result).transpose()
        })
        .collect()
}
