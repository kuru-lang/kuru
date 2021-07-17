# Operators
Binary arithmetic operators must operate on `Bin` types.  Arithmetic operators
on regular types may not wrap or saturate.

## Standalone Operators
 - `+` Numerical/Vector Addition
 - `-` Numerical/Vector Subtraction (or prefix negation operator)
 - `*` Numerical/Vector Multiplication
 - `**` Numerical/Vector Dot Product, Matrix Multiply
 - `/` Numerical/Vector Division
 - `//` Numerical/Vector Integer division (same as `/` for `Int`s)
 - `%` Numerical/Vector Modulo (Not Remainder)
 - `%%` Numerical/Vector Remainder
 - `^` Numerical/Vector Exponent
 - `>>` BITWISE Right Shift (Always fills in zeros and drops overflowed bits)
 - `<<` BITWISE Left Shift (Always fills in zeros and drops overflowed bits)
 - `!` PATTERN_MATCHING NOT, BITWISE NOT
 - `|` PATTERN_MATCHING IOR, BITWISE IOR
 - `&` PATTERN_MATCHING AND, BITWISE AND
 - `\` PATTERN_MATCHING XOR, BITWISE XOR
 - `...` PATTERN_MATCHING RANGE, TYPE RANGE
 - `,` Separator `('a', 4)`
 - `;` Repeat `[1; 2] = [1, 2]`
 - `;;` Multi-Repeat `[[1, 2] ;; 2] = [1, 2, 1, 2]`
 - `:` Assignment
 - `::` Swap
 - `@` Exclusive Reference (prefix)
 - `.` Accessor (postfix)
 - `?` Try (postfix return on nil)
 - `<` Vector Less Than
 - `>` Vector More Than
 - `!=` Vector Not Equal To
 - `=` Vector Equal To
 - `<=` Vector Less Than Or Equal To
 - `>=` Vector More Than Or Equal To
 - `%=` Is divisible by

## Assignment Arithmetic
 - `+:` Numerical/Vector Addition
 - `-:` Numerical/Vector Subtraction
 - `*:` Numerical/Vector Multiplication
 - `**:` Numerical/Vector Dot Product, Matrix Multiply
 - `/:` Numerical/Vector Division
 - `//:` Numerical/Vector Integer Division
 - `%:` Numerical/Vector Modulo (Not Remainder)
 - `%%:` Numerical/Vector Remainder
 - `^:` Numerical/Vector Exponent
 - `>>:` BITWISE Right Shift (Always fills in zeros and drops overflowed bits)
 - `<<:` BITWISE Left Shift (Always fills in zeros and drops overflowed bits)
 - `|:` BITWISE IOR
 - `&:` BITWISE AND
 - `\:` BITWISE XOR
