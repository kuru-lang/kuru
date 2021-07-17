# Semantics (version 0.0.19)
Unlike other languages, Aratar proves that your code never reaches unreachable
code.  If the compiler can prove your code will reach unreachable code or loop
infinitely, then your code won't compile.  The exception to this rule is async
event loops.  Recursion is not allowed.

## Borrow Checking
Aratar uses a simple borrow-checking mechanism and hides all lifetimes.  All
variables (and therefore borrows) go out of scope at the earliest possible
location (statement where it's last used).  If you need to keep something in
scope for longer you can use this trick:

```aratar
let var_a: 5
# var_a would usually go out of scope here
let var_b: 12
let _: var_a # but actually goes out of scope here
```

```aratar
let a: @0
let b: @0

let f: Fn(@i, @j) { # ... # }

# Doesn't Compile: `a` can't be borrowed in multiple places at the same time.
f(@a, @a)

# Compiles (only because `a` implements `Copy`, otherwise it would move).
let b: @a # Evaluate a, make it a mutable reference, assign to b (copy).
a(@a, @b)
```

## Type Checking / Bounds Checking
```aratar
# Won't compile
let int Int[0~]: -1
let hex Int: 0xA8

# Will compile
let int Int[-1~]: -1
let hex Hex: 0xA8
```

## Control Flow Analysis
At compile time, unbounded types like `Int` must resolve to a specific bounded
type.  In the "Will Compile" example the type for `a` is `Int[0~1]`.

```aratar
let loop: Fn(a: Int, b: Int) -> Int {
    if a = b { return a }
    let a: a + 1
    loop(a, b)
}

# Won't Compile
loop(1, 0)

# Will Compile
loop(0, 1)
```

### Non-Data Dependant Infinite Loops
```aratar
# Won't Compile
let recursive: fn(a: Int) {
    recursive(a)
}
recursive(12)
```

### Iterators
```aratar
# An example infinite iterator.
struct IteratorA(index Int, upto Int)

let IteratorA.Iterator(
    next: Fn(@iter) {
        Some(iter.index)
    }
)

# An example finite iterator.
struct IteratorB(index @Int, upto Int)

let IteratorB.Iterator: [
    next: Fn(@iter) -> Int {
        if iter.index = iter.upto {
            None()
        } else {
            iter.index +: 1
            Some(iter.index)
        }
    }
]

# Won't Compile
def start: Fn(@env) {
    for item: IteratorA(index: 0, upto: 256) {
        dbg(item)
    }
}

# Will Compile
def start: Fn(@env) {
    for item: IteratorB(index: 0, upto: 256) {
        dbg(item)
    }
}

# Won't Compile
def start: Fn(@env) {
    for item: IteratorB(index: 1, upto: 0) {
        dbg(item)
    }
}
```

### Exception: Async Event Loops
```aratar
# This function is entered once at the start of the program.
def start(@sys) {
    # ... (Initialization)
    sys.input(input)
}

# This function is entered whenever the program receives user input.
let input: Fn(@sys, event) {
    if event [
        Text(text: text.is_letter() & !text.is_whitespace()) {
            dbg(text)
        }
        Key(Press, key: Escape() | Q()) {
            dbg("Quit key used: ", key)
            sys.quit()
        }
        Mouse(_x: _, _y: _) { # FIXME # }
        _: {}
    ]
}
```

## Visibility
```aratar
# Define a struct (exported by default)
struct Struct(
    # Not Exported
    let value_a @Int
    # Exported
    def value_b @Hex
    # Exported constructor
    def: Fn() -> Struct {
        Struct(value_a: 222, value_b: 0xDE)
    }
)

# Call constructor (compiles)
let struct: Struct(value_a: 222, value_b: 0xDE)
# Load (Read) and print (compiles)
dbg(struct.value_a)
# Store (Assign) (compiles)
struct.value_b: 0x00
```

In another module:

```aratar
use main[Struct]

# Call constructor (doesn't compile, raw constructors are not exported)
let struct: Struct(value_a: 222, value_b: 0xDE)
# Load (Read) and print (doesn't compile, let statements not exported)
dbg(struct.value_a)
# Store (Assign) (doesn't compile, public struct items are always read-only)
struct.value_b: 0x00

# Compiles:
let struct: Struct
dbg(struct.value_b)
```

## Equivalence And Lazy Expression
```aratar
'scope { }
```

is the same as:

```aratar
'scope fn() { }
```

### Iterators
```aratar
let state: @10
let iter: {
    state -: 1
    match state [
        0: { None() }
        _: { Some(state + 1) }
    }
}
let iter: iter.Iter.map Fn[number] {
    dbg(number)
}
dbg("Running iterator:")
let _: iter # Iterator will run on free
```

## Lists
It is impossible to index a list out of bounds.  The compiler must catch it at
compile time.

```aratar
let list: [1, 2, 3]
list.get(2) # Compiles
list.get(3) # Doesn't compile
```

## Trait Objects And Specialization
Possible concrete types must be decided on at compile time.

```aratar
fn function(trait .Trait) {
    match trait {
        ConcreteType {
            dbg("Is concrete")
        }
        OtherType {
            dbg("Is other")
        }
        type: _ { # optional, allows other types to be passed #
            dbg(type)
        }
    }
}

function(ConcreteType)
function(OtherType)
function(Int)
```
