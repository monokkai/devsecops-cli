use colored::Colorize;
use reqwest::blocking::get;

use crate::cli::ScanArgs;

pub fn handle(args: ScanArgs) {
    println!("{} {}", "🔍 Scanning:".blue(), args.path.green());

    let url = format!("https://api.github.com/advisories?path={}", args.path);
    match get(&url) {
        Ok(response) if response.status().is_success() => {
            match response.json::<Vec<serde_json::Value>>() {
                Ok(advisories) => {
                    println!("{} {}", "Found vulnerabilities:".red(), advisories.len());
                }
                Err(e) => eprintln!("{} {}", "❌ JSON error:".red(), e),
            }
        }
        Ok(response) => eprintln!("{} {}", "❌ API error:".red(), response.status()),
        Err(e) => eprintln!("{} {}", "❌ Request failed:".red(), e),
    }
}
