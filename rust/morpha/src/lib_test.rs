#[test]
fn lex_test() {
    use super::Morpha;
    use lex::lexer::{Accum, Lexeme};
    use std::io::{Cursor, Result};

    let c = Cursor::new(vec![0; 15]);
    assert_eq!(
        Morpha(Accum::Root)
            .lex(c)
            .collect::<Result<Vec<Lexeme>>>()
            .unwrap(),
        vec![Lexeme::Unknown]
    );
}
