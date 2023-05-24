use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{parse, parser::Token};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    name: String,
    amount: Option<String>,
    unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    name: Option<String>,
    metadata: HashMap<String, String>,
    ingredients: Vec<Ingredient>,
    timers: Vec<Timer>,
    materials: Vec<Material>,
    instructions: String,
    backstory: Option<String>,
}

impl From<Vec<Token<'_>>> for Recipe {
    fn from(tokens: Vec<Token>) -> Self {
        let mut metadata = HashMap::new();
        let mut ingredients = Vec::new();
        let mut timers = Vec::new();
        let mut materials = Vec::new();
        let mut instructions = String::new();
        let mut backstory = String::new();
        for token in tokens {
            let display_token = format!("{}", token);
            instructions.push_str(&display_token);

            match token {
                Token::Metadata { key, value } => {
                    metadata.insert(key.to_string(), value.to_string());
                }
                Token::Ingredient { name, amount } => {
                    let i = Ingredient {
                        name: name.to_string(),
                        amount: amount.map(|v| v.to_string()),
                        unit: None,
                    };
                    ingredients.push(i);
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
        let instructions = instructions.trim().to_string();
        Self {
            name,
            ingredients,
            instructions,
            timers,
            materials,
            metadata,
            backstory: {
                if backstory.is_empty() {
                    None
                } else {
                    Some(backstory)
                }
            },
        }
    }
}

impl TryFrom<&str> for Recipe {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = parse(value.trim());
        match tokens {
            Ok((_, tokens)) => Ok(tokens.into()),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Recipe;

    #[test]
    fn test_recipe_ok() {
        let recipe = ">> name: buddha bowl
        Put the boiled {quinoa}(200gr) in the base of the m{bowl}.
        We are going to add 3 ingredients on top: fat, protein and minerals.
        Stir fry {tofu}(200gr) and add them on one side of the bowl.
        Boil {sweet potatoes}(100 gr) for t{10 minutes} and add them on a side of the bowl.
        Cut the {avocado}(1/2) in slices and add them on the last side of the bowl.
        Cover with a mix of {pumpkin seeds} and {spring onions}
        ";
        let recipe = Recipe::try_from(recipe).expect("recipe did not work");
        println!("{recipe:?}");

        let serialized = serde_json::to_string_pretty(&recipe).unwrap();
        println!("{serialized}");
    }

    #[test]
    fn test_recipe_2_ok() {
        let recipe = "
        >> name: Potatoes a la Jean-Claude
        >> tags: vegan
        >> servings: 2
        Preheat the oven to 180 C.
        Cut the {red potatoes}(500gr) into fourths.
        Put them in a m{bowl}, then add the {garlic}(8), add {oil},
        {salt}, {pepper} and {rosemary} to your likeing.
        Mix everything and place them on an oven plate.
        Roast for t{20 minutes}, then mix it and roast for another t{20 minutes}.
        Enjoy!";
        let recipe = Recipe::try_from(recipe).expect("recipe did not work");
        println!("{recipe:?}");

        let serialized = serde_json::to_string_pretty(&recipe).unwrap();
        println!("{serialized}");
    }

    #[test]
    fn test_recipe_with_backstory_ok() {
        let recipe = "
        >> name: Potatoes a la Jean-Claude
        >> tags: vegan
        >> servings: 2
        Preheat the oven to 180 C.
        Cut the {red potatoes}(500gr) into fourths.
        Put them in a m{bowl}, then add the {garlic}(8), add {oil},
        {salt}, {pepper} and {rosemary} to your likeing.
        Mix everything and place them on an oven plate.
        Roast for t{20 minutes}, then mix it and roast for another t{20 minutes}.
        Enjoy!
        ---
        Old recipe from my grandpa
        ";
        let recipe = Recipe::try_from(recipe).expect("recipe did not work");
        println!("{recipe:?}");

        let serialized = serde_json::to_string_pretty(&recipe).unwrap();
        println!("{serialized}");
    }
}
