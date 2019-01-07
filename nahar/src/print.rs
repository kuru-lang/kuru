use std::io::{stdout, Write};

use crate::parse::{PartsIter, parse};
use crate::color;

pub fn io_error(error: std::io::Error, command: &str) -> Result<(), std::io::Error> {
    match error.kind() {
        std::io::ErrorKind::NotFound =>
            fail(parse(&format!("File \"\"{}\"\" not found.", command)))?,
        std::io::ErrorKind::PermissionDenied =>
            fail(parse(&format!("You don't have permission to run function: {}.", command)))?,
        error =>
            fail(parse(&format!("Internal error {:?}: {}.", error, command)))?
    }

    Ok(())
}

pub fn echo(args: PartsIter) -> Result<(), std::io::Error> {
    print(args, "", [color::White, color::Black, color::White], false)
}

pub fn info(args: PartsIter) -> Result<(), std::io::Error> {
    print!(" ");
    print(args, "INFO", [color::White, color::Black, color::White], false)
}

pub fn warn(args: PartsIter) -> Result<(), std::io::Error> {
    print!(" ");
    print(args, "WARN", [color::White, color::Blue, color::Blue], false)
}

pub fn fail(args: PartsIter) -> Result<(), std::io::Error> {
    print!(" ");
    print(args, "FAIL", [color::White, color::Red, color::Red], true)
}

pub fn print(args: PartsIter, label: &str, colors: [color::Color; 3], all_bold: bool) -> Result<(), std::io::Error> {
    color::set(true, colors[0], colors[1]);
    print!("{}", label);
    stdout().flush()?;
    color::set(all_bold, colors[2], color::Black);
    stdout().flush()?;
    print!(" ");
    let mut args = args.peekable();
    'print_echo: loop {
        if let Some(arg) = args.next() {
            if args.peek() == None {
                print!("{}", arg);
            } else {
                print!("{} ", arg);
            }
        } else {
            break 'print_echo;
        }
    }
    print!("\x7f\n");
    color::set(false, color::White, color::Black);
    stdout().flush()?;

    Ok(())
}
