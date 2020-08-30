# Aratar
A simple programming language inspired by Rust and Python.  This repository is
home for the aratar REPL interpeter which depends on the yeet compiler and the
dive runtime.  For default builds, yote (Yo Text Editor) is included allowing
for an easy terminal IDE experience.

## Goals
 - Rust-Like Safety - Programs never will segmentation fault, safe
   multi-threading
 - Better Keywords - Useful identifiers like `type` and `int` shall not be
   keywords
 - Fast Runtime - Faster than C using language-specific optimizations.
 - Fast Compile - Compile times should be very small because of simple compilation rules.
 - Easy To Learn - Few Keywords and Syntax Rules
 - Usable as a REPL / Terminal IDE
 - Simple Lifetime Rules - No possible compiler errors for lifetime issues
 - Full Hardware Interface Support In Standard Library (not just files and
   networking, but also graphics including GUI, audio, input, etc.)
 - Compile specifically for RISC-V, but also have machine language transpilers
 - Free On Last Use - Similar to RAII but prevents the need to create many inner
   scopes
 - Good documentation
 - State-of-the-art mathematical programming (combines the best of high-level
   abstractions and low-level optimizations)
 - More pattern matching than Rust - Everywhere (and using mathematical sets)
 - Bounded variables - allowing invalid input to be caught at compile-time

