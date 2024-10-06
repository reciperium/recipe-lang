use std::fmt::Display;

use winnow::ascii::{line_ending, multispace0, space0, space1};
use winnow::combinator::{alt, cut_err, delimited, opt, preceded, repeat, rest};
use winnow::error::{ContextError, ParseError, StrContext, StrContextValue};
use winnow::stream::AsChar;
use winnow::token::{take_till, take_until, take_while};
use winnow::{Located, PResult, Parser};

type Input<'a> = Located<&'a str>;

/// Parses a valid string from the input.
///
/// This function takes a mutable reference to a string slice and parses a valid string from it.
/// A valid string can contain alphanumeric characters as well as certain symbols and spaces.
/// The function returns a `PResult` containing the parsed valid string.
fn parse_valid_string<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    let spaces_and_symbols = "\t /-_@.,%#'";
    take_while(1.., move |c: char| {
        c.is_alphanumeric() || spaces_and_symbols.contains(c)
    })
    .parse_next(input)
}

/// Parse comments in the form of:
///
/// ```recp
/// /* */
/// ```
fn parse_comment<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    delimited(
        "/*",
        cut_err(take_until(0.., "*/"))
            .context(StrContext::Expected(StrContextValue::StringLiteral("*/")))
            .map(|v: &str| v.trim()),
        ("*/", space0),
    )
    .parse_next(input)
}

/// Parse curly braces delimited utf-8
///
/// ```recp
/// {salt}
/// {tomatoes}
/// ```
fn parse_curly<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    delimited(
        "{",
        parse_valid_string.map(|v| v.trim()),
        cut_err("}").context(StrContext::Expected(StrContextValue::CharLiteral('}'))),
        // "}"
    )
    .parse_next(input)
}

/// The amount of an ingredient must be numeric
/// with a few symbols allowed.
///
/// ```recp
/// 1
/// 3.2
/// 3,2
/// 3_000_000
/// 2/3
/// ```
fn parse_quantity<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    let spaces_and_symbols = ".,/_";

    cut_err(
        take_while(1.., move |c: char| {
            c.is_numeric() || spaces_and_symbols.contains(c)
        })
        .verify(|s: &str| {
            // NEXT: Can this be improved?
            let has_repeated_symbols = s
                .as_bytes()
                .windows(2)
                .any(|v| v[0] == v[1] && spaces_and_symbols.contains(char::from(v[0])));
            let last_char = &s[s.len() - 1..];
            !spaces_and_symbols.contains(last_char) && !has_repeated_symbols
        }),
    )
    .context(StrContext::Expected(StrContextValue::Description(
        "a quantity value, like 3, 1.2, 1/2 or 1_000",
    )))
    .parse_next(input)
}

/// Parse units like kg, kilograms, pinch, etc.
fn parse_unit<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    parse_valid_string.parse_next(input)
}

/// Ingredient amounts are surrounded by parenthesis
fn parse_ingredient_amount<'a>(
    input: &mut Input<'a>,
) -> PResult<(Option<&'a str>, Option<&'a str>)> {
    delimited(
        ("(", space0),
        (
            opt(parse_quantity),
            opt(preceded(space0, parse_unit.map(|v| v.trim()))),
        ),
        cut_err(")").context(StrContext::Expected(StrContextValue::CharLiteral(')'))), // cut_err(")"),
    )
    // .context(StrContext::Expected(StrContextValue::CharLiteral('}')))
    .parse_next(input)
}

/// Ingredients come in these formats:
///
/// ```recp
/// {quinoa}(200gr)
/// {tomatoes}(2)
/// {sweet potatoes}(2)
/// ```
fn parse_ingredient<'a>(
    input: &mut Input<'a>,
) -> PResult<(&'a str, Option<(Option<&'a str>, Option<&'a str>)>)> {
    (parse_curly, opt(parse_ingredient_amount)).parse_next(input)
}

/// Materials format:
///
/// ```recp
/// &{pot}
/// &{small jar}
/// &{stick}
/// ```
fn parse_material<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    preceded("&", parse_curly).parse_next(input)
}

/// Materials format:
///
/// ```recp
/// t{25 minutes}
/// t{10 sec}
/// ```
fn parse_timer<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    preceded("t", parse_curly).parse_next(input)
}

