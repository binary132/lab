use std::io::{BufRead, Error};

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

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    BlockOpen,
    BlockClose,
}

impl<R: BufRead> Iterator for Lex<R> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.done, self.tokens.len()) {
            (true, _) => None,
            (_, 0) => self.read_more(),
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

    fn read_more(&mut self) -> Option<<Self as Iterator>::Item> {
        // Some(self.0.fill_buf())
        Some(Lexeme::BlockOpen)
    }
}
