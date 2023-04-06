# Changelog

## Unreleased

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
