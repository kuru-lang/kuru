# Hello World
Let's start by making a file `hello_world.src`, with the most basic program:

```aratar
pub def run() {
    log("Hello, world!")
}
```

Which outputs (run by typing `hello_world.src`):

```output
Hello, world!
```

# Breaking It Down
## `pub`
The `pub` is a modifier that makes our function public.

## `def`
This keyword starts the definition of a constant, type, or function.

## `run`
This is the name of our function.  "run" is a special function name, that when
made public, becomes the function that's called when our program runs.

## `()`
Functions names must be followed by `(`, then a list of parameters, then `)`.
This function has no parameters.

## `{}`
The curly brackets are used to show visibly where the scope of the function
begins and ends.

## `log("Hello, world!")`
The `log` function is a special function in that it's the only function that is
always in scope.  You call functions the same way you would in other languages.

# Next Section
[Comments](comments.md)












## The `export` Keyword
The `export` keyword exports an item to either a program using your code as a
library or the system that's running your program.  Programs must define a
`program()` function, and libraries must not.

## The `def` Keyword
The `def` keyword defines an item.  You can use it to define the following:

### Functions
The following is a function without parameters, or a return value:
```aratar
def my_function()
```

Function with parameters:

```aratar
def my_function(a Int, b Text)
```

Function with return value:
```aratar
def my_function Int()
```

### Structs
Use `struct` to define a tuple with named fields.  Public fields are disallowed
on purpose.

```aratar
struct MyStruct(
    field_a Text: "default value"
    field_b Int
)
```

### Enums
Use `enum` to define a set of units and tuples.  This type inherits a conversion
to `Copy` and `Text` from contained types.

```aratar
enum MyEnum: Copy Text (
    VariantA
    VariantB
    VariantC
)
```

### Generics
Structs and enums may be made generic over types and values.

```aratar
# The type for generics is automatically assumed to be `Type`, so you don't have
# to add it.
enum Either[A, B Type](
    VariantA(A)
    VariantB(B)
)

# Non-type generics need a type appended to their definition, though, and may be
# followed by modifiers.
struct Struct[N Int >0](
    values [Byte; N]
)
```

### Constants
Repetition is disallowed in sets.

```aratar
def MyList: [1, 2, 3, 4, 5, 6, 7]
def MyListWithRepetition: [0; 4, 1; 4, 2; 4, 3; 4]
def MyTuple: (1, 2, 3)
def MyTupleWithRepetition: (1; 2, "c"; 2)
def MySet: {1, 2, 3}
def MyInstance: Type()
```

### Type def
```
def MyType: YourType
```

## Scopes


```aratar
def function(a Num, b Num, c Num):
    function_a(a, b, c)
    function_a(
        a
            + b
            + c,
        a - b,
        c,
    )
    Builder()
        .function_a()
        .function_b()
        .function_c()
        .finish()
    let closure(x, y): x + y
    let x_from_scope:
        let a: closure(0, 1)
        let b: closure(2, 3)
        closure(a, b)
```

## The `log()` Function and Variable Arguments
By default, there are a few functions and types imported into scope.  You may
not define functions or types with the same names.  One of these is the `log`
function, which has the following function signature (square brackets are used
for generics, as they are a list of types):

```aratar
def log(as_text List[&Text])
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
