use crate::analyzer::analyze;
use crate::models::ScanResult;
use walkdir::WalkDir;

pub fn scan(path: &str) -> Vec<ScanResult> {

    let mut results = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {

        if entry.file_type().is_file() {

            let file = entry.path().display().to_string();

            results.push(analyze(&file));

        }

    }

    results
}
