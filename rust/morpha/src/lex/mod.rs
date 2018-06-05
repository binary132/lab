use std::collections::VecDeque;
use std::io::{BufRead, Error, Result};

#[cfg(test)]
mod mod_test;

pub mod lexer;
use self::lexer::{Lexeme, Lexer, Partial};

/// Lex implements Iterator over the underlying BufRead.  It may read
/// ahead until the end of the Reader source.  If an error occurred
/// during read, or while parsing UTF-8, it will be set in the `err`
/// field and no further read or tokenizing will be possible.
#[derive(Debug)]
pub struct Lex<'a, R: BufRead, L: 'a + lexer::Lexer> {
    done: bool,
    tokens: VecDeque<Lexeme>,
    r: R,
    state: &'a mut L,

    err: Option<Error>,
}

impl<'a, R: BufRead, L: 'a + Lexer> Iterator for Lex<'a, R, L> {
    type Item = lexer::Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.done, self.tokens.len()) {
            (true, _) => None,

            (_, 0) => if let Err(e) = self.read_more() {
                self.err = Some(e);
                return None;
            } else {
                self.tokens.pop_front()
            },

            (_, _) => self.tokens.pop_front(),
        }
    }
}
impl<'a, R: BufRead, L: 'a + Lexer> Lex<'a, R, L> {
    // TODO: Deal with incomplete Lexemes / BufRead
    pub fn err(self) -> Option<Error> {
        self.err
    }

    pub fn from(r: R, l: &'a mut L) -> Self {
        Lex {
            done: false,
            tokens: VecDeque::new(),
            r: r,
            state: l,

            err: None,
        }
    }

    // read_more consumes another buffer's worth from the BufReader and
    // fills "into" with Lexemes from it.
    fn read_more(&mut self) -> Result<()> {
        let mut count = 0;
        {
            let buf = self.r.fill_buf()?;

            // For each next Lexeme in the buffer, consume, until buffer is
            // empty or a bad Lexeme is found.
            //
            // If a partial Lexeme is found, and the buffer is empty, ingest
            // more from the BufRead.  A BufRead over an incomplete input
            // results in an intermediate state which can be reused in a new
            // call to read_more.
            while count < buf.len() {
                let (state, n) = self.state.next(&buf[count..]);
                count += match state {
                    // We have a bad Lexeme.
		    // Partial::More(Accum::Err(e)) => {
		    //     Error::new(ErrorKind::InvalidInput, e)
		    // }

		    // We have a complete Lexeme with no error.  Try
		    // to consume another.
                    Partial::Done(l) => {
                        self.tokens.push_back(l);
                        n
                    }
                    _ => 0,
                };
                // accum::Partial::More(a) => println!("more: {:?}", a),
                // accum::Partial::Done(l) => println!("lexeme {:?}", l),
            }
        }
        self.r.consume(count);

        Ok(())
    }
}
