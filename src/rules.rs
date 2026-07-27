pub fn has_double_extension(name: &str) -> bool {

    name.ends_with(".pdf.exe")
        || name.ends_with(".jpg.exe")
        || name.ends_with(".doc.exe")
}

pub fn is_hidden(name: &str) -> bool {

    name.contains("/.")
        || name.contains("\\.")
}
