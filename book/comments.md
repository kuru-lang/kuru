# Comments
There's a uniform syntax for Aratar comments:

```aratar
###
Multi-Line Comment, good for long chunks of documentation
###

# Single-Line Comment, next is empty comment
function(1 ##, 2 # Inline Comment #)
```

## Justification
`//` and `/* */` are historical artifacts that look like division and
multiplication.  `#` is quicker to type, but is lacking in most languages in
that you can't do an inline comment.

# Next Section
[Command Line Arguments](cl_args.md)
