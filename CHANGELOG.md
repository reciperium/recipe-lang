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
