use colored::*;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde_json::json;

use crate::config::Config;
use crate::{DEFAULT_MODEL, DEFAULT_NUMBER, DEFAULT_TEMPERATURE, DEFAULT_TOP_P};

#[cfg(test)]
use mockito;

// OPENAI URLS
const COMPLETIONS_URL: &str = "/v1/completions";
const MODELS_URL: &str = "/v1/models";

const MAX_TOKENS: u32 = 1000;

// CRATES.IO URLS
const VERSIONS_URL: &str = "/v1/crates/gpto/versions";
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
struct Response {
    // id: String,
    // object: String,
    // created: u32,
    // model: String,
    choices: Vec<Choice>,
    // usage: Usage,
}

#[derive(Deserialize)]
struct CargoResponse {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
struct Version {
    num: String,
}

#[derive(Deserialize)]
struct Models {
    data: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    id: String,
}

/// Get completions from input prompt
pub fn completions(
    config: Config,
    prompt: &str,
    model: Option<&str>,
    suffix: Option<String>,
    number: Option<u8>,
    temperature: Option<f32>,
    top_p: Option<f32>,
) -> Result<String, String> {
    let number = number.unwrap_or(DEFAULT_NUMBER);
    let temperature = temperature.unwrap_or(DEFAULT_TEMPERATURE);
    let top_p = top_p.unwrap_or(DEFAULT_TOP_P);
    let model = model
        .map(|x| x.to_string())
        .unwrap_or_else(|| config.model.unwrap_or_else(|| String::from(DEFAULT_MODEL)));
    let body = json!({ 
        "model": model, 
        "prompt": prompt,  
        "max_tokens": MAX_TOKENS, 
        "n": number, 
        "temperature": temperature, 
        "top_p": top_p });

    let json_response = post_openai(config.token, COMPLETIONS_URL.to_string(), body)?;
    let response: Response =
        serde_json::from_str(&json_response).or(Err("Could not serialize to CargoResponse"))?;

    let output = response
        .choices
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<String>>()
        .join("\n\n---");
    let suffix = suffix.unwrap_or_else(|| String::from(""));

    Ok(format!("{}{}", output, suffix))
}
pub fn models(config: Config) -> Result<String, String> {
    let json_response = get_openai(config.token, MODELS_URL.to_string())?;
    let models: Models =
        serde_json::from_str(&json_response).or(Err("Could not serialize to Models"))?;
    let model_list: Vec<String> = models.data.into_iter().map(|model| model.id).collect();
    let result = format!("{}\n\n{}", "Models: ".green(), model_list.join("\n"));
    Ok(result)
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

fn get_openai(token: String, url: String) -> Result<String, String> {
    #[cfg(not(test))]
    let openai_url: &str = "https://api.openai.com";

    #[cfg(test)]
    let openai_url: &str = &mockito::server_url();

    let request_url = format!("{}{}", openai_url, url);
    let authorization: &str = &format!("Bearer {}", token);

    let response = Client::new()
        .get(request_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, authorization)
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        Ok((response.text()).or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}

/// Get latest version number from Cargo.io
pub fn get_latest_version() -> Result<String, String> {
    #[cfg(not(test))]
    let cargo_url: &str = "https://crates.io/api";

    #[cfg(test)]
    let cargo_url: &str = &mockito::server_url();

    let request_url = format!("{}{}", cargo_url, VERSIONS_URL);

    let response = Client::new()
        .get(request_url)
        .header(USER_AGENT, "GPTO")
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        let cr: CargoResponse =
            serde_json::from_str(&response.text().or(Err("Could not read response text"))?)
                .or(Err("Could not serialize to CargoResponse"))?;
        Ok(cr.versions.first().unwrap().num.clone())
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}
