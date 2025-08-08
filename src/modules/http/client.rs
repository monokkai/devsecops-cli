use colored::*;
use reqwest::{Method, Response};
use serde_json::{Value, json};
use std::collections::HashMap;

pub async fn execute_request(
    method: Method,
    url: &str,
    body: Option<Value>,
    headers: HashMap<String, String>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut request = client.request(method, url);

    for (key, value) in headers {
        request = request.header(key, value);
    }

    if let Some(body) = body {
        request = request.json(&body);
    }

    let response = request.send().await?;
    Ok(response)
}

pub async fn print_response(response: Response) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "Status:".bold(),
        response.status().to_string().blue()
    );

    println!("{}", "Headers:".bold());
    for (key, value) in response.headers() {
        println!(
            "  {}: {}",
            key.to_string().green(),
            value.to_str()?.yellow()
        );
    }

    println!("{}", "Body:".bold());
    let text = response.text().await?;
    println!("{}", text);

    Ok(())
}
