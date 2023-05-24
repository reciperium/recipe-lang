# Recipe Lang (recp)

> Write recipes in a human readable and machine parseable language

> **Warning**
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

## Sample

```recp
Take {potatoes}(3) and wrap them in m{aluminium foil}.
Throw them in the fire of the grill
Wait for t{1 hour}
```

## Longer sample

```recp
>> name: Potatoes a la Jean-Claude
>> tags: vegan
>> servings: 2

Preheat the oven to 180 C.
Cut the {red potatoes}(500gr) into fourths.
Put them in a m{bowl}, then add the {garlic}(8), add {oil},
{salt}, {pepper} and {rosemary} to your liking.
Mix everything and place them on an oven plate.
Roast for t{20 minutes}, then mix it and roast for another t{20 minutes}.
Enjoy!
```

In a file called: `potatoes-ala-jean-claude.recp`

## Features

Recipe lang supports:

- Ingredients with the tag `{ingredient_name}` or with amount: `{ingredient_name}(200gr)`
- Materials: `m{pot}`
- Timers: `t{15 minutes}`
- Recipe links: `@{woile/tomato-sauce}` (NOT IMPLEMENTED YET)
- Metadata: with `>> tags: abc, easy, high-fiber`
- Backstory: Separated by `---`, where you can add the history, see [examples/buddha-bowl.recp](examples/buddha-bowl.recp)
- Comments: with `/* my comment */`

Check more examples in the [examples folder](./examples/).


A VSCode extension is in the works.
