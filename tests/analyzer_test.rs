use suspicious_file_detector::rules::has_double_extension;

#[test]
fn detect_double_extension() {

    assert!(has_double_extension("invoice.pdf.exe"));
}
