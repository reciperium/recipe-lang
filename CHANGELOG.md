## v0.9.5 (2026-01-19)

### Fix

- **deps**: bump serde_json in the all-cargo-dependencies group (#25)

## v0.9.4 (2026-01-12)

### Fix

- **deps**: bump the all-cargo-dependencies group across 1 directory with 4 updates (#24)

## v0.9.3 (2025-11-23)

### Fix

- format
- **ci**: add dependabot

## v0.9.2 (2025-11-23)

### Fix

- **ci**: dist release

## v0.9.1 (2025-11-23)

### Fix

- **ci**: update actions

## v0.9.0 (2025-11-23)

### Fix

- update dependencies
- remove commonmark from backstory

## v0.8.7 (2025-01-30)

### Fix

- bump to winnow 0.7

## v0.8.6 (2025-01-30)

### Fix

- **flake**: properly createa a rust toolchain
- parse ingredients when connected to a word by `'` or ```

## v0.8.5 (2024-11-06)

### Fix

- improve error message

## v0.8.4 (2024-10-07)

### Fix

- **parser**: properly handle spaces and multilines

## v0.8.3 (2024-10-06)

### Fix

- parse backstory and word properly
- update docs and flake

## v0.8.2 (2024-08-20)

### Fix

- update readme

## v0.8.1 (2024-05-27)

### Fix

- use proper feature for wasm

## v0.8.0 (2024-05-27)

### Feat

- add support for tsify under wasm feature

### Fix

- improve error messages

## v0.7.0 (2024-04-24)

### BREAKING CHANGE

- json output is no longer like:
```json
{"token": "Ingredient", "name": "foo", "amount": "1", "unit": "gr"}
```
now:
```json
{"token": "Ingredient", "content": {"name": "foo", "amount": "1", "unit": "gr"}}
```
The problem is that serde was failing to serialize enum variants that contained a single string

### Fix

- use serde content for token enum

## v0.6.0 (2024-04-24)

### BREAKING CHANGE

- The serialized output is no longer like
```
{"Ingredient": {"name": "foo", "amount": "1", "unit": "gr"}}
```
but instead
```
{"token": "Ingredient", "name": "foo", "amount": "1", "unit": "gr"}
```

### Feat

- add tag to token and add support for json-schema

## v0.5.0 (2024-04-23)

### Fix

- make serde optional and add serialize if enabled

## v0.4.3 (2024-04-18)

### Fix

- use repeat instead of repeat_till

## v0.4.2 (2024-04-16)

### Fix

- format code

## v0.4.1 (2024-04-15)

### Fix

- remove unnecessary parsers

## v0.4.0 (2024-04-15)

### Feat

- migrate from nom to winnow

### Fix

- clean flake
- use cargo resolver 2

## v0.3.1 (2023-11-10)

### Fix

- update cargo-dist to rust 1.73

## v0.3.0 (2023-11-10)

### Feat

- add flakestry release

## v0.2.9 (2023-11-06)

### Fix

- parse parenthesis as word properly

## v0.2.8 (2023-06-11)

### Fix

- grant write permissions

## v0.2.7 (2023-06-11)

### Fix

- cargo issue on bump

## v0.2.6 (2023-06-11)

### Fix

- ci is not commiting

## v0.2.5 (2023-06-11)

### Fix

- ci

## v0.2.4 (2023-06-11)

### Fix

- ci git configuration on bump

## v0.2.3 (2023-06-11)

### Fix

- flake with cargo workspace dependencies

## v0.2.2 (2023-06-11)

### Fix

- flake build

## v0.2.1 (2023-06-10)

### Fix

- flake improvements

## v0.2.0 (2023-06-09)

### BREAKING CHANGE

- `m` for materials was replaced by `&` because reading a material prefixed with an `m` doesn't read well. For eexample `m{bowl}` is mentally read as `mbowl` when compared to the silent `&{bowl}`. To migrate replace in your recipes the `m{` for `&{`

### Feat

- add recipe cli `recp`
- add recipe references
- add parser for amout (quantity + unit)
- add backstory section
- recipe parser

### Fix

- prepare for release
- improve display of instructions
- use `&` insetad if `m` for materials
- fail on unclosed curly and paren
- update description in cargo
- parse end of comment and spaces to prevent multi spaces
