# Recipe lang

Version `0.1.0`

## Language spec

Recipe Lang aims to be a general language to describe recipes of any kind (food, art, construction, etc.).

For example:
- how to prepare a soup
- how to make a burrito
- how to make your own deodorant
- how to make your tooth paste
- how to build a wooden chair

The design aims to be simple, and easy to remember. Recipe lang takes inspiration from [cook-lang](https://cooklang.org/), which is focused on cooking recipes.

### How to use

The idea is for users to write the description of a recipe, and after that, you make some of the information machine readable, with the help of symbols.
The way to highlight information for the machine should be simple, familiar and easy to remember.

The idea is to use `{}` around a word or a set of words, e.g: `{spring onions}`, for an ingredient of the recipe.

On top of that, we can start adding different categories by adding a prefix to the `{}`, which would have a different meaning, e.g: for a timer: `t{20 minutes}`, and there's a huge number of symbols and letters we can use to convey new things for machines to be able to read.


### Ingredients

Ingredients should be surrounded by curly braces `{}`. Inside you can have any utf-8 value.
After the curly braces `{}`, the amount can be optionally added, which should be surrounded by parenthesis `()`. There is no separation between the curly braces and parenthesis `{}()`.
Amount starts with a number, and it's optionally preceded by a utf-8 text after a space.
This text correlates with a unit.

```
{salt}
{tomatoes}(3)
{water}(200 ml)
{black pepper}(1 pinch)
{smashed potatoes}(200 gr)
```

### Timer

Timers are similar to ingredients, we use curly braces, but prefixed with a `t`, like `t{25 min}`

```recp
t{25 minutes}
```

## Materials

Materials are similar to ingredients, we use curly braces, but prefixed with an `m`, like `m{material name}`

```recp
m{pot}
m{small jar}
m{stick}
```

### Comments

The comments can be placed anywhere

```recp
/* Sample */
```

```recp
Take the {tomatoes}(2) and /* TODO: Update recipe later */ move
them to bla
```

### Metadata

Metadata is relevant information of a recipe that doesn't make the recipe itself.

```recp
>> name: Buddha bowl
>> servings: 2
>> tags: vegan, easy
```

### Backstory

It's common for a recipe to have a backstory, for example: your family recipe for tomatoe sauce, or your mother taught you how to build a cabin, etc.

Recipe-lang considers important to keep track of the history, notes, stores of recipes.

In order to add a `backstory`, the user should provide 3 dashes (`---`) without spaces in between, surrounded by new lines `\n` (`\n---\n`) and optionally spaces, so this is valid: `\n   ---- \n`.

Everything after the `---` will be taken as backstory till the end of the recipe (eof).

Let's take a look at this example recipe:

```recp
Add {tomatoes}(200gr) to a m{pot} with spices: {pepper}, {salt}, {oregano}.
Boil  for t{1 hour}.
Add {basilicum} at will.
Use a m{mixer} to bring everything together.
Let it sit in the fridge for t{2 hours}

---

I learnt this recipe while traveling around Italy (2016), in a town called Sorrento
```

We can see that the `---` are separating the backstory from the actual recipe.

The backstory **should** use [commonMark](https://spec.commonmark.org/) (A strongly defined, highly compatible specification of Markdown).