extern crate morpha;

use morpha::{lex::{accum::Accum, Lexeme},
             Morpha};
use std::io::Result;

fn main() {
    // TODO: Support compiling a file or inline script a la Perl.
    // TODO: Support "language server"-like features somehow.
    // TODO: Cache intermediate results / tags etc.?  Use Sled?
    // TODO: Operate in daemon mode?
    let r = std::io::stdin();
    {
        // Obtain a lock on stdin reader.
        let r = r.lock();

        // We're going to consume the input until the comp end, or EOF.
        let mut m = Morpha(Accum::Root);

        let lex: Result<Vec<Lexeme>> = m.lex(r)
            .take_while(|t| match t {
                Ok(Lexeme::Fin) => false,
                _ => true,
            })
            .collect();
        println!("{:?}", lex);
    }
}
