# Hardware Interfaces
Safely interface with hardware.  When Aratar is targeting bare metal,
this involves sending bits to the
[South Bridge](https://en.wikipedia.org/wiki/Southbridge_%28computing%29),
and handling hardware interrupts.  When targeting an OS, it involves
running syscalls to send messages to the kernel, and handling kernel
interrupts.  When targeting a specific type of desktop environment
(like Wayland + ALSA + evdev, etc., for example), it involves
dynamically linking to system libraries (this is the most common).  For
a given CPU x86\_64, you might target:

- `x86_64.exe` if you're writing a kernel or driver.
- `x86_64-linux.elf` if you're writing a desktop environment, or a
  program targeting a linux device without a desktop environment.
- `x86_64-linux-cala.elf` if you're writing a userspace program to be
  used in a desktop environment.  This is what most people will want to
  use.
- `dive.app` - This is a weird one because the desktop
  environment in Dive OS is just a program, so there's no need for a
  separate .cala target.  Also weird, because the file will run on any
  architecture.

For the hardware interfaces to be safe, there needs to be a description
of the hardware capabilities.  This description must be correct for the
program to be considered safe, so it's not Aratar code itself, but
rather a MuON data file that is built in to the compiler.

For an example, let's show how to write a hardware description for MIPS
assembly (using SPIM as the kernel).  We'll test it with QtSpim, using
the target `mips-spim.asm`.

```muon
# For this example, say we're writing targeting a system with no
# desktop environment.  That means we'll be defining syscalls, making
# sure there's no way to trigger undefined behavior.

# Log interface, define a function that takes a string and prints it
# out.
log:

  # Define function to write to information log, `print()`
  print:

    # Use optional field `syscall`, using MuON substitute for first
    # field, code.  We're ignoring syscalls 1, 2, and 3 because we can
    # format integers and float point numbers ourselves.
    syscall: 4

      # MIPS print syscall requires a null-terminated string.
      data: Text<ZERO>

      # Offset by zero bytes.  The only thing in the StringZero
      # structure is a pointer to a null-terminated string.
      args: addr

    # All logs should end in a newline.
    syscall: 11
      args: Char.NewLine

  # Define function to write to debug log, `debug()`, SPIM only has one
  # log level, so it's going to be exactly the same as `print()`.
  debug:
    syscall: 4
      data: Text<ZERO>
      args: addr
```

`dive.app` target is also one that uses syscalls:

```muon
log:
  print:
    syscall: 1
      data: Text
      args: size
      args: addr
  debug:
    syscall: 2
      data: Text
      args: size
      args: addr

# Hardware for character input.
keyboard:
  press: 1
    eval: Char($v0)
  release: 2
    eval: Char($v0)

# Hardware for joystick input.
joystick:
  press: 3
    eval: Key(Int($v0))
  release: 4
    eval: Key(Int($v0))
  move: 5
    eval: Move(Int($v0), Dec<[-1:1], 16.16>($v1), Dec<[-1:1], 16.16>($v2))
  axis: 6
    eval: Axis(Int($v0), Dec<[-1:1], 16.16>($v1))

# Hardware for cursor input.
cursor:
  press: 7
    eval: Key(Int($v0))
  release: 8
    eval: Key(Int($v0))
  move: 9
    eval: Move(Int($v0), Dec<[-1:1], 16.16>($v1), Dec<[-1:1], 16.16>($v2))
  scroll: 10
    
```

MuON Spec file for Aratar hardware file format:

```muon
:::
# Log hardware interface
log: optional record
  # Hardware interface for `print()` function (stderr).
  print: record OutputInterface
    # Choose either syscall (for syscalls), c_api (for dynamically
    # linked), or dl_api (for dynamically loaded).
    syscall: optional record Syscall
      # The System Call Code.
      code: int
      # The aratar data structure that will be read by the hardware.
      data: text
      # List of fields from data structure to pass as arguments into
      # the syscall.
      args: list text
      # An Aratar constructor using actual register names for it's
      # arguments, that will evaluate to the return value of the
      # function call.
      eval: text
    c_api: optional record
      # TODO
    dl_api: optional record
      # TODO

  # Hardware interface for `debug()` function (stdout).
  debug: optional record OutputInterface

# Keyboard hardware interface
keyboard: optional record
  # Hardware interface for `key()` pressed interrupt.
  press: record InputInterface
    # Interrupt code.
    code: int
    # Optional syscall for actually getting data, if extra data is
    # associated with the interrupt.  The constructor associated with
    # the syscall is evaluated and becomes an argument to the
    # programmer's asynchronous callback.
    syscall: optional Syscall
    # An Aratar constructor using actual register names for it's
    # arguments, that will evaluate and become an argument to the
    # programmer's asynchronous callback.
    eval: optional text
  # InputInterface for when key is lifted.
  release: record InputInterface

# Cursor hardware interface
cursor: optional record
  # InputInterface for when mouse button is pressed.
  press: record InputInterface
  # InputInterface for when mouse button is lifted.
  release: record InputInterface
  # InputInterface for mouse coordinates move.
  move: record InputInterface
:::
```
