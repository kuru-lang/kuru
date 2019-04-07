# Nahar 0.0.8

## Cases
```
snake_case              # Always Variable (functions count as variables too).
CamelCase               # Always a Type (Block of Data).
SCREAM_SNAKE            # A constant value.
_snake_case             # _ in front allows for stuff to be unused without compiler warnings.
`snake_case             # ` in front allows usage of variables that have been shadowed, add more `
                        # symbols to go back on the stack of shadowed variables.
```

## Standard Library Functions
```
+($src0 [$src1])    # + Add 1 or more numbers in [$src1] to $src0 together and save to dst.
-($src0 [$src1])    # - Subtract 1 or more numbers in [$src1] from $src0
*($src0 [$src1])    # * Multiply 1 or more numbers in [$src1] to $src0
/($src0 [$src1])    # \ Divide 1 or more numbers in [$src1], and then $src0 / $src1
                    #     /(4 8 4 16) = (4 / (8 / (4 / 16)))
%($src0 [$src1])    # % Modulo (same as divide)
&($src0 [$src1])    # & And 2 or more numbers together.
|($src0 [$src1])    # | Inclusive or 2 or more numbers together.
^($src0 [$src1])    # ^ Exclusive or 2 or more numbers together.
```

## Expressions
All lines of code follow the pattern:
```
# snake_case name followed by space-separated parameters.
function_name param_1 param2 param3
```

In order to call addition method inside (works for other methods):
```
function_name param_1 param2a + param2b param3
# or
function_name param_1 {param2a + param2b} param3
```

Expressions on their own print their evaluation to the terminal
```
# Prints out 2 to the terminal
1 + 1
# Prints "Hello, world!" to the terminal
"Hello" + ", world!"
```

## Declare variables
```
# Local Variables
let VARNAME: Int 0 # local immutable / constant
let varname: Int 0 # local variable / function (varies on input)
# Public Variables
let VARNAME: Int 0 # define a public constant.
let varname: Int 0 # define a public variable, not publicly mutatable.
```

## Variable Types
Variables operate as big-endian in cases where it matters (when you're actually looking at the
bits).  Otherwise, the system default is used.

```
# Dynamic Size Data
Bin     # Binary Data String
Hex     # Hexadecimal Data String
Int     # Big Integer
Fixed   # Big Fixed Point (Decimal String)

# Binary Data
?       # 1 bit Boolean value.
Bin8    # 8-bit Binary Data
Bin16   # 16-bit Binary Data
Bin32   # 32-bit Binary Data
Bin64   # 64-bit Binary Data
Bin128  # 128-bit Binary Data
Bin7?   # 7-bit optional
Bin15?  # 15-bit optional
Bin31?  # 31-bit optional
Bin63?  # 63-bit optional
Bin127? # 127-bit optional

# Hexadecimal Data
Hex8    # 8-bit Hexadecimal Data
Hex16   # 16-bit Hexadecimal Data
Hex32   # 32-bit Hexadecimal Data
Hex64   # 64-bit Hexadecimal Data
Hex128   # 64-bit Hexadecimal Data
Hex7?   # 7-bit optional
Hex15?  # 15-bit optional
Hex31?  # 31-bit optional
Hex63?  # 63-bit optional
Hex127? # 127-bit optional

# Signed Integer Data (-128 is reserved for NIL, range is -127 to 127)
Sint8       # 8-bit  Signed Integer
Sint16      # 16-bit Signed Integer
Sint32      # 32-bit Signed Integer
Sint64      # 64-bit Signed Integer
Sint128     # 128-Bit Signed Integer
Sint8?      # 8-bit  Optional
Sint16?     # 16-bit Optional
Sint32?     # 32-bit Optional
Sint64?     # 64-bit Optional
Sint128?    # 128-Bit Optional

# Unsigned Integer Data
Uint8       # 8-bit  Signed Integer
Uint16      # 16-bit Signed Integer
Uint32      # 32-bit Signed Integer
Uint64      # 64-bit Signed Integer
Uint128     # 128-Bit Signed Integer

# Floating-Point Data
Float32     # 32-bit Floating Point
Float64     # 64-bit Floating Point

# Fixed-Point Data
Fixed8      # 4+4 bit Fixed Point
Fixed16     # 8+8 bit Fixed Point
Fixed32     # 16+16 bit Fixed Point
Fixed64     # 32+32 bit Fixed Point
Fixed128    # 64+64 bit Fixed Point
Norm8       # 1+7 bit Fixed Point (-1 (Sint8 -127) to 1 (Sint8 127))
Norm16      # 1+15 bit Fixed Point (-1 to 1)
Norm32      # 1+31 bit Fixed Point (-1 to 1)
Norm64      # 1+63 bit Fixed Point (-1 to 1)

# Lists
List-T      # A list of T

