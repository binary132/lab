#[test]
fn lex_test() {
    use super::Morpha;
    use lex::lexer::{Accum, Lexeme, Lexer};
    use std::io::{Cursor, Result};

    let c = Cursor::new(vec![0; 15]);
    assert_eq!(
        Morpha(Accum::root())
            .lex(c)
            .collect::<Result<Vec<Lexeme>>>()
            .unwrap(),
        vec![Lexeme::Unknown]
    );
}
