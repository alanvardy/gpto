#[cfg(test)]
extern crate matches;

extern crate clap;

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::*;

mod config;
mod request;

const APP: &str = "GPTO";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Alan Vardy <alan@vardy.cc>";
const ABOUT: &str = "A tiny unofficial OpenAI GPT3 client";

pub const DEFAULT_MODEL: &str = "text-davinci-003";
pub const DEFAULT_NUMBER: u8 = 1;
pub const DEFAULT_TEMPERATURE: f32 = 1.0;
pub const DEFAULT_TOP_P: f32 = 1.0;

struct Arguments<'a> {
    prompt: Option<String>,
    suffix: Option<String>,
    number: Option<u8>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    config_path: Option<&'a str>,
    model: Option<&'a str>,
    models: bool,
}

fn main() {
    let app = Command::new(APP)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT);
    let matches = app
        .arg(
            Arg::new("prompt")
                .short('p')
                .long("prompt")
                .required(false)
                .action(ArgAction::Append)
                .num_args(1..)
                .value_parser(clap::value_parser!(String))
                .help("The prompt(s) to generate completions for"),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .required(false)
                .action(ArgAction::Append)
                .num_args(1..)
                .value_parser(clap::value_parser!(String))
                .help("The suffix that comes after a completion of inserted text. Defaults to an empty string"),
        )
        .arg(
            Arg::new("configuration path")
                .short('o')
                .long("config")
                .num_args(1)
                .required(false)
                .value_name("path to config file")
                .help("Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .num_args(1)
                .required(false)
                .value_parser(clap::value_parser!(u8))
                .value_name("integer")
                .help(format!("How many completions to generate for each prompt. Defaults to {}", DEFAULT_NUMBER)),
        )
        .arg(
            Arg::new("temperature")
                .short('t')
                .long("temperature")
                .num_args(1)
                .required(false)
                .value_parser(clap::value_parser!(f32))
                .value_name("float")
                .help(format!("What sampling temperature to use. 
                Higher values means the model will take more risks. 
                Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer. 
                Defaults to {}", DEFAULT_TEMPERATURE)),
        )
        .arg(
            Arg::new("top_p")
                .short('k')
                .long("top_p")
                .num_args(1)
                .required(false)
                .value_parser(clap::value_parser!(f32))
                .value_name("float")
                .help(format!("An alternative to sampling with temperature, called nucleus sampling, 
                where the model considers the results of the tokens with top_p probability mass. 
                So 0.1 means only the tokens comprising the top 10% probability mass are considered.
                We generally recommend altering this or temperature but not both.
                Defaults to {}", DEFAULT_TOP_P)),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .num_args(1)
                .required(false)
                .value_name("model name")
                .help(format!(
                    "Model to use for completion. Defaults to {}. Use --models to see complete list.",
                    DEFAULT_MODEL
                )),
        )
        .arg(flag_arg(
            "models",
            'd',
            "models",
            "Returns a list of models from OpenAI",
        ))
        .get_matches();

    let prompt = matches
        .get_many("prompt")
        .map(|values| values.cloned().collect::<Vec<String>>().join(" "));

    let suffix = matches
        .get_many("suffix")
        .map(|values| values.cloned().collect::<Vec<String>>().join(" "));

    let arguments = Arguments {
        prompt,
        suffix,
        models: has_flag(matches.clone(), "models"),
        config_path: matches
            .get_one::<String>("configuration path")
            .map(|s| s.as_str()),
        model: matches.get_one::<String>("model").map(|s| s.as_str()),
        number: matches.get_one::<u8>("number").map(|s| s.to_owned()),
        temperature: matches.get_one::<f32>("temperature").map(|s| s.to_owned()),
        top_p: matches.get_one::<f32>("top_p").map(|s| s.to_owned()),
    };

    match dispatch(arguments) {
        Ok(output) => {
            println!("{}", output);
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
            prompt: Some(prompt),
            suffix,
            config_path: _,
            models: false,
            number,
            model,
            temperature,
            top_p,
        } => request::completions(config, &prompt, model, suffix, number, temperature, top_p),
        Arguments {
            prompt: None,
            config_path: _,
            models: true,
            model: _,
            number: None,
            suffix: None,
            temperature: None,
            top_p: None,
        } => request::models(config),
        Arguments {
            prompt: None,
            config_path: _,
            models: false,
            model: _,
            number: None,
            suffix: None,
            temperature: None,
            top_p: None,
        } => Err(String::from(
            "gtfo cannot be run without parameters. To see available parameters use --help",
        )),
        Arguments {
            prompt: _,
            config_path: _,
            models: _,
            model: _,
            suffix: _,
            number: _,
            temperature: _,
            top_p: _,
        } => Err(String::from(
            "Invalid parameters. To see available parameters use --help",
        )),
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
