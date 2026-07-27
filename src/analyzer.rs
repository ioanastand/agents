use crate::models::ScanResult;
use crate::rules::*;

pub fn analyze(file: &str) -> ScanResult {

    if has_double_extension(file) {

        return ScanResult {
            file: file.into(),
            risk: "High".into(),
            reason: "Double extension".into(),
        };
    }

    if is_hidden(file) {

        return ScanResult {
            file: file.into(),
            risk: "Medium".into(),
            reason: "Hidden file".into(),
        };
    }

    ScanResult {
        file: file.into(),
        risk: "Low".into(),
        reason: "No suspicious patterns".into(),
    }
}
