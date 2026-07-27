use crate::models::ScanResult;
use colored::*;

pub fn print_results(results: &[ScanResult]) {

    println!("{:<35} {:<10} {}", "FILE", "RISK", "REASON");
    println!("{}", "-".repeat(70));

    for r in results {

        let risk = match r.risk.as_str() {

            "High" => r.risk.red(),

            "Medium" => r.risk.yellow(),

            _ => r.risk.green()

        };

        println!(
            "{:<35} {:<12} {}",
            r.file,
            risk,
            r.reason
        );
    }
}
