#[cfg(test)]
extern crate matches;

extern crate clap;

use std::io;

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::*;

mod config;
mod request;

const APP: &str = "GPTO";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Alan Vardy <alan@vardy.cc>";
const ABOUT: &str = "A tiny unofficial OpenAI client";

pub const MODEL_DEFAULT: &str = "gpt-3.5-turbo";
pub const MODEL_HELP: &str = "Model to use for completion. Defaults to gpt-3.5-turbo. This CLI uses the /v1/chat/completions endpoint, see https://platform.openai.com/docs/models/gpt-3 for models available";

pub const NUMBER_DEFAULT: u8 = 1;
pub const TEMPERATURE_DEFAULT: f32 = 1.0;
pub const TEMPERATURE_HELP: &str = "What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling)    for ones with a well-defined answer. Defaults to 1.0";
pub const TOP_P_DEFAULT: f32 = 1.0;
pub const TOP_P_HELP: &str = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or temperature but not both. Defaults to 1.0";

pub const SUFFIX_HELP: &str =
    "The suffix that comes after a completion of inserted text. Defaults to an empty string";

pub const CONFIG_HELP: &str =
    "Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg";
pub const PROMPT_HELP: &str =
    "The prompt(s) to generate completions for. Also accepts text from stdin";
pub const CONVERSATION_HELP: &str =
    "Start a conversation with an optional description of the bot's role";
pub const NUMBER_HELP: &str = "How many completions to generate for each prompt. Defaults to 1";
pub const ECHO_DEFAULT: bool = false;
pub const MODELS_HELP: &str = "Returns a list of models from OpenAI";
pub const ENDPOINT_HELP: &str =
    "endpoint to be used instead of OpenAIs, i.e. http://localhost:11434";

pub struct Arguments<'a> {
    prompt: Option<String>,
    disable_spinner: bool,
    conversation: Option<String>,
    endpoint: Option<String>,
    suffix: Option<String>,
    number: Option<u8>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    config_path: Option<&'a str>,
    model: Option<&'a str>,
}

fn main() {
    let app = Command::new(APP)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT);
    let matches = app
        .arg(flag_nullable_string(
            "prompt",
            'p',
            "Prompt text",
            PROMPT_HELP,
        ))
        .arg(flag_arg(
            "disable spinner",
            'x',
            "disable spinner",
            "Disable the spinner and message when querying",
        ))
        .arg(flag_nullable_string(
            "conversation",
            'c',
            "Instructions to bot",
            CONVERSATION_HELP,
        ))
        .arg(flag_string(
            "suffix",
            's',
            "Text to be appended to end of response",
            SUFFIX_HELP,
        ))
        .arg(flag_string("endpoint", 'e', "url", ENDPOINT_HELP))
        .arg(flag_float("temperature", 't', TEMPERATURE_HELP))
        .arg(flag_integer("number", 'n', NUMBER_HELP))
        .arg(flag_float("top_p", 'k', TOP_P_HELP))
        .arg(flag_string_no_spaces(
            "model",
            'm',
            "model name",
            MODEL_HELP,
        ))
        .arg(flag_string_no_spaces(
            "config",
            'o',
            "path to config file",
            CONFIG_HELP,
        ))
        .get_matches();

    let arguments = Arguments {
        disable_spinner: has_flag(matches.clone(), "disable spinner"),
        prompt: string_or_stdin(matches.clone()),
        suffix: join_string(matches.clone(), "suffix"),
        conversation: join_string(matches.clone(), "conversation"),
        endpoint: join_string(matches.clone(), "conversation"),
        config_path: matches.get_one::<String>("config").map(|s| s.as_str()),
        model: matches.get_one::<String>("model").map(|s| s.as_str()),
        number: matches.get_one::<u8>("number").map(|s| s.to_owned()),
        temperature: matches.get_one::<f32>("temperature").map(|s| s.to_owned()),
        top_p: matches.get_one::<f32>("top_p").map(|s| s.to_owned()),
    };

    match dispatch(arguments) {
        Ok(output) => {
            println!("{output}");
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e.red());
            std::process::exit(1);
        }
    }
}

fn dispatch(arguments: Arguments) -> Result<String, String> {
    let config: config::Config = config::get_or_create(arguments.config_path)?;

    match arguments {
        Arguments {
            prompt: Some(_),
            conversation: None,
            ..
        } => request::completions(arguments, config),
        Arguments {
            conversation: Some(_),
            prompt: None,
            ..
        } => request::conversation(arguments, config),
        Arguments {
            disable_spinner: _,
            prompt: None,
            config_path: _,
            model: _,
            number: None,
            suffix: None,
            temperature: None,
            top_p: None,
            conversation: None,
            endpoint: None,
        } => Err(String::from(
            "gtpo cannot be run without parameters. To see available parameters use --help",
        )),
        Arguments { .. } => Err(String::from(
            "Invalid parameters. To see available parameters use --help",
        )),
    }
}

fn flag_string_no_spaces(
    long: &'static str,
    short: char,
    value_name: &'static str,
    help: &'static str,
) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .num_args(1)
        .required(false)
        .value_name(value_name)
        .help(help)
}

fn flag_string(long: &'static str, short: char, value: &'static str, help: &'static str) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .required(false)
        .action(ArgAction::Append)
        .num_args(1..)
        .value_parser(clap::value_parser!(String))
        .value_name(value)
        .help(help)
}

fn flag_nullable_string(
    long: &'static str,
    short: char,
    value: &'static str,
    help: &'static str,
) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .required(false)
        .action(ArgAction::Append)
        .num_args(0..)
        .value_parser(clap::value_parser!(String))
        .value_name(value)
        .help(help)
}
fn flag_float(long: &'static str, short: char, help: &'static str) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .num_args(1)
        .required(false)
        .value_parser(clap::value_parser!(f32))
        .value_name("float")
        .help(help)
}
fn flag_integer(long: &'static str, short: char, help: &'static str) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .num_args(1)
        .required(false)
        .value_parser(clap::value_parser!(u8))
        .value_name("integer")
        .help(help)
}

fn join_string(matches: ArgMatches, long: &str) -> Option<String> {
    matches
        .get_many(long)
        .map(|values| values.cloned().collect::<Vec<String>>().join(" "))
}
fn string_or_stdin(matches: ArgMatches) -> Option<String> {
    let result = join_string(matches, "prompt");
    match result.clone() {
        Some(string) => {
            if string.is_empty() {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                Some(buffer)
            } else {
                result
            }
        }
        None => None,
    }
}
fn flag_arg(id: &'static str, short: char, long: &'static str, help: &'static str) -> Arg {
    Arg::new(id)
        .short(short)
        .long(long)
        .value_parser(["yes", "no"])
        .num_args(0..1)
        .default_value("no")
        .default_missing_value("yes")
        .required(false)
        .help(help)
}
fn has_flag(matches: ArgMatches, id: &'static str) -> bool {
    matches.get_one::<String>(id) == Some(&String::from("yes"))
}
