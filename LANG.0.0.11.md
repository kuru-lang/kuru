# Nahar 0.0.11
Safe (like Rust), simple (like Python, Bash), containerized and flexible.

# Starting
Nahar can be both used as a script or compiled with a C API.  Extra spaces and tabs are ignored.
Convention is to use snake\_case for everything.  You can shadow fields.

```nahar
#!nahar 0.0.11
'''
# About This File
This is a simple example.
'''

# Calculate 3 - (2 - 1) 
3 - {2 - 1}

"Single line text block"

"""
This is multi-line text block.
Hello, world!
"""
```

Using variables
```
# Define a constant (by making it an immutable reference, it's now an inlined constant)
int @my_constant: 5

# Create a field
int [0 255] v: my_constant + 1

loop a {
    v
    try v +: 1 {
        a exit
    }
}
```

Control Structures
```
if a = 2 {
    "a = 2"
}
elif b = 2 {
    "b = 2"
}
else {
    "neither a or b is 2"
}

int[0 2] a: prompt

match a [
    0: "a = 0"
    1: {
        "a = 1"
    }
    _: "a = 2"
]
```

# Syntax
The interpreter searches for the following patterns (you can define your own patterns):
```
.       — Path separator
'''     — Multi-line code block open/close (default markdown documentation).
#…#     — Comment (Goes to newline, unless # seen first, then comment ends there).
#!…     — Attribute.
'c'     — Single Unicode character (4 bytes).
"Text"  — Text.
"""     — Multi-line text block.
{}      — Array (Scope is evaluated, and put in other scopes, like mathematical parentheses).
()      — Parentheses (ordered list of types), Exclusive for range
[]      — Select (ordered list of values), Inclusive for range
>>      — Shift Right
<<      — Shift Left
!       — This is first.
*,/,%   — This is second.
+,-     — This is third.
&,|,^   — This is fourth.
=,!=,<,
>,<=,>= — This is last.
->      — Returns (in function definitions), also "pipe"
?       — Return failure if failed, otherwise continue.
@       — Immutable reference
$       — Pseudo-regisister (hex) `$2A0` or self pseudo-register `$self`
;       — Repeat
,       — Function separator (usually newline is good enough, but just in case)
_       — Infer: force the compiler to guess.
0x…     — Hexadecimal Values
0b…     — Binary Values
0-9     — Decimal Values
opt<…>  — Option Type
int<…>  — Integer, int<255> = int<0:255> = int<0~256> = uint8_t
dec<…>  — Fixed Point Decimal, dec<0x0.0:0xF.F> = 1 byte
sci<…>  — Floating Point Scientific Numbers, sci<8,24> = 32-bit floating point 8-bit exponent,
            23-bit mantissa, 1-bit sign.
raw<…>  — Raw bits, raw<32> = uint32
fn      — Define a funtion.
ffi     — Define a C FFI function
if      — If statement
else    — Else statement
elif    — Else if statement
try     — Operations that may fail must use a try block, or use ? in failable function.
loop    — An infinite loop (note: infinite loops not actually allowed, must be proved to exit).
match   — Pattern matching
type    — Define type (struct) [tagged union] {array}
```

### Rules
```
function        # snake_case for functions, types and variable names.
@type           # @snake_case for references.
_unused         # _snake_case for unused functions and variable names.

{u32; 128}      # Fixed size array.
{1 2 3}         # {} for lists `var list: [3 * s32](1 2 3)`.
(1 'a')         # () for tuples (parentheses).
[0 255]         # [] for ranges and indexing

$0              # Virtual register (infinite number, numbered by hexadecimal - any type).
```

## Function Details
Functions can left and right operands and a return value.

```c
# c prototype
int32_t int32_function(int32_t* rtn_b, int32_t* rtn_c, const int32_t* input_a, int32_t intput_b);
```

```nr
# ffi binding for the c prototype (l_operand mutable is always first, r_operand immutable last).
ffi @int32 function: @mut int32 rtn_c, @int32 input_a, int32 input_b -> int32
```

## Standard Library
```
def opt<t> [
    y(t)
    n
]
```

## Assembly


```
# Tagged union of assembly instructions.
type asm [
    # Add each component of two vectors together (`dst0: src0 + src1`).
    add "$" dst0 "$" src0 "$" src1
    # Add all components of vector together (`dst0: src0 + src1`)
    addv "$" dst0 "$" src0
    # Get the max of each component in two vectors (`dst0: max src0, src1`)
    max "$" dst0 "$" src0 "$" src1
    # Get the maximum of all components of a vector (`dst0: max src0[..]`).
    maxv "$" dst0 "$" src0
    # Inclusive or each component of two vectors together (`dst0: src0 + src1`)
    ior "$" dst0 "$" src0 "$" src1
    # Inclusive or all components of a vector (`dst0: max src0[..]`).
    iorv "$" dst0 "$" src0
    # Exclusive or each component of two vectors together (`dst0: src0 + src1`)
    xor "$" dst0 "$" src0 "$" src1
    # Exclusive or all components of a vector (`dst0: max src0[..]`).
    xorv "$" dst0 "$" src0
]

// Call the assembly function to put inline assembly
int[_] $0: 2
int[_] $1: 3
asm {
    add $0 $0 $1
}
assert $0 = 5
```

## Labels & Functions
```
my_function                         # call a function.
add 2 3                             # print return value of add.

fn my_function: {                   # define a function.
    "Hello, world!"                 # print "Hello, world!".
}

fn my_function: -> text {           # function that returns text.
    "Hello, world!"                 # last operation's return value must match function's
}                                   # return value.
fn add: s32 a, s32 b -> s32 {       # function to add 2 numbers together.
    a sat+ b
}
fn add: s32 a b -> i32 {            # same function using same type for both parameters.
    a sat+ b
}

// Define the `=` operator on numbers.  `loperand.function_name(roperands...) -> return { code }`
fn num =: @num v -> ? {
    _ $0: v               // Move reference into a pseudo-register.
    _ $1: ?               // Uninitialized option, so can still be set.

    asm [
        sbe $1 $self $0     // Run Set on Bits Equal
    ]

    $1                      // Return the register from `sbe`.
}
```

## Arithmetic Modes
```
// Integer inclusive from 0 to 1.
int[0 1] a: 1
// Attempt to increment 1 (won't compile).
a +: 1
// Instead do this:
int[1 2] b: (a + 1)
```

## Loops
```
loop a {
    a next  // continue the loop
    a exit  // break from the loop
}
```
