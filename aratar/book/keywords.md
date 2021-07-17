# Keywords
 - `export`
 - `import`
 - `fn`

# Keywords (old)
There are only 8 keywords in Aratar.

## `lib`
Import a library.

The keyword is followed by the name of the library to grab, then the version
number.  Optionally a URL may follow the version.

```aratar
# The following two are equivalent
lib sys 0.0.24
lib sys 0.0.24 https://github.com/aratar-lang/lib/blob/main/sys-0.0.24.lib

# Local copy or patch
lib sys 0.0.24 file://sys.lib
```

## `pub`
Export an item to a public interface.  Note that no public items are publicly
mutable, only publicly observable and functions are additionally callable.

```aratar
# All variants, fields and methods are private.
pub Type

# Documentation for public type
pub MyOption:
    # Documentation for public variant
    Purple
    # Documentation for public variant
    Green

# Documentation for public method
pub MyOption.is_green()
```

## `opt`
Make an option type.  Option types are one of a set of values, and may have
associated data in a tuple.

```aratar
opt OptionType:
    Purple
    Green

opt IntOrText:
    Int(Int)
    Text(Text)
```

## `rec`
Make a record type.  Record types are like tuples, except they have named fields
and don't use tuple syntax.

```aratar
rec IntAndText:
    int: Int
    text: Text
```

## `def`
Define a constant, function or method.  All of these are data that doens't
change.

```aratar
def Pi: 3.14

def Type.Tau: 2.0 * Pi

def function():
    # do something

def Type.method(self):
    # self could be named anything

def Type.OtherType(self):
    # do casting from Type to OtherType
```

## `let`
Declare a variable.

```aratar
# Shared data reference (immutable)
let a: 5

# Exclusive data reference (mutable)
let a: @5

# Uninitialized shared data reference.
let a: Int

# Uninitialized exclusive data reference.
let a: @Int
```

## `for`
Iterate over an iterator.

```aratar
# Simple for loop reads, "for i over range 0 5".
for i; Range(0, 5):
    log(i)
    
for (i, k); ["hello", "world"].enumerate():
    log(i, ": ", world)
```

## `con`
Check condition to divert control flow.  You may create wildcard patterns by
typing an unused or ceded identifier name.  If an identifier with the same name
is in scope, it will fail to compile.

```aratar
con i:
    <5: # do something when i < 5
    _: # else

con maybe:
    Some(value): # do something with associated data when maybe = Some(_)
    None:
        # do something when there's no associated data on the Maybe[T] type
```
