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