/// Parse a reference to another recipe
///
/// ```recp
/// @{woile/special-tomato-sauce}
/// @{woile/special-tomato-sauce}(100 ml)
/// ```
fn parse_recipe_ref<'a>(
    input: &mut Input<'a>,
) -> PResult<(&'a str, Option<(Option<&'a str>, Option<&'a str>)>)> {
    preceded("@", (parse_curly, opt(parse_ingredient_amount))).parse_next(input)
}

/// Tokens are separated into words
fn parse_word<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    take_till(1.., AsChar::is_space).parse_next(input)
}

fn parse_metadata<'a>(input: &mut Input<'a>) -> PResult<(&'a str, &'a str)> {
    preceded(
        (">>", space0),
        (
            take_while(1.., |c| c != ':'),
            preceded((":", space0), take_until(0.., "\n")),
        ),
    )
    .parse_next(input)
}

/// The backstory is separated by `---`, and it consumes till the end
/// ```recp
/// my recipe bla with {ingredient1}
/// ---
/// This recipe was given by my grandma
/// ```
fn parse_backstory<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    preceded(
        delimited(
            preceded(line_ending, multispace0),
            "---",
            preceded(line_ending, multispace0),
        ),
        rest,
    )
    .parse_next(input)
}

/* ****************
* The main parser
**************** */

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
// if you use `zod` for example, using a tag makes it easy to use an undiscriminated union
#[cfg_attr(feature = "serde", serde(tag = "token", content = "content"))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub enum Token<'a> {
    Metadata {
        key: &'a str,
        value: &'a str,
    },
    Ingredient {
        name: &'a str,
        quantity: Option<&'a str>,
        unit: Option<&'a str>,
    },
    // Reference to another recipe
    RecipeRef {
        name: &'a str,
        quantity: Option<&'a str>,
        unit: Option<&'a str>,
    },
    Timer(&'a str),
    Material(&'a str),
    Word(&'a str),
    Space(&'a str),
    Comment(&'a str),
    Backstory(&'a str),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ingredient {
                name,
                quantity: _,
                unit: _,
            } => write!(f, "{}", name),
            Token::RecipeRef {
                name,
                quantity: _,
                unit: _,
            } => write!(f, "\"{}\"", name),
            Token::Backstory(v)
            | Token::Timer(v)
            | Token::Material(v)
            | Token::Word(v)
            | Token::Space(v) => {
                write!(f, "{}", v)
            }
            Token::Metadata { key: _, value: _ } => Ok(()),
            Token::Comment(_) => Ok(()),
        }
    }
}

pub fn recipe_value<'a>(input: &mut Input<'a>) -> PResult<Token<'a>> {
    alt((
        parse_metadata.map(|(key, value)| Token::Metadata { key, value }),
        parse_material.map(|m| Token::Material(m)),
        parse_timer.map(|t| Token::Timer(t)),
        // Because ingredient doesn't have a prefix before the curly braces, e.g: `&{}`
        // it must always be parsed after timer and material
        parse_ingredient.map(|(name, amount)| {
            let mut quantity = None;
            let mut unit = None;
            if let Some((_quantity, _unit)) = amount {
                quantity = _quantity;
                unit = _unit;
            };

            Token::Ingredient {
                name,
                quantity,
                unit,
            }
        }),
        parse_recipe_ref.map(|(name, amount)| {
            let mut quantity = None;
            let mut unit = None;
            if let Some((_quantity, _unit)) = amount {
                quantity = _quantity;
                unit = _unit;
            };

            Token::RecipeRef {
                name,
                quantity,
                unit,
            }
        }),
        parse_backstory.map(|v| Token::Backstory(v)),
        parse_comment.map(|v| Token::Comment(v)),
        "(".map(|v| Token::Word(v)),
        parse_word.map(|v| Token::Word(v)),
        space1.map(|v| Token::Space(v)),
    ))
    .parse_next(input)
}

pub fn recipe<'a>(input: &mut Input<'a>) -> PResult<Vec<Token<'a>>> {
    repeat(0.., recipe_value).parse_next(input)
}

