use walkdir::WalkDir;

pub fn files(path: &str) -> Vec<String> {

    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect()
}
