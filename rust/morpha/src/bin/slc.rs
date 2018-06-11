extern crate morpha;

use morpha::{lex::lexer::{Accum, Lexeme},
             Morpha};
use std::io::Result;

fn main() {
    // TODO: Support compiling a file or inline script a la Perl.
    // TODO: Support language server features somehow.
    // TODO: Cache intermediate results / tags etc. between runs?
    // TODO: Operate in daemon mode?
    let r = std::io::stdin();
    {
        // Obtain a lock on stdin reader.
        let r = r.lock();

        // We're going to consume the input until the composition end.
        let mut m = Morpha(Accum::Root);

        let lex: Result<Vec<Lexeme>> = m.lex(r).collect();
        println!("{:?}", lex);
    }
}
