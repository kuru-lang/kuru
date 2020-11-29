# Variables
You define a variable with the `let` keyword:

```aratar
let var: 5
```

Notice that `:` is the assignment operator instead of `=` or `<-`.  `:` does not
look like equivalence like `=` nor have parsing issues like `<-`, which is why
it was chosen.  Note also that variables are immutable by default (you can't
change their value).

## Exclusive References
You may define a variable as mutable (you can change it), by using the `@`
character.

```aratar
let var: @5
# You can now change the variable.
var: 4
```

Only exclusive references can be changed.  You can also only have one exclusive
reference at a time, so the following will fail to compile:

```aratar
fn function(a @Int, b Int) {
    a +: b
}

let var: @5
function(@var, var)
```

Note that all values are pass-by-reference in Aratar.  Small types will be
optimized into pass-by-value by the compiler.  As long as your types are
subtypes of `.Copy`, you can do the following:

```aratar
let a: 4
let b: @a # `b` now references a mutable copy of `a`'s data
let c: a # `a` and `c` reference the same immutable data.
let d: b # `d` now references an immutable copy of `b`'s data
let e: @b # `e` now references a mutable copy of `b`'s data
```

Otherwise, the references are moved (note that the actual data stays pinned -
meaning the location referenced is immovable):

```aratar
let a: "4"
let b: @a # `b` now references `a`'s data, and reference `a` is ceded
let a: "4" # Must redefine `a` because old `a` has been ceded
let c: a # `a` and `c` reference the same immutable data.
let d: b # `d` now references `b`'s data, and reference `b` is ceded
let e: @d # `e` now references `d`'s data, and reference `d` is ceded
```

Note that this follows Rust's paradigm that there may be either:
 - 1 mutable reference
 - Any number of immutable references
At any point in the control flow.  At any point where this would not be true,
either a reference is ceded or the data is copied.  Note also that the rules for
`let` statements do not follow the same rules for function calls:

```aratar
fn function(a Text, b Text) {
    log(a, b)
}

let a: @"4"
let b: @a.clone()

function(a, b) # variables `a` and `b` are borrowed, but not ceded.

a.cat("D")
b.cat("W")

function(a, b) # can call again with the same parameters, but different values.
```

## Cede
This is the same as Rust's "drop" or C++'s "delete":

```aratar
# Make `Cede` as supertype of `Type`
fn Type.Cede(@self) -> Cede {
    # Do some stuff before the associated data with self gets ceded.
    Cede()
}
```

## Justification For `@` Operator
Most systems programming languages use `&` for references, but this is confusing
as that also means AND.