# Vectors and Matrices
MatN-T          # A 2-dimensional square matrix of T
                # ```
                # let matrix: Mat4-Fixed32
                #     1 0 0 0
                #     0 1 0 0
                #     0 0 1 0
                #     0 0 0 1
                # ```

Stream
```

## Associated Constants
These may be defined as functions which are evaluated at compile time.

```
# Nothing.  Not all types can be NIL.
NIL
# True (1)
TRUE
# False (0)
FALSE
# pi (3.14... Archimedes' constant)
PI
# e (Euler's number)
E
```

## Syntax
```
{}  # Block of code with specific rules, also mathematical parenthesis.
()  # Tuple.
[]  # 1 or more (array, iterator).
[]? # 0 or more (array, iterator).
?   # 1 or 0.
:   # Set.
+   # Add.
-   # Subtract.
*   # Multiply.
/   # Divide.
%   # Modulo (Remainder).
.   # Path separator, decimal point (like `.` in Java or `::` in Rust or `/` in Bash, 1.5).
->  # Pipe (like `|` in bash), or arguments before function name.
    #   "Hello, World" -> less  # Bash: `echo "Hello, World" | less`
    #   {1.5 -> sin}            # Same as `sin(1.5)`
    #   var a, 1.5 -> a:        # Same as `var a, a: 1.5`
    #   "2.5" -> a.from_str     # Same as `a.from_str("2.5")`
!   # Not.
!=  # Is not Equal.
=   # Is equal.
>   # More than.
<   # Less than.
>=  # More than or equal to.
<=  # Less than or equal to.
..  # Range inclusive lower bound, exclusive upper bound.
,,  # Range inclusive lower bound, inclusive upper bound.
,   # Separate multiple function/macro calls on one line.
;   # Iterator / Repeater
>>  # Shift right (Bits)
<<  # Shift left (Bits)
\   # Return / Break
$   # IO Function or any function that affects the global state, or environment variable.
`   # Unshadow
"   # Unicode (Utf8) Text (length, bytes)
b"  # Unicode (Utf8) C String (bytes NULL-terminated) .asciiz
'   # Unicode (Utf8) Text (32-bits)
b'  # Unicode (Utf8) C Char (8-bit)
@   # Address / Pointer / Reference
~   # 
&   # And
|   # Or
^   # Xor
!!  # Return value is "Does not return"
```

## Function Definitions
```
## A Private Function
## -param_a: A dynamically sized integer.  Function can't modify the value.
## -param_b: An optional 32-bit integer (0 by default).  Function can modify the value.
## -param_c: An optional 0+ element array of 8 bit integers.
@function(let param_a: Int, var param_b: Int32 0, let param_c: [Uint8 0; 4]?): () {
    # Return unit tuple.
    \function: ()
    # Return default (unit tuple)
    \function
}

## A Public Function with no parameters.
## -function: Returns a 32-bit fixed point number (0.0 is the default).
def function(): Fixed32 0.0 {
    # Return 1.0
    \function: 1.0
    # Return default (0.0)
    \function
}

## Define a function / code block macro.
def macro {
    # Define code parser by repeating
    [
        # Parse the structure of `example_text: 12`
        let keyword: Text,
        ' '?,
        ':',
        ' '?,
        let number: Uint32,
    ]?
} {
    
}
```

## Call a function
Function calls and variable declarations can either be separated by a , or newline.

```
var some_number: 1.0, some_number: sin some_number
function {12 + 1} 4
let matrix: [
    [1 0]
    [0 1]
]
+(1 3 2)
```

## Matching
If Else If Else / Match statement.

```
# Declare Variable
var boolean: FALSE
boolean: TRUE

# If boolean is true, then print "It's true!"
boolean = TRUE {
    "It's true!"
}
# Comparison on last used compared variable, if first block wasn't run.
= FALSE {
    "It's false!"
}
# Match on anything else
= {
    error "Boolean value is not true of false!  This should never happen."
}
```

## Use as terminal
Each shell is run inside of a container, so root directory is the root of the container.

```
# cd into root directory by typing the path `\` & print out contents.
.
# cd into a directory under root.  Directory at path `\dir\`
.dir
# open a file, if it's a binary then run it with root as containing folder of the binary.
.dir.file
# run a terminal command
$rmdir .dir
```

## Standard library
```
ffi putc_unlocked(let c: Int32, let file_stream: @)
ffi flockfile(let file_stream: @)
ffi funlockfile(let file_stream: @)
ffi stdout

def warn(let text: Text) {
    flockfile(stdout)
    char; text {
        putc_unlocked char stdout
    }
    funlockfile(stdout)
}

def error(let text: Text): !! {
    warn text
    exit
}

def debug(let text: Text) {
    DEBUG {
        text
    }
}
```

## Structure
Everything is a file in a containerized file system.

```
.           # Root Folder.
.path       # Files that are on the path.
.path.def   # Macro for defining constants.
.path.var   # Macro for defining variables.
.save       # Files that persist data.
```
