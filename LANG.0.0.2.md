# Operators

## Declare
```
@CONSTANT 3 # IMMUTABLE GLOBAL DATA: all functions in module can access.
$CONSTANT 3 # MUTABLE GLOBAL DATA: all functions in module can access and modify.
@var_a 3 # IMMUTABLE DATA: var_a will always be 3, and can't be changed.
$var_a 3 # VARIABLE DATA: var_a will initially be 3, and can be changed later.
# Immutable Function
# parameter 1: var_a is one byte signed integer.
# parameter 2: var_b is 4 byte unsigned integer, with default value of 12.
@fnc_a @var_a .Si1, @var_b .Ui4 12; .Ui4
	# Code for this Function (must be indented 1 tab).
```

## Assigns
```
var_a : 3      # "Move Assign": Move 3 to var_a: `set var_a to 3`
var_a :: 3     # "Copy Assign": Copy 3 to var_a: `set var_a to 3's copy`
var_a +: 3     # "Add Assign": Add 3 to var_a: `increment var_a by 3`
var_a -: 3     # "Sub Assign": Subtract 3 to var_a: `decrement var_a by 3`
var_a *: 3     # "Mul Assign": Multiply 3 to var_a: `set var_a to var_a times 3`
var_a /: 3     # "Div Assign": Divide 3 to var_a: `set var_a to var_a divided by 3`
var_a %: 3     # "Mod Assign": Modulo 3 to var_a: `set var_a to var_a mod 3`
var_a ^: 3     # "Exp Assign": Exponent 3 to var_a: `set var_a to var_a³`
var_a :!       # "Neg Assign": Negate var_a to var_a: `set var_a to it's negation` 
```

## Swap
```
var_a >< var_b # "Swap": Swap var_a and var_b
```

## Math
```
var_a + 3  # "Add": Calculate var_a plus 3.
var_a - 3  # "Sub": Calculate var_a minus 3.
var_a * 3  # "Mul": Calculate var_a times 3.
var_a / 3  # "Div": Calculate var_a divided by 3.
var_a // 3 # "Idv": Calculate var_a divided by 3 (Integer divide `1.5 // 1 = 1`).
var_a % 3  # "Mod": Calculate var_a modulo 3.
var_a ^ 3  # "Exp": Calculate var_a cubed.
```

## Brackets
```
var_a : (1 + 2) * 3   #     Parenthesis (): Do something first before other operations.
var_a : {1 2 "hi"}    #  Curly Brackets {}: Ordered pair
var_a : {Ui16 0; 8}   #                     or Array
var_a : {Ui16 0; 8};  #                     or List (.Lst).
var_a : [1 2]         # Square Brackets []: Inclusive range.
```

## Comparisons (and Boolean Operators)
```
var_a = var_b  # Calculate if var_a equals var_b (EQUALS).
var_a \ var_b  # Calculate if var_a does not equal var_b (XOR).
var_a < var_b  # Calculate if var_a is less than var_b.
var_a > var_b  # Calculate if var_a is more than var_b.
var_a >= var_b # Calculate if var_a is more than or equal to var_b.
var_a <= var_b # Calculate if var_a is less than or equal to var_b.
var_a |= var_b # Calculate if var_a divides var_b.

var_a = var_b, var_c  # Calculate if var_a equals var_b and var_c.
var_a \ var_b, var_c  # Calculate if none of var_a, var_b and var_c are equal.
var_a < var_b, var_c  # Calculate if var_a is less than var_b and var_c.
var_a > var_b, var_c  # Calculate if var_a is more than var_b and var_c.
var_a <= var_b, var_c  # Calculate if var_a is less than or equal to var_b and var_c.
var_a >= var_b, var_c  # Calculate if var_a is more than or equal to var_b and var_c.
var_a |= var_b, var_c # Calculate if var_a divides var_b and var_c.
```

## Bitwise Operators
```
var_a & var_b # var_a AND var_b
var_a | var_b # var_a OR var_b
var_a ` var_b # var_a XOR var_b
var_a !       # var_a NEGATE
```

## Option Unwrapping
```
# Returns N, if par_a is N.  Otherwise prints inner value and returns Y.
@fnc_a par_a .Opt .Ui4; .Opt
    .print "par_a is ", par_a?, "."
    Y
# Abort program if par_a is N.  Otherwise prints inner value.
@fnc_a par_a .Opt .Ui4;
    .print "par_a is ", par_a?, "."
```

## Control Flow: Match and Iterate
```
var_a = var_b # if var_a = var_b
        # Code that runs if var_a equals var_b.  Must be indented 1 tab.
= var_c # else if var_a = var_c
        # Code that runs if var_a does not equal var_b, but equals var_c.  Must be indented 1 tab.
_ # else
        # Code that runs if var_a does not equal var_b or var_c.  Must be indented 1 tab.
        
var_a = var_b # if var_a = var_b
        # Code that runs if var_a equals var_b.  Must be indented 1 tab.
_ var_b = var_c # else if var_b = var_c
        # Code that runs if var_a equals var_b is false, but var_b equals var_c.  Must be indented 1 tab.

~loop; # loop (Iterator over the unit struct)
	# Code that runs in an infinite loop.  `loop` is a counter starting at zero.
	loop;         # break
	loop; .Opt.Y  # break loop (loop returns .Opt.Y)
	loop~         # continue

~for [0, 4]; # for (Iterator over a range)
	# Code that runs 5 times.  First time `for` equals 0, last time `for` equals 4.
	for; # break
	for~ # continue

~repeat 5; # repeat (Iterator over a counter)
	# Code that runs 5 times.  First time `repeat` equals 0, last time `repeat` equals 4.
	repeat; # break
	repeat~ # continue

~5; # nameless repeat
~; # nameless infinite loop
~[0, 4]; # nameless for
~i iter; # for (pass iterator type - implements function `~; .Opt _`, like Rust's `next()`)
```

