# Control Flow

## If
```aratar
let a: 4
if a = 2 | 3 {
    log("This shouldn't happen, as `a` is 4 - not 2 or 3")
}
```

## Else
```aratar
let a: 4
if a & Text {
    log("The number 4 can be turned into text: ", a)
} else if a = 4 {
    log("This shouldn't happen")
} else {
    log("This shouldn't happen, either")
}
```

## Logic
Logical decision making is done in a way inspired by Lisp:

```aratar
let a: 4
if all(a = 4, a & Text) {
    log("It works!")
}
```

 - `all(values [&Bool]): Bool`: Returns true if they're all true (AND)
 - `one(values [&Bool]): Bool`: Returns true if only one is true (XOR)
 - `any(values [&Bool]): Bool`: Returns true if any are true (IOR)
 - `not(values [&Bool]): Bool`: Returns true if none are true (NOR)

## If Let
While similar to Rust's `if let`, has significant differences:

```aratar
let a: Some(4)
if Some(let _a) = a {
    # …
}
```

## Match
Match statements test equivalence (`=`).  They are slightly different than
Rust's.

```aratar
let a: Some(4)
match a [
    Some(let _a) { ## }
    None { ## }
]
```

You can use 1-equivalence to make your code shorter:

```aratar
enum LotsOfValues [
    A(Int, Int, Int)
    B(Int, Int, Int)
]

let a: LotsOfValues.A(1, 2, 3)

match a [
    # Note that shadowing variable names works.
    A(let (a, b, c)) { log(a, " ", b, " ", c) }
    B(let tuple) {
        log(tuple.0, " ", tuple.1, " ", tuple.2)
    }
]
```

You can also do "trait" specialization:

```aratar
struct A()
struct B()

struct TraitA()
struct TraitB()

fn A.TraitA(_self) -> TraitA {
    TraitA()
}

fn B.TraitB(_self) -> TraitB {
    TraitB()
}

fn specialized(var: &TraitA | &TraitB) {
    match var [
        &TraitA { log("Specialized A Code") }
        &TraitB { log("Specialized B Code") }
    ]
}

export fn run() {
    specialized(A()) # Prints "Specialized A Code"
    specialized(B()) # Prints "Specialized B Code"
}
```

## Iteration
```aratar
let a: [1, 2, 3, 4]
# The Fn keyword, unlike fn, infers return type and parameter types.
# The `.iter()` creates an iterator adapter.
let iter: a.iter(Fn(value) {
    value -: 1
})
# Use a for loop to go through the data (prints out "0", "1", "2", "3").
for value: iter {
    log(value)
}
```

## Asynchronous Iteration
Because `Screen` and `InputListener` are infinite iterators, you must wrap them
in an `Until` iterator.  The thing about `Until` is that the first parameter
must be a type that occurs in the domain of the second parameter's iteration
type.  This makes it impossible to make a program that can never stop - note
however that if the user never does anything the program will run forever.

```aratar
fn run() {
    let screen: Screen("My Application Name")
    let input: InputListener()
    let listener: Until(Input(Esc), [screen, input])
    for event: listener {
        # `event`'s type is `Input | Draw`, so we have to match to use it.
        match event [
            Input(_) { # Ignore # }
            Draw(canvas) {
                canvas.redraw(0, 255, 0)
            }
        ]
    }
}
```

## Simple Iteration
Integers have supertype `Iter`, so you can do this:

```aratar
# Prints "0", "1", "2", "3", "4"
for i: 5 {
    log(i)
}
```

If you don't want to start at zero, you can also do:

```aratar
# Prints "1", "2", "3", "4", "5"
for i: Range(1, 6) {
    log(i)
}
```

## Complicated Iteration
Sometimes, it may be necessary to make an iterator that can't be built from
simple adapters (calling `.iter()`)

```aratar
# Initialize `a` with integers from 0 to 9
let a [Int; 10]
for (i, v): Enumerate(@a) {
    v: i
}

# Build an iterator over the even numbers.
let iter: Iter(Fn(yield) {
    for i: a {
        if i %= 2 {
            yield(i)
        }
    }
})

# Print "0", "2", "4", "6", "8"
for i: iter {
    log(i)
}
```
