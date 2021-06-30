# Intro
Aratar is a programming language that aims for legibility, simplicity, speed,
and correctness.  It takes a lot of inspiration from Rust, but is intended to be
easier to learn and borrows concepts from other languages like Python, Swift and
Lisp.  There are additional elements unique to the language.  It may be called,
"The Systems Scripting Language".

## Goals
 - Simple; and easy to learn, read, and write syntax.
 - Compile only to web assembly, transpile to native later.
 - Promote responsive asynchronous programming styles, prevents deadlocks and
   infinite loops.
 - Always pass-by-reference for simple borrow checker (no garbage collection).
 - Type safe and memory safe.
 - Prevent numeric overflows/underflows, stack overflows and logical crashes.
 - Few powerful language features.
 - Official IDE support

# Getting Started
First, let's start with the basic hello world.

```aratar
export gn():
    log("Hello, world!")
```

## `export`
Tells the compiler the following item should be exported and made available to
the caller.

## `gn`
The generator keyword is used here to create an anonymous generator.  Exported
anonymous generators are called when you first run a program (Like `main()` from
other programming languages).

## `()`
Parameters are contained in the parentheses, we have none.

## `:`
The colon is used for assignment.  Here we are assigning the return value of our
function to the following scope (which must be indented).

## `log("Hello, world!")`
Here we are calling a built-in function that takes a variable number of
parameters that implement `To[Text]`.

# Functions
We'll do the basic thing of making a function that adds two integers.

```aratar
fn add(a Int, b Int) -> Int:
    a + b

export gn(a Int, b Int):
    let result: add(a, b)
    log("Result is: ", result)
```

# Comments
There are three types of comments in aratar.

```aratar
# This is a single-line comment

###
This is a multi-line comment, you may use extra # to disambiguate comment blocks
###

# This is an inline comment #
```

# Literals
```
# Integer
15
1_000_000
0xDEAD_BEEF
0b1111_0000

# Fixed Point / Floating Point
1.00
1_000.0
2.0_E4.5

# Text
"this is some text"

# Set, cannot repeat
{1, 2, 3}

# List, can repeat
[1, 2, 3, 1, 2, 3]

# List with repetition
[0; 4]

# Tuple, can be different types
("hi", 4)
```

# Types
Aratar is strongly-typed, but allows type inference in many scenarios.

```aratar
# All fields are private unless marked with `export`, but they are immutable.
# Exported fields should be listed first.  Tuple constructors are always not
# exported.
def NamedRecord(
    export field_a Int,
    # Private field
    field_b Text,
)

# Only variants that store public unnamed types are public.  Each variant is
# semantically and syntactically equivalent to a struct and may be assigned a
# constant value.  The structs are publically mutable.
def TypeOptions {
    VariantA: 0,
    VariantB(Int),
    # Private Variant
    VariantC(text Text): 1,
}

# This is semantically equivalent to `struct Unit()`
def Unit

# This type is completely public (constructor and fields mutable), because the
# field have no name.
def NamedTuple(Int, Text)

# Wrap a type for a custom purpose.
def NewType(Text)

# Typedef
def MyText: Text
# Types are just constants of type `Type`.
def MyText Type: Text
```

# Constants
```aratar
def Pi: 3.14
```

# Generics
Generics are useful for a couple of reasons, one is for optional types.  Aratar
defines:

```aratar
def Opt[T] {
    Some(T),
    None,
}
```

# Asynchronous
```aratar
export gn():
    let keyboard: Keyboard
    let screen: Screen
    let running: @True

    when event; (keyboard, screen):
        match event
            on(_key Key): break
            on(canvas Canvas): canvas.clear(0xFF0000)
```

# Iterator
```aratar
gn iter(input [Int 0~255]):
    for i; input:
        yield i * 4

export gn():
    let a: [1, 2, 3, 4]
    let b: []
    for i; iter(a):
        b.push(i)
    assert(b = [4, 8, 12, 16])
```

# Traits
```aratar
def fn To[T](self) -> T

struct One

fn One.To[Int](_self) -> Int:
    1
```

======

# Keywords
 1. `export`: Make an item public
 2. `fn`: Declare a function or anonymous function
 3. `def`: Define a compile-time value (may be a type or constant)
 4. `let`: Declare a variable
 5. `gn`: Declare a generator (like function, but yield and no return)
 6. `break`: Early return from loop
 7. `return`: Early return from function
 8. `for`: Iteration loop
 9. `match`: Pattern matching
 10. `if`: Check if something is true, then do something
 11. `else`: Else
 12. `import`: Import an item into scope
 13. `yield`: Yield to the caller function with a value
 14. `when`: Asynchronous for loop
 15. `on`: Anonymous functions in match statement
 16. `derive`: Define a derive macro (creates trait implementations)

# Syntax
 - `#`: Comment
 - `()`: Tuple, Parameters
 - `[]`: List, Generics
 - `{}`: Mathematical Set
 - `->`: Return Value
 - `:`: Assignment
 - `;`: Repeat (in list), From (for loop, when loop)
 - `,`: Separator Within Brackets
 - `"`: Literal Text
 - `0123456789`: Start of Literal Number
 - `~`: Ordered Type Range
 - `*`: Multiply
 - `+`: Add
 - `-`: Subtract
 - `/`: Divide
 - `@`: Unique reference (by default all references are shared)

