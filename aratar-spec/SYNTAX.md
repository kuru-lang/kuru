# Syntax (version 0.0.20)

## Token Types
Starts with:
 - `\t`|`\r`: Compiler Error
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
     - `elif`: Compressed `else { if {} }`
     - `match`: Pattern matching
     - `yield`: Suspend the function and yield to it's caller optionally with a value.
     - `return`: Early exit from outermost scope or named inner scope
     - `syscall`: A syscall (unsafe, can only call from module named "sys")
     - `all`: Built-in Function (`all(TRUE, TRUE) = TRUE`)
     - `any`: Built-in Function (`any(FALSE, TRUE) = TRUE`)
   - `A`~`Z`: Compiler error
 - `A`~`Z`:
   - `0`~`9`|`A`~`Z` | `a`~`z` | `_`: Type (constants, structs, enums, and enum variants)
     - `NAN`: Literal Constant Not A Number
     - `INF`: Literal Constant Infinity
     - `TRUE`: Literal Constant True
     - `FALSE`: Literal Constant False
     - `SOME`: Literal Constant Opt.Some
     - `NONE`: Literal Constant Opt.None
     - `OK`: Literal Constant Try.Ok
     - `ERR`: Literal Constant Try.Err
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
 - `#`: Comment Open (Until newline or `#`)
   - `##`: Empty Comment
   - `###`: Multi-Line Comment Open/Close (on it's own line)
 - `'`: Loop Label
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
   - `^^`: (Vector) Bitwise XOR (`TRUE ^^ TRUE = FALSE`)
   - `^^;`: Multi-Bitwise-XOR (`([TRUE, FALSE] ^^; TRUE) = [FALSE, TRUE]`)
   - `&`: Pattern Matching And / Sets Intersection (`∩`)
   - `&;`: Multi-Intersection
   - `&&`: (Vector) Bitwise AND (`&`)
   - `&&;`: Multi-Bitwise AND
   - `|`: Pattern Matching Or / Sets Union (`∪`)
   - `|;`: Multi-Union
   - `||`: (Vector) Bitwise OR (`|`)
   - `||;`: Multi-Bitwise OR
   - `=`: (Vector) Equal (`=`)
   - `=;`: Multi-Equal (`([4, 3] =; 3) = [FALSE, TRUE]`)
   - `==`: UNUSED (Purposely)
   - `===`: Memory Indices Equal (If references point to same location)
   - `===;`: Multi-Ref-Comparision (If references point to same location)
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
   - `~`: Inclusive Range (For pattern matching and type bounds)
   - `~~`: (Vector) Bitwise Not
   - `-`: (Vector) Negation, Subtraction (`-`)
   - `-;`: Multi-Subtraction
   - `!`: Pattern Matching Not, (Vector) Boolean Not (`¬`)
   - `!=`: (Vector) Not equal (`≠`)
   - `!=;`: Multi-Not Equal
   - `@`: Mutable Reference
   - `?`: Try: Early return on error for outermost scope or named inner scope.
 - `$`: Auto-Derive Conversion Implementations
 - `$$`: Template Syntax (TBD, possibly not needed)
 - `(`|`)`: Tuple/Arguments Open / Close
 - `[`|`]`: List/Map/Set/Generics Open / Close
 - `{`|`}`: Scope/Order Open / Close
 - `.`:
   - `.`: Accessor
   - `..`: UNUSED (Purposely)
   - `...`: Variable arguments
 - `\`: Equivalent to Rust's `r#`
   - `\"`: 2nd: Equivalent to Rust's `r#"` (Until `"\`)
   - `\\"`: 3rd: Equivalent to Rust's `r##"` (Until `"\\`)
   - `\\\"`: 4th: Equivalent to Rust's `r###"` (Until `"\\\`)
 - ` `: Token Separator
 - `,`|`\n`: List Separator

## Types
The following are valid **type**s:
 - `TypeName`: Uses UpperCamelCase
 - `Type[Generic]`: Generics On Types
 - `(TypeA, TypeB)`: Tuple of Type
 - `(Type) = Type`: 1-Tuple Type is equivalent to the type itself
 - `List[Type; 1] = Type`: 1-List Type is equivalent to the type itself
 - `(Type; 1) = Type`: Tuples may use repeat syntax as well
 - `Int[0~255]`: Type with bounds based on variable generics
 - `Int[0~255 %= 2]`: Type bounded with only even numbers from 0 up to 254
 - `Fn(Type) = Func[(), (Type)]`: Function
 - `Fn(Type) -> Return = Func[(Return), (Type)]`: Function with return type

## Literals
 - `(1, "abc")`: Tuple
 - `[1, 2, 3]`: List
 - `Int[0~255](34)`: Literal Unsigned Integer Byte
 - `34`: Literal Inferred Type Integer
 - `"text"`: Literal Text
 - `List[Int[0~255]; 3][1, 2, 3]`: Literal Unsigned Integer Byte List
 - `(4) = 4`: 1-Tuple Equivalence
 - `[4] = 4`: 1-List Equivalence
 - `{4} = 4`: Literal In It's Own Scope

## Order of Operations
There are 8 operator precedence levels, plus "parenthesis"
 0. `()`, `[]`, `{}`
 1. `-` (negation), `+-`, `~~`, `!`, `@`
 2. `<<(;)`, `>>(;)`
 3. `&&(;)`, `||(;)`, `^^(;)`
 4. `^(;)`
 5. `*(;)`, `**(;)`, `/(;)`, `//(;)`, `%(;)`, `%%(;)`
 6. `+(;)`, `++(;)`, `-(;)` (subtraction)
 7. `<=(;)`, `=(;)`, `>=(;)`, `!=(;)`, `<(;)`, `>(;)`
 8. `;`, `;;`

## Examples
```aratar
import sys

# Use functional replacement for `and` and `or` statements.
if all(
    any(a = 2, a != 4)
    any(b = 5, b = 7)
) {
    dbg("a = ", a, " and b = ", b)
}
if all(
   any(a = 2, b = 5)
   any(a != 4, b = 7)
) {
    dbg("a = ", a, " and b = ", b)
}

# Declare a function that adds a list of numbers together.
def add(numbers) -> _ {
    let @ret: Num.ZERO
    for num: numbers {
        ret +: num
    }
    ret
}

let b: 42
let @var: SOME(b)

# Maybe change `var` #

match var [
    SOME(b) {
        dbg("`var` was not changed")
    }
    SOME(a: -1 ~ 1) {
        dbg("`var` changed inner value to be close to zero: ", a)
    }
    SOME(a: _) {
        dbg("`var` changed inner value to ", a)
    }
    NONE() {
        dbg("`var` changed to NONE")
    }
]
```

## Types
```aratar
$Text, $Try
enum Opt[T](
    NONE: 0
    SOME[@T]: 1
)

$Text
struct Complex(
    real: @Real
    imag: @Imaginary
)

$Text
struct Range(
    start: Int
    end: Int
)

# List of built-in types
Func
Bool
Text
Opt[T]
Int[Domain: ~]
Float[Domain: ~]
Double[Domain: ~]
Data
Imaginary[Domain: ~]
Real[Domain: ~]
Complex[Domain: ~]
Dec[Domain: ~]

# The Err Type.
struct Err(
    Text: fn() -> Text
)

# The Try Type
$Text?
enum Try[T](
    ERR(.Err): 0
    OK(T): 1
)
```

# Error Handling
```aratar
# Example Error Type
struct Error()

# Add a method to `Error`.
fn Error.test(self) -> () {
    dbg("Hello, world!")
}

# Implement conversion to `Text`.
fn Error.Text(self) -> Text {
    "Example Error Message"
}

# Implement conversion to `Err`, allowing it to be used in the `Try[T]` type.
fn Error.Err(self) -> Err {
    # Err constructor takes a function.
    Err(fn() { self.Text() })
}

# Returns OK if should_error is FALSE, ERR if should_error is TRUE
fn maybe_error(should_error Bool) -> Try[()] {
    if should_error {
        ERR(Error())
    } else {
        OK(())
    }
}

# Entry point for a failable program
def fn start(_sys) -> Try[()] {
    # Should early return `Try.ERR`.  Note that this can be simplified to just
    # calling `maybe_error(TRUE)`.
    OK(maybe_error(TRUE)?)
}
```

# Numbers
```
# Fails to compile
let a Int[1~3|5~7]: 4
# Succeeds compile
let a Int[1~3|5~7]: 5
```

# Ideas

## Asynchronous Code
**async.rtr**
```aratar
export start

# Program to wait one second then print out "1"
def start() {
    let thread: Thread(thread)
    let result: thread.yield()
    Term.print("Result = ", result)
}

def thread() -> Int {
    let seconds: 1
    Duration.Seconds(seconds).yield()
    seconds
}
```

Asynchronous code is built on `Future`:
```aratar
struct Future[T](
    # Cooperatively gives up CPU until the `Future` is evaluated.
    def yield() -> T
)
```

Implementing `Future`:
```aratar
# Import C API from Aratar runtime
import aasi_timer(
    secs: Int64[0~],
    nanos: Int32[0~999_999_999]
) -> MemAddr

struct Task

def Task.Future: Future(
    yield: Fn() {
        # Set up callback
        let addr: aratar_sys_timer(1, 0)
        # `export` keyword takes a pattern; when matched, stops waiting
        api aasi_expire(addr)
    }
)
```

## Asynchronous Code With Adhoc Enums
```aratar
export start

def start() {
    # Use "Select" operator (`||`) to run both futures at the same time.  This
    # builds an Adhoc enum with two variants with selector and result.
    match Select(get_float(), get_int()).yield() [
        Float(sel: _, a: _) {
            Term.print("Got float: ", a, " - Then int: ", sel.yield())
        }
        Int(sel: _, a: _) {
            Term.print("Got int: ", a, " - Then float: ", sel.yield())
        }
    ]
    # Alternatively, you may write want to structure your program like this:
    for result: Select(get_float(), get_int()) {
        match result [
            Float(a: _) {
                Term.print("Got float: ", a)
            }
            Int(a: _) {
                Term.print("Got int: ", a)
            }
        ]
    }
    # Get the results of the two `Future`s (takes 1000 millis, instead of 1500
    # because they're running at the same time)
    let _: [get_float(), get_int()].yield() # Returns [Real; 2]
    let _: (get_float(), get_int()).yield() # Returns (Float, Int)
    let _: {get_float(), get_int()}.yield() # Returns Int
}

def get_float() -> Float {
    Duration.from_millis(1000).yield()
    4.0
}

def get_int() -> Int {
    Duration.from_millis(500).yield()
    55
}
```

## Variadic Arguments
Variadic arguments must always be the last arguments.

```aratar
export Term[print]

struct Term()

# Print something that can be turned into text to the terminal.
def Term.print(...: .Text) {
    for text_from: ... {
        for byte: text_from {
            Term.output.write(text_from.Text().List{Byte})
        }
    }
    Term.output.write(Text.Newline().List{Byte})
}
```
