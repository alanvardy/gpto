#[cfg(test)]
extern crate matches;

extern crate clap;

use std::io;

use clap::{Parser, Subcommand};
use colored::*;

mod config;
mod request;

const NAME: &str = "GPTO";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Alan Vardy <alan@vardy.cc>";
const ABOUT: &str = "A tiny unofficial OpenAI client";

pub const ECHO_DEFAULT: bool = false;
pub const MODELS_HELP: &str = "Returns a list of models from OpenAI";

#[derive(Parser, Clone)]
#[command(name = NAME)]
#[command(version = VERSION)]
#[command(about = ABOUT, long_about = None)]
#[command(author = AUTHOR, version)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    /// Disable the spinner and message when querying
    disable_spinner: bool,

    #[arg(short, long, default_value_t = String::new())]
    /// Text to be appended to end of response
    suffix: String,

    #[arg(short, long)]
    /// Text to be appended to end of response, defaults to gpt-3.5-turbo and can be set in config
    model: Option<String>,

    #[arg(short, long)]
    /// Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg
    config: Option<String>,

    #[arg(short, long)]
    /// URL to be queried, defaults to https://api.openai.com and can be set in config
    endpoint: Option<String>,

    #[arg(short, long, default_value_t = 1.0)]
    /// What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer
    temperature: f32,

    #[arg(short, long, default_value_t = 1)]
    /// How many completions to generate for each prompt
    number: u8,

    #[arg(short = 'o', long, default_value_t = 1.0)]
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or temperature but not both.
    top_p: f32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// The prompt(s) to generate completions for. Also accepts text from stdin
    Prompt {
        #[arg(short, long)]
        text: Option<String>,
    },

    // Start a conversation with an optional description of the bot's role
    Conversation {
        #[arg(short, long, default_value_t = String::new())]
        instructions: String,
    },
}
fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Prompt { text } => {
            let text = string_or_stdin(text);
            request::completions(cli.clone(), text)
        }
        Commands::Conversation { instructions } => request::conversation(cli.clone(), instructions),
    };

    match result {
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

fn string_or_stdin(text: &Option<String>) -> String {
    match text {
        Some(string) => string.clone(),
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    }
}
