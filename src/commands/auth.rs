use colored::Colorize;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use reqwest::blocking::Client;
use serde_json::json;

use crate::cli::{AuthAction, AuthArgs};

pub fn handle(args: AuthArgs) {
    match args.action {
        AuthAction::Jwt { token } => {
            println!("{}", "🔑 Validating JWT".blue());
            match decode::<serde_json::Value>(
                &token,
                &DecodingKey::from_secret(b"secret"),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(data) => println!("{}\n{}", "✅ Valid JWT:".green(), json!(data.claims)),
                Err(e) => eprintln!("{} {}", "❌ Invalid JWT:".red(), e),
            }
        }
        AuthAction::Github { token } => {
            println!("{}", "🌐 Verifying GitHub token".blue());
            match Client::new()
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "devsecops-cli")
                .send()
            {
                Ok(res) if res.status().is_success() => {
                    println!("{}", "✅ Valid GitHub token".green());
                }
                Ok(res) => eprintln!("{} {}", "❌ Invalid token:".red(), res.status()),
                Err(e) => eprintln!("{} {}", "❌ Request failed:".red(), e),
            }
        }
    }
}
