# Command Line Arguments
Command line arguments are simpler in no other language:

```aratar
pub def run(a Int, b Int) {
    log(a + b)
}
```

You can run your program in the aratar REPL with:

```aratar
program.src 1, 3
```

```output
4
```

But, what happens if you pass in something that's not an integer (or the
incorrect number of arguments)?

```aratar
program.src yo
```

Outputs:

```output
Usage: program.src A B

A   An integer
B   An integer
```

# Next Section
[Traits](traits.md)
