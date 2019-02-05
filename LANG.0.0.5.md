# Nahar 0.0.5
Must have this header:

```
#!nahar 0.0.5
```

# Mutable vs Immutable
Mutable variables are `snake_case`.  Immutable variables are `CamelCase`.

# Domains
```
'command'
function # Shell makes functions for all of the commands.
ConstantOrType
MACRO # A function that runs during compile time.
$local_mutable_variable
$LocalImmutableVariable
$ENVIRONMENT_VARIABLE
```

# Lang Built-In
Signed integers reserve one value for option.  For 8 bit -128 to 127, -128 is reserved for N.  Chars
reserve `'\0'` for N.  Structs will look for one of these values.  Ref will reserve Null for 

Naming: `Name` + `Length`

```
Opt-    # 1 Bit Y or N, Y can wrap a type.
List-   # Dynamically growable array.
Ref-    # Reference to type.
Float4  # 4 Bytes Signed Fixed Point (Single Precision)
Float8  # 8 Bytes Signed Fixed Point (Double Precision)
Sint1   # 1 Byte Signed Integer
Sint2   # 2 Bytes Signed Integer
Sint4   # 4 Bytes Signed Integer
Sint8   # 8 Bytes Signed Integer
SintB   # Dynamic Size Unsigned Integer
Uint1   # 1 Byte Signed Integer
Uint2   # 2 Bytes Signed Integer
Uint4   # 4 Bytes Signed Integer
Uint8   # 8 Bytes Signed Integer
UintB   # Dynamic Size Unsigned Integer
Sfix1   # 1 Byte Signed Fixed Point
Sfix2   # 2 Bytes Signed Fixed Point
Sfix4   # 4 Bytes Signed Fixed Point
Sfix8   # 8 Bytes Signed Fixed Point
SfixB   # Dynamic Size Unsigned Fixed Point
Ufix1   # 1 Byte Signed Fixed Point
Ufix2   # 2 Bytes Signed Fixed Point
Ufix4   # 4 Bytes Signed Fixed Point
Ufix8   # 8 Bytes Signed Fixed Point
UfixB   # Dynamic Size Unsigned Fixed Point
Hex1    # 1 Byte Signed Hexadecimal
Hex2    # 2 Bytes Signed Hexadecimal
Hex4    # 4 Bytes Signed Hexadecimal
Hex8    # 8 Bytes Signed Hexadecimal
HexB    # Dynamic Size Unsigned Hexadecimal
Bin1    # 1 Byte Signed Binary
Bin2    # 2 Bytes Signed Binary
Bin4    # 4 Bytes Signed Binary
Bin8    # 8 Bytes Signed Binary
BinB    # Dynamic Size Unsigned Binary
Ascii   # 1 byte (ASCII)
Ucs2    # 2 bytes (UTF-16 - 1 codepoint)
Char    # 4 bytes (UTF-8 - 1 codepoint)
Text    # Text
```
