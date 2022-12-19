use std::path::Path;

pub fn to_relative_path(path: &String, base_path: &String) -> String {
    if path.starts_with(base_path) {
        return String::from(path);
    }

    return Path::new(base_path).join(path).to_str().unwrap().to_string();
}