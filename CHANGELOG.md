# Changelog

## Unreleased

## 2024-04-07 0.2.1

- Add a separate flag for stdin, can now input text and have std at the same time
- Add a flag for setting max tokens
- Add timeout flag

## 2024-04-04 0.2.0

- Breaking changes to API, `prompt` and `conversation` are now sub commands
- Deprecated null values in config, please remove.
- Show better errors when no response is received

## 2024-03-28 0.1.8

- Add custom endpoints
- Added ability to change timeout in `gpto.cfg`
- Pretty print config on creation

## 2023-04-16 0.1.7

- Disable spinners with env `DISABLE_SPINNER`
- Disable spinners with flag `x`
- `prompt` also accepts text from stdin

## 2023-04-05 0.1.6

- Set the default model to `gpt-3.5-turbo`
- Use chat completions endpoint (so that GPT-4 can be used)
- Remove the models flag as it no longer applies for the chat completions endpoint
- Remove the echo flag as it is no longer supported by the new endpoint
- Add `--conversation` option

## 2023-03-19 0.1.5

- Remove the spinner once it stops

## 2023-03-19 0.1.4

- Add `--echo` option
- Resolve new Clippy warnings
- Add spinner effect while querying
- Remove `mockito` from dev dependencies

## 2022-12-29 0.1.3

- Add `--suffix` option
- Add `--number` option
- Add `--temperature` option
- Add `--top_p` option

## 2022-12-18 0.1.2

- Add `--models` option
- Add `--model` option
- Improve documentation

## 2022-12-18 0.1.1

- BREAKING change: use `--prompt` instead of `--text` to match API
- Add Crates.io version check
- Add manual test shell script
- Add publish checklist
- Update all dependencies

## 2022-12-18 0.1.0

- Repo initialized
- Created a super basic request for completions that outputs the first completion
- Save the token to a config file
