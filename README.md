# Nahar
A simple programming language inspired by Rust and Python.  The goal is to have
the features of Rust with a Python-like syntax and no keywords.

## Types
`Si7` is rounded up to `Si8`, and the additional bit may be filled by `Opt`.

* `Si8`: Signed int 8-bit.
* `Si16`: Signed int 16-bit.
* `Si32`: Signed int 32-bit.
* `Si64`: Signed int 64-bit.
* `Ui8`: Unsigned int 8-bit.
* `Ui16`: Unsigned int 16-bit.
* `Ui32`: Unsigned int 32-bit.
* `Ui64`: Unsigned int 64-bit.
* `Hx8`: Hexadecimal 8-bit.
* `Hx16`: Hexadecimal 16-bit.
* `Hx32`: Hexadecimal 32-bit.
* `Hx64`: Hexadecimal 64-bit.
* `Bi8`: Binary 8-bit.
* `Bi16`: Binary 16-bit.
* `Bi32`: Binary 32-bit.
* `Bi64`: Binary 64-bit.
* `Bool`: Boolean 1-bit (rounded up to 8).
* `Date`: Month, Day, Year
* `Time`: Hours, Minutes, Seconds
* `Month`: Ui4 (1-12)
* `Day`: Ui5 (1-31)
* `Year`: Si31
* `Hours`: Ui5 Hours 0-23
* `Minutes`: Ui6 Minutes 0-59
* `Seconds`: Ui6 Seconds 0-59
* `Nanos`: Ui20 Nanoseconds 0-999,999
* `Text`: Text (A Vector of Characters)
* `Char`: Unicode Character (32 bits)
* `List`: List (32 bit len, 32 bit capacity, 64 bit pointer)
* `Fp32`: Floating-point 32-bit
* `Fp64`: Floating-point 64-bit
* `Vec2<T>`: Type with 2 values of the same type: `(X,Y,Z,W), (R,G,B,A)`
* `Vec3<T>`: Type with 3 values of the same type: `(X,Y,Z,W), (R,G,B,A)`
* `Vec4<T>`: Type with 3 values of the same type: `(X,Y,Z,W), (R,G,B,A)`
* `Mat2<T>`: 2x2 Matrix
* `Mat3<T>`: 3x3 Matrix
* `Mat4<T>`: 4x4 Matrix
* `Bits`: Bit Vector
* `Num`: An unlimited size number.
* `Opt`: Optional Type (`Opt<T>, (Some(T) None)`)

## Keywords & Syntax
Most programming languages have keywords, and sometimes you wish to name a
variable or function one of those keywords.  Not anymore!  There are a few
constants, though: `TRUE, FALSE, NAN, SOME(_), NONE`, but they're only in scope
when the type is requested.

### Struct
```
Struct
	field_a Si32
```

### :
* `mut_var: 4 // declare or set a mutable variable`
* `CONST_VAR: 5 // declare a constant`

### =
* `false: 1 = 2 // compare equality`
```
// Define an Enum
Enum
	VARIANT_A = 1
	VARIANT_B Ui8 = 2; 
```

### < and >
`false = 2 < 2 // check if a number is less than another`
`true = 3 > 2 // check if a number is more than another`
`true = 2 <= 2 // check if a number is less than or equal another`
`false = 1 >= 2 // check if a number is more than or equal another`
`VAR = 1 >> 2 // bitshift right`
`VAR = 1 << 2 // bitshift left`

### @ & .
```
// Define a function that takes a reference to an Si8 and prints it
function; ref @Si8
	cl.out .ref
```
`ref = @4.Si8 // Set a mutable variable to a reference to Si8 4`

### Operators
* `+ // Addition`
* `+: // Addition Assign`
* `- // Subtraction`
* `-: // Subtraction Assign`
* `* // Multiplication`
* `* // Multiplication Assign`
* `/ // Division`
* `/: // Division Assign`
* `^ // Power (Exponent)`
* `^: // Power (Exponent) Assign`
* `% // Modulo (Remainder)`
* `%: // Modulo (Remainder) Assign`
* `& // And & Bitwise And`
* `&: // And & Bitwise And Assign`
* `| // Or & Bitwise Or`
* `|: // Or & Bitwise Or Assign`
* `~ // Xor & Bitwise Xor`
* `~: // Xor & Bitwise Xor Assign`
* `! // Not`
* `var: ! // Not Assign`

