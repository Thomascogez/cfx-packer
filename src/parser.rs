use regex::Regex;
use glob::glob;


pub fn extract_file_paths(file_content: &str) -> Vec<String> {
    let re = Regex::new(r#"('|")(.+/.+)+('|")"#).unwrap();
    file_content
        .lines()
        .filter(|line| re.is_match(line))
        .map(|path| re.find(path).unwrap().as_str().replace("\"", "").replace("'", ""))
        .collect()
}


pub fn resolve_file_paths(file_paths: Vec<String>) -> Vec<String> {
    file_paths
        .iter()
        .map(|path| glob(&path).expect("Failed to read glob pattern"))
        .flatten()
        .map(|path| path.unwrap().to_str().unwrap().to_string())
        .collect()
}