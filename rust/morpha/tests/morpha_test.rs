extern crate morpha;

#[cfg(test)]
mod test {
    #[test]
    fn hello_test() {
        use morpha::{lex::lexer::{Accum, Lexeme},
                     Morpha};
        use std::io::{Cursor, Result};

        let c = vec![0; 15];
        let mut m = Morpha(Accum::Root);
        let l = m.lex(Cursor::new(c));

        assert_eq!(
            l.collect::<Result<Vec<Lexeme>>>().unwrap(),
            vec![Lexeme::BlockOpen; 15]
        );
    }
}