### `$`
Functions and variables starting with `$` are imported from FFI or exported.
(Like Rust's `extern "C"` and `pub`)

### `#`
Types starting with `#` are copyable types, otherwise move semantics are used.

### `_`
Functions and variables starting with `_` are allowed to be unused, like Rust.

### Operator Overloading / Constructors & Free Function
Just define a function which the name of the function is the operator.
* `+ - / * ^ % & | ~ !`

Free function
* `@; self MyStruct`

Textify function
* `Text; self MyStruct`

Constructors for MyStruct
* `MyStruct; param Ui8, param2 Si32`

Constructors for OtherStruct (Rust's Into & From)
* `OtherStruct; self MyStruct`

### Generics
```
Struct T
	field T
```

```
Enum Ui32 T
	VARIANT T: 0.Ui32
```

### Matches
```
match_var = 2 // if match_var is 2
	term.out "Match Variable is 2"
_ 3 // else if match_var is 3
	term.out "Match Variable is 3"
_ match_var2 = 1 // else if match_var2 is 1
	term.out "Match Variable is not 2 or 3, Match Variable #2 is 1"
_ // else
	term.out "None were true"
```

### Loops
```
// While Loop
@ TRUE
// For Loop, i = 0 then i = 1, repeats twice.
@ {0 2} i
```

### Backtick
Like `"` but for multi-line strings (first and last newlines ignored)

### Single Quote
`'a' // define a Char`

### `?`
`function?`: if `Opt` is `None`, early return `None`.  Like Rust's `?`.

### `()`
Set order of operations, like any other programming language & math.

### `{1 2}` or `{Ui16 Ui32}`
A Tuple

### `{1 2}` or `{Ui16 * 2}`
An Array

### `MyStruct { field: 0 }`
A Struct Literal

### `[1 2]`
Inclusive Range.  Converts to Exclusive Range Tuple `{1 3}`

### `.0`
Index an array/tuple

## Scope and freeing
Most of the time a variable will be freed once it gets to it's last use in a
function.

### General Free Case
```
var: 1
// var is freed here
term.out "Hello, world!"
```

### Match Case
```
var: 1
var = 2
	term.out "Var is " + var
	// var freed here
_ 3
	// var freed here
	term.out "Var is not 2, but 3"
_
	// var freed here
	term.out "Var is not 2 or 3"
```

### Loop Case
```
var: 1
@TRUE
	var +: 1
	term.out var
	// var not freed here
	term.out "Hello, World"
// var freed here
```

### Reference Case
This is the most complicated free detection.  This is almost compile-time
reference counting.

```
var: 1
ref: @var // ref is now dependant on var
var +: 2 // Last use of var, so
// var would be free here
ref -: 1 // Except ref uses var, it's dependant
// so var and ref are freed here
```

This actually isn't that complicated, what is complicated is when `List` gets
involved.
```
var: 1
a: @var // a depends on var
b: @2
c: @3
list: List a b c // list depends on a, which depends on var
list.push list.0 // list now depends on a twice
list.pop
list.remove 0
// Now it should be freed, but Lists can have many things happen during
// runtime - it's almost impossible to calculate this point.
term.out list
// Now this is the last use of list, so var can be freed.
```

But, if we make references a non-copyable type, things are easier.
```
var: 1
a: @var // a depends on var
b: @2
c: @3
list: List a b c // a has now *moved* into list.
list.remove 0 // A is now moved out of the list, and the return value is not
// used, so a's free location is here.  Also var isn't used again, so it's freed
term.out list
```

Then we can avoid any type of runtime garbage collection for any situation.

### More reference case
What if we want to build a dependency tree of variables?  Let's say we want
sprite to depend on window.

```
Window
	some_field Ui32
	Window;
		Window {
			some_field: 0.Ui32
		}

Sprite
	window @Window // A struct with a reference inside is marked
	Sprite; window @Window
		Sprite { window }
	draw; how_many @Ui32
		window.some_field +: how_many

window: Window
sprite: Sprite @window // window is now borrowed by sprite, because sprite is
// marked for the first parameter.
sprite2: Sprite @window // now also by sprite 2
term.print sprite
how_many: 4
sprite.draw how_many // reference is taken by draw, but parameter is not marked
// so how_many is freed here
term.print sprite2
// all dependants on window have had their last use, so window is freed.
```

How about mark swapping?
```
Swappity
	value @Ui32
	Swappity; value @Ui32 // `value` is Marked
		Swappity { value }
	swap; value @Ui32 // `value` is Marked the same way
		value: value

a: @4
swap: Swappity a // Value in the struct
swap.swap @5 // Marked the same, so swap doesn't depend on a anymore, then replaced.
// a freed here
term.out swap
// End of code so everything else freed.
```
