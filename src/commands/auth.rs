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
        }
    }
}
