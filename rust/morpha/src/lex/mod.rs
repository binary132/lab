use std::io::BufRead;

#[cfg(test)]
mod mod_test;

#[derive(Debug, PartialEq)]
pub struct Lex<R: BufRead>(R);

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    BlockOpen,
    BlockClose,
}

impl<R> Iterator for Lex<R>
where
    R: BufRead,
{
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        // Some(self.0.fill_buf())
        Some(Lexeme::BlockOpen)
    }
}

impl<R> Lex<R>
where
    R: BufRead,
{
    pub fn from(r: R) -> Lex<R> {
        Lex(r)
    }
}
