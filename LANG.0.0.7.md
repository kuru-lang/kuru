# Nahar 0.0.7

## Parser
```
# Must have this header                                 (Begins with #!)
#!nahar 0.0.7
```

Parser follows these rules.
```
# Declare variable / function
CamelCase-Generic field …
# Define a type
CamelCaseStruct-Generic …
# Call a function
snake_case …
# Call a macro
SCREAMING_SNAKE_CASE …
```

* snake_case:               keywords (define variable), functions, commands from path.
* CamelCase:                define a type.
* SCREAMING\_SNAKE\_CASE:   macros, enum variant (type of macro).

### Data Type Definitions

#### Built-In Types
```
Get-T # Declare a variable, private setter, public getter.
Set-T # Declare a variable, public  setter, public getter.
```

#### Example
```
Struct
    # Private Fields
    Vec4 CONSTANT: 1 1 1 1
    Vec4 IMMUTABLE
    Uint32 mutable_variable: 10

    # Public Fields
    Get-Uint32 public_getter_private_mutable: [1 2 3 4] # Matrix on the stack (fixed-size)
    Set-[Uint32] public_mutable_private_mutable: [1 2 3 4] # Matrix on the heap (dynamic-size)

# Use Uint32 as the tag's type.
TaggedUnion: Uint32
    # Define first tag (start at 1, rather than default which is 0)
    VARIANT_A: 1
    VARIANT_B
    VARIANT_C
        mut mutable_field: Float32 1.0

# Alias
Alias: Uint32

# Generics
Struct Type
    Type variable: DEFAULT

# Generic Tagged Union
Option-Type: Data8
    SOME: 0
        Type value
    NONE
```
