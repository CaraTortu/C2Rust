use std::fs;
use std::path::Path;

pub fn files_in_path(path: &str) -> String {

    if !Path::new(path).is_dir() {
        return "Path does not exist".to_owned();
    }

    let paths = fs::read_dir(path).unwrap();

    let p: Vec<String> = paths.map(|d| d.unwrap().path().to_string_lossy().to_string()).collect();
    return p.join(" ");
}