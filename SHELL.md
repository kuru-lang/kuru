# Nahar Shell (Help)
Here's a list of the standard functions:

```nahar
cd .directory/               # Change directory.
ls .directory/               # List files in directory.
mk .file/                    # Make empty file or directory.
rm .file/                    # Trash file or directory.
fg nahar_proc_id             # Move command to foreground.
let var_name: 0              # Declare a variable.
let VAR_NAME: 0              # Declare a constant.
run command                  # Run command in background.  Returns nahar process id.
"INFO text"                  # Print out string (stdout).  Special codes for graphics.
warn "WARN text"             # Print out warning (stderr).
fail "FAIL text"             # Print out error & exit (stderr).
quit $return_var             # Exit on success.
undo                         # Undo last command in shell (doesn't work for apps on $PATH).
load lib-name 0.1.0          # Load semver version of a library.
module.function arg_1 arg_2  # Call a function in a module.
help                         # Print out this help message.
# Operations
set $var_name 0              # Shortcut: var_name: 0
add 1 2                      # Shortcut: (1 + 2)
sub 1 2                      # Shortcut: (1 - 2)
mul 1 2                      # Shortcut: (1 * 2)
div 1 2                      # Shortcut: (1 / 2)
mod 1 2                      # Shortcut: (1 % 2)
not Opt-Y                    # Negate to N
all Y Y Y                    # All must be Y to return Y (and).
ior N N N                    # At least one is Y to return Y (inclusive or).
xor Y N N                    # Only one is Y to return Y (exclusive or).
ieq 4 4 5                    # All are equal to return Y (is equal).  Shortcut (4 = 4)
neq 4 4 5                    # None are equal to return Y (not equal).  Shortcut (4 ! 4)
Type
	Ui32
	label: Si32          # Define a struct type, with optional labels.
Type
	VARIANT: 0           # Define an enum type, with optional labels.
Opt-Value;                   # Match on Opt-Value
	Y: info "Value is Y" # Test case
	_: info "Value is N" # Handle remaining cases, closing the match statement.
```

## Constants
```
Y  # Opt-Y
N  # Opt-N
PI # Pi (3.14 ...)
```

## Directory Types
There a 3 types of starting directories (`..` does not exist for safety purposes, scripts are contained):
```nahar
/file # absolute path (/ is root directory)
.file # relative path (. is current directory)
~file # home path (~ is home directory)
```

## Cases
```
$SCREAMING_CAMEL_CASE # Environment variables
SCREAMING_CAMEL_CASE  # Constants
snake_case            # Variables, functions
worm-case             # Library names
CamelCase             # Types
CamelCase-SCREAM      # Enum Variants
lowercase             # modules, standard functions
/path/case
.path/case
~path/case            # Path
```
