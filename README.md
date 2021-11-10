# Kuru
A simple programming language inspired by Rust and Python.

```kuru
import kuru(0.1)

export fn() {
    kuru.log("Hello, world!")
}
```

## Contents
 - [Installation](#installation)
 - [Types](#types)
 - [DOLU](#dolu)
 - [Keywords](#keywords)

## Installation
**TODO**

## Types
The most important type in Kuru is a `Set`.  All types in Kuru are subtypes of
`Set`.  There are also specific operators that always operate on `Set`s.

```kuru
import kuru(0.1)
import kuru[Set]

export fn() {
    let set: Set(4)
    kuru.log(set)
}
```

```output
Set(4)
```

The `Set[T]` type is implemented as a `List[T]`, with the additional invariant
that no two equivalent `T`s can be present in it at the same time.

### Set Operators
```kuru
# Set union operator
0 | 2 | 5

# Set range operator
-1:1

# Set intersection operator (resolves to the set of just `Int(0)`)
{-1:1} & {0 | 2 | 5}
```

### Comparison
Comparisons always operate on sets.

```kuru
if var = 0:15 {
    kuru.log("variable is between 0 and 15 inclusive")
}

if var != 0:15 {
    kuru.log("variable is not between 0 and 15 inclusive")
}
```

### Type Definitions
```kuru
# `Int` struct takes optional generic of a bounded set of literal integers
struct Int[Set[Literal.Int]?](
    Literal.Int
)

# `Opt` enum takes generic
enum Opt[T]
    | None
    | Some[T]

```

## DOLU
DOLU (Discard On Last Use) is similar to RAII but slightly different.

```kuru
fn dolu() {
    kuru.log("Hello, world!")
    # Returned `Log` future is discarded here (`await()` called in `discard()`)

    # Get log future
    let log: kuru.log("Hello, world!")
    # Await the future (yield to executor with the future)
    log.await()
    # `log` is discarded

    # … (rest of function that doesn't use `log` variable)
}
```

## Limited Closures
```kuru
fn takes_function(func: Fn(Int, Text) -> Text) -> Text {
    return func(5, "hello")
}

fn my_function(a: Int, b: Float, c: Text) -> Text {
    "".cat(a).cat(b).cat(c)
}

export fn() {
    takes_function(fn => my_function(_, 3.14, _))
}
```

## Keywords
There are only 16 keywords in Kuru (8 compile-time keywords, and 8 run-time
keywords).  No more are planned.

```kuru
import # import a definition
export # export a definition
fn # define a function (syntactic sugar for `static _: Fn() {}`)
static # define an unchanging value
struct # define a data structure (type theory name: record)
enum # define an data enumeration (type theory name: discriminated union)
typedef # define a data type alias
trait # define a data trait

let # declare a variable
for # iterate over
yield # create a suspension point in a function (yield with a value or future)
if # control flow (when boolean expression is `True`)
else # control flow (then, when boolean expression is `False`)
match # control flow (check multiple `Set`s for equivalence)
break # early return from `for`
return # early return from `fn`
```
