use serde::Serialize;

#[derive(Serialize)]
pub struct ScanResult {
    pub file: String,
    pub risk: String,
    pub reason: String,
}
