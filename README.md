# Aratar
A simple programming language inspired by Rust and Python.

## Goals
* Rust-Like Safety - Programs never will have segmentation fault, and can multi-process.
* No Keywords - Variable and function names should be unrestricted.
* Fast - Should not be slower than C.
* Easy To Learn - Small number of rules for syntax.
* Simple Compile - Compile times should be very small because of simple compilation rules.
* Usable as a shell similar to BASH - Similar syntax / functionality for scripting.

## Scope and freeing
Most of the time a variable will be freed once it gets to it's last use in a
function.

### General Free Case
```
var: 1
// var is freed here
term.out "Hello, world!"
```

### Match Case
```
var: 1
var = 2
	term.out "Var is " + var
	// var freed here
_ 3
	// var freed here
	term.out "Var is not 2, but 3"
_
	// var freed here
	term.out "Var is not 2 or 3"
```

### Loop Case
```
var: 1
@TRUE
	var +: 1
	term.out var
	// var not freed here
	term.out "Hello, World"
// var freed here
```

### Reference Case
This is the most complicated free detection.  This is almost compile-time
reference counting.

```
var: 1
ref: @var // ref is now dependant on var
var +: 2 // Last use of var, so
// var would be free here
ref -: 1 // Except ref uses var, it's dependant
// so var and ref are freed here
```

This actually isn't that complicated, what is complicated is when `List` gets
involved.
```
var: 1
a: @var // a depends on var
b: @2
c: @3
list: List a b c // list depends on a, which depends on var
list.push list.0 // list now depends on a twice
list.pop
list.remove 0
// Now it should be freed, but Lists can have many things happen during
// runtime - it's almost impossible to calculate this point.
term.out list
// Now this is the last use of list, so var can be freed.
```

But, if we make references a non-copyable type, things are easier.
```
var: 1
a: @var // a depends on var
b: @2
c: @3
list: List a b c // a has now *moved* into list.
list.remove 0 // A is now moved out of the list, and the return value is not
// used, so a's free location is here.  Also var isn't used again, so it's freed
term.out list
```

Then we can avoid any type of runtime garbage collection for any situation.

### More reference case
What if we want to build a dependency tree of variables?  Let's say we want
sprite to depend on window.

```
Window
	some_field Ui32
	Window;
		Window {
			some_field: 0.Ui32
		}

Sprite
	window @Window // A struct with a reference inside is marked
	Sprite; window @Window
		Sprite { window }
	draw; how_many @Ui32
		window.some_field +: how_many

window: Window
sprite: Sprite @window // window is now borrowed by sprite, because sprite is
// marked for the first parameter.
sprite2: Sprite @window // now also by sprite 2
term.print sprite
how_many: 4
sprite.draw how_many // reference is taken by draw, but parameter is not marked
// so how_many is freed here
term.print sprite2
// all dependants on window have had their last use, so window is freed.
```

How about mark swapping?
```
Swappity
	value @Ui32
	Swappity; value @Ui32 // `value` is Marked
		Swappity { value }
	swap; value @Ui32 // `value` is Marked the same way
		value: value

a: @4
swap: Swappity a // Value in the struct
swap.swap @5 // Marked the same, so swap doesn't depend on a anymore, then replaced.
// a freed here
term.out swap
// End of code so everything else freed.
```
