# Recp

> A cli to display recipes written in recipe-lang

See [recipe-lang](../../spec.md)

## Installation

TODO

## Usage

Given a hummus recipe `hummus.recp`:

```recp
>> name: hummus classic
>> tags: vegan, high-protein, high-fiber
>> lang: en

Add {boiled chickpeas}(400 gr) (1 can) to the blender with {garlic}(1),
{tahini}(2 tsp), {lemon}(1/2), {olive oil}(2 tsp), {salt} and {pepper}.

Blend for t{3 minutes}.

Serve or store.
```

We can use the `recp` cli to pretty display the recipe

```sh
recp show hummus.recp
```

```
Hummus Classic

Ingredients

  boiled chickpeas              400 gr
  garlic                        1
  tahini                        2 tsp
  lemon                         1/2
  olive oil                     2 tsp
  salt
  pepper


Instructions

Add boiled chickpeas (1 can) to the blender with garlic,
tahini, lemon, olive oil, salt and pepper.

Blend for 3 minutes.

Serve or store.
```