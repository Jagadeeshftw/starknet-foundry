use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn extract_matches_from_file(file_path: &Path, re: &Regex) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let matches: Vec<String> = re
        .captures_iter(&content)
        .filter_map(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .collect();

    Ok(matches)
}

pub fn extract_matches_from_directory(
    dir_path: &Path,
    re: &Regex,
    extension: Option<&str>,
) -> io::Result<Vec<String>> {
    let mut all_snippets = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            all_snippets.extend(extract_matches_from_directory(&path, re, extension)?);
        } else if let Some(ext) = extension {
            if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                let snippets = extract_matches_from_file(&path, re)?;
                if !snippets.is_empty() {
                    all_snippets.extend(snippets);
                }
            }
        }
    }

    Ok(all_snippets)
}

#[must_use]
pub fn get_parent_dir(levels_up: usize) -> PathBuf {
    let mut dir = env::current_dir().expect("Failed to get the current directory");

    for _ in 0..levels_up {
        dir = dir
            .parent()
            .expect("Failed to navigate to parent directory")
            .to_owned();
    }

    dir
}

pub fn parse_snippet_str_to_command_args(snippet: &str) -> Vec<String> {
    let cleaned_snippet = snippet
        .lines()
        .map(str::trim_end)
        .collect::<Vec<&str>>()
        .join(" ")
        .replace(" \\", "");

    shell_words::split(&cleaned_snippet)
        .expect("Failed to parse snippet string")
        .into_iter()
        .map(|arg| arg.trim().to_string())
        .collect()
}
