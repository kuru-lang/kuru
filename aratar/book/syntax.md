# Syntax

## Brackets
Due to 1-equivalence (a tuple of one item is equal to that item, an array of one
item is equal to that item), you may use any type of bracket as parentheses.
 - Tuple: `(7, "D")`
 - Struct: `StructName(field: "value", a: 4)`
 - List: `[1, 3, 5]`
 - Map: `[0: 1, 2: 5, 1: 3]`
 - Unordered List: `[Red: 0.0, Green: 0.0, Blue: 0.0]`

The difference between unordered lists and maps is that unordered lists must
always be indexable by any possible key, whereas maps do not have to contain
every possible key, and can therefore change size at runtime.

## Literals
 - Text: `"Some Text"` (There is no "Character" in Aratar)
 - Integer: `4`
 - Decimal: `4.0` (The trailing `0` is required)
 - Hexadecimal: `0xFFFF`
 - Binary: `0b1100_0011` (Any number can use `_` to make it visually pleasing)
