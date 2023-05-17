# Recipe Lang (recp)

> Write recipes in a human readable and machine parseable language

## About

Recipe Lang aims to be a general language to describe recipes of any kind (food, art, construction, etc.).

For example:
- how to prepare a soup
- how to make a burrito
- how to make your own deodorant
- how to make your tooth paste
- how to build a wooden chair

To learn more read the [specification](./spec.md)

## Sample

```recp
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
```

File format: `.recp` (/re c p/) reads like recipe

