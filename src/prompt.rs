use std::io;

use serde_json::json;

use crate::config;
use crate::request;
use crate::Cli;

const MAX_TOKENS: u32 = 1000;

/// Get completions from input prompt
pub fn completions(cli: Cli, text: &Option<String>, stdin: &bool) -> Result<String, String> {
    let text = string_or_stdin(text, stdin)?;
    let config = config::get_or_create(cli.config)?;

    // Config or CLI
    let model = cli.model.unwrap_or(config.model());
    let endpoint = cli.endpoint.unwrap_or(config.endpoint());

    // Config only
    let token = config.token.clone();
    let timeout = config.timeout;

    // CLI only
    let temperature = cli.temperature;
    let top_p = cli.top_p;
    let disable_spinner = cli.disable_spinner;
    let number = cli.number;
    let suffix = cli.suffix;

    let body = json!({
        "model": model, 
        "max_tokens": MAX_TOKENS,
        "messages": [{"role": "user", "content": text}],
        "n": number, 
        "temperature": temperature, 
        "top_p": top_p});

    request::completions(endpoint, body, token, disable_spinner, timeout, suffix)
}

fn string_or_stdin(text: &Option<String>, stdin: &bool) -> Result<String, String> {
    let text = text.clone().unwrap_or_default();

    let prompt = if *stdin {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        format!("{text}\n{buffer}")
    } else {
        text
    };

    if prompt.is_empty() {
        Err(String::from("No prompt provided"))
    } else {
        Ok(prompt)
    }
}
