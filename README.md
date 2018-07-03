# Nahar
A simple programming language inspired by Rust and Python.

## Modules & Objects
Module code runs at the start of the program by default.  A module can become an
object with the `obj` keyword, which works like `mod`.
There can be any number of instances of an object, but only one of a module.

## Types
`Si7` is rounded up to `Si8`, and the additional byte may be filled by `Opt`.

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
* `Vec`: Vector (32 bit len, 32 bit capacity, 64 bit pointer)
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
* `Range<T>`: A range `(min, max)`

## Keywords
Most programming languages have keywords, and sometimes you wish to name your
variable one of those keywords.  Not anymore!

Only ten keywords `mod, obj, dep, fun, var, con, mac, ffi, fvi, set`:
* `mod symbol`: Export symbols from a module
* `obj symbol`: Export symbols from an object
* `dep dependency`: List dependencies
* `fun my_function`: Declare a function
* `var my_variable`: Declare a variable
* `con MY_CONSTANT`: Declare a constant
* `mac my_macro`: Declare a macro (function evaluated at compile time).
* `ffi c_function`: Prototype from C/Windows API
* `fvi c_global`: Global variable form C/Windows API
* `set free free_function`: Set a built-in function.

There's only 5 multi-character syntax: `## @~ >= <= !=`
Syntax `~ @ : & | ^ ! = > < ( ) { } [] `` " ' % + - * / . ? $`.
* `# My Comment`: Line comment.
* `##`: Multi-line comment.
* `~Type`: Mutable variant of the type, immutable by default.
* `@Type`: Reference variant of the type, move semantics by default.
* `@~Type`: Mutable reference to the type.
* `varname: 0`: Assign a variable.
* `true & true`: And
* `true | false`: Inclusive Or
* `true ^ true`: Exclusive Or
* `3 ^ 2`: Exponents (3² = 9)
* `!false`: Not
* `true = true`: Is Equal?
* `2 > 2`: More Than
* `2 < 2`: Less Than
* `2 >= 2`: More Than or Equal to
* `2 <= 2`: Less Than or Equal to
* `2 != 2`: Not equal
* `fnc my_function(...)`: Circle Brackets - define function
* `my_function(...)`: Circle Brackets - call function
* `{1 + 2}`: Fancy Brackets - Set order of operations
* `[Type]`: Square Brackets - Array Types.
* `[1, 2, 3]`: Square Brackets - Define arrays.
* ``string``: Backtick - Multi-line string (first and last newlines ignored).
* `"string"`: Double Quotes - Define String
* `'c'`: Single Quotes - Define Char
* `3 % 2`: Modulo - Remainder
* `3 + 2`: Addition
* `3 - 2`: Subtraction
* `3 * 2`: Multiplication
* `3 / 2`: Division
* `module_name.fnname`: Access a function from a module.
* `function()?`: if `Opt` is `None`, early return `None`.
* `var a $ v1(Si8) v2: v1(4)`: Define state variants for a variable.

## Set
* `set free free_function` - Set a function to execute when variable's out of scope.
* `set + addition_function` - Operator overloading for + (must be same types)
* `set - subtraction_function` - Operator overloading for - (must be same types)
* `set * multiplication_function` - Operator overloading for * (must be same types)
* `set / division_function` - Operator overloading for / (must be same types)
* `set % modulo_function` - Operator overloading for % (must be same types)
* `set & and_function` - Operator overloading for & (must be same types)
* `set | or_function` - Operator overloading for | (must be same types)
* `set ^ xor_function` - Operator overloading for ^ (must be same types)
* `set ! not_function` - Operator overloading for ! (must be same types)
* `set String to_string` - Set a function to convert into a String (works with other types).
