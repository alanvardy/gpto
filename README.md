# GPTO (Geppetto)

[![Build Status](https://github.com/alanvardy/gpto/workflows/ci/badge.svg)](https://github.com/alanvardy/gpto) [![codecov](https://codecov.io/gh/alanvardy/gpto/branch/master/graph/badge.svg?token=9FBJK1SU0K)](https://codecov.io/gh/alanvardy/gpto) [![Crates.io](https://img.shields.io/crates/v/gpto.svg)](https://crates.io/crates/gpto)

An Unofficial OpenAI GPT3 Terminal Client

```bash
> gpto -h

A tiny unofficial OpenAI GPT3 client

Usage: gpto [OPTIONS]

Options:
  -p, --prompt <prompt>...           Prompt to be completed
  -o, --config <CONFIGURATION PATH>  Absolute path of configuration. Defaults to $XDG_CONFIG_HOME/gpto.cfg
  -h, --help                         Print help information
  -V, --version                      Print version information
```

## Usage

Get a completion with default parameters

```bash
> gpto --prompt tell me a joke

Q: What did the fish say when it hit the wall?
A: Dam!
```

Give an exhaustive list of all models

```bash
> gpto --models

Models: 

babbage
ada
davinci
babbage-code-search-code
text-similarity-babbage-001
... and so on
```
