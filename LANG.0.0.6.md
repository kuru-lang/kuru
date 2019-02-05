# Nahar 0.0.6
Must have this header:

```
#!nahar 0.0.6
```

# Domains
```
"command" "--flag" "argument" var $VAR      # use shell to call a program from $PATH
[Uint32 function] Uint32 arg_a Sint32 arg_b # define a function (returns Uint32)
function arg_a arg_b                        # call a function
Vec4                                        # define a struct
    Uint32 mutable_field
    Uint32 CONSTANT_FIELD
    [Vec4] Uint32*4 components              # define a constructor.
    [Text]                                  # define a converter.
    [function] Uint32 arg_a                 # define a function on the type
Enum                                        # define a enum
    VARIANT_A: 0
    VARIANT_B Uint32: 1                     # variant with data
    VARIANT_C
    VARIANT_D Uint32
    [Enum] Uint32 param                     # (same as struct) define a constructor
    [Uint32 function] Uint32 arg_a          # (same as struct) define a function on the type
Int: Sint32                                  # alias a type
Trait:                                      # a trait (see Rust language).
    [Char implement_this] Uint32 param      # function definition.
Vec4 1 0 0 0                                # call a constructor
#!math                                      # import all from "math.zip" library.
#!math.PI                                   # import PI from "math.zip" library.
#!math.sin cos PI                           # import multiple items from a library.
Vec4 var_a: 1 0 0 0                         # define a local mutable variable
var_a: 1 0 0 0                              # set a local mutable variable
Vec4 VAR_A: 1 0 0 0                         # define an immutable variable (local or global).
$ENVIRONMENT_VARIABLE                       # environment variable
var_a = 1 0 0 0                             # if statement: if var_a equals ...
= 1 _ _ _                                   # case (if else) statement
=                                           # else statement
2 * (2 + 2)                                 # evaluate something before something else
1 2 3 "string"                              # tuple
Vec4*2 var_a:
    1 0 0 0
    1 0 0 0
Uint32*6 var_b: 0 0 1 1 2 2      # define arrays
value: var_b.0                   # index array or tuple
slice: var_b.0~4                 # slice array or tuple from 0 to 4 (indices 0, 1, 2, and 3).
slice: var_b.0~=4                # slice array or tuple from 0 to 4 (inclusive: 0, 1, 2, 3, and 4)
```

# About
Code is linear and not compiled as a tree, even though file system suggests.  Published libraries
must be dual-licensed MIT/Boost.  Code will be checked at compile-time to make sure that nothing
can fail.  You can call the `fail` function to determine parts of the code you want to be
unreachable.  Code will also be checked at compile time for infinite loops - no function may have
an infinite loop.  In order to get a loop like a game loop you must use the `looper` function.
FFI is not included in the language on purpose because FFI is unsafe.  The standard library takes
care of interfacing to the system (which is done purely through stdout escape sequences that the
nahar engine interprets).

```
#!nahar 0.0.6

# Entry Point for Program (variables defined here are usable only in other functions in this file)
Uint32 i: 0
info "Hello, World!"
looper loop

[loop]
Uint32 ONE: 1
info i ": " (another_function) ", " ONE
i = 4
    looper stop
i +: 1

[stop]
another_function
info "Can't go on " _ "...." # Use `_` for last returned value.
exit

# Create Function Entry Point
[function]
Uint32 a: 2
# Iterate 4 times (same as `0~4`)
4
    a +: 1

[Uint32 another_function]
function
```

# If
If statements are actually just iterators over boolean values.

```
# Repeats the print twice.
2
    info "Hello, World (Prints Twice)"

# If 1 equals one, which it does, print once.  Otherwise, don't print.
1 = 1
    info "Hello, World (Prints Once)"

# Use `~` which means infinite or max range to make an infinite loop
~
    info "This will never stop printing"
```

# Types
```
Opt-    # 1 Bit Y or N, Y can wrap a type.
List-   # Dynamically growable array.
Ref-    # Reference to type.
Float32 # 4 Bytes Signed Fixed Point (Single Precision)
Float64 # 8 Bytes Signed Fixed Point (Double Precision)
Sint8   # 1 Byte Signed Integer
Sint16  # 2 Bytes Signed Integer
Sint32  # 4 Bytes Signed Integer
Sint64  # 8 Bytes Signed Integer
SintB   # Dynamic Size Unsigned Integer
Uint8   # 1 Byte Signed Integer
Uint16  # 2 Bytes Signed Integer
Uint32  # 4 Bytes Signed Integer
Uint64  # 8 Bytes Signed Integer
UintB   # Dynamic Size Unsigned Integer
Sfix8   # 1 Byte Signed Fixed Point
Sfix16  # 2 Bytes Signed Fixed Point
Sfix32  # 4 Bytes Signed Fixed Point
Sfix64  # 8 Bytes Signed Fixed Point
SfixB   # Dynamic Size Unsigned Fixed Point
Ufix8   # 1 Byte Signed Fixed Point
Ufix16  # 2 Bytes Signed Fixed Point
Ufix32  # 4 Bytes Signed Fixed Point
Ufix64  # 8 Bytes Signed Fixed Point
UfixB   # Dynamic Size Unsigned Fixed Point
Hex8    # 1 Byte Signed Hexadecimal
Hex16   # 2 Bytes Signed Hexadecimal
Hex32   # 4 Bytes Signed Hexadecimal
Hex64   # 8 Bytes Signed Hexadecimal
HexB    # Dynamic Size Unsigned Hexadecimal
Bin8    # 1 Byte Signed Binary
Bin16   # 2 Bytes Signed Binary
Bin32   # 4 Bytes Signed Binary
Bin64   # 8 Bytes Signed Binary
BinB    # Dynamic Size Unsigned Binary
Ascii   # 1 byte (ASCII)
Ucs2    # 2 bytes (UTF-16 - 1 codepoint)
Char    # 4 bytes (UTF-8 - 1 codepoint)
Text    # Text
Data    # A string of bytes.
```

# Standard Library
```
# Functions
[info] {Text text}      # Print to stdout
[warn] {Text text}      # Print to stderr
[fail] {Text text}      # Print to stderr and fail (during compile time).
[looper] [loop]         # Set the loop.  Initially, the next loop is set to `exit`.
[exit]                  # Quit the program.
[save] _ file           # Save a file's program state (can be any type).
[_ load]                # Load a file's program state (can be any type).
```

# Syntax
```
{Sint32 iter}           # Slice iterator `iter` over `Sint32`
2 * (1 + 2)             # Parenthesis - like math
[Sint32 fn] Char c      # Function definition
```
