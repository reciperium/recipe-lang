# Recipe Lang (recp)

[LOGO HERE](https://github.com/reciperium/recipe-lang/issues/1)

> Write recipes understood by humans and machines

> **Warning**
>
> Recipe lang is in it's infancy

## About

Recipe Lang aims to be a general language to describe recipes of any kind (food, art, construction, etc.).

For example:
- how to prepare a soup
- how to make a burrito
- how to make your own deodorant
- how to make your tooth paste
- how to build a wooden chair

To learn more read the [specification](./spec.md)

File extension: `.recp` (/re c p/) reads like recipe.


```recp
Take {potatoes}(3) and wrap them in &{aluminium foil}.
Throw them in the fire of the grill
Wait for t{1 hour}
```

## Installation

### Nix

On Mac or Linux you can run:

```sh
nix profile install 'github:reciperium/recipe-lang#recp'
```

### Cargo

```sh
cargo install recp
```

## Features

Recipe lang supports:

- Ingredients with the tag `{ingredient_name}` or with amount: `{ingredient_name}(200gr)`
- Materials: `&{pot}`
- Timers: `t{15 minutes}`
- Recipe links: `@{woile/tomato-sauce}` (NOT IMPLEMENTED YET)
- Metadata: with `>> tags: abc, easy, high-fiber`
- Backstory: Separated by `---`, where you can add the history, see [examples/buddha-bowl.recp](examples/buddha-bowl.recp)
- Comments: with `/* my comment */`

Check more examples in the [examples folder](./examples/).


A VSCode extension is in the works.

## Sample

```recp
>> name: Potatoes a la Jean-Claude
>> tags: vegan
>> servings: 2

Preheat the oven to 180 C.
Cut the {red potatoes}(500gr) into fourths.
Put them in a &{bowl}, then add the {garlic}(8), add {oil},
{salt}, {pepper} and {rosemary} to your liking.
Mix everything and place them on an oven plate.
Roast for t{20 minutes}, then mix it and roast for another t{20 minutes}.
Enjoy!
```

In a file called: `potatoes-ala-jean-claude.recp`

We can the read this recipe with the `recp` cli:

```sh
recp show potatoes-ala-jean-claude.recp
```

It will show

```
Potatoes A La Jean Claude

Ingredients

  red potatoes                  500 gr
  garlic                        8
  oil
  salt
  pepper
  rosemary


Instructions

Preheat the oven to 180 C.
Cut the red potatoes into fourths.
Put them in a bowl, then add the garlic, add oil,
salt, pepper and rosemary to your liking.
Mix everything and place them on an oven plate.
Roast for 20 minutes, then mix it and roast for another 20 minutes.
Enjoy!
```
