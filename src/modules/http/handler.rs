use crate::cli::HttpAction;
use crate::modules::http::client;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const WHITE: &str = "\x1b[37m";
const RESET: &str = "\x1b[0m";

pub async fn handle(action: HttpAction) -> Result<(), Box<dyn std::error::Error>> {
    let (method, url, body, headers) = match action {
        HttpAction::Get { url, headers } => (Method::GET, url, None, headers),
        HttpAction::Post { url, body, headers } => {
            (Method::POST, url, body.map(|b| json!(b)), headers)
        }
        HttpAction::Put { url, body, headers } => {
            (Method::PUT, url, body.map(|b| json!(b)), headers)
        }
        HttpAction::Patch { url, body, headers } => {
            (Method::PATCH, url, body.map(|b| json!(b)), headers)
        }
        HttpAction::Delete { url, headers } => (Method::DELETE, url, None, headers),
        HttpAction::Head { url, headers } => (Method::HEAD, url, None, headers),
        HttpAction::Options { url, headers } => (Method::OPTIONS, url, None, headers),
    };

    let headers_map = parse_headers(headers);
    let response = client::execute_request(method, &url, body, headers_map.clone()).await?;

    let status_color = if response.status().is_success() {
        GREEN
    } else {
        RED
    };
    println!("{}Status: {}{}", status_color, response.status(), RESET);

    println!("{}Headers:{}\n", RED, RESET);
    for (k, v) in response.headers() {
        println!("{}{}: {}{}", RED, k, v.to_str().unwrap_or(""), RESET);
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let body_color = if content_type.contains("text/html")
        || content_type.contains("application/json")
        || content_type.contains("text/plain")
    {
        WHITE
    } else {
        RED
    };

    let body_text = response.text().await?;
    println!(
        "\n{}Body:{}\n{}{}{}",
        RED, RESET, body_color, body_text, RESET
    );

    Ok(())
}

fn parse_headers(raw: Vec<String>) -> HashMap<String, String> {
    raw.iter()
        .filter_map(|h| {
            let mut parts = h.splitn(2, ':');
            Some((
                parts.next()?.trim().to_string(),
                parts.next()?.trim().to_string(),
            ))
        })
        .collect()
}
