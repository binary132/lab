use std::io::BufRead;

pub mod lex;

use lex::lexer::Lexer;

#[cfg(test)]
mod lib_test;

#[derive(Debug, PartialEq)]
pub struct Morpha<L: Lexer>(pub L);

impl<L: Lexer> Morpha<L> {
    /// lex creates an iterator over the tokens of the BufRead.
    pub fn lex<'a, R: BufRead>(self, r: R) -> lex::Lex<'a, R, L> {
        lex::Lex::from(r, &mut self.0)
    }
}
