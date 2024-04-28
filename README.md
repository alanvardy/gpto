# GPTO (Geppetto)

[![Build Status](https://github.com/alanvardy/gpto/workflows/ci/badge.svg)](https://github.com/alanvardy/gpto) [![codecov](https://codecov.io/gh/alanvardy/gpto/branch/master/graph/badge.svg?token=9FBJK1SU0K)](https://codecov.io/gh/alanvardy/gpto) [![Crates.io](https://img.shields.io/crates/v/gpto.svg)](https://crates.io/crates/gpto)

An Unofficial OpenAI Terminal Client

```bash
> gpto -h

A tiny unofficial OpenAI client

Usage: gpto [OPTIONS] <COMMAND>

Commands:
  prompt        The prompt(s) to generate completions for. Also accepts text from stdin
  conversation
  help          Print this message or the help of the given subcommand(s)

Options:
  -d, --disable-spinner            Disable the spinner and message when querying
  -s, --suffix <SUFFIX>            Text to be appended to end of response [default: ]
  -m, --model <MODEL>              Text to be appended to end of response, defaults to gpt-3.5-turbo and can be set in config
  -c, --config <CONFIG>            Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg
  -e, --endpoint <ENDPOINT>        URL to be queried, defaults to https://api.openai.com and can be set in config
  -t, --temperature <TEMPERATURE>  What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer [default: 1]
  -n, --number <NUMBER>            How many completions to generate for each prompt [default: 1]
  -a, --max-tokens <MAX_TOKENS>    Maximum number of tokens to use for each request [default: 1000]
  -i, --timeout <TIMEOUT>          Maximum length of time in seconds to wait for an API request to complete
  -o, --top-p <TOP_P>              An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or temperature but not both [default: 1]
  -h, --help                       Print help
  -V, --version                    Print version
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
> gpto prompt --text "tell me a joke"

Q: What did the fish say when it hit the wall?
A: Dam!
```

Get completions using text from stdin (without displaying the spinner)

```bash
> echo "what is one plus one" | gpto prompt -d

Two
```

Get a completion with a different model (this example uses the leading code completion model). And yes, the generated code is not idiomatic!

Read more about models [here](https://platform.openai.com/docs/models/gpt-3). This CLI app uses the `/v1/chat/completions` endpoint.

```bash
> gpto -m gpt-4 prompt -t language is elixir\nwrite a function that raises an error if the argument is not an integer and multiplies it by 2 if it is an integer

def multiply_by_two(x)
  raise ArgumentError, "Argument is not an integer" unless x.is_a? Integer
  x * 2
end
```

## Using a local LLM

You can run a local LLM using [ollama](https://github.com/ollama/ollama) and connect to with GPTO.

Get the model you want

```bash
ollama pull llama3
```

Start the ollama server

```bash
ollama serve
```

And then in another terminal run GPTO

```bash
gpto -m llama3 -e http://localhost:11434 prompt -t "Hello Llama"
```

## Setting timeout

Timeout is 30s by default, this can be altered by changing the `timeout` option in `gpto.cfg`
