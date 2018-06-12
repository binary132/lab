#[test]
fn lex_test() {
    use super::Morpha;
    use lex::lexer::{Accum, Lexeme};
    use std::io::{BufRead, Cursor, Result};

    struct Test {
        program: Box<BufRead>,
        expect: Vec<Lexeme>,
    }
    for t in vec![Test {
        program: Box::new(Cursor::new("{hello}")),
        expect: vec![
            Lexeme::BlockOpen,
            Lexeme::Name("hello".to_string()),
            Lexeme::BlockClose,
            Lexeme::Fin,
        ],
    }] {
        let result: Result<Vec<Lexeme>> =
            Morpha(Accum::Root).lex(t.program).collect();

        assert_eq!(result.unwrap(), t.expect, "correct lexemes");
    }
}
