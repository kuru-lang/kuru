use std::io::BufRead;

enum Version {
    /// v0.1.0 => the first version
    V0_1_0,
}

fn parse_args() -> (std::fs::File, Version) {
    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(arg);
    }
    if args.len() != 2 {
        eprintln!("Must have exactly 2 arguments (version, filename)");
        std::process::exit(1);
    }
    let filename = args.pop().unwrap();
    let file = if let Ok(file) = std::fs::File::open(&filename) {
        file
    } else {
        eprintln!("Couldn't find file {}", filename);
        std::process::exit(1);
    };
    let version = args.pop().unwrap();
    let version = match version.as_str() {
        "0.1.0" => Version::V0_1_0,
        version => {
            eprintln!("Unknown version: {}", version);
            std::process::exit(1);
        }
    };
    (file, version)
}

fn main() -> std::io::Result<()> {
    // Read the file passed in via command line arguments
    let (file, _version) = parse_args();
    // Buffer the file reader.
    let mut file = std::io::BufReader::new(file);
    // Go through each line of the file.
    let mut line = String::new();
    while file.read_line(&mut line)? != 0 {
        line.pop();
        // Go through each token on the line.
        for token in Tokens::new(&line) {
            match token {
                Ok(token) => {
                    dbg!(token);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        line.clear();
    }
    // Succeeded
    Ok(())
}

/// A token.
#[derive(Debug)]
enum Token<'a> {
    /// `#\n`, `# #` or `###\n###` style comment block.
    Comment(&'a str),
    /// Identifier
    Ident(&'a str),
    /// Text
    Text(&'a str),
    /// List start token `[`
    ListStart,
    /// List end token `]`
    ListEnd,
    /// Set start token `{`
    SetStart,
    /// Set end token `}`
    SetEnd,
    /// Tuple start token `(`
    TupleStart,
    /// Tuple end token `)`
    TupleEnd,
    /// Separator token `,`
    Separator,
    /// Try Operator `?`
    Try,
    /// Reference Operator `@`
    Ref,
    /// Assignment Operator `:`
    Assign,
    /// Access Operator `.`
    Access,
}

/// Token iterator on a line.
struct Tokens<'a> {
    text: &'a str,
}

impl<'a> Tokens<'a> {
    fn new(text: &'a str) -> Self {
        Tokens {
            text
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let original = self.text;
        let chr = self.text.chars().next()?;
        self.text = &self.text[chr.len_utf8()..];
        match chr {
            // Space (skip).
            ' ' => self.next(),
            // Comment types.
            '#' => {
                if let Some(index) = self.text.find('#') {
                    // Note comment
                    let comment_text = &self.text[..index];
                    self.text = &self.text[index+1..];
                    Some(Ok(Token::Comment(comment_text)))
                } else {
                    // Line comment
                    let comment_text = self.text;
                    self.text = &self.text[self.text.len()..];
                    Some(Ok(Token::Comment(comment_text)))
                }
            }
            // Text
            '"' => {
                if let Some(index) = self.text.find('"') {
                    let text = &self.text[..index];
                    self.text = &self.text[index+1..];
                    Some(Ok(Token::Text(text)))
                } else {
                    self.text = &self.text[self.text.len()..];
                    Some(Err(Error::Text(original)))
                }
            },
            // Standalone character tokens.
            '[' => Some(Ok(Token::ListStart)),
            ']' => Some(Ok(Token::ListEnd)),
            '{' => Some(Ok(Token::SetStart)),
            '}' => Some(Ok(Token::SetEnd)),
            '(' => Some(Ok(Token::TupleStart)),
            ')' => Some(Ok(Token::TupleEnd)),
            '?' => Some(Ok(Token::Try)),
            ',' => Some(Ok(Token::Separator)),
            '@' => Some(Ok(Token::Ref)),
            ':' => Some(Ok(Token::Assign)),
            '.' => Some(Ok(Token::Access)),
            
            '~' => todo!(),
            '!' => todo!(),
            '`' => todo!(),
            '$' => todo!(),
            '%' => todo!(),
            '^' => todo!(),
            '&' => todo!(),
            '*' => todo!(),
            '-' => todo!(),
            '=' => todo!(),
            '+' => todo!(),
            '\\' => todo!(),
            '|' => todo!(),
            ';' => todo!(),
            '\'' => todo!(),
            '<' => todo!(),
            '>' => todo!(),
            '/' => todo!(),
            // Ident
            x if x.is_alphanumeric() => {
                for (i, ch) in self.text.char_indices() {
                    if !ch.is_alphanumeric() {
                        let ident_text = &original[..i+x.len_utf8()];
                        self.text = &self.text[i..];
                        return Some(Ok(Token::Ident(ident_text)));
                    }
                }
                let ident_text = self.text;
                self.text = &self.text[self.text.len()..];
                Some(Ok(Token::Ident(ident_text)))
            }
            // Unknown
            _ => {
                let unknown_text = self.text;
                self.text = &self.text[self.text.len()..];
                Some(Err(Error::Unknown(unknown_text)))
            }
        }
    }
}

enum Error<'a> {
    // Unknown / Unexpected Symbol
    Unknown(&'a str),
    // Unterminated text.
    Text(&'a str),
}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Unknown(symbol) => write!(f, "Unexpected symbol: {}", symbol),
            Text(text) => write!(f, "You're missing a closing double quote (\"): {}", text),
        }
    }
}
