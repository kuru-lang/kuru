use std::io::prelude::*;
use std::process;

pub(super) struct CharStream<R: Read> {
    reader: R,
}

impl<R: Read> CharStream<R> {
    pub(super) fn new(reader: R) -> Self {
        CharStream { reader }
    }
}

impl<R: Read> Iterator for CharStream<R> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c = [0u8; 4];
        let mut ch = None;
        for i in 0..4 {
            if self.reader.read_exact(&mut c[i..i + 1]).is_err() {
                if i == 0 {
                    return None;
                }
                eprintln!("Unexpected end of file!");
                process::exit(1);
            }
            match std::str::from_utf8(&c[..i + 1]) {
                Ok(c) => {
                    ch = c.chars().next();
                    break;
                }
                Err(_e) if _e.error_len().is_some() => {
                    eprintln!("Input file is not valid UTF-8!");
                    process::exit(1);
                }
                Err(_) => { /* Continue */ }
            }
        }
        // FIXME: can prove always Some?, optimization
        Some(ch.unwrap())
    }
}
