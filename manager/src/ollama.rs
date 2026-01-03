use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn extract_receipt(
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String> {
    let client = Client::new();

    let req = OllamaRequest {
        model,
        prompt,
        stream: false,
    };

    let resp = client
        .post(format!("{}/api/generate", ollama_url))
        .json(&req)
        .send()
        .await
        .context("Failed to send request to Ollama")?
        .error_for_status()
        .context("Ollama returned error status")?;

    let body: OllamaResponse = resp
        .json()
        .await
        .context("Failed to parse Ollama response")?;

    Ok(body.response)
}
