# Aratar Runtime
The Aratar runtime is a shared library that allows for cross-platform hardware
support with Cala.

## Projects and Relations
 - Shared Binary Your Application
   - Cala For Aratar (Statically Linked Up)
 - Binary Aratar (Fake OS APIs)
   - Cala For OS (Statically Linked Up)
 - OS (Real OS APIs)
   - Hardware Drivers

Run Aratar with no arguments, you get a (C)REPL, the C being compile.
 - Wrap Code in `info()` function so that expressions get logged (printed)
 - Aratar => Aratar Bytecode (*.prog)
 - Aratar Bytecode (*.prog) => C, Rust, or Assembly
 - C, Rust, or Assembly => Machine Code
 - Execute Machine Code
 - Loop back to prompt

Run Aratar with a .yote file, and it will recompile based on the delta, and run.

Run Aratar with a .code file, and it will generate a .yote file and check old
.yote file and recompile based on the delta, and run.
