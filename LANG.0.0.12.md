# Nahar 0.0.12
Simple (like Python) & Safe (like Rust).

## Format
There shall be 90 columns, and tabs are 6 spaces.  Why?  Because a lot of people really
hate 4 because it's too small and a lot really hate 8 because it's too big.  So 6 seems to
be a good middle point (it also, in my opinion looks the best because it's the easiest to
read).  6 also goes into 90 15 times, which if you're indenting that much you're doing
something wrong (80 is not enough, but you certainly don't need 100).  Since it's a
non-standard indentation (not 8) we must use spaces instead of tabs, unfortunately.

## Example
```nahar
#!nahar 0.0.12

# Lone expression evaluates & prints (this prints 2)
3 - (2 - 1)

# 2 ways to write parentheses:
# 1. As a 1-element vector
3 - {2 - 1}
# 2. As a 1-element tuple
3 - (2 - 1)
```

## Getting started
```nahar
# Define a constant `c` for 3-space big int vector
big.c : {1 2 3}

# Define a variable `a`
a s32 := {1 2 3}

# Mutate a variable `a`
a := 3 2 1
a += {1 2 3}

#
assert a = {4 4 4}

# 
var_a += {3 2 1}
# Make var_a immutable.
var_a.lock
# This won't compile now
swap var_a var_b
```

## Functions
```nahar
# Functions are defined the same way as variable.
my_function fn: opt      # Returns an option (`yes` or `no`).
            param_a @s32 # Pass by reference a signed 32 bit integer.
            param_b i_   # Pass by value an unknown-size integer.
{
      # Shorthand for `on opt.yes [`
      if param_a = s32(param_b).catch[range_err:param_a] != 0 {
            yes
      } else 1 = 2 = 2 {
            # This panic will never be triggered, unless you change 1 to 2 - then it will
            # be triggered while compiling.
            panic "1 does not equal 2 and does not equal 2, even though 2 does equal 2"
      } else {
            "param_a does not equal param_b"; no
      }
}


# 1 line (bad practice)
my_function T fn -> opt # Returns an option (`yes` or `no`). # (param_a @s32 # A \
pointer to a signed 32 bit integer. #, param_b u32, # Pass by value 32 bit \
integer. #): { /*…*/ }
```

## Enums are Tagged Unions
```
# A union is defined with union[] - table
opt enum [
      no    : 0 # 
      yes t?: _ # 
]

# A union of types where all of the types have fields with same name.
any enum [
      {var}
]
```

## Structs
```
# A struct is defined with struct() - tuple
list struct(
      length   :u32      0   # This is how big the list is.
      capacity :u32      0   # This is how much space is allocated in `pointer`.
      pointer  :ptr(var) nil # A pointer to the place where the list actually is.
)
```

## Matches
```nahar
# Commas at end of lines are optional.
pos struct(x:u32 y:u32),
vec struct(x:u32 y:u32),
movement type: any(pos, vec),

a movement := pos(1, 2),
match a [
      pos: "({pos.x}, {pos.y})",
      vec: "({hex(vec.x)}, {hex(vec.y)})",
],
```

## Syntax List
```nahar
#! Set an project-wide attribute. `#!warn(no_docs)`
#? Compile only if an condition is met. `#? linux`
#  "Comment Open/Close" - This is like /* */ in some languages, but always ends
                          on the same line.
:  "rel" operator
:= "set mov"
+= "set add"
-= "set sub"
*= "set mul"
/= "set div"
%= "set mod"
.= "set dot"
&= "set and" `a :& b`
|= "set ior" `a :| b`
^= "set xor" `a :^ b`
~= "set not" `bool_value :~`
+  "add"
-  "sub"
*  "mul"
/  "div"
%  "mod"
&  "and"
|  "ior" 
^  "xor"
~  "not"
.  "dot" (dot product / matrix multiplication)
   "ts0" (token seperator 0 - Space)
,  "ts1" (token seperator 1 - Comma)
;  "ts2" (token seperator 2 - Semicolon)
{} "vector" (Fixed size - one type)
() "tuple" (Fixed size - one or more types)
[] "table" Matches values to names, or names to values.
>< "exchange" `a >< b`
=  "is equal" or "set" depending on context.
<  "is less"
>  "is greater"
<= "is less or equal"
>= "is greater or equal"
.. "exclusive range"
:: "inclusive range"
|| "Pipe"
!! "Unreachable" cause a compiler panic if block of code is reachable where resides.
```

## Functions / Keywords
```
api    # 

# For errors
error  # Cause a runtime error (these must be dealt with by the programmer).
break  # 
if     # Begin conditional statement
elif   # Else if
else   # Else
all()  # Multi-And
some() # Multi-Or
none() # Multi-Not(And)

# Data Types
byte   # Generic data.
big    # A big integer (infinite range).
num    # A big fixed point
sci    # A big float
hex    # A hexadecimal string
text   # A text string
list t # Dynamically growable list of another type
fn r() # A function

# Traits
i_     # An integer (implements i_)
       #     i0::128 -- An 8 bit integer that doesn't allow values over 128 or under 0
s_     # Signed integer (implements i_ & s_)
       #     s32     -- A 32 bit signed integer
       #     s7      -- A 7 bit signed integer, stored as 8 bits in RAM.
u_     # Unsigned integer (implements i_ & u_)
       #     u16     -- A 16 bit unsigned integer.
f_     # Floating Point (implements f_)
       #     f32     -- A 32 bit floating point.
       #     f64     -- A 64 bit floating point.


def Define a new constant.
new Create a new variable.
del Manually delete a variable or constant.
// 
mk Make a file
rm Remove a file
```

## Project Attributes
```
#![
      project: app                  # An executable program.
      project: api                  # In C called a library, in Rust called a crate.  We
                                    # call it an api.
      features: asm                 # Allow assembly functions.
      authors: "Jeron Aldaron Lau"  # List of authors
      license: "MIT / BSL-1.0"      # License
      version: "0.0.12"             # Version of project
      tags: "stn" "nahar" "opt" 
            "any"                   # Tags to make project easier to find
]
```
