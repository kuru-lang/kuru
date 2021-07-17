# Aratar (v0.0.21) Programming Language Specification
Aratar takes inspiration mostly from Rust, but also borrows ideas from many
other languages (Ada, Python, Mathematica, C, Haskell, Swift, Lisp, JavaScript,
Alan, R, GLSL and Dart).
It's designed to be at least as easy as Python, while still being strongly typed
with functional programming constructs and a focus on determinism and
correctness verified at compile time.

## Types of Code
 - Project: A program or bundle
   - Program (prg): has a `start()` function (known as a "binary" in other
     languages).
     - App: program with a user interface
     - Component: program without a user interface, but data interface
   - Bundle (bnd): does not have a `start()` function (known as a "static
     library" in other languages).

## Compiling and Linking
Aratar is designed to compile directly to RISC-V machine code.
After which, a translator may be used to run it on another ISA.
Like Rust, Aratar is designed to do compile-time inclusion of components in the
resulting program.
But, unique to Aratar, only the definitions that are used are included in the
compiled program.
This is possible because of a breadth-first dependency traversal (rather than
other languages that do depth-first; compile dependencies up to the root node).
Parts of components are compiled as they are needed.

## Safety
Aratar, like Rust, doesn't let you use raw pointers.
Data may only be owned by one thread at a time - there are no mutexes or any
kind of reference counting.
All lifetimes are decided at compile-time.
To share data, use a `Stream[T; n]`.
`n` is the number of events that the `Stream` queue can hold.
Streams are one-directional, but you may have any number of producers and
consumers.
Streams make use of asynchronous programming.

## Asynchronous Programming
The asynchronous programming model takes inspiration from Python (with the
`yield` keyword), Rust (with `.await`, except called `.next()` - same as
iterator), JavaScript (with a similar simple single-threaded runtime) and Alan
(with automatic parallelization, if available).

```aratar
#!/opt/rinvey/aratar 0.0.21

# Run two timers at the same time.
export fn start() {
    let slow: Timer(Millis(1000))
    let quick: Timer(Millis(250))
    # Repeat 5 times, will always print in this order:
    #   quick (250 ms)
    #   quick (500 ms)
    #   quick (750 ms)
    #   slow (1000 ms)
    #   quick (1000 ms)
    for i: 5 {
        match [slow, quick].next()! {
            # You can either ignore the unit-tuple return value
            (slow, _) { log("slow"), slow.reset() }
            # …Or specify the unit tuple structure.
            (quick, ()) { log("quick"), quick.reset() }
        }
    }
}
```

```aratar
#!/opt/rinvey/aratar 0.0.21

# Run two timers at the same time.
export fn start() {
    # Print the messages yielded from the future.
    for message: future() {
        log(message)
    }
}

# Functions defined with `@` are known as generators.  Return statements are not
# allowed in generators, but is the only place you can use yield statements,
# which suspend the generator and return a value.  Generators do not execute any
# code until you call `next()` or use a `for` loop.  Calling `next()` again
# continues executing where it left off until the next yield statement.
fn @future() {
    yield "started future, waiting on timer"
    Timer(Millis(250)).next()!
    yield "waited, press any key"
    Keyboard().next()!
    yield "you pressed a key"
    Timer(Millis(250)).next()!
    yield "waited some more"
}
```

```
#!/opt/rinvey/aratar 0.0.21

export fn start() {
    # Go through the entire generator.
    for message: future() until message = 3 { }
}

fn @future() {
    for i: 4 {
        yield i
    }
}
```

```
#!/opt/rinvey/aratar 0.0.21

# Import the aratar bundle function; event() which gets the next event.
import aratar.event

export fn start() {
    # Put this task asleep until get_event() future returns EventName
    until EventName(let event_data) = event() { }
    # Match on returned data.
    match event_data {
        1...3 { # # }
        _ { # # }
    }
}
```

## Turing-Incomplete
The Aratar language does not provide a mechanism to allow functions not to halt.
There is no `while` loop, much like the Alan programming language.  All
iteration is provably finite and done with `for` and `until` loops.  The async
executor handles the case where you'd want an infinite loop for the user
interface.

## Constraints
Types may have constraints added:

```aratar
Int 0...100
```

This means the integer can only ever be between 0 and 100.  This type will
always be stored as a 8-bit unsigned integer, as Aratar always picks the
smallest type.  These constraints must be violated; if they are, it's caught at
compile time based on the constraints and arithmetic used on any variables of
that type.

```aratar
List[Int %2; ...5]
```

This means a list with up to 5 even integers in it.

```aratar
Float[~0.0]
```

This refers to a non-zero floating-point number (non-zero numbers are required
for division on types that don't have an NaN in their domain, but Float does).

### Modifiers
There are more than just constraints, also:

 - `@`: Mutable (Must be the first modifier)

Classes (Must be first or second):
 - `Int`: Integer 
 - `Real`: Real Number Expression
 - `Dec`: Fixed Point
 - `Float`: Floating Point
 - `Complex`: Complex Number Expression
 - `Hex`: Hexadecimal Integer
 - `Bin`: Base 2 Binary Integer
 - `Data`: Raw Big Endian Data
 - `List[T]`: A growable array
 - `Text`: UTF-8 Text (Is a special type of `List`)
 - `Num`: Any type that can do arithmetic
 - `Table`: Key(s)-value(s) mapping (HashMap or Database table)

All classes exist on the heap by default.  If the compiler can prove they don't
need to be, then they'll exist on the stack.  The compiler uses whether or not
the variable changes size to determine if it needs to be on the heap.  The
easiest way to guarantee it's on the stack is to add constraints.  Classes are
just structs that are built in to the compiler and may be implemented
differently depending on how they are used.  Aratar classes are also the
equivalent of mathematical sets.

## Match
```aratar
#!/opt/rinvey/aratar 0.0.21

# A path operation
enum PathOp [
    MOVE(Fixed16, Fixed16) # Move to a point
    # Straight line to end point
    LINE(Fixed16, Fixed16)
    # Quadratic bézier curve
    # (control point and end point)
    QUAD(Fixed16, Fixed16, Fixed16, Fixed16)
]

# Demonstrate a match statement.
export fn start(
    path_op PathOp # A path operation
) {
    match path_op [
        MOVE(let(x, y)) { # … # }
        LINE(let(x, y)) { # … # }
        QUAD(let(x1, y1, x2, y2)) { # … # }
    ]
}
```

## I/O
```aratar
#!/opt/rinvey/aratar 0.0.21

import aratar[log, concat]

# Read from a file.
#
# The `start()` function should be commented with the program-level
# documentation.
export fn start(
    url Url # The path or URL of the file to open.
) -> Try[] {
    if not(url.is_file()) {
        return FAIL("Must provide a file path.")
    }
    let contents @_: List()
    for data: Stream(url) {
        contents ++: data
    }
    if SOME(let contents) = Opt[Text](contents) {
        log(contents)
    } else {
        return FAIL(concat("File ", url, " is not valid UTF-8."))
    }
    OK()
}
```

```aratar
#!/opt/rinvey/aratar 0.0.21

import aratar[log]

# Read from standard input.
export fn start() {
    let in: Keyboard()
    let line: in.next()!
    log(line)
}
```

```aratar
#!/opt/rinvey/aratar 0.0.21

import aratar[log]

# Use a timer.
export fn start() {
    let in Stream: Keyboard()
    Timer(Millis(999)).next()!
    log("Almost 1 second has passed!")
    # Closures don't specify parameter or return types.
    for _: Timer(Millis(500)).until(in, fn(key) { key = " " }) {
        log("Another half second has passed")
    }
}
```

### URL Structures
 - `/absolute/path/to/file`: A shortcut
 - `~/file`: A shortcut
 - `./file`: A shortcut
 - `file://absolute/path/to/file`
 - `http://subdomain.website.tld/webpage`
 - `https://subdomain.website.tld/webpage`
 - `mailto:email@email.tld`
 - `ftp:localhost`
 - `tcp:10.0.0.0:25565`
 - `udp:10.0.0.0:22222`

## Singular Comment Syntax
```aratar
###
This is a multi-line comment.
###

# This is a line-comment; when above a function, documents that function.
export fn start() {
    function(a # This is an inline comment, next is empty comment #, b ##)
}
```

## Small But Complete Standard Library
The standard library is the most minimal version of batteries-included possible.
What this means is things like opening a window and playing an audio sample will
be in the standard library.  Most standard libraries at least have file
saving/loading as a system iterface, but these APIs are included to make system
interface support feel more complete.  Things like parsing a WAV file will not
be included, though.

## Quick Iteration Times
The goal for the compiler is to compile very fast and produce very fast code.
One way this is done is by compiling modules before their dependencies, and by
using very simple incremental compilation (if a function changes, recompile the
function, not the entire module).  The compiler also shall use techniques
inspired from `cargo-watch` and `cargo-web`.

## Casting
```aratar
#!/opt/rinvey/aratar 0.0.21

import aratar[assert]

# Demonstrate casting.
export fn start() {
    # Cast an integer into Text (same as GLSL casting).  If the type has a
    # method called `.Text(self) -> Text` then it can be converted this way.
    let a Text: Text(15)
    assert(a = "15")
}
```

## Rust-Cfg/Lisp Style Logic
 - `fn not(in [Bool]) -> Bool`: Returns `TRUE` if all parameters are `FALSE`.
 - `fn any(in [Bool]) -> Bool`: Returns `TRUE` if 1+ parameters are `TRUE`.
 - `fn all(in [Bool]) -> Bool`: Returns `TRUE` if all parameters are `TRUE`.
 - `fn one(in [Bool]) -> Bool`: Returns `TRUE` if exactly 1 parameter is `TRUE`.

```aratar
# This uses tuple with same-type fields coercing into a list.
assert(all(TRUE, FALSE) = FALSE)

# You may also pass in a List.
assert(all([TRUE, FALSE]) = FALSE)
```

### Similar Functions
 - `fn sum(in [Number]) -> Number`: Add numbers together (get sum).
 - `fn product(in [Number]) -> Number`: Multiply numbers together (get product).
 - `fn concat[T](in [List[T]]) -> List[T]`: Concatenate lists together.

## Parameter-Based Namespaces
```aratar
#!/opt/rinvey/aratar 0.0.21

struct TypeA()
struct TypeB()

# Define a constructor (casting conversion) for `Text`, taking a `TypeA`
export fn Text(_from TypeA) -> Text {
    "Type A"
}

# Define another constructor for `Text`, that doesn't conflict.
export fn Text(_from TypeB) -> Text {
    "Type B"
}
```

## Traits
Traits are not a built-in language feature, but a programming pattern, much like
builders.  You can define a struct as a "trait" and implement conversions into
that struct from multiple types.

```aratar
#!/opt/rinvey/aratar 0.0.21

struct Trait(field: Int)

fn Trait.hello_world(self) {
    log("Hello, world ", self.field, "!")
}

# Struct that can be converted into `Trait`
struct StructA()

fn Trait(_from StructA) -> Trait {
    Trait(field: 4)
}

# Struct that can be converted into `Trait`
struct StructB(field: Int)

fn Trait(from StructB) -> Trait {
    let field = from.field
    # If the name of the field matches the local variable, you can use this
    # shortcut to only type it once.
    Trait(field: _)
}

# This function takes any type that can be converted into `Trait`.
fn use_trait(trait .Trait) {
    match trait [
        StructA { log("specialized codepath for StructA") }
        StructB { log("specialized codepath for StructB") }
        # Will give "unreachable" warning in this example.
        _ { log("default codepath (optional, only to support other structs)") }
    ]
    Trait(trait).hello_world()
}

export fn start() {
    # Log "specialized codepath for StructA", "Hello, world 4!"
    use_trait(StructA())
    # Log "specialized codepath for StructB", "Hello, world 15!"
    use_trait(StructB(field: 15))
}
```

## Literals
```aratar
fn() { } # Closure
[1, 2, 3] # List
(1, 2, 3) # Tuple
["key": 0, "num": 2] # Table
Struct(key: 0, num: 2) # Struct, only usable from module the struct's defined in
# Table with multiple keys (actually just one key that's a struct).
[
    Keys(id: 0, name: "zero"): Values(level: 4)
]
```

### Equivalence
Tuples and lists with item count of 1 have type system equivalence to their
contained item.

```aratar
assert([1] = 1)
assert((1) = 1)
```

This makes it possible to use all 3 types of brackets as parentheses for
mathematical expressions:

```aratar
assert({(4 + [3 * 2]) / 5} = 2)
```

### Strict-Type Safety
In most languages, like Rust, you can do:
```rust
let a: u32 = 7 & 9;
```

But in Aratar, that is forbidden because doing a bitwise and is only logical on
the bits themselves.  You can get around it by doing:

```aratar
let a: Int(Bin(7) * Bin(9))
```

That is the equivalent program.  Notice that the operator is `*`; that is
because you are multiplying the individual bits in the binary type, since
multiplying bits as if they are an integer doesn't make sense.

```aratar
# Binary Operators
* # Bitwise Logical AND (multiplying of bits)
+ # Bitwise Logical OR (saturating adding of bits)
- # Bitwise Logical XOR (wrapping subtraction of bits)
~ # Bitwise Logical NOT
>> # Bitwise Logical Shift Right
<< # Bitwise Logical Shift Left
= # Bitwise Equivalence
<> # Bitwise Inequivalence
```

## Style
Recommended: 4 space tabs, 80 column width

- `snake_case`: variable
- `snake_case()`: function
- `CamelCase`: type
- `CamelCase()`: casting method / struct instance / enum variant / constant

## Syntax
 - TAB|VERTICAL\_TAB|CARRAIGE\_RETURN: Compiler Error
 - `_`:
   - `0~9`|`a-z`|`_`: Unused identifier
   - `A-Z`: Compiler error
 - `a`~`z`:
   - `0~9`|`a-z`|`_`: Identifier (functions, variables, and keywords)
     - `import`: Import a definition from a module `import aratar.fft{0.1}`
     - `export`: Export a definition.
     - `def`: Define a constant.
     - `let`: Declare a variable `let samples: @[1, 2, 3, 4, 5, 6, 7, 8]`
     - `enum`: Define Associated Union Type
     - `struct`: Define Record Type
     - `typedef`: Define Type-Safe Type Alias (Must use casting)
     - `fn`: Define a function
     - `for`: Iteration
     - `if`: Conditional Branching
     - `else`: Alternative Conditional Branch
     - `match`: Pattern matching
     - `yield`: Suspend the function and yield to it's caller optionally with a value.
     - `return`: Early exit from outermost scope or named inner scope
     - `until`: Used in sleep statement, but also as the opposite of a while loop.
     - `syscall`: Call into system component
   - `A`~`Z`: Compiler error
 - `A`~`Z`:
   - `0`~`9`|`A`~`Z` | `a`~`z` | `_`: Type (constants, structs, enums, and enum variants)
     - `NAN`: Literal Constant Not A Number
     - `INF`: Literal Constant Infinity
     - `TRUE`: Literal Constant (Bool) true
     - `FALSE`: Literal Constant (Bool) false
     - `SOME`: Literal Constant (Opt) some
     - `NONE`: Literal Constant (Opt) none
     - `OK`: Literal Constant (Try) ok: success
     - `FAIL`: Literal Constant (Try) failure: error
     - `DEFAULT`: Literal Constant Default Value
 - `0`~`9`: Literal Constant Number
   - `0`~`9`|`_`|`.`: Decimal (base 10) Literal Constant
     - `e`: ×10^x
     - `f`: Floating point Literal Constant
     - `i`|`j`: Imaginary Literal Constant
   - `0x`: Hexadecimal (base 16) Literal Constant
     - `0`~`9`|`A`~`F`: Hexadecimal Literal Constant
     - `a`~`z`|`G`~`Z`: Compiler Error
   - `0b`:
     - `0~1`: Binary (base 2) Literal Constant
   - `0o`:
     - `0~7`: Octal (base 8) Literal Constant
 - `"`: Literal Text (Until `"`, unless it's `""` then include `"` in literal)
   - `""`: Empty Text
   - `"""`: Multi-Line Text Open/Close (on it's own line)
   - `""""`: Super Multi-Line Text Open/Close (on it's own line)
 - `#`: Comment Open (Until newline or `#`)
   - `##`: Empty Comment
   - `###`: Multi-Line Comment Open/Close (on it's own line)
   - `####`: Super Multi-Line Comment Open/Close (on it's own line)
 - `'`: Scope Label
 - `:`: Assignment
   - `::`: Swap
 - `%`|`&`|`*`|`+`|`/`|`;`|`<`|`=`|`>`|`^`|`|`: Binary Operator (limited to 4
   ascii graphemes)
   - `+`: (Vector) Addition (`[4, 3] + [2, 4] = [6, 7]`)
   - `+;`: Multi-Addition (`[1, 2, 3] +; 1 = [2, 3, 4]`)
   - `++`: Concatenate / Join (`"ab" ++ "cd" = "abcd"`)
   - `++;`: Multi-Join (`["a", "b", "c"] ++; "d" = ["ad", "bd", "cd"]`)
   - `+-`: Either Positive or Negative (`±`)
   - `*`: (Vector) Multiplication (`[4, 3] * [2, 4] = [8, 12]`)
   - `*;`: Multi-Multiplication (`[1, 2, 3] *; 3 = [3, 6, 9]`)
   - `**`: Dot Product / Matrix Multiply (`[1, 2, 3] ** [2, 1, 1] = [7] = 7`)
   - `**;`: Multi-Dot Product (`[[1, 2], [3, 4]] **; [5, 6] = [17, 39]`)
   - `/`: (Vector) Division (`16 / 6 = Frac(8, 3)`)
   - `/;`: Multi-Division (`[16, 32] /; 6 = [Frac(8, 3), Frac(16, 3)]`)
   - `//`: Integer division rounding to zero (`16 / 6 = 2`, `-16 / 6 = -2`)
   - `//;`: Multi-Integer division (`[16, 32] //; 6 = [2, 5]`)
   - `%`: (Vector) Modulo (`8 % 3 = 2`, `-8 % 3 = 1`, `8 % -3 = -1`, `-8 % -3 = -2`)
   - `%;`: Multi-Modulo (`[8, 9] %; 3 = [2, 0]`)
   - `%%`: (Vector) Remainder (`8 % 3 = 2`, `-8 % 3 = -2`, `8 % -3 = -2`, `-8 % -3 = 2`)
   - `%%;`: Multi-Modulo (`[8, 9] %%; 3 = [2, 0]`)
   - `%=`: (Vector) Is divisible by (`(8 %= 3) = FALSE`, `(0 %= 3) = TRUE`)
   - `%=;`: Multi-Divides (`([8, 9] %=; 3) = [FALSE, TRUE]`)
   - `^`: (Vector) Power (`4 ^ 2 = 4² = 16`)
   - `^;`: Multi-Power (`[4, 3] ^; 2 = [16, 9]`)
   - `&`: Pattern Matching And / Sets Intersection (`∩`)
   - `&;`: Multi-Intersection
   - `|`: Pattern Matching Or / Sets Union (`∪`)
   - `|;`: Multi-Union
   - `=`: (Vector) Equal (`=`)
   - `=;`: Multi-Equal (`([4, 3] =; 3) = [FALSE, TRUE]`)
   - `>`: (Vector) Greater Than (`>`)
   - `>;`: Multi-Greater Than
   - `>=`: (Vector) Greater Than or Equal (`≥`)
   - `>=;`: Multi-Greater Than Or Equal
   - `>>`: (Vector) Right Shift Bits Logical (`»`)
   - `>>;`: Multi-Right Shift Bits Logical
   - `<`: (Vector) Less Than (`<`)
   - `<;`: Multi-Less Than
   - `<=`: (Vector) Less Than or Equal (`≤`)
   - `<=;`: Multi-Less Than Or Equal To
   - `<<`: (Vector) Left Shift Bits Logical (`«`)
   - `<<;`: Multi-Left Shift Bits Logical
   - `;`: Data Repetition (`"abc" ; 2 = "abcabc"`, `(4; 3, 5) = (4, 4, 4, 5)`)
   - `;;`: Multi-Repitition (`"abc" ;; 2 = "aabbcc"`, `((4, 5) ;; 2) = (4, 4, 5, 5)`)
 - `!`|`@`|`?`|`-`|`~`: Possibly Unary Operator
   - `~`: Bitwise Not
   - `-`: (Vector) Negation, Subtraction (`-`)
   - `-;`: Multi-Subtraction
   - `<>`: (Vector) Not equal (`≠`)
   - `<>;`: Multi-Not Equal
   - `@`: Mutable Reference
   - `?`: Try: Early return on error for outermost scope or named inner scope.
   - `!`: Compiler-Try: Fail to compile if it's possible to fail.
 - `$`: Auto-Derive Type Conversions
 - `$$`: Template Syntax (TBD, possibly not needed)
 - `(`|`)`: Tuple/Arguments Open / Close
 - `[`|`]`: List/Map/Set/Generics Open / Close
 - `{`|`}`: Scope/Order Open / Close
 - `.`:
   - `.`: Accessor
   - `..`: UNUSED (Purposely)
   - `...`: Inclusive Range (For pattern matching and type bounds)
 - `\`: Start program or component
 - ` `: Token Separator
 - `,`|NEWLINE: List Separator

# Specification
- [Syntax](SYNTAX.md)
- [Semantics](SEMANTICS.md)
