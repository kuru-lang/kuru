# Traits
Traits are like other types, except that their exact representation is flexible.
The type `Int` is a trait that represents any integer type.  Which type is used
is determined at compile time.

## Implementing Your Own `Int` Type
```aratar
def CustomInt: Int
```

Now the type `CustomInt`, is essentially equivalent to `Int`.  It has all of the
same methods, and the same representation.  But, it's its own unique type (a
subtype of `Int`).  So, you can't use any `Int` where `CustomInt` is required.
`Int` implements it's own traits, one of which is `Type` (all types implement
`Type`).

### Implementing Trait Methods
By default, `def` types implement all of the traits that the supertype does.
You may override these.  So, we're going to override a method on `Type`.

```aratar
def CustomInt.Type.invalid(self) -> Text {
    "An operand in the addition of two numbers"
}
```

Have an idea what we're doing yet?  Here's the final example:

```aratar
def Operand: Int

def Operand.Type.invalid(self) -> Text {
    "An operand in the addition of two integers"
}

def run(a Operand, b Operand) {
    log(a + b)
}
```

Running:

```aratar
program.src yo
```

Outputs:

```output
Usage: program.src A B

A   An operand in the addition of two integers
B   An operand in the addition of two integers
```

# Next Section
[Variables](variables.md)
