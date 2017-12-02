extern crate morpha;

#[cfg(test)]
mod test {
    #[test]
    fn hello_test() {
        use morpha::Morpha;
        use std::io::Cursor;
        use morpha::lex::Lexeme;

        let c = vec![0; 15];

        assert_eq!(
            Morpha::new().lex(Cursor::new(c)).next(),
            Some(Lexeme::BlockOpen)
        );
    }
}
