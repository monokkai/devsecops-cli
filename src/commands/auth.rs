use colored::*;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use reqwest::blocking::Client;
use serde_json::json;

pub fn handle(args: super::cli::AuthAgrs) {
    match args.action {
        AuthAction::Jwt { token } => {
            println!("🔑 Validating JWT...");
            let decoded = decode::<serde_json::Value>(
                &token,
                &DecodingKey::from_secret(b"secret"),
                &Validation::new(Algorithm::HS256),
            );

            match decoded {
                Ok(token_data) => {
                    println!("{}\n{}", "✅ Valid JWT:".green(), json!(token_data.claims))
                }
                Err(e) => eprintln!("{}: {}", "❌ Invalid JWT".red(), e),
            }
        }

        AuthAction::Github { token } => {
            println!("🌐 Verifying GitHub token...");

            let client = Client::new();
            let response = client
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "DevSecOps-CLI")
                .send();

            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        println!("{}", "✅ Valid GitHub token".green());
                        println!("User: {:?}", res.json::<serde_json::Value>().unwrap());
                    } else {
                        eprintln!("{}: {}", "❌ Invalid token".red(), res.status());
                    }
                }

                Err(e) => eprintln!("{}: {}", "❌ Request failed".red(), e),
            }
        }
    }
}
