# Nahar 0.0.10
This programming language provides a platform-independant interface inside of a container.  Safety
is also ensured, like Rust.  There are no global variables or functions.

## About
Everything is snake_case, and in scope.  There is scope shadowing (and item inference).

## Items
* Literal `0, "Hi", 'c'`
* Tuple `()`
* Array `[]`
* Scope `{}`
* Type `?`

## Syntax
Like most programming languages, you can use +-*/%&|^ with left\_operand and right\_operand.
The order of operations (all else is sequential):
```
!               // This is first.
*,/,%           // This is second.
+,-             // This is third.
&,|,^           // This is fourth.
=,!=,<,>,<=,>=  // This is last.
```

### Ascii
```
!   // assert !.opt.y = n
%   // "4 mod 3 = " + {4 % 3}
^   // "4 xor 3 = " + {4 ^ 3}
&   // "4 and 3 = " + {4 & 3}
|   // "4 ior 3 = " + {4 | 3}
-   // "4 sub 3 = " + {4 - 3}
+   // "4 add 3 = " + {4 + 3}
*   // "4 mul 3 = " + {4 * 3}
/   // "4 div 3 = " + {4 / 3}
//  // comment (ignore).
/*  // open comment.
*/  // close comment.
=   // is equal
!=  // is not equal
<   // is less than
<=  // is less than or is equal
>   // is more than
>=  // is less than or is equal
@   // reference
`   // define label for a loop.
#   // continue loop.
\   // break loop.
$   // pseudo-regisister (hex) `$2A0` or self pseudo-register `$self`
()  // tuple
[]  // array
{}  // scope
~   // Exclusive range 0~10
:   // Inclusive range 0:9, Assignment
;   // Separate functions.  Terminate expression.
,   // Repeat syntax.
"   // Single line string
'   // Character.
''  // Multi-line string (more ' too).
.   // Path separator.
?   // Option
_   // Infer: force the compiler to guess.

```

### Rules
```
function        // snake_case for functions, types and variable names.
@type           // @snake_case for references.
_unused         // _snake_case for unused functions and variable names.
`label          // #snake_case for labels.

[u32, 128]      // Fixed size array.
[1 2 3]         // [] for lists `var list: [3 * .i32](1 2 3)`.
(1 'a')         // () for tuples (parentheses).
{1 + 1}         // {} for scopes.

$0              // Virtual register (infinite number, numbered by hexadecimal - any type).
"Text"          // String (\" for ", \ before newline to ignore).
''              // Raw Multi-line string (Can have as many (2+) ' as want).
Say "Hi"
''
```

## Keywords
These actually generate the assembly code.  They cannot be created, so these are the only ones.
Future versions of the language may add these, or remove them.

```
type        // Define a Type.
let         // Declare an immutable data type.
var         // Declare a mutable data type.
```

## Built-In Types

### Traits
```
num         // number trait.
text        // 
```

```
// boolean / option
?                           // ?=boolean, u32?=optional integer
// unsigned integers
u8
u16
u32
u64
u128
u256
// signed integers
s8
s16
s32
s64
s128
s256
// inclusive ranged integers
int 0~128                  // 0 through 128 exclusive (127 inclusive).
int 0:127                  // 0 through 127 inclusive (128 exclusive - Same).
// just the raw Bits
r8
r16
r32
r64
r128
r256
```

## Function Details
Functions can left and right operands and a return value.

```c
// c prototype
int function(int* rtn_b, int* rtn_c, const int* input_a, int intput_b);
```

```nr
// ffi binding for the c prototype (l_operand mutable is always first, r_operand immutable last).
s32 (rtn_a @s32, rtn_b @s32).function(input_a @s32, input_b s32);
```

## Standard Library
```
type opt(t).(
    y(t)
    n
)
```

## Assembly


```
// Tagged union of assembly instructions.
type .asm
    x: .int 0~
[
    // Add each component of two vectors together (`dst0: src0 + src1`).
    add "$" dst0 "$" src0 "$" src1
    // Add all components of vector together (`dst0: src0 + src1`)
    addv "$" dst0 "$" src0
    // Get the max of each component in two vectors (`dst0: max src0, src1`)
    max "$" dst0 "$" src0 "$" src1
    // Get the maximum of all components of a vector (`dst0: max src0[..]`).
    maxv "$" dst0 "$" src0
    // Inclusive or each component of two vectors together (`dst0: src0 + src1`)
    ior "$" dst0 "$" src0 "$" src1
    // Inclusive or all components of a vector (`dst0: max src0[..]`).
    iorv "$" dst0 "$" src0
    // Exclusive or each component of two vectors together (`dst0: src0 + src1`)
    xor "$" dst0 "$" src0 "$" src1
    // Exclusive or all components of a vector (`dst0: max src0[..]`).
    xorv "$" dst0 "$" src0
]

// Call the assembly function to put inline assembly
let $0: 2
let $1: 3
asm [
    add $0 $0 $1
]
assert $0 = 5
```

## Labels & Functions
```
my_function                         // call a function.
add 2 3                             // print return value of add.

() my_function() {
}

() my_function() {                  // define a function.
    "Hello, world!"                 // print "Hello, world!".
}
text my_function() {                // function that returns text.
    "Hello, world!"                 // last operation's return value must match function's
}                                   // return value.
i32 add(i32 a i32 b) {              // function to add 2 numbers together.
    a + b
}
i32 add(i32 a,b) {                  // same function using repeat syntax.
    a + b
}

// Define the `=` operator on numbers.  `loperand.function_name(roperands...) -> return { code }`
? num.=(@num v) {
    let $0: v               // Move reference into a pseudo-register.
    let $1: ?               // Uninitialized option, so can still be set.

    asm [
        sbe $1 $self $0     // Run Set on Bits Equal
    ]

    $1                      // Return the register from `sbe`.
}
```

## Arithmetic Modes
```
// Integer inclusive from 0 to 1.
int<0:1> a: 1
// Attempt to increment 1 (won't compile).
a +: 1
// Instead do this:
int<1:2> b: {a + 1}
```

## Loops
```
#a loop {
    a.continue  // continue the loop
    a.break     // break from the loop
}
```
