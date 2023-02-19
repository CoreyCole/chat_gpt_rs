use std::env;
use leptos::ServerFnError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

pub async fn complete(prompt: String) -> Result<String, ServerFnError> {
    // Set up the API endpoint and model parameters
    let endpoint = "https://api.openai.com/v1/engines/davinci/completions";
    let max_tokens = 1024;

    // Get the API key from an environment variable
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY environment variable must be set");

    // Set up the request headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Set up the request body
    let request_body = format!("{{\"prompt\": \"{}\", \"max_tokens\": {}}}", prompt, max_tokens);

    // Send the POST request
    let client = reqwest::Client::new();
    let response = client.post(endpoint)
        .headers(headers)
        .body(request_body)
        .send()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Failed to send request: {}", e)))?;

    if !response.status().is_success() {
        let status_code = response.status();
        let error_body = response.text().await.map_err(|e| ServerFnError::ServerError(format!("Failed to read response body: {}", e)))?;
        return Ok(format!("Received error response ({}): {}", status_code, error_body));
    }

    // Print the response body
    let response_body = response.text().await.unwrap();
    Ok(response_body)
}