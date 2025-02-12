use console::style;
use recipe_parser::Token;
use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use inflector::Inflector;
use std::io::Write;
use tabwriter::TabWriter;
mod recipe;
use recipe::Recipe;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, visible_alias = "s")]
    Show { recipes: Vec<PathBuf> },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Show { recipes } => {
            for recipe_path in recipes {
                let content =
                    fs::read_to_string(recipe_path).expect("Could not read the given file");
                let recipe = Recipe::try_from(content.as_str());
                if let Err(error) = recipe {
                    eprintln!("Failed to parse the recipe file:\n\n{}", error);
                    std::process::exit(1);
                }
                let recipe = recipe.unwrap();
                if let Some(name) = recipe.name {
                    println!("{}\n", style(name.to_title_case()).bold().blue());
                }
                if recipe.ingredients.len() > 0 || recipe.recipes_refs.len() > 0 {
                    println!("{}\n", style("Ingredients").underlined());
                }
                if recipe.ingredients.len() > 0 {
                    for ing in recipe.ingredients {
                        let amount = format!(
                            "{} {}",
                            ing.quantity.unwrap_or_default(),
                            ing.unit.unwrap_or_default()
                        );
                        let mut tw = TabWriter::new(vec![]).minwidth(32).padding(10);
                        write!(&mut tw, "  {}\t{}", style(ing.name).cyan().bold(), amount).unwrap();
                        tw.flush().unwrap();
                        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
                        println!("{}", written);
                        // println!("{} {} ", style(ing.name).bold(), amount.trim())
                    }
                }
                if recipe.recipes_refs.len() > 0 {
                    for ing in recipe.recipes_refs {
                        let amount = format!(
                            "{} {}",
                            ing.quantity.unwrap_or_default(),
                            ing.unit.unwrap_or_default()
                        );
                        let mut tw = TabWriter::new(vec![]).minwidth(32).padding(10);
                        write!(
                            &mut tw,
                            "  {}\t{}",
                            style(ing.name).magenta().bold(),
                            amount
                        )
                        .unwrap();
                        tw.flush().unwrap();
                        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
                        println!("{}", written);
                        // println!("{} {} ", style(ing.name).bold(), amount.trim())
                    }
                }
                println!("\n\n{}\n", style("Instructions").underlined().bold());
                let instru: String = recipe
                    .instructions
                    .iter()
                    .filter_map(|instruct| match instruct {
                        Token::Metadata { key: _, value: _ } => None,
                        Token::Ingredient {
                            name,
                            quantity: _,
                            unit: _,
                        } => Some(format!("{}", style(name).cyan().bold())),
                        Token::RecipeRef {
                            name,
                            quantity: _,
                            unit: _,
                        } => Some(format!("{}", style(name).magenta())),
                        Token::Timer(v) => Some(format!("{}", style(v).red().bold())),
                        Token::Material(v) => Some(format!("{}", style(v).yellow())),
                        Token::Word(v) | Token::Space(v) => Some(format!("{v}")),

                        Token::Comment(_) => None,
                        Token::Backstory(_) => None,
                    })
                    .collect();

                println!("{}", instru.trim())
                // println!("{}", recipe.instructions);
            }
        }
    }
}
