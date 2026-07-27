# Suspicious File Detector

A lightweight Rust CLI application that scans local directories for files matching common suspicious patterns.

This tool is intended for educational purposes and basic security auditing. It does **not** detect malware.

## Features

- Recursive directory scan
- Double extension detection
- Hidden file detection
- Risk scoring
- JSON report
- Colored terminal output

## Run

```bash
cargo run -- ./data/sample_files
```

Example

```
invoice.pdf.exe     HIGH
secret.tmp          MEDIUM
photo.jpg           LOW
```

## Future Improvements

- SHA-256 hashing
- YARA rule support
- HTML report
- File entropy analysis
- Digital signature validation
