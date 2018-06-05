use std::io::BufRead;

pub mod lex;

use lex::lexer::Lexer;

#[cfg(test)]
mod lib_test;

#[derive(Debug, PartialEq)]
pub struct Morpha<L: Lexer>(L);

impl<L: Lexer> Morpha<L> {
    /// lex creates an iterator over the tokens of the BufRead.
    pub fn lex<R: BufRead>(&mut self, r: R) -> lex::Lex<R, L> {
        lex::Lex::from(r, &mut self.0)
    }
}
