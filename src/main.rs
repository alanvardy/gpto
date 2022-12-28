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

struct Arguments<'a> {
    prompt: Option<String>,
    suffix: Option<String>,
    number: Option<u8>,
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
                .value_name("CONFIGURATION PATH")
                .help("Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .num_args(1)
                .required(false)
                .value_parser(clap::value_parser!(u8))
                .value_name("NUMBER")
                .help("How many completions to generate for each prompt. Defaults to 1"),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .num_args(1)
                .required(false)
                .value_name("MODEL")
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
        } => request::completions(config, &prompt, model, suffix, number),
        Arguments {
            prompt: None,
            config_path: _,
            models: true,
            model: _,
            number: None,
            suffix: None,
        } => request::models(config),
        Arguments {
            prompt: None,
            config_path: _,
            models: false,
            model: _,
            number: None,
            suffix: None,
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
