use std::fs;

pub fn files_in_path(path: &str) -> String {
    let paths = fs::read_dir(path).unwrap();
    let p: Vec<String> = paths.map(|d| d.unwrap().path().to_string_lossy().to_string()).collect();
    return p.join(" ");
}