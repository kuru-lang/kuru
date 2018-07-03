#
	Nahar's Standard Library - Plop Grizzly - <jeron.lau@plopgrizzly.com>

	Copyright Jeron A. Lau 2018.
	Distributed under the Boost Software License, Version 1.0.
	(See accompanying file LICENSE.md or copy at
	https://www.boost.org/LICENSE_1_0.txt)

# Command Line interface
mod out text in prompt
ffi putchar(c Si32) Si32 # int putchar(int c);

# Print a line out to the terminal.
fun out(line @Text)
	text({line + '\n'})

# Print text out to the terminal, without newline.
fun text(text @Text)
	text.iter(c putchar(c))

# Get command line input, if a characters been entered. (TODO)
fun in() Opt<Char>
	None

# A blocking prompt (TODO)
fun prompt() Text
	""
