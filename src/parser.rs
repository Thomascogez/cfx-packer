use regex::Regex;
use glob::glob;

use crate::utils;


pub fn extract_file_paths(file_content: &str) -> Vec<String> {
    let re = Regex::new(r#"('|")(.+/.+)+('|")"#).unwrap();
    file_content
        .lines()
        .filter(|line| re.is_match(line))
        .map(|path| re.find(path).unwrap().as_str().replace("\"", "").replace("'", ""))
        .collect()
}


pub fn resolve_file_paths(file_paths: Vec<String>, base_path: &String) -> Vec<String> {
    file_paths
        .iter()
        .map(|path| glob( &utils::to_relative_path(&path, base_path)).expect("Failed to read glob pattern"))
        .flatten()
        .filter(|path| path.as_ref().unwrap().is_file())
        .map(|path| path.unwrap().to_str().unwrap().to_string())
        .collect()
}