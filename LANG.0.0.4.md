# Nahar
All keywords are symbols, so functions, variables and constants can have no limit on what
they can be labeled.

## File Directory Structure
Stuff is compiled from `code` to `exec`.
```
'my_project/'
├── 'code/'                # This is where all of the source *code* files are kept.
│   ├── 'my_project.code'  # The code file that shares the name of the project is entry point.
│   ├── 'my_module.code'   # Define a new module by creating a new_file.
│   └── 'MyStruct.code'    # Define a new data structure by creating a NewFile.
├── 'data/'                # This is where all of the data files / resources are kept.
│   └── 'DATA_FILE.png'    # Define a new immutable data literal struct by creating a NEW_FOLDER.
├── 'exec/'                # This is where all of the *exec*utable files are kept for distribution.
│   ├── 'my_project.exec'  # The portable executable file that shares the name of the project.
│   ├── 'my_project.apk'   # The Android application package that shares the name of the project.
│   ├── 'my_project.dmg'   # The Mac OS application package that shares the name of the project.
│   ├── 'my_project.exe'   # The Windows installer file that shares the name of the project.
│   └── 'my_project/'      # Linux flatpak repository that can later be published on flathub.
├── '.gitignore'           # Git Stuff: ignore exec/
└── '.git/'                # Git Stuff
```

# Reference
Nahar code is interpreted line-by-line, and split by spaces (except when in double-quotes).
The first token (what comes from splitting by spaces) determines what the line does.  The first
token is known as an operation.  All other tokens are known as parameters.  Here's a list of all the
types of operations and parameters.

## Prefixes & Cases
Commands & standard library functions have no prefix.  Since they are files, they can be snake_case, CamelCase or SCREAMING\_SNAKE\_CASE:
```
cd 'folder/'
```

Imports use `!`:
```
!0.0.4                # Semver Version: Import Language Standard Library.
```

Imported Functions use `~`:
```
~Pi                   # From Standard Library
~info "Some info"
~library.function     # Function from library `library`
```

Variables use `$`:
```
$ENVIRONMENT_VARIABLE # SCREAMING_SNAKE_CASE: Environment-Specific Variables
$ProgramConstVariable # CamelCase:            Program-Specific Constants / Tagged Unions - Immutable
$program_mut_variable # snake_case:           Program-Specific Variables - Mutable
```

Functions use `@` (These are only items that are exported):
```
@ITER                 # Iterators.
@ConstFunction        # Functions that are always evaluated at compile time.
@runtime_function     # Regular Functions.
```

Structures/Types use `.`:
```
.DATA_FILENAME        # A Data Structure Literal (Filename doesn't include extension).
.Struct               # A Data Structure Type.
.u4                   # Standard Library types (This one is 4 bytes unsigned integer).
```

## Operations
### Imports 🔗
A line that begins with a `!` is an import.
```
!0.0.4             # Import specific version of the standard library.
!'path/lib.zip'    # Import an external library.
!SDL2              # Dynamically link to a C library.
```

### Type Definitions
A line that begins with `CamelCase`, (generics), `:` then a newline is an alias, struct, or enum.

* Alias has no block.
* Struct has a block that lists fields, and each field is in snake_case (begins lowercase).
* Enum has a block that lists variants, and each variant is in CamelCase (begins uppercase).

Each field or variant must be on it's own line.

```
.AliasA .S4

.StructA .Type
	.s4 field_a
	# Generic
	.Type field_b
	# Field with default value
	.s4 field_c 50

.u4 $TaggedUnionA .Type 
	# Accessed with `$TaggedUnionA.VariantA` or `$VariantA` if type of tagged union is implied.
	VariantA 1
	# Some variants may have additional dat
	VariantB 2 S4
	# Generic
	VariantE .Type
$TaggedUnionB
	VariantA 3
	# Compiler will give lowest available value (.u4 0)
	VariantB
```
### Data Definitions
A line that begins with `$`, `@` or `%` is a variable declaration.

```
.u4 $variable 3 0 17  # Mutable data 📝.
# Infer type.
_   $variable 3 0 17
# 2D array (4x4).
$matrix 1.0 0.0 0.0 0.0
        0.0 1.0 0.0 0.0
        0.0 0.0 1.0 0.0
        0.0 0.0 0.0 1.0
.u4  @constant 3 0 17  # Immutable data 🔒.
# .u4 ~exported U4 3 0 17  # Exported Immutable data 📦.  TODO: Only works for functions.
```

### Function Definitions
Functions are just variables.  They are identified by only providing the (return) type but not the
value on the first line.  This is followed by an indented sequence of parameters which are defined
like struct fields.

```
# A function that returns a U4.
.u4 @function
.u4 $var_a
.u4 $var_b: 2
	# First instruction of our code.
	echo "Hello, world!"
	# Escape function with value 4 (non-indented escapes mean it's the last peice of code in the
	# function).
	`function 4
```

### Iterator Definitions
```
# A loop called "my_loop" that returns a .u4 with value 4 (looping over o Y).
.u4 @MY_LOOP .o Y
	# Escape right away.
	`MY_LOOP 4
```

### Command Call
```
some-text       # Plain text is interpreted as a command call (calls the function on the system).
                # If the command is called `a.1` you must escape it with `a..1`
```

# Function Call
```
@function       # Text that begins with `@` is interpreted as a function call on either a module or
                # a variable.
```

## Parameters
```
# values
'path/to/file_or_folder'    # path - starts with single quote
"A string"                  # string - starts with double quote (escapes spaces)
123                         # integer / decimal number - starts with 0-9
0x04DE_12AF                 # hexadecimal number - starts with 0x
0b1010_0101                 # binary number - starts with 0b
some-text                   # variable name - plain text

# other
1+2                         # expression - contains a value followed by operation(s).
```

## Standard Types
Type prefixes:
* p - Percent
* n - Normalized
* f - Fixed Point
* d - Decimal
* u - Unsigned
* s - Signed
* h - Hexadecimal
* b - Binary
* c - Character
```
.p1 / .n1 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 8 bit
.p2 / .n2 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 16 bit
.p4 / .n4 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 32 bit
.p8 / .n8 - Unsigned/Signed Normalized Decimal [0 to 1 range / -1 to 1 range] 64 bit
.f1 / .d1 - Unsigned/Signed Fixed Point (4.4 / 3.4) 8 bit
.f2 / .d2 - Unsigned/Signed Fixed Point (8.8 / 7.8) 16 bit
.f4 / .d4 - Unsigned/Signed Fixed Point (16.16 / 15.16) 32 bit
.f8 / .d8 - Unsigned/Signed Fixed Point (32.32 / 31.32) 64 bit
.f_ / .d_ - Big Unsigned/Signed Fixed Point
.u1 / .s1 - Unsigned/Signed 1 Byte Integer
.u2 / .s2 - Unsigned/Signed 2 Byte Integer
.u4 / .s4 - Unsigned/Signed 4 Byte Integer
.u8 / .s8 - Unsigned/Signed 8 Byte Integer
.u_ / .s_ - Big Unsigned/Signed Integer
.h1 - Hexadecimal 1 byte.
.h2 - Hexadecimal 2 byte.
.h4 - Hexadecimal 4 byte.
.h8 - Hexadecimal 8 byte.
.h_ - Hexadecimal Sequence.
.b1 - Binary 1 byte.
.b2 - Binary 2 byte.
.b4 - Binary 4 byte.
.b8 - Binary 8 byte.
.b_ - Binary Sequence.
.c1 - Ascii Character 1 byte
.c2 - UTF-16 Character 2 byte
.c4 - UTF-8 Unicode 4 byte
.c8 - Unicode with Emoji support
.c_ - Character Sequence - Text (unicode characters [characters are from 1-4 bytes, use for appends])
.l .T - List of T (Dynamic Size).
.o .T - Optional (Y T {yes} or N {no})
     $ x O Y # Create an Optional type:1 bit
     $ x O Y 45 # Create an Optional Si4 type (value -2147483648 is used to store N):4 byte
     $ x Y # Create an Optional type:1 bit
     $ x N # Create an Optional type:1 bit

# Matrices, Arrays (Fixed Size)
.d4*1x2 - 8 bytes 2D-Coordinates X and Y: (D4, D4).
.d4*1x4 - 16 bytes A 4D-Vector or Position: (D4, D4, D4, D4).
.d4*4x4 - 64 bytes 4D-Matrix: [D4; 16]
.u4*16 - 16 element array
```

# Compiler
Code is split into lines (split by `\n` or `,`).
Lines are split into items (split by spaces).  Item type is checked by the first byte.

```
#       Comment
!       Import
~       Global Function
$       Variable
@       Local Function / Iterator (Label)
._      Type
"       String
'       Character
0-9     Number
    0b  Number.Binary
    0x  Number.Hexadecimal
        Number.Decimal (Default)
other   Command
:+-/*:: Operation (also: ^% &|\=><>=<= ?)
`       Escape (Label)
```
