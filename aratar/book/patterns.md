# Patterns & Types
All types are represented as a pattern of all possible values.

```aratar
def UInt8: 0~255
def Normalized: 0.0~1.0

def False: 0
def True: 1
def Bool: False | True

let a Bool: @False
a: True

# Any number divisible by 1, any integer.
def Int: _ \1

# Each part of the match must be a constant or a pattern of constants with an
# optional name preceding it.
match x
on 0~255 => log("Can fit in a UInt8")
on y \2 => log(y, " is divisible by 2")
on _ ~ => log("x is either not divisible by 2, or not within the range 0~255")
```

## 
