# NAHAR 0.0.1

## Standard Library
The standard library is called `~`.  You can call functions from the standard library the same way
you call from any other library or module: `~.print "Həllø, Wôrld‽"`.  But there is a shortcut: `~print "Həllø, Wôrld‽"`.
In fact, this is how operators work:
```
// Set a variable with function `:`
x.: 5
x : 5
// Add something (variable or literal) to a variable with function `+`
y.: x.+ 5
y : x + 5
// Assign to self with function `+:`
x.+: 5
x +: 5
// Copy a value into a variable with function `::`
x.:: 5
x :: 5
```

## Functions
Functions exist as variables.

## Types
```
Uf1 / Sf1 / Nf1 - Unsigned/Signed/Normalized(0-1) 1 byte (4.4) Fixed Point
Uf2 / Sf2 / Nf2 - Unsigned/Signed/Normalized(0-1) 2 byte (8.8) Fixed Point
Uf4 / Sf4 / Nf4 - Unsigned/Signed/Normalized(0-1) 4 byte (16.16) Fixed Point
Uf8 / Sf8 / Nf8 - Unsigned/Signed/Normalized(0-1) 8 byte (32.32) Fixed Point
Uf_ / Sf_ / Nf_ - Unsigned/Signed/Normalized(0-1) Big Fixed Point
Ui1 / Si1 - Unsigned/Signed 1 byte Integer
Ui2 / Si2 - Unsigned/Signed 2 byte Integer
Ui4 / Si4 - Unsigned/Signed 4 byte Integer
Ui8 / Si8 - Unsigned/Signed 8 byte Integer
Ui_ / Si_ - Unsigned/Signed Big Int
Hx1 - Hexadecimal 1 byte.
Hx2 - Hexadecimal 2 byte.
Hx4 - Hexadecimal 4 byte.
Hx8 - Hexadecimal 8 byte.
Hx_ - Hexadecimal String
Bi1 - Binary 1 byte.
Bi2 - Binary 2 byte.
Bi4 - Binary 4 byte.
Bi8 - Binary 8 byte.
Bi_ - Binary String.
Chr - Character (4 bytes unicode).
Str - Character String (Wrapper around Lst<Chr> [characters are always 4 bytes, use for inserts]).
Txt - Text (Sequence of unicode characters [characters are from 1-4 bytes, use for appends]).
Lst<T> - List of T (Dynamic Size).
Opt<T> - Optional (Y T {yes} or N {no})
      $ x: Opt.Y # Create an Optional type:1 bit
      $ x: Opt.Y 45 # Create an Optional Si4 type (value -2147483648 is used to store N):4 byte
      $ x: Y # Create an Optional type:1 bit
      $ x: N # Create an Optional type:1 bit
Ref<T> - Reference
Fnc<T, U> - Function reference.  Generic T is return type, Generic U is tuple of parameters.
Cxy - 8 bytes Coordinates X and Y: (Sf4, Sf4).
Vec - 16 bytes A Vector or Position: (Sf4, Sf4, Sf4, Sf4).
Mat - 64 bytes 4 Dimensional Matrix: [Sf4; 16]
(_, _) Tuple
[_, _] Array
{code} Either a code block or parenthesis for math: { $x $x / {$y + $z} }.
```