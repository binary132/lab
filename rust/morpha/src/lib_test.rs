#[test]
fn lex_test() {
    use super::Morpha;
    use lex::lexer::{Accum, Lexeme, Lexer};
    use std::io::Cursor;

    let c = Cursor::new(vec![0; 15]);
    assert_eq!(
        Morpha(Accum::root())
            .lex(c)
            .collect::<Vec<Lexeme>>(),
        vec![Lexeme::Unknown]
    );
}
