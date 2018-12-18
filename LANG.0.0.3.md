# Keywords
All keywords are symbols, so functions, variables and constants can have no limit on what
they can be labeled.

## Declarations
```
@constant 3 # Immutable data 🔒.
$variable 3 # Mutable data 📝.
%exported 3 # All exported data is constant 📦.
!SDL2       # Dynamically link to a C library 🔗.
```

## Functions: Parameters and Reference Parameters
```
@my_function ReturnType; @param_name ParamType, $mutable_param ParamType
	# Function Code
	my_function return_value;
@function_takes_refs; param_name @ParamType, param_mutref $ParamType
	# Function Code
	function_takes_refs;
# Call a function exposed by a C API.
!ffi_function .Si4; param_name $.Si4
```

## Iterators
```
loop ~ # An infinite loop. 🔁
	loop; # Return from loop.
loop Type ~ # An infinite loop with return value.
	loop value; # Break with value of type `Type`
loop ~ [0, 1] # Iterate over a range (from 0 to 1 inclusive (2 times)).
	loop, # Continue
loop ~ 5 # Iterate 5 times.  Same as `loop ~ [0, 4]`.
	.print loop # prints 0 on the first iteration ... 4 on the fifth.
loop Type value ~ # An infinite loop with default return value.
	loop value, # Continue with updated return value.
	value: 5 # Update return value.
```

## If Statements
```
var = 1
	# Executes if var is 1.
_
	# Executes if var is not one.

var = Enum.VariantA # Boolean Expression
	# Executes if var is Enum.VariantA
= Enum.VariantB # Second half of boolean expression
	# Executes if var is Enum.VariantB
_ var_b = 3 # Else if
	# Executes if var is not Enum.VariantA or Enum.VariantB, but var_b is 3
>= 4 # Second half of boolean expression
	# Executes if var_b is more than or equal to 4.
_ var_c = 10 & var = Enum.VariantC # if var_c equals 10 and var equals Enum.VariantC.
	# Executes if boolean expression evaluates Y.
_
	# Else
```

# Compiler
Splits into tokens (comments are ignored):
* values (integers, tokens containing '.' (decimal#s, enum variants), 'c', "strings", or declared
labels)
* labels (snake_case, may or may not begin with @$%!)
* types (CamelCase, may or may not begin with @$%!)
Separators
* tab
* space
* semicolon
* comma
* tilde

How to compile:
* Declarations:
`label[must begin @$%!] (Type) value`
* Functions:
`label[must begin @$%!] (Type) semicolon {label[may begin @$%!] Type(may begin @$%!) comma*}`
* Return:
`label (value) semicolon`
* Function calls:
`label[declared] {value comma*}`
* Iterators:
`(label) (Type) tilde (value)`
* Continue
`label comma`


# Operators
All operators are actually functions on a type, and they may not always exist: `:` is the same as
`.:`.

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

### Index an array/tuple
```
array.0 # index at 0
```

### `_`
Functions and variables starting with `_` are allowed to be unused, like Rust.

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
@TypeA _; @var_a .Si4
	# Code for this Constructor
	TypeA
		# Fields
@TypeA.fnc_a; @var_a .Si1, @var_b .Ui4 12
	# Code for this Function (must be indented 1 tab).
```

## Setting Functions to other Functions
```
@fnc_a .Ui4; @var_a .Si1
	# Code for this Function (must be indented 1 tab).
$fnc_b fnc_a # Set a function pointer to a function.
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
Type prefixes:
* P - Percent
* N - Normalized
* F - Fixed Point
* D - Decimal
* U - Unsigned
* S - Signed
* H - Hexadecimal
* B - Binary
```
Num - Big Fixed Point
Big - Big Int
Hex - Hexadecimal String
Bin - Binary String
P8 / N8 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 8 bit
P16 / N16 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 16 bit
P32 / N32 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 32 bit
P64 / N64 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 64 bit
F8 / D8 - Unsigned/Signed Fixed Point (4.4 / 3.4) 8 bit
F16 / D16 - Unsigned/Signed Fixed Point (8.8 / 7.8) 16 bit
F32 / D32 - Unsigned/Signed Fixed Point (16.16 / 15.16) 32 bit
F64 / D64 - Unsigned/Signed Fixed Point (32.32 / 31.32) 64 bit
U8 / S8 - Unsigned/Signed 1 byte Integer
U16 / S16 - Unsigned/Signed 2 byte Integer
U32 / S32 - Unsigned/Signed 4 byte Integer
U64 / S64 - Unsigned/Signed 8 byte Integer
H8 - Hexadecimal 1 byte.
H16 - Hexadecimal 2 byte.
H32 - Hexadecimal 4 byte.
H64 - Hexadecimal 8 byte.
B8 - Binary 1 byte.
B16 - Binary 2 byte.
B32 - Binary 4 byte.
B64 - Binary 8 byte.
Chr - Character (4 bytes unicode).
Str - Character String (Wrapper around Lst<Chr> [characters are always 4 bytes, use for inserts]).
Txt - Text (Sequence of unicode characters [characters are from 1-4 bytes, use for appends]).
Lst T - List of T (Dynamic Size).
Opt T - Optional (Y T {yes} or N {no})
      $ x: Opt.Y # Create an Optional type:1 bit
      $ x: Opt.Y 45 # Create an Optional Si4 type (value -2147483648 is used to store N):4 byte
      $ x: Y # Create an Optional type:1 bit
      $ x: N # Create an Optional type:1 bit
Cxy - 8 bytes Coordinates X and Y: (D32, D32).
Vec - 16 bytes A Vector or Position: (D32, D32, D32, D32).
Mat - 64 bytes 4 Dimensional Matrix: [D32; 16]
```
