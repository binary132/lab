use std::io::{BufRead, Error, Result};

#[cfg(test)]
mod mod_test;

/// Lex implements Iterator over the underlying BufRead.  It may read
/// ahead until the end of the Reader source.  If an error occurred
/// during read, or while parsing UTF-8, it will be set in the `err`
/// field and no further read or tokenizing will be possible.
#[derive(Debug)]
pub struct Lex<R: BufRead> {
    done: bool,
    tokens: Vec<Lexeme>,
    r: R,

    err: Option<Error>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    Unknown,
    BlockOpen,
    BlockClose,
}

impl<R: BufRead> Iterator for Lex<R> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.done, self.tokens.len()) {
            (true, _) => None,
            (_, 0) => match Lex::read_more(&mut self.r, &mut self.tokens) {
                Err(e) => {
                    self.err = Some(e);
                    None
                }
                _ => self.tokens.pop(),
            },
            (_, _) => self.tokens.pop(),
        }
    }
}

impl<R: BufRead> Lex<R> {
    pub fn err(self) -> Option<Error> {
        self.err
    }

    pub fn from(r: R) -> Self {
        Lex {
            done: false,
            tokens: Vec::new(),
            r: r,

            err: None,
        }
    }

    // read_more consumes another buffer's worth from the BufReader and
    // fills "into" with Lexemes from it.
    fn read_more(from: &mut R, into: &mut Vec<Lexeme>) -> Result<()> {
        let ln = {
            let buf = from.fill_buf()?;

            for ch in buf {
                into.push(Lex::<R>::ingest(*ch as char));
            }

            buf.len()
        };

        from.consume(ln);

        Ok(())
        // Some(Lexeme::BlockOpen)
    }

    fn ingest(ch: char) -> Lexeme {
        match ch {
            '{' => Lexeme::BlockOpen,
            '}' => Lexeme::BlockClose,
            _ => Lexeme::Unknown,
        }
    }
}
