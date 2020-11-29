# Operators

## Arithmetic Operators
Arithmetic operators do pure mathematical operations (no wrapping, saturation,
or abort).  The reason they can do this is because all types have defined
domains that are checked at compile time.  There is an exception though; You may
use wrapping or saturating types.
 - `+` Numerical/Vector Addition
 - `-` Numerical/Vector Subtraction
 - `*` Numerical/Vector Multiplication
 - `/` Numerical/Vector Division
 - `%` Numerical/Vector Modulo (Not Remainder)
 - `^` Numerical/Vector Exponent
 - `++` Numerical Maximum
 - `--` Numerical Minimum
 - `**` Dot Product / Matrix Multiply
 - `//` Numerical/Vector Integer Division (Syntax from Python)
 - `%%` Numerical/Vector Remainder (Of A Division With `//`)
 - `^^` Binary XOR (one)
 - `&&` Binary AND (all)
 - `||` Binary IOR (any)
 - `>>` Binary Shift Right (Left Operand Binary, Right Operand IntU)
 - `<<` Binary Shift Left (Left Operand Binary, Right Operand IntU)

## Pattern Matching
These are used in type definitions, match statements and if statements.
 - `|` Refer to one of either value, XOR one of either type.
 - `&` Refer to require boolean expression to be true, XOR have a supertype.

## Assignment Operators
 - `:` Generic assignment
 - `::` Swap assignment
 - `+:` Add assignment (can use with all other artithmetic operators)
 
## Repitition Operator
 - `;` - `[1; 3] = [1, 1, 1,]`
 - `;;` - `[[1, 2] ;; 3] = [1, 2, 1, 2, 1, 2]`
 - `+;` - `[1, 2, 3] +; 1 = [2, 3, 4]` (may use other arithmetic or comparison)

## Comparison Operators
 - `<` Vector Less Than
 - `>` Vector More Than
 - `<>` Vector Not Equal To (Syntax from R)
 - `=` Vector Equal To
 - `<=` Vector Less Than Or Equal To
 - `>=` Vector More Than Or Equal To
 - `%=` Is divisible by

## Prefix Unary Operators
 - `-` Negation (same as subtraction)
 - `~` Binary NOT (Syntax from C)
 - `@` Exclusive Reference

## Postfix Unary Operators
 - `?` Try Operator (early return on nil value, syntax from Rust)
 - `!` Infallible Operator (fail to compile on nil value)

## Accessor Operator
 - `.` Access fields of a struct or type (same as Java)
