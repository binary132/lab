#[test]
fn new_test() {
    use super::Morpha;
    assert_eq!(Morpha::new(), Morpha);
}

#[test]
fn lex_test() {
    use std::io::Cursor;
    use super::Morpha;
    use lex::Lexeme;

    let c = Cursor::new(vec![0; 15]);
    assert_eq!(Morpha::new().lex(c).next(), Some(Lexeme::BlockOpen));
}
