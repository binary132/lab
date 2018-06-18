use std::collections::VecDeque;
use std::io::{BufRead, Error, ErrorKind, Result};

#[cfg(test)]
mod mod_test;

pub mod accum;

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    BlockOpen,
    BlockClose,
    Name(String),
    Fin,
}

#[derive(Debug, PartialEq)]
pub enum Partial<M, D> {
    More(M),
    Done(M, D),
    Fin(M, D),
}

use std::result::Result as StdResult;
type SResult<T> = StdResult<Partial<T, Lexeme>, String>;

pub trait Lexer: Sized + Clone {
    fn root(self) -> Self;
    fn next(self, &[u8]) -> (SResult<Self>, usize);
}

/// Lex implements Iterator over the underlying BufRead.  It may read
/// ahead until the end of the Reader source.  If an error occurred
/// during read, or while parsing UTF-8, it will be set in the `err`
/// field and no further read or tokenizing will be possible.
pub struct Lex<'a, R: BufRead, L: 'a + Lexer> {
    done: bool,
    tokens: VecDeque<Lexeme>,
    r: R,
    state: &'a mut L,
}

enum IsEOF {
    Yes,
    No,
}

impl<'a, R: BufRead, L: Lexer> Iterator for Lex<'a, R, L> {
    type Item = Result<Lexeme>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.done, self.tokens.len()) {
            (true, _) => None,

            (_, 0) => match self.read_more() {
                Err(e) => Some(Err(e)),

                // Not EOF yet.
                Ok(IsEOF::No) => match self.tokens.pop_front() {
                    Some(t) => Some(Ok(t)),
                    None => None,
                },

                // EOF.
                Ok(IsEOF::Yes) => None,
            },

            (_, _) => match self.tokens.pop_front() {
                Some(t) => Some(Ok(t)),
                None => None,
            },
        }
    }
}

impl<'a, R: BufRead, L: Lexer> Lex<'a, R, L> {
    // TODO: Deal with incomplete Lexemes / BufRead
    pub fn from(r: R, l: &'a mut L) -> Self {
        Lex {
            done: false,
            tokens: VecDeque::new(),
            r: r,
            state: l,
        }
    }

    pub fn more(self, r: R) -> Self {
        Lex {
            done: self.done,
            tokens: self.tokens,
            r: r,
            state: self.state,
        }
    }

    // read_more consumes another buffer's worth from the BufReader and
    // fills "into" with Lexemes from it.  If EOF, it returns true.
    fn read_more(&mut self) -> Result<IsEOF> {
        let mut consumed = 0;
        let mut state = self.state.clone();
        let mut done = false;

        // Loop:
        //   Read if necessary, and borrow buffer from self.r
        //   In case of EOF, None and set done to true.
        //   If we've consumed the whole buffer, recur
        //   Ingest more until empty or Partial::More
        //   Store state in case we return

        {
            // Note that calling fill_buf repeatedly doesn't result in a
            // syscall every time.
            let buf = self.r.fill_buf()?;
            if buf.len() == 0 {
                self.done = true;
                return Ok(IsEOF::Yes);
            }

            while !done && consumed < buf.len() {
                // Consume from this buffer until we run out of input.
                let (next, n) = match state.next(&buf[consumed..]) {
                    (Err(e), _) => {
                        return Err(Error::new(ErrorKind::InvalidData, e))
                    }

                    // (Err("Comp finished".to_string()), _) => return Ok(IsEOF::Yes),
                    (Ok(Partial::More(a)), n) => (a, n),

                    (Ok(Partial::Done(a, l)), n) => {
                        self.tokens.push_back(l);
                        (a, n)
                    }

                    (Ok(Partial::Fin(a, l)), n) => {
                        self.tokens.push_back(l);
                        self.tokens.push_back(Lexeme::Fin);
                        done = true;
                        (a, n)
                    }
                };

                consumed += n;
                state = next;
            }
        }

        *self.state = state;
        self.r.consume(consumed);

        Ok(IsEOF::No)
    }
}
