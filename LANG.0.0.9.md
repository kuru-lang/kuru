# Nahar 0.0.9
This programming language provides a platform-independant interface inside of a container.  Safety
is also ensured, like Rust.  There are no global variables or functions.

## Filesystem
Any program in this language will not have access to the files on the system (for security).  But,
it will have access to it's own files, and files that are shared with it (Open with…, Drag n' Drop).
Also there will be no access to the machine's environment variables.

```
devr                   # Devices (READ_ONLY).
devw                   # Devices (WRITE_ONLY).
path                   # This is where all of the available commands are (READ_ONLY).
data                   # This is where all of the variables & constants are ().
file                   # This is where the user's persistant storage is (READ_WRITE).
conf                   # This is where the user's config files are.
```

## 
```
// Syntax
.           // Path separator or Decimal Point `1.0`
..          // Inclusive-Exclusive range `0..10`
...         // Inclusive Range `0...9`
:           // Set a variable.
::          // 
-           // Subtraction `a: 2 - 1`
-:          // Subtraction assign `a-: 1`
+           // Addition `a: 2 + 1`
+:          // Addition `a+: 1`
/           // Divide `a: 4 / 2`
/:          // Divide assign `a/: 2`
//          // Comment or Documentation `// # Markdown supported`
*           // Multiply `a: 2 * 2`
*:          // Multiply assign `a*: 2`
%           // Modulo `a: 3 % 2`
%:          // Modulo assign `a%: 2`
@           // Mutable Reference `var b: @Int32`
\           // Escape (Return, Break) `\function "Return value"`
\\          // 
~           // Not
~:          // Not Assign
&           // And
&:          // And Assign
|           // Or
|:          // Or Assign
^           // Xor
^:          // Xor Assign
?           // Option operator: Rust's ? Operator, or Option/Boolean type.
;           // Separate lines of code on one line.
,           // Separate lines of code on one line.
[]          // An array `a: 2 * [1 2]`
()          // A tuple (data) `a: 2 * {(1 2) + (2 1)}`. Single-element tuples are automatically
            // unpacked.
{}          // A scope (code) `a: 2 * {3 + 2}`.

// Keywords
type        // Define a type (data layout: struct or tagged union).
when        // Define an iteratation on data.
let         // Define constant, or function.
var         // Define variable, or function pointer.
pub         // Define a public field.
```

## Cases

## `type`, `let`, `var`, `pub` and `when`
```
// 
type TaggedUnion: Uint8 (
    VARIANT_A(): 1
    VARIANT_B(let a: Uint32)
    VARIANT_C: 0xFF
);

// 
type Structure: (
    api field_b;

    let field_a: Int
    let field_c: 4 * [Uint8 0]
    var field_b: Int32 0

    // Constructor to create a new Structure.
    // * a: What field_a gets set to.
    let (let a) {
        field_a: a
    }

    // Method on structure to set field_b to b.
    // * b: what to set field_b to.
    let set_b(
        let b
    ) {
        field_b: b
    }

    // Method on structure
    let calculate(): Int {
        field_a + field_b
    }
);

// Define a constant.  Only code nested under scope can access.
let x: 10
// Define a constant for a type in scope.  Only code nested under scope can access.
let Float32.PI: 3.14

// Define a function.  Only code nested under scope can access.
let function(
    let Int: field_a
    var field_b: Int32 0
    let field_c: 4*[Uint8 0]
): () {
    // Attempt to return 4 times, it will actually return the first time.
    on 0..4 { \function }
};

// Iterate on 0 through 3
when 0..4 {
};

when i: 0..4 {
    
};

// Match when values.
when variable_a = 2 {
    
} < 2 {
    
} variable_b = "Value" {
    
} _ {
    
};
```