# Types
 - `Bool`: Boolean (Enum either True or False)
 - `Text`: Unbounded Text
 - `Int`: Unbounded (Big) Integer
 - `Type`: Type
 - `Opt[T]`: Optional
 - `Key`: Key on a keyboard
 - `Keyboard`: Keyboard future
 - `Screen`: Screen future
 - `Canvas`: GPU-Accelerated graphics surface
 - `To[T]`: Into trait

# Functions
 - `fn log(what To[Text], ...)`
 - `fn assert(cond Bool.True)`














=======

## Compile only to web assembly, transpile to other architectures from there
Aratar compile to web assembly, which means it can easily run in a web browser.
Web assembly is a good itermediate representation as well as it makes no
guarantees about the number of registers, but is designed to be converted into
target-specific machine code quickly.  This reduces the amount of work to
maintain the compiler and also means it can run on any computer architecture
that has a web browser with no extra work.

## Promote responsive asynchronous programming styles, prevents deadlocks.
Rust is a great language for concurrent programming, but it does not prevent
deadlocks (where two threads are waiting for each other).  Aratar does prevent
deadlocks.  It also provides asynchronous programming without any additional
syntax.

### Counter Example
```aratar
struct State(
    io: Io
    counter: Int
)

# Implement 5 Second Delay Callback on State.
def State => Delay[5 # Fixed-point seconds #](
    def delay(state) {
        log("5 Seconds have passed")
    }
)

# Implement 1 Second Timer Callback on State
def State => Timer[1 # Fixed-point seconds #](
    def timer(@state) {
        state.counter +: 1
        log("Looped ", state.counter, " times.")
    }
)

# Implement Escape Key Callback on State
def State => KeyPress[Esc # Key #](
    def exit(state) {
        log("Quitting Program…")
        state.io.exit()
    }
)

def ~run() {
    let state: @State(
        io: Io()
        counter: 0
    )
    state.io.loop()
}
```

### Message Passing Example
FIXME: how should this work, preventing deadlocks?

```aratar
enum Message: [
    Dummy
]

struct State(
    io: Io
    channel: Channel[Message]
)

def ~run() {
    let state: @State(
        io: Io()
        # A two-way channel
        channel: Channel()
    )
    # 
    state.io.loop(state)
        .task(task, state.channel.child(), done)
}

###
A separate task running in parallel, that sends a message.
###
def task(chan Channel[Message]) {
    chan.send(Dummy)
}

###
The parent task receiving the message.
###
def done(state @State, message Message) {
    match message
    | Dummy => log("Got Dummy message")
}
```

### TEST: Server, Client Chat Program
Server

```aratar
def ~run() {
    
}
```

Client

```aratar
struct State()

def ~run() {
    
}
```

===

## Install
If you have cargo, run `cargo install aratar`.  This will install the Aratar
REPL, build tool, compiler, and default code editor.

## Start the REPL
Starting it is as simple as:

```shell
aratar
```

## Using the REPL
The REPL works like any shell, with some major differences.

```aratar
cd /my/path/with spaces
cargo-install aratar
cargo install, aratar
ls ./subdir, -lah
```

Arguments must be separated by commas (`,`) instead of spaces.  When you run
a command with a `-` in it, if it's not found, it will treat the text after the
dash as an argument.

You may also do math in the REPL:

```aratar
5 * (3 - 1)
```

If the expression doesn't have a value, it won't print anything out.

### Shell Variables
Shell variables are like other variables in aratar, to use in a command prefix
with `$`.

```aratar
let ten: 5 * (3 - 1)
echo $ten
```

```aratar
echo ${5 * (3 - 1)}
```

## Using The Code Editor
You can open aratar's code editor by typing `!` followed by the name of the file
you wish to open or create.

```aratar
!hello_world.src
```

You may press the `ESCAPE` key to get out of the editor.  Shortcuts like CTRL-C,
CTRL-V, CTRL-X, and CTRL-Z work and have alternate actions with ALT both in the
editor and in the shell.  To force quit a program, press `CTRL`+`ESCAPE` or
`SHIFT`+`ESCAPE`.

You may also just type

```aratar
!
```

And you'll go back to the last openned file.

Additionally, you can append `:` to the filename followed by a line number or
item name.

## Running A Program
Besides typing a name of a program that's on the `PATH`, you may also start a
program by typing it's name:

```aratar
hello_world
```

## Running a Directory
```aratar
my/folder/
```

This will open the folder.  If instead of pressing enter to run the directory,
you press tab it will list the directory contents.  You can then type `CTRL` +
`BACKSPACE` to clear the line.  `CTRL`+`D` will clear the line, then go to the
next line.

## Help
You can use `?` to get help on some Aratar syntax or library items.

```aratar
? Int.abs()
```

## Running Aratar Programs From Source Code
```aratar
aratar hello_world.src
hello_world.src
```

Either one of the above commands will compile hello_world.src, then run it.  If
the source file is changed while the program is running, the program will
restart with the new changes.

# Differences to Actual Programming Language
You may not start an expression with `?`, `!` or the name of a command on the
`PATH` in a source file.  These are all convinience plug-ins into the language.

# Next
[Hello World](hello_world.md)
