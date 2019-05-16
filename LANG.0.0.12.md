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
# Define a constant `c`
c si: {1 2 3}

# Define a variable `a`
a si = {1 2 3}

# Mutate a variable `a`
a = 3 2 1
a += {1 2 3}

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
my_function fn -> opt          # Returns an option (`yes` or `no`).
                  param_a @s32 # A pointer to a signed 32 bit integer.
                  param_b u32  # Pass by value 32 bit integer.
{
      # Cast param_b to s32.  If it fails from overflow, set param_a as temporary value.
      if param_a = s32(param_b)[e: param_a] {
            yes
      } elif 1 == 2 {
            panic "1 does not equal 2"
      } else {
            # Use , to do 2 things on the same line.  Print out a string then return.
            "param_a does not equal param_b", no
      }
      panic "Should have returned already"
}


// 1 line (bad practice)
my_function fn -> opt # Returns an option (`yes` or `no`). # (param_a @s32 # A \
pointer to a signed 32 bit integer. #, param_b u32 # Pass by value 32 bit \
integer. #): { /*…*/ }
```

## Tagged Unions
```
opt type T? [
      no     :0
      yes T? :_
]
```

## Syntax List
```nahar
#! Set an project-wide attribute. `#!warn(no_docs)`
#? Compile only if an condition is met. `#[=linux]`
#  "Comment Open/Close" - This is like /* */ in some languages, but always ends
                          on the same line.
:  "def" operator
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
```

## Functions / Keywords
```
# For errors
panic  # Cause a compiler-panic if the block of code where the fail resides is reachable.
error  # Cause a runtime error (these must be dealt with by the programmer).
break  # 
if     # Begin conditional statement
elif   # Else if
else   # Else

byte   # Generic data.
sint() # Signed integer
       #     sint(32bit) -- A 32 bit integer
       #     sint(0-128) -- An 8 bit integer that doesn't allow values over 128
       #     sint        -- 
uint() #
list() #
fn     #
type   #


def Define a new constant.
new Create a new variable.
del Manually delete a variable or constant.
// 
mk Make a file
rm Remove a file
```
