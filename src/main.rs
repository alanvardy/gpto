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

struct Arguments<'a> {
    prompt: Option<String>,
    config_path: Option<&'a str>,
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
                .help("Prompt to be completed"),
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

    let arguments = Arguments {
        prompt,
        models: has_flag(matches.clone(), "models"),
        config_path: matches
            .get_one::<String>("configuration path")
            .map(|s| s.as_str()),
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
            config_path: _,
            models: false,
        } => request::completions(config, &prompt),
        Arguments {
            prompt: None,
            config_path: _,
            models: true,
        } => request::models(config),
        Arguments {
            prompt: None,
            config_path: _,
            models: false,
        } => Err(String::from(
            "gtfo cannot be run without parameters. To see available parameters use --help",
        )),
        Arguments {
            prompt: _,
            config_path: _,
            models: _,
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
