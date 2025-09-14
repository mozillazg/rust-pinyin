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
// Additional tests (Rust built-in #[test] harness; no external test libraries).
// These tests focus on robustness of list_test_cases and run_test_cases logic,
// plus direct verification of Pinyin converters (happy paths, edge cases).
// -----------------------------------------------------------------------------

// Generate a reasonably unique file stem to avoid collisions in parallel runs.
fn unique_name(prefix: &str) -> String {
    let pid = std::process::id();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format\!("{}_{}_{}", prefix, pid, ts)
}

#[test]
fn list_test_cases_filters_txt_files() -> io::Result<()> {
    // Ensure DATA_PATH exists
    let dir = PathBuf::from(DATA_PATH);
    fs::create_dir_all(&dir)?;

    // Create a .txt file that should be picked up and a non-.txt that should be ignored
    let stem = unique_name("list_cases");
    let input_path = dir.join(format\!("{stem}.txt"));
    let ignore_path = dir.join(format\!("{stem}.md"));

    fs::write(&input_path, "中\n")?;
    fs::write(&ignore_path, "ignored")?;

    let cases = list_test_cases()?;
    let names: Vec<_> = cases
        .iter()
        .filter_map(|p| p.file_name())
        .collect();

    assert\!(
        names.contains(&input_path.file_name().unwrap()),
        "Expected .txt input file to be listed"
    );
    assert\!(
        \!names.contains(&ignore_path.file_name().unwrap()),
        "Non-.txt file should not be listed"
    );

    // Cleanup
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&ignore_path);
    Ok(())
}

#[test]
#[cfg(feature = "plain")]
fn mapping_plain_and_first_letter_and_placeholder() {
    // Validates core mapping behavior and placeholder for non-Chinese chars.
    let s = "中国A";
    let plain = s
        .to_pinyin()
        .map(|p| p.map_or("-", Pinyin::plain))
        .collect::<Vec<_>>();
    assert_eq\!(plain, vec\!["zhong", "guo", "-"], "plain mapping mismatch");

    let first = s
        .to_pinyin()
        .map(|p| p.map_or("-", Pinyin::first_letter))
        .collect::<Vec<_>>();
    assert_eq\!(first, vec\!["z", "g", "-"], "first letter mapping mismatch");
}

#[test]
#[cfg(feature = "with_tone")]
fn mapping_with_tone_known_chars() {
    let s = "中国";
    let tone = s
        .to_pinyin()
        .map(|p| p.map_or("-", Pinyin::with_tone))
        .collect::<Vec<_>>();
    assert_eq\!(tone, vec\!["zhōng", "guó"], "tone mapping mismatch");
}

#[test]
#[cfg(feature = "with_tone_num")]
fn mapping_with_tone_num_known_chars() {
    let s = "中国";
    let tone = s
        .to_pinyin()
        .map(|p| p.map_or("-", Pinyin::with_tone_num))
        .collect::<Vec<_>>();
    assert_eq\!(tone, vec\!["zhong1", "guo2"], "tone-num mapping mismatch");
}

#[test]
#[cfg(feature = "with_tone_num_end")]
fn mapping_with_tone_num_end_known_chars() {
    let s = "中国";
    let tone = s
        .to_pinyin()
        .map(|p| p.map_or("-", Pinyin::with_tone_num_end))
        .collect::<Vec<_>>();
    assert_eq\!(tone, vec\!["zhong1", "guo2"], "tone-num-end mapping mismatch");
}

#[test]
#[cfg(feature = "plain")]
fn run_test_cases_creates_expected_when_missing_plain() -> io::Result<()> {
    // Create a new .txt input with no corresponding expected file and ensure
    // run_test_cases creates the expected file with correct contents.
    let dir = PathBuf::from(DATA_PATH);
    fs::create_dir_all(&dir)?;
    let stem = unique_name("create_missing");
    let input_path = dir.join(format\!("{stem}.txt"));
    let expected_path = input_path.with_extension("txt-plain");

    // Ensure a clean slate
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&expected_path);

    fs::write(&input_path, "中A\n")?;

    // Execute
    run_test_cases("plain", Pinyin::plain)?;

    assert\!(expected_path.exists(), "Expected file was not created");

    // Validate expected content equals mapping produced by converter
    let expected_line = {
        let file = File::open(&expected_path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        // first (and only) line
        reader.read_line(&mut line)?;
        line.trim_end().to_string()
    };

    let mapped = "中A"
        .to_pinyin()
        .map(|p| p.map_or("-", Pinyin::plain))
        .collect::<Vec<_>>()
        .join(",");

    assert_eq\!(expected_line, mapped, "Created expected content mismatch");

    // Cleanup
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&expected_path);
    Ok(())
}

#[test]
#[cfg(feature = "plain")]
fn run_test_cases_handles_empty_line_plain() -> io::Result<()> {
    // Ensure empty input lines are handled by producing blank expected lines.
    let dir = PathBuf::from(DATA_PATH);
    fs::create_dir_all(&dir)?;
    let stem = unique_name("empty_line");
    let input_path = dir.join(format\!("{stem}.txt"));
    let expected_path = input_path.with_extension("txt-plain");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&expected_path);

    // Write a single empty line
    fs::write(&input_path, "\n")?;

    run_test_cases("plain", Pinyin::plain)?;

    // Read first expected line; should be empty after trimming terminators.
    let expected_line = {
        let file = File::open(&expected_path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line.trim_end().to_string()
    };
    assert_eq\!(expected_line, "", "Empty input line should yield empty expected line");

    // Cleanup
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&expected_path);
    Ok(())
}

#[test]
#[cfg(feature = "plain")]
fn run_test_cases_panics_on_mismatch_plain() -> io::Result<()> {
    // Intentionally create a mismatched expected file and ensure run_test_cases panics
    // with a helpful message that includes line number and file name.
    let dir = PathBuf::from(DATA_PATH);
    fs::create_dir_all(&dir)?;
    let stem = unique_name("temp_mismatch");
    let input_path = dir.join(format\!("{stem}.txt"));
    let expected_path = input_path.with_extension("txt-plain");

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&expected_path);

    fs::write(&input_path, "中\n")?;
    fs::write(&expected_path, "WRONG\n")?;

    let res = std::panic::catch_unwind(|| {
        // unwrap to convert io::Result<()> into a panic if any I/O error happens;
        // the assertion within run_test_cases should trigger the panic we expect.
        run_test_cases("plain", Pinyin::plain).unwrap();
    });

    assert\!(res.is_err(), "run_test_cases should panic on content mismatch");
    if let Err(panic) = res {
        let msg = if let Some(s) = panic.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = panic.downcast_ref::<&str>() {
            s.to_string()
        } else {
            String::new()
        };
        assert\!(
            msg.contains("unmatched result on line 0 in temp_mismatch_"),
            "panic message should include line number and file name, got: {}",
            msg
        );
    }

    // Cleanup
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&expected_path);
    Ok(())
}
