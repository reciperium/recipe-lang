use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use recipe_parser::{parse, Token};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
    pub unit: Option<String>,
}

type RecipeRef = Ingredient;

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    pub duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
}

#[derive(Debug)]
pub struct Recipe<'a> {
    pub name: Option<String>,
    pub metadata: HashMap<String, String>,
    pub ingredients: Vec<Ingredient>,
    pub recipes_refs: Vec<RecipeRef>,
    pub timers: Vec<Timer>,
    pub materials: Vec<Material>,
    pub backstory: Option<String>,
    pub instructions: Vec<Token<'a>>,
}

impl<'a> TryFrom<&'a str> for Recipe<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let tokens = parse(value.trim());
        match tokens {
            Ok((_, tokens)) => {
                let mut metadata = HashMap::new();

                let mut ingredients = Vec::new();
                let mut recipes_refs = Vec::new();
                let mut timers = Vec::new();
                let mut materials = Vec::new();
                let mut backstory = String::new();

                for token in tokens.clone() {
                    match token {
                        Token::Metadata { key, value } => {
                            metadata.insert(key.to_string(), value.to_string());
                        }
                        Token::Ingredient {
                            name,
                            quantity,
                            unit,
                        } => {
                            let i = Ingredient {
                                name: name.to_string(),
                                quantity: quantity.map(|v| v.to_string()),
                                unit: unit.map(|v| v.to_string()),
                            };
                            ingredients.push(i);
                        }
                        Token::RecipeRef {
                            name,
                            quantity,
                            unit,
                        } => {
                            let i = RecipeRef {
                                name: name.to_string(),
                                quantity: quantity.map(|v| v.to_string()),
                                unit: unit.map(|v| v.to_string()),
                            };
                            recipes_refs.push(i);
                        }

                        Token::Timer(t) => timers.push(Timer {
                            duration: t.to_string(),
                        }),
                        Token::Material(material) => materials.push(Material {
                            name: material.to_string(),
                        }),
                        Token::Backstory(bs) => backstory.push_str(bs),
                        _ => {}
                    };
                }
                let name = metadata.get("name").cloned();
                Ok(Self {
                    name,
                    ingredients,
                    timers,
                    materials,
                    metadata,
                    recipes_refs,
                    backstory: {
                        if backstory.is_empty() {
                            None
                        } else {
                            Some(backstory)
                        }
                    },
                    instructions: tokens,
                })
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Recipe;

    #[test]
    fn test_invalid_recipes() {
        let invalid_recipe = "
        >>> name: invalid-recipe
        this is an {invalid recipe
        ";
        let recipe = Recipe::try_from(invalid_recipe);
        println!("{recipe:?}");
    }
}
