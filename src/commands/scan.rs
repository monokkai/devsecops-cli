use colored::*;
use reqwest;

pub fn handle(args: cli::ScanArgs) {
    println!("🔍 Scanning {} on vulnerabilities...", args.path.green());

    // API
    let url = format!("https://api.github.com/advisories?path={}", args.path);
    let response = reqwest::blocking::get(&url).unwrap();

    if response.status().is_success() {
        let advisories: Vec<serde_json::Value> = response.json().unwrap();
        println!(
            "Found vulnerabilities: {}",
            advisories.len().to_string().red()
        );
    } else {
        eprintln!("API Error: {}", response.status());
    }
}
