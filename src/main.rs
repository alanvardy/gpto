#[cfg(test)]
extern crate matches;

extern crate clap;

use clap::{Arg, ArgAction, Command};
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
        .get_matches();

    let prompt = matches
        .get_many("prompt")
        .map(|values| values.cloned().collect::<Vec<String>>().join(" "));

    let arguments = Arguments {
        prompt,
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
        } => request::completions(config, &prompt),
        Arguments {
            prompt: None,
            config_path: _,
        } => Err(String::from(
            "gtfo cannot be run without parameters. To see available parameters use --help",
        )),
    }
}
