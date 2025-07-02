use std::collections::HashMap;
use std::env;
use std::time::Duration;

use inquire::Text;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde_json::json;
use spinners::{Spinner, Spinners};

use crate::config;

use crate::Cli;

const COMPLETIONS_URL: &str = "/v1/chat/completions";
const VERSIONS_URL: &str = "/v1/crates/gpto/versions";

const SPINNER: Spinners = Spinners::Dots4;
const MESSAGE: &str = "Querying API";

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
    message: Message,
    index: u16,
    finish_reason: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Message {
    role: String,
    content: String,
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

/// Start an interactive conversation
pub fn conversation(cli: Cli, instructions: &str) -> Result<String, String> {
    let config = config::get_or_create(cli.config)?;

    let mut messages: Vec<HashMap<String, String>> = Vec::new();
    put_message(&mut messages, "system", instructions);

    let model = cli.model.unwrap_or(config.model);
    let endpoint = cli.endpoint.unwrap_or(config.endpoint);

    loop {
        let input = Text::new("Ask a question or quit > ")
            .prompt()
            .map_err(|e| e.to_string())?;

        if input.as_str() == "quit" || input.as_str() == "q" {
            break;
        }

        put_message(&mut messages, "user", &input);

        let body = json!({ 
            "model": model, 
            "max_tokens": cli.max_tokens,
            "messages": messages,
            "n": cli.number, 
            "temperature": cli.temperature, 
            "top_p": cli.top_p});

        let url = format!("{endpoint}{COMPLETIONS_URL}");

        let json_response = post_openai(
            config.token.clone(),
            url,
            body,
            cli.disable_spinner,
            config.timeout,
        )?;
        let response: Response = serde_json::from_str(&json_response)
            .or(Err("Could not serialize response from chat completion"))?;

        let output = response
            .choices
            .into_iter()
            .map(|x| x.message.content)
            .collect::<Vec<String>>()
            .join("\n\n---\n\n");

        println!("{output}\n");
        put_message(&mut messages, "assistant", &output)
    }

    Ok("Done".to_string())
}

fn put_message(messages: &mut Vec<HashMap<String, String>>, role: &str, content: &str) {
    let mut message: HashMap<String, String> = HashMap::new();
    message.insert("role".to_string(), role.to_string());
    message.insert("content".to_string(), content.to_string());
    messages.push(message);
}

/// Get completions from input prompt
pub fn completions(
    endpoint: String,
    body: serde_json::Value,
    token: String,
    disable_spinner: bool,
    timeout: u64,
    suffix: String,
) -> Result<String, String> {
    let url = format!("{endpoint}{COMPLETIONS_URL}");
    let json_response = post_openai(token, url, body, disable_spinner, timeout)?;
    let response: Response = serde_json::from_str(&json_response)
        .or(Err("Could not serialize response from chat completion"))?;

    let output = response
        .choices
        .into_iter()
        .map(|x| x.message.content)
        .collect::<Vec<String>>()
        .join("\n\n---\n\n");

    Ok(format!("{output}{suffix}"))
}

fn post_openai(
    token: String,
    url: String,
    body: serde_json::Value,
    disable_spinner: bool,
    timeout: u64,
) -> Result<String, String> {
    let authorization: &str = &format!("Bearer {token}");
    let timeout = Duration::from_secs(timeout);

    let spinner = maybe_start_spinner(disable_spinner);
    let response = Client::new()
        .post(url)
        .timeout(timeout)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, authorization)
        .json(&body)
        .send()
        .map_err(|e| format!("Did not get response from server\n{e:?}"))?;
    maybe_stop_spinner(spinner);

    if response.status().is_success() {
        Ok((response.text()).or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}

/// Get latest version number from Cargo.io
pub fn get_latest_version() -> Result<String, String> {
    let cargo_url: &str = "https://crates.io/api";

    let request_url = format!("{cargo_url}{VERSIONS_URL}");

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

fn maybe_start_spinner(disable_spinner: bool) -> Option<Spinner> {
    if env::var("DISABLE_SPINNER").is_ok() || disable_spinner {
        None
    } else {
        let sp = Spinner::new(SPINNER, MESSAGE.into());
        Some(sp)
    }
}

fn maybe_stop_spinner(spinner: Option<Spinner>) {
    if let Some(mut sp) = spinner {
        sp.stop();
        print!("\x1b[2K\r");
    };
}
