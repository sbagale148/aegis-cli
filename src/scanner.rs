use ignore::Walk;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Secret found in {file}:{line_num} - {pattern_name}")]
    SecretFound {
        file: String,
        line_num: usize,
        pattern_name: String,
    },
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub fn scan_path(path: &PathBuf, is_file: bool) -> Result<(), ScanError> {
    // Define secret patterns
    let patterns = vec![
        (Regex::new(r"(?i)aws.*[[:^ascii:]]*[0-9a-f]{4}").unwrap(), "AWS Client ID"),
        (Regex::new(r"(?i)github.*[[:^ascii:]]*[0-9a-zA-Z]{35,40}").unwrap(), "GitHub Token"),
        (Regex::new(r"(?i)api[_-]?key.*[[:^ascii:]]*[0-9a-zA-Z]{32,64}").unwrap(), "Generic API Key"),
        // Add more patterns here
    ];

    if is_file {
        scan_file(path, &patterns)
    } else {
        for result in Walk::new(path) {
            match result {
                Ok(entry) => {
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        scan_file(entry.path(), &patterns)?;
                    }
                }
                Err(err) => eprintln!("ERROR: {}", err),
            }
        }
        Ok(())
    }
}

fn scan_file(file_path: &std::path::Path, patterns: &[(Regex, &str)]) -> Result<(), ScanError> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line_num = line_num + 1; // Lines are 1-indexed

        for (regex, pattern_name) in patterns {
            if regex.is_match(&line) {
                return Err(ScanError::SecretFound {
                    file: file_path.display().to_string(),
                    line_num,
                    pattern_name: pattern_name.to_string(),
                });
            }
        }
    }
    Ok(())
}