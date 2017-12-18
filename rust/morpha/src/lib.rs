use std::io::BufRead;

pub mod lex;

#[cfg(test)]
mod lib_test;

#[derive(Debug, PartialEq)]
pub struct Morpha;

impl Morpha {
    pub fn new() -> Self {
        Morpha
    }

    /// lex consumes the next token from the given Reader.
    pub fn lex<R: BufRead>(&self, r: R) -> lex::Lex<R> {
        lex::Lex::from(r)
    }
}
