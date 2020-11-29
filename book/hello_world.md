# Hello World
Let's start with the most basic program:

```aratar
export fn run() {
    log("Hello, world!")
}
```

Which outputs:

```output
Hello, world!
```

## The `export` Keyword
`export` is how you make your functions, types and constants available for
either the operating system or an program that imports your code as a library.

### Naming And Other Languages
The `export` keyword is named to be the opposite of the `import` keyword; It can
be equated with Java's `public` and Rust's `pub`.

## The `fn` Keyword
The function keyword can define a named function, and may have any number of
paramters and an optional return type.
```aratar
fn function(a Int, b Int): Int {
    a + b
}
```

### Naming And Other Languages
The `fn` keyword and syntax is taken directly from Rust, besides the removal of
`:` to set the type in the arguments, and `:` instead of `->` for return types.

## The `run()` Function
If your code exports a `run()` function, then it is interpreted as a program -
otherwise it's a library (a collection of reusable functions, types and
constants).

## Scopes
Scopes are written with `{}`.  Operations within the scopes are separated by
newlines (similar to Python, minus the `\`s).  Alternatively, commas may also be
used.

```aratar
{
    function_a(a, b, c)
    function_a(
        a + b
          + c,
        b,
        c,
    )
    Builder()
        .function_a()
        .function_b()
        .function_c()
        .finish()
}
```

## The `log()` Function and Variable Arguments
By default, there are a few functions and types imported into scope.  You may
not define functions or types with the same names.  One of these is the `log`
function, which has the following function signature:

```aratar
fn log(as_text List[&Text])
```

As you can see, it takes a list of `&Text`.  `&Text` is any type that can be
converted into `Text`.  You may use a `&Text` type the same way you use a `Text`
type.  So simple usage of `log()` would look like this:

```aratar
log(["I have the number: ", 5, ", what do you have?"])
```

This prints: "I have the number: 5, what do you have?".  You can further
simplify the function call to this:

```aratar
log("I have the number: ", 5, ", what do you have?")
```

The reason this works is because the type `(List[&Text])` is equivalent to the
type `List[&Text]` which can be automatically casted from a tuple containing all
`&Text` values.  Function calls must use `()`.  Through this mechanism you can
create variable argument functions.

[Next: Comments](comments.md)
