# recipe-parser

> Parser implementation of recipe-lang

## Installation

```sh
cargo add recipe-parser
```

## Usage

```rs
use recipe_parser::{parse, Token};

fn main() {
    let recipe_raw = "Boil {potatoes}(3)";
    let recipe: Vec<Token> = parse(recipe_raw);
    println!("{recipe:?}");
}
```