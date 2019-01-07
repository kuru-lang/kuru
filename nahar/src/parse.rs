pub struct PartsIter<'a> {
    string: &'a str,
}

impl<'a> Iterator for PartsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.string = self.string.trim();

        let mut open = false;
        let mut opened = None;

        for (i, c) in self.string.char_indices() {
            if c.is_whitespace() && open == false {
                // Everything up to the whitespace, not including it.
                let rtn = if let Some(op) = opened {
                    op
                } else {
                    unsafe { self.string.get_unchecked(..i) }
                };
                self.string = unsafe { self.string.get_unchecked(i..) };
                return Some(rtn);
            } else if c == '"' {
                if open {
                    opened = Some(unsafe { self.string.get_unchecked(1..i) });
                }
                open = !open;
            }
        }

        if self.string.len() == 0 {
            return None;
        } else {
            let rtn = if let Some(op) = opened {
                op
            } else {
                self.string
            };
            self.string = unsafe { self.string.get_unchecked(..0) };
            return Some(rtn);
        }
    }
}

/// Parse a function call.
///
/// `function arg_1 "arg 2" (arg + 3)`
pub fn parse<'a>(string: &'a str) -> PartsIter<'a> {
    PartsIter {
        string,
    }
}
