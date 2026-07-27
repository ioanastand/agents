mod analyzer;
mod exporter;
mod filesystem;
mod models;
mod rules;
mod scanner;
mod utils;

fn main() {
    let results = scanner::scan("./data/sample_files");

    utils::print_results(&results);

    exporter::save(&results);
}
