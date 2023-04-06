# GPTO (Geppetto)

[![Build Status](https://github.com/alanvardy/gpto/workflows/ci/badge.svg)](https://github.com/alanvardy/gpto) [![codecov](https://codecov.io/gh/alanvardy/gpto/branch/master/graph/badge.svg?token=9FBJK1SU0K)](https://codecov.io/gh/alanvardy/gpto) [![Crates.io](https://img.shields.io/crates/v/gpto.svg)](https://crates.io/crates/gpto)

An Unofficial OpenAI Terminal Client

```bash
> gpto -h

A tiny unofficial OpenAI client

Usage: gpto [OPTIONS]

Options:
  -p, --prompt <Prompt text>...
          The prompt(s) to generate completions for
  -c, --conversation [<Instructions to bot>...]
          Start a conversation with an optional description of the bot's role
  -s, --suffix <Text to be appended to end of response>...
          The suffix that comes after a completion of inserted text. Defaults to an empty string
  -t, --temperature <float>
          What sampling temperature to use. 
               Higher values means the model will take more risks. 
               Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer. 
               Defaults to 1.0
  -n, --number <integer>
          How many completions to generate for each prompt. Defaults to 1
  -k, --top_p <float>
          An alternative to sampling with temperature, called nucleus sampling,
               where the model considers the results of the tokens with top_p probability mass.
               So 0.1 means only the tokens comprising the top 10% probability mass are considered.
               We generally recommend altering this or temperature but not both.
               Defaults to 1.0
  -m, --model <model name>
          
              Model to use for completion. Defaults to gpt-3.5-turbo.
              This CLI uses the /v1/chat/completions endpoint,
              see https://platform.openai.com/docs/models/gpt-3 for models available
              
  -o, --config <path to config file>
          Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg
  -h, --help
          Print help
  -V, --version
          Print version
  ```

[Learn more about how to use text completion](https://beta.openai.com/docs/guides/completion/introduction)

## Install from Crates.io

[Install Rust](https://www.rust-lang.org/tools/install)

```bash
# Linux and MacOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install GTPO

```bash
cargo install gtpo
```

## Install from AUR

```bash
# Use yay or another AUR helper
yay gpto-bin
```

## Install from GitHub

[Install Rust](https://www.rust-lang.org/tools/install)

Clone the project

```bash
git clone git@github.com:alanvardy/gpto.git
cd gpto
./test.sh # run the tests
cargo build --release
```

You can then find the binary in `/target/release/`

## Usage

Get a completion with default parameters

```bash
> gpto --prompt tell me a joke

Q: What did the fish say when it hit the wall?
A: Dam!
```

Get a completion with a different model (this example uses the leading code completion model). And yes, the generated code is not idiomatic!

Read more about models [here](https://platform.openai.com/docs/models/gpt-3). This CLI app uses the `/v1/chat/completions` endpoint.

```bash
> gpto -m gpt-4 -p language is elixir\nwrite a function that raises an error if the argument is not an integer and multiplies it by 2 if it is an integer

def multiply_by_two(x)
  raise ArgumentError, "Argument is not an integer" unless x.is_a? Integer
  x * 2
end
```
