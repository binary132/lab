use super::{Accum, Result};
use lex::{Lexeme, Lexer, Partial};

#[test]
fn accum_lexer_root_test() {
    assert_eq!(Accum::Root.root(), Accum::Root);
}

#[test]
fn accum_lexer_next_test() {
    struct Test<'a> {
        should: &'a str,
        given: Accum,
        with: &'a [u8],
        expect: Result<Accum>,
        expect_n: usize,
    }

    vec![
        Test {
            should: "return error for Root::next(x)",
            given: Accum::Root,
            with: b"x",
            expect: Err("must begin with BlockOpen".to_string()),
            expect_n: 0,
        },
        Test {
            should: "return Accum::Comp(0) for Root::next({)",
            given: Accum::Root,
            with: b"{",
            expect: Ok(Partial::Done(Accum::Comp(0), Lexeme::BlockOpen)),
            expect_n: 1,
        },
        Test {
            should: "return Accum::Comp(1) for root()",
            given: Accum::Root,
            with: b"{",
            expect: Ok(Partial::Done(Accum::Comp(0), Lexeme::BlockOpen)),
            expect_n: 1,
        },
    ].into_iter()
        .for_each(|t| {
            println!("should {}", t.should);
            let (next, n) = t.given.next(t.with);

            match (next, t.expect) {
                (Ok(next), Ok(exp_next)) => {
                    assert_eq!(next, exp_next, "next step matched");
                    assert_eq!(n, t.expect_n, "next count matched");
                }
                (Err(got_err), Err(exp_err)) => {
                    assert_eq!(got_err, exp_err);
                    assert_eq!(n, t.expect_n, "next count was 0");
                }
                (got, expect) => panic!(
                    "expected {:?}, but got ({:?}, {:?})",
                    expect, got, n
                ),
            }
        });
}
