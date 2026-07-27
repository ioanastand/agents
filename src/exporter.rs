use crate::models::ScanResult;
use std::fs;

pub fn save(results: &[ScanResult]) {

    let json = serde_json::to_string_pretty(results).unwrap();

    fs::write("data/report.json", json).unwrap();
}
