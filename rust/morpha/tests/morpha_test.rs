extern crate morpha;

#[cfg(test)]
mod test {
    #[test]
    fn hello_test() {
        use morpha::{lex::lexer::{Accum, Lexeme},
                     Morpha};
        use std::io::{Cursor, Result};

        let c = Cursor::new("{hello}");
        let mut m = Morpha(Accum::Root);
        let l = m.lex(c);

        assert_eq!(
            l.collect::<Result<Vec<Lexeme>>>().unwrap(),
            vec![
                Lexeme::BlockOpen,
                Lexeme::Name("hello".to_string()),
                Lexeme::BlockClose,
                Lexeme::Fin,
            ]
        );
    }
}
