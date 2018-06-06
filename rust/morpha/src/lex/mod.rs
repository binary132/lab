use std::collections::VecDeque;
use std::io::{BufRead, Error, ErrorKind, Result};

#[cfg(test)]
mod mod_test;

pub mod lexer;
use self::lexer::{Lexeme, Lexer, Partial};

/// Lex implements Iterator over the underlying BufRead.  It may read
/// ahead until the end of the Reader source.  If an error occurred
/// during read, or while parsing UTF-8, it will be set in the `err`
/// field and no further read or tokenizing will be possible.
#[derive(Debug)]
pub struct Lex<'a, R: BufRead> {
    done: bool,
    tokens: VecDeque<Lexeme>,
    r: R,
    state: Box<&'a mut Lexer>,
}

impl<'a, R: BufRead, L: 'a + Lexer> Iterator for Lex<'a, R, L> {
    type Item = Result<lexer::Lexeme>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.done, self.tokens.len()) {
            (true, _) => None,

            (_, 0) => match self.read_more() {
                Err(e) => Some(Err(e)),
                _ => Some(Ok(self.tokens.pop_front()?)),
            },

            (_, _) => Some(Ok(self.tokens.pop_front()?)),
        }
    }
}

impl<'a, R: BufRead, L: 'a + Lexer> Lex<'a, R, L> {
    // TODO: Deal with incomplete Lexemes / BufRead
    pub fn from(r: R, l: &'a mut L) -> Self {
        Lex {
            done: false,
            tokens: VecDeque::new(),
            r: r,
            state: Box::new(l),
        }
    }

    // read_more consumes another buffer's worth from the BufReader and
    // fills "into" with Lexemes from it.
    fn read_more(&mut self) -> Result<()> {
        let mut count = 0;
        {
            let mut buf = self.r.fill_buf()?;
            if buf.len() == 0 {
                self.done = true;
                return Ok(());
            }

            let mut state = self.state;

            while count < buf.len() {
                let (state, n) = state.next(&buf[count..]);

                match state {
                    // Bad lexeme.
                    Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),

                    // Incomplete lexeme at the end.  Read more, unless EOF.
                    Ok(Partial::More(a)) => {
                        let bb = self.r.fill_buf()?;
                        if bb.len() == 0 {
                            self.done = true;
                            // TODO: Store next lexer step.
                            self.state = Box::new(a);
                            return Err(Error::from(ErrorKind::UnexpectedEof));
                        }
                    }

                    // Complete lexeme.
                    Ok(Partial::Done(l)) => self.tokens.push_back(l),
                }
                count += n;
            }
        }
        self.r.consume(count);

        Ok(())
    }

    fn append(buf: &mut [u8], r: R) -> Result<()> {
        let bb = r.fill_buf()?;
        if bb.len() == 0 {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        buf.concat(bb);
        Ok(())
    }
}
