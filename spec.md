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

### Complete sample

`woile-tomato-sauce.recp`

```recp
>> name: woile tomato sauce
>> lang: en-US
>> tags: easy, tasty

Boil {tomatoes}(3), {onion}(1) and {garlic}(3) for t{1 hour} in {water}(500 ml) with
{olive oil}(2 tbsp).
Add {oregano}, {basil}, {salt} and {pepper} at will.
Blend everything together using a &{hand blender}.
Let it sit in the fridge for t{1 hour}

---

I developed this recipe while living in Amsterdam
```

### Ingredients

Ingredients should be surrounded by curly braces `{}`. Inside you can have any utf-8 value.
After the curly braces `{}`, the amount can be optionally added, which should be surrounded by parenthesis `()`. There is no separation between the curly braces and parenthesis `{}()`.
The **amount** is composed by **quantity** and **unit**, which are both optional.
The **quantity** represents a numerical value like: `1` or `1/2`.
The **unit** represents a system of measurement, e.g: `kg`, `grams`, `cup`. The preferred system is to use the SI (International System of Units), which is the metric system.

```
{salt}
{tomatoes}(3)
{water}(200 ml)
{black pepper}(1 pinch)
{smashed potatoes}(200 gr)
{water}(1/2 cup)
```

### Timer

Timers are similar to ingredients, we use curly braces, but prefixed with a `t`, like `t{25 min}`

```recp
t{25 minutes}
```

## Materials

Materials are similar to ingredients, we use curly braces, but prefixed with an `&` (ampersand), like `&{material name}`

```recp
&{pot}
&{small jar}
&{stick}
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

### Recipe references

You can link to other recipes by using the `@{}` tag.

For example, if we want to link to the sample provided at the beginning:

```recp
Heat @{woile-tomato-sauce}(200 ml) in a pan
```

The values used should be relative URL's or relative paths. URL path if you are inside an app, or
a relative path if in your folder.

It should be displayed using the name of the recipe in the file or url.

### Metadata

Metadata is relevant information of a recipe that doesn't make the recipe itself.

```recp
>> name: Buddha bowl
>> servings: 2
>> tags: vegan, easy
```

### Backstory

It's common for a recipe to have a backstory, for example: your family recipe for tomato sauce, or a family member taught you how to build a cabin, etc.

Recipe-lang considers important to keep track of the history, notes, stories of recipes.

In order to add a `backstory`, the user should provide 3 dashes (`---`) without spaces in between, surrounded by new lines `\n` (`\n---\n`) and optionally spaces, so this is valid: `\n   ---- \n`.

Everything after the `---` will be taken as backstory till the end of the recipe (eof).

Let's take a look at this example recipe:

```recp
Add {tomatoes}(200gr) to a &{pot} with spices: {pepper}, {salt}, {oregano}.
Boil  for t{1 hour}.
Add {basilicum} at will.
Use a &{mixer} to bring everything together.
Let it sit in the fridge for t{2 hours}

---

I learnt this recipe while traveling around Italy (2016), in a town called Sorrento
```

The backstory in this case would be parsed as:
> I learnt this recipe while traveling around Italy (2016), in a town called Sorrento

We can see how the `---` is separating the backstory from the actual recipe.

The backstory **should** use [commonMark](https://spec.commonmark.org/) (A strongly defined, highly compatible specification of Markdown).