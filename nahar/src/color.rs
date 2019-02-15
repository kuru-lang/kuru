//! # Color Documentation (Syntax Highlighting Rules)
//! * Red: Function DNE Bold: Type DNE & Error,
//! * Green: Code,      Bold: Function Exists,
//! * Yellow: Number,   Bold: Prompt,
//! * Blue: Warn,       Bold: Warn,
//! * Magenta: Comment, Bold: () / Syntax Stuff,
//! * Cyan: Strings,    Bold: Environment Variable,
//! * White: Stdout,    Bold: Type Exists,

use std::io::{stdout, Write};

const RESET: &'static str = "\x1B[0m";

const BOLD: &'static str = "\x1B[1m";
const LINE: &'static str = "\x1B[K";

const BLACK: &'static str = "\x1B[30m";
const RED: &'static str = "\x1B[31m";
const GREEN: &'static str = "\x1B[32m";
const YELLOW: &'static str = "\x1B[33m";
const BLUE: &'static str = "\x1B[34m";
const MAGENTA: &'static str = "\x1B[35m";
const CYAN: &'static str = "\x1B[36m";
const WHITE: &'static str = "\x1B[37m";

const BG_RED: &'static str = "\x1B[41m";
const BG_GREEN: &'static str = "\x1B[42m";
const BG_YELLOW: &'static str = "\x1B[43m";
const BG_BLUE: &'static str = "\x1B[44m";
const BG_MAGENTA: &'static str = "\x1B[45m";
const BG_CYAN: &'static str = "\x1B[46m";
const BG_WHITE: &'static str = "\x1B[47m";

#[allow(unused)]
#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub fn line() {
    print!("{}", LINE);
}

pub use self::Color::*;

pub fn set(bold: bool, fg: Color, bg: Color) {
    if bold == false || bg == Black {
        print!("{}", RESET);
    }

    // Set emphasis
    if bold {
        print!("{}", BOLD);
    }
    // Set foreground
    print!("{}", 
        match fg {
            Black => BLACK,
            Red => RED,
            Green => GREEN,
            Yellow => YELLOW,
            Blue => BLUE,
            Magenta => MAGENTA,
            Cyan => CYAN,
            White => WHITE,
        }
    );
    // set backgrond
    print!("{}",
        match bg {
            Black => "",
            Red => BG_RED,
            Green => BG_GREEN,
            Yellow => BG_YELLOW,
            Blue => BG_BLUE,
            Magenta => BG_MAGENTA,
            Cyan => BG_CYAN,
            White => BG_WHITE,
        }
    );
//    stdout().flush().unwrap();
}

pub fn reset() {
    print!("{}", RESET);
    stdout().flush().unwrap();
}
