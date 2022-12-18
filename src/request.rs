use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_json::json;

use crate::config::Config;

#[cfg(test)]
use mockito;

const COMPLETIONS_URL: &str = "/v1/completions";
const MODEL: &str = "text-davinci-003";
const TEMPERATURE: f32 = 0.0;
const MAX_TOKENS: u32 = 1000;
#[derive(Deserialize)]
#[allow(dead_code)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
#[derive(Deserialize)]
#[allow(dead_code)]
struct Choice {
    text: String,
    index: u16,
    finish_reason: String,
}
#[derive(Deserialize)]
#[allow(dead_code)]
struct Response {
    id: String,
    object: String,
    created: u32,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

/// Get completions from input text
pub fn completions(config: Config, text: &str) -> Result<String, String> {
    let body = json!({ "model": MODEL, "prompt": text,  "temperature": TEMPERATURE, "max_tokens": MAX_TOKENS });

    let json_response = post_openai(config.token, COMPLETIONS_URL.to_string(), body)?;
    let response: Response =
        serde_json::from_str(&json_response).or(Err("Could not serialize to CargoResponse"))?;

    Ok(response.choices.first().unwrap().text.clone())
}

fn post_openai(token: String, url: String, body: serde_json::Value) -> Result<String, String> {
    #[cfg(not(test))]
    let openai_url: &str = "https://api.openai.com";

    #[cfg(test)]
    let openai_url: &str = &mockito::server_url();

    let request_url = format!("{}{}", openai_url, url);
    let authorization: &str = &format!("Bearer {}", token);

    let response = Client::new()
        .post(request_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, authorization)
        .json(&body)
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        Ok((response.text()).or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}