### Index an array/tuple
```
array.0 # index at 0
```

### `_`
Functions and variables starting with `_` are allowed to be unused, like Rust.

## Reference Types
All references are immutable (but not necessarily the data they point to).
```
@Type # Type Definition: Reference to Immutable data
$Type # Type Definition: Reference to Mutable data
```

## Export Labels
You can export immutable labels (function or constants).
```
@CONSTANT 3 # IMMUTABLE GLOBAL DATA: all functions in module can access.
%CONSTANT 3 # IMMUTABLE GLOBAL DATA: all functions in code can access.
%function_a var_a $.Si4;
	# Function code
!ffi_import: var_a $.Si4; .Si4 # Import C API.
!"SDL2" # Statically link to C library.
!!"SDL2" # Dynamically link to C library.
```

## Strings and characters.
```
'c' # A unicode character (4 bytes).
"This is a string"
# First and last newlines are ignored in a multiline string.
"
This is a 
multi-line string
"
```

## Structs
Structs are defined by at least two characters in CamelCase, followed by an indented list of snake_case fields.  Fields in structs can be $(mutable), @(immutable), a reference, or %(public
immutable).  Structs can only be copied with `::`, otherwise they always use move semantics.
```
@StructA T
	$field_a Si32
	$field_b T
```

## Enums
Enums are defined by at least two characters in CamelCase, followed by an indented list of CamelCase variants.  When the enum is visible, so are all of the variants.

```
@EnumA T
	VariantA Ui32 1
	VariantB Ui32 2, T
	VariantC # Compiler will give lowest available value (Ui32 0)
	VariantD 3 # Type for enum can be infered.  If no type is set it will pick the smallest unsigned type for the number of variants.
```

## Implementing Functions On Structs and Enums
```
@TypeA @var_a .Si4; _
	# Code for this Constructor
	TypeA
		# Fields
@TypeA.fnc_a @var_a .Si1, @var_b .Ui4 12;
        # Code for this Function (must be indented 1 tab).
```

## Setting Functions to other Functions
```
@fnc_a @var_a .Si1; .Ui4
        # Code for this Function (must be indented 1 tab).
$fnc_b fnc_a # Same as:
$fnc_b @var_a .Si1; .Ui4 fnc_a
fnc_b: fnc_a # Can be assigned like other variables.
```

## Comments
```
# This is a comment.
```

# Rules
## Scope and freeing
A variable will be freed at it's last use.

# Standard Library
The standard library is the module with no name.  You can call functions from the standard library
the same way you call from any other library or module: `.print "Həllø, Wôrld‽"`.
```
// Set a variable with function `:`
x.: 5
x : 5
// Add something (variable or literal) to a variable with function `+`
y.: x.+ 5
y : x + 5
// Assign to self with function `+:`
x.+: 5
x +: 5
// Copy a value into a variable with function `::`
x.:: 5
x :: 5
```

## Standard Library Types
```
Uf1 / Sf1 / Nf1 - Unsigned/Signed/Normalized(0-1) 1 byte (4.4) Fixed Point
Uf2 / Sf2 / Nf2 - Unsigned/Signed/Normalized(0-1) 2 byte (8.8) Fixed Point
Uf4 / Sf4 / Nf4 - Unsigned/Signed/Normalized(0-1) 4 byte (16.16) Fixed Point
Uf8 / Sf8 / Nf8 - Unsigned/Signed/Normalized(0-1) 8 byte (32.32) Fixed Point
Uf_ / Sf_ / Nf_ - Unsigned/Signed/Normalized(0-1) Big Fixed Point
Ui1 / Si1 - Unsigned/Signed 1 byte Integer
Ui2 / Si2 - Unsigned/Signed 2 byte Integer
Ui4 / Si4 - Unsigned/Signed 4 byte Integer
Ui8 / Si8 - Unsigned/Signed 8 byte Integer
Ui_ / Si_ - Unsigned/Signed Big Int
Hx1 - Hexadecimal 1 byte.
Hx2 - Hexadecimal 2 byte.
Hx4 - Hexadecimal 4 byte.
Hx8 - Hexadecimal 8 byte.
Hx_ - Hexadecimal String
Bi1 - Binary 1 byte.
Bi2 - Binary 2 byte.
Bi4 - Binary 4 byte.
Bi8 - Binary 8 byte.
Bi_ - Binary String.
Chr - Character (4 bytes unicode).
Str - Character String (Wrapper around Lst<Chr> [characters are always 4 bytes, use for inserts]).
Txt - Text (Sequence of unicode characters [characters are from 1-4 bytes, use for appends]).
Lst T - List of T (Dynamic Size).
Opt T - Optional (Y T {yes} or N {no})
      $ x: Opt.Y # Create an Optional type:1 bit
      $ x: Opt.Y 45 # Create an Optional Si4 type (value -2147483648 is used to store N):4 byte
      $ x: Y # Create an Optional type:1 bit
      $ x: N # Create an Optional type:1 bit
Cxy - 8 bytes Coordinates X and Y: (Sf4, Sf4).
Vec - 16 bytes A Vector or Position: (Sf4, Sf4, Sf4, Sf4).
Mat - 64 bytes 4 Dimensional Matrix: [Sf4; 16]
```
