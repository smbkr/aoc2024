mod report;

use crate::report::check_report;
use std::env::args;
use std::fs;
use std::process::exit;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let file_path = args.first().unwrap_or_else(|| {
        println!("Please specify a file to open.");
        exit(1); // exit the program here
    });

    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
            exit(1); // exit the program with an error code
        }
    };

    let reports_results = contents.lines().map(check_report);

    let safe_reports_count = 0; // ????

    println!("Safe reports: {}", safe_reports_count);
}
