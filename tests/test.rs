#![cfg(test)]
use std::str::FromStr;

extern crate test_generator;
use test_generator::test_resources;

use version::{process_files, InputHandle};

#[test_resources("res/versions.txt")]
fn should_process_file(resource: &str) {
    let handle = InputHandle::from_str(resource).unwrap();

    let mut lines: Vec<String> = Vec::new();

    let collect_lines = |line: String| {
        lines.push(line);
    };

    process_files(&Vec::from([handle.clone(), handle.clone()]), collect_lines).unwrap();

    assert_eq!(lines.len(), 10);
    assert_eq!(
        lines,
        vec![
            "2.4.7",
            "no semver",
            "10.3.5",
            "1.2.6",
            "2.4.5",
            "2.4.7",
            "no semver",
            "10.3.5",
            "1.2.6",
            "2.4.5"
        ]
    );
}