/// Parse recipe tokens from a string
///
/// Example:
///
/// ```
/// use recipe_parser::parse;
///
/// let input = "Take the {potatoe}(1) and boil it";
/// let result = parse(input).expect("recipe could not be parsed");
///
/// println!("{result:?}");
/// ```
pub fn parse(input: &str) -> Result<Vec<Token<'_>>, ParseError<Located<&str>, ContextError>> {
    let input = Located::new(input);
    recipe.parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("salt", "salt")]
    #[case("sweet potato", "sweet potato")]
    #[case("ToMaToeS", "ToMaToeS")]
    #[case("1/2 lemon", "1/2 lemon")]
    #[case("my-best-sauce", "my-best-sauce")]
    #[case("1.2", "1.2")]
    #[case("1,2", "1,2")]
    #[case("1_200", "1_200")]
    #[case("@woile", "@woile")]
    #[case("10%", "10%")]
    #[case("#vegan", "#vegan")]
    #[case("mango's", "mango's")]
    fn test_parse_valid_string(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let valid_str = parse_valid_string(&mut input).unwrap();
        assert_eq!(valid_str, expected)
    }

    #[rstest]
    #[case("/* */", "")]
    #[case("/* hello */", "hello")]
    #[case("/* multi\nline\ncomment */", "multi\nline\ncomment")]
    fn test_parse_comment_ok(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let comment = parse_comment(&mut input).expect("failed to parse comment");
        assert_eq!(comment, expected)
    }

    #[test]
    fn test_parse_comment_wrong() {
        let mut input = Located::new("/* unclosed");
        let res = parse_comment(&mut input);
        assert!(res.is_err());

        let err = res.unwrap_err();
        println!("{:?}", err);
        assert!(matches!(err, winnow::error::ErrMode::Cut(_)));
    }

    #[rstest]
    #[case("{salt}", "salt")]
    #[case("{black pepper}", "black pepper")]
    #[case("{smashed potatoes}", "smashed potatoes")]
    #[case("{15 minutes}", "15 minutes")]
    #[case("{   15 minutes  }", "15 minutes")]
    fn test_parse_curly_ok(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let content = parse_curly(&mut input).expect("to work");
        assert_eq!(expected, content);
    }

    #[test]
    fn test_parse_curly_wrong() {
        let mut input = Located::new("{}");
        let res = parse_curly(&mut input);
        assert!(res.is_err());

        let mut input = Located::new("{unclosed");
        let res = parse_curly(&mut input);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, winnow::error::ErrMode::Cut(_)));
    }

    #[rstest]
    #[case("200", "200")]
    #[case("2.1", "2.1")]
    #[case("2_1", "2_1")]
    #[case("2,1", "2,1")]
    #[case("2.1", "2.1")]
    #[case("1/2", "1/2")]
    #[case(".2", ".2")]
    fn test_parse_quantity_ok(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let content = parse_quantity(&mut input).expect("to work");
        assert_eq!(expected, content);
    }

    #[rstest]
    #[case("2.")]
    #[case("2..0")]
    #[case("2,,0")]
    #[case("2//0")]
    fn test_parse_quantity_invalid(#[case] input: String) {
        // TODO: Add verify function to validate the last char
        let mut input = Located::new(input.as_str());
        let res = parse_quantity(&mut input);
        let err = res.unwrap_err();
        assert!(matches!(err, winnow::error::ErrMode::Cut(_)));
    }

    #[rstest]
    #[case("(200gr)", (Some("200"), Some("gr")))]
    #[case("(1/2)", (Some("1/2"), None))]
    #[case("(100 gr)", (Some("100"), Some("gr")))]
    #[case("(10 ml)", (Some("10"), Some("ml")))]
    #[case("( 10 ml )", (Some("10"), Some("ml")))]
    #[case("(1.5 cups)", (Some("1.5"), Some("cups")))]
    fn test_parse_ingredient_amount_ok(
        #[case] input: String,
        #[case] expected: (Option<&str>, Option<&str>),
    ) {
        let mut input = Located::new(input.as_str());
        let content = parse_ingredient_amount(&mut input).expect("to work");
        assert_eq!(expected, content);
    }

    #[rstest]
    #[case("()")]
    #[case("(unclosed")]
    fn test_parse_ingredient_amount_invalid_quantity(#[case] input: String) {
        let mut input = Located::new(input.as_str());
        let res = parse_ingredient_amount(&mut input);
        match res {
            Ok(_) => {
                // should fail the test
                assert!(false);
            }
            Err(e) => match e {
                winnow::error::ErrMode::Cut(err) => {
                    println!("{}", err);
                    assert_eq!(
                        "expected a quantity value, like 3, 1.2, 1/2 or 1_000",
                        err.to_string()
                    );
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            },
        }
    }

    #[rstest]
    #[case("(1.5")]
    fn test_parse_ingredient_amount_invalid_amount(#[case] input: String) {
        let mut input = Located::new(input.as_str());
        let res = parse_ingredient_amount(&mut input);
        match res {
            Ok(_) => {
                // should fail the test
                assert!(false);
            }
            Err(e) => match e {
                winnow::error::ErrMode::Cut(err) => {
                    println!("{}", err);
                    assert_eq!("expected `)`", err.to_string());
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            },
        }
    }

    #[rstest]
    #[case("{sweet potato}(200gr)", "sweet potato", Some((Some("200"),Some("gr"))))]
    #[case("{sweet potato}", "sweet potato", None)]
    fn test_parse_ingredient_ok(
        #[case] input: String,
        #[case] expected_ingredient: &str,
        #[case] expected_amount: Option<(Option<&str>, Option<&str>)>,
    ) {
        let mut input = Located::new(input.as_str());
        let (ingredient, amount) = parse_ingredient(&mut input).unwrap();
        assert_eq!(expected_ingredient, ingredient);
        assert_eq!(expected_amount, amount);
    }

    #[rstest]
    #[case("&{pot}", "pot")]
    #[case("&{small jar}", "small jar")]
    #[case("&{stick}", "stick")]
    #[case("&{bricks}", "bricks")]
    fn test_parse_material_ok(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let material = parse_material(&mut input).expect("Failed to parse material");
        assert_eq!(material, expected)
    }

    #[rstest]
    #[case("t{1 minute}", "1 minute")]
    fn test_parse_timer_ok(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let timer = parse_timer(&mut input).expect("Failed to parse timer");
        assert_eq!(timer, expected)
    }

    #[rstest]
    #[case("@{woile/tomato-sauce}(200gr)", "woile/tomato-sauce", Some((Some("200"),Some("gr"))))]
    #[case("@{woile/tomato-sauce}", "woile/tomato-sauce", None)]
    #[case("@{special stew}", "special stew", None)]
    fn test_parse_recipe_ok(
        #[case] input: String,
        #[case] expected_recipe: &str,
        #[case] expected_amount: Option<(Option<&str>, Option<&str>)>,
    ) {
        let mut input = Located::new(input.as_str());
        let (recipe, amount) = parse_recipe_ref(&mut input).unwrap();
        assert_eq!(expected_recipe, recipe);
        assert_eq!(expected_amount, amount);
    }

    #[rstest]
    #[case(">> tags: vegan\n", ("tags", "vegan"))]
    #[case(">> key: pepe\n", ("key", "pepe"))]
    #[case(">>key: pepe\n", ("key", "pepe"))]
    #[case(">>    key: pepe\n", ("key", "pepe"))]
    #[case(">>    key:     pepe\n", ("key", "pepe"))]
    #[case(">>    key:\t\tpepe\n", ("key", "pepe"))]
    #[case(">>    key:pepe\n", ("key", "pepe"))]
    fn test_parse_metadata_ok(#[case] input: String, #[case] expected: (&str, &str)) {
        let mut input = Located::new(input.as_str());
        let metadata = parse_metadata(&mut input).expect("Failed to parse metadata");
        assert_eq!(metadata, expected)
    }

    #[rstest]
    #[case("\n---\nwhat a backstory", "what a backstory")]
    #[case("\n   ---\nwhat a backstory", "what a backstory")]
    #[case("\n   ---\n\nwhat a backstory", "what a backstory")]
    #[case("\n   ---\n\nthis is **markdown**", "this is **markdown**")]
    #[case("\n   ---\n\nthis is [markdown](url)", "this is [markdown](url)")]
    fn test_parse_backstory_ok(#[case] input: String, #[case] expected: &str) {
        let mut input = Located::new(input.as_str());
        let backsotry = parse_backstory(&mut input).expect("failed to parse backstory");
        assert_eq!(backsotry, expected)
    }

    #[rstest]
    #[case("\n---    \nwhat a backstory")]
    fn test_parse_backstory_fail(#[case] input: String) {
        let mut input = Located::new(input.as_str());
        let out = parse_backstory(&mut input);
        assert!(out.is_err())
    }

    #[rstest]
    #[case(" ", Token::Space(" "))]
    #[case("{holis}(100 gr)", Token::Ingredient { name: "holis", quantity: Some("100"), unit: Some("gr") })]
    fn test_recipe_value_ok(#[case] input: &str, #[case] expected: Token) {
        let mut input = Located::new(input);
        let token = recipe_value(&mut input).expect("failed to parse token");
        assert_eq!(token, expected)
    }

    #[test]
    fn test_recipe_ok() {
        let input = "Boil the quinoa for t{5 minutes} in a &{pot}.\nPut the boiled {quinoa}(200gr) in the base of the bowl.";
        let expected = "Boil the quinoa for 5 minutes in a pot.\nPut the boiled quinoa in the base of the bowl.";
        let recipe = recipe.parse(Located::new(input)).expect("parse failed");
        let fmt_recipe = recipe
            .iter()
            .fold(String::new(), |acc, val| format!("{acc}{val}"));
        println!("{}", fmt_recipe);

        assert_eq!(expected, fmt_recipe);
        println!("{:?}", recipe);
    }

    #[rstest]
    #[case("Foo. ")]
    #[case("Foo.")]
    #[case("Foo, bar")]
    #[case("Foo,bar")]
    #[case("Foo,")]
    #[case("Foo, ")]
    #[case("Foo,\n")]
    #[case("Foo.\n")]
    #[case("Foo.\nfoo")]
    #[case("Foo,\nfoo")]
    fn test_symbol_parsing(#[case] input: &str) {
        let recipe_result = parse(input);

        assert!(recipe_result.is_ok());
    }

    #[test]
    fn test_parse_ok() {
        let input = "Boil the quinoa for t{5 minutes} in a &{pot}.\nPut the boiled {quinoa}(200gr) in the base of the bowl.";
        let expected = "Boil the quinoa for 5 minutes in a pot.\nPut the boiled quinoa in the base of the bowl.";
        let recipe = parse(input).expect("parse failed");
        let fmt_recipe = recipe
            .iter()
            .fold(String::new(), |acc, val| format!("{acc}{val}"));
        println!("{}", fmt_recipe);

        assert_eq!(expected, fmt_recipe);
        println!("{:?}", recipe);
    }

    #[test]
    fn test_parse_with_backstory_ok() {
        let input = "Foo. \n---\nA backstory";
        let expected = vec![
            Token::Word("Foo."),
            Token::Space(" "),
            Token::Backstory("A backstory"),
        ];
        let recipe = parse(input).expect("parse failed");

        println!("{:?}", recipe);

        assert_eq!(expected, recipe);
        println!("{:?}", recipe);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_token_serialization_works() {
        let token = Token::Ingredient {
            name: "quinoa",
            quantity: Some("200"),
            unit: Some("gr"),
        };

        let serialized = serde_json::to_string(&token).expect("failed to serialize");
        println!("{}", serialized);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_token_serialization_creates_right_payload() {
        let token = Token::Ingredient {
            name: "quinoa",
            quantity: Some("200"),
            unit: Some("gr"),
        };

        let serialized = serde_json::to_string(&token).expect("failed to serialize");
        assert_eq!(
            serialized,
            r#"{"token":"Ingredient","content":{"name":"quinoa","quantity":"200","unit":"gr"}}"#
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_token_serialization_creates_right_payload_single_string() {
        let token = Token::Word("holis");

        let serialized = serde_json::to_string(&token).expect("failed to serialize");
        assert_eq!(serialized, r#"{"token":"Word","content":"holis"}"#);
    }

    #[test]
    #[cfg(feature = "schemars")]
    fn test_token_json_schema_generation() {
        use schemars::schema_for;
        let schema = schema_for!(Token);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
