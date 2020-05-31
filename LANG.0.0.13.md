# Nahar 0.0.13
Simple (like Python & Bash) & Safe (like Rust).

# Format
There shall be 80 columns (to make code easier to read for people with bad
eyesight - and on small screens), and tabs are 4 spaces because it's popular.
Functions and variables may not have more than 20 characters in their name, and
should avoid using acronyms and word fragments.  Up to 2 parameters can have
one letter names.

# Example
```nahar
#!nahar 0.0.13

# Lone expressions that evaluate & print 2
# Style #1: As a 1-element vector
3 - {2 - 1}
# Style #2: As a 1-element tuple
3 - (2 - 1)
```

# Keywords
## Keywords For Creating Variables
### `let`
The `let` keyword is used to define an immutable variable (constant).

```nahar
let my_variable: u32 0
let my_variable: 0
let my_variable: u32
```

### `var`
The `var` keyword is used to define a mutable variable.

```nahar
var my_variable: 0u32
var my_variable: 0
var my_variable: u32
```

### Functions
```nahar
let my_function: fn(
    # `my_function` docs.
    let a: u32?   # `a` parameter docs - this is an optional parameter (type).
    let b?: @text # `b` parameter docs - this is optional name, still required.
    out next: u32 # `next` output docs - this is like a return value.
) {
    "Hello, world {a.x}!"
    out(next: a + 1)
}

let my_function: fn
    # `my_function` docs.
    let a: u32?   # `a` parameter docs - this is an optional parameter (type).
    let b?: @text # `b` parameter docs - this is optional name, still required.
    out next: u32 # `next` output docs - this is like a return value.
{
    "Hello, world " par_a.hex "!"
    out(next: par_a + 1)
}

my_function(a: 0, "Nahar!")
my_function a: 0, "Nahar!"
```

# Syntax
```
() tuple - if it can be inferred the () are not needed.
{} vector - a fixed sized array of 1 type
[] list - a dynamically sized array 1 type.
"" text - a fixed-size array of characters.
'' character - a single character.
.. range `inclusive..exclusive`
:: range `inclusive::inclusive`
@  a reference (pointer)
.  path separator (get something from inside something else)
,  element separator (can also use '\n' - the newline character)
;  repeat (for vectors and lists)
:  assignment operator
+  addition operator
-  subtraction operator
*  multiplication operator
/  division operator
%  modulo operator
+: assignment addition operator
-: assignment subtraction operator
*: assignment multiplication operator
/: assignment division operator
%: assignment modulo operator
=  equivalence operator
!= not equivalence operator
<  less than operator
>  more than operator
<= less than or equal operator
>= more than or equal operator
<< shift left logical operator
>> shift right logical operator
!  bitwise not operator
&  bitwise and operator
|  bitwise or operator
^  bitwise xor operator
```

# Types
```
```

# Examples
```
var a: 0
a +: 2
assert_eq a, 2
```

# List of Keywords
```
let                  # An immutable value.
                     # "let" name:text ":" (is:type ^ value:is)?
var                  # A mutable value.
out                  # Output variable (return value & return statement).
tok                  # A token.
def                  # Define a type.
api                  # Export a C API.
ffi                  # Import a C API.
```

# List of Standard Functions
```
assert_eq            # Panic if not all of the parameters passed in are equal.
assert_ne            # Panic if any of the parameters are equal.
not                  # Boolean NOT.  1 paramater.
all                  # Boolean AND.  $ paramaters.
any                  # Boolean IOR.  $ paramaters.
one                  # Boolean XOR.  $ paramaters.
```

# List of Standard Types
```
u8                   # Short for u8.0 (fixed point)
u16
u32
u64
u128
s8
s16
s32
s64
s128
f32
f64
(u8, u8, u8)         # A tuple of 3 `u8`s
{u8; 100}            # A 100-element vector of `u8`s
[u8; 100]            # A 100-element list of `u8`s
```
