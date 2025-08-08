use colored::Colorize;
use reqwest::blocking::Client;
use serde_json::Value;

use crate::cli::ScanArgs;

pub fn handle(args: ScanArgs) {
    println!("{} {}", "🔍 Scanning:".blue(), args.path.green());

    let client = Client::new();
    let url = format!("https://api.github.com/advisories?path={}", args.path);

    match client.get(&url).send() {
        Ok(response) if response.status().is_success() => match response.json::<Vec<Value>>() {
            Ok(advisories) => {
                println!("{} {}", "Found vulnerabilities:".red(), advisories.len());
            }
            Err(e) => eprintln!("{} {}", "❌ JSON error:".red(), e),
        },
        Ok(response) => eprintln!("{} {}", "❌ API error:".red(), response.status()),
        Err(e) => eprintln!("{} {}", "❌ Request failed:".red(), e),
    }
}
