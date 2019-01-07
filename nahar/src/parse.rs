pub struct PartsIter<'a> {
    string: &'a str,
}

impl<'a> Iterator for PartsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.string = self.string.trim();

        let mut in_string = false;

        for (i, c) in self.string.char_indices() {
            if c.is_whitespace() {
                // Everything up to the whitespace, not including it.
                let rtn = unsafe { self.string.get_unchecked(..i) };
                self.string = unsafe { self.string.get_unchecked(i..) };
                return Some(rtn);
            }
        }

        if self.string.len() == 0 {
            return None;
        } else {
            let rtn = self.string;
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
