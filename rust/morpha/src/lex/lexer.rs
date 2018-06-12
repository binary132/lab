// TODO: pub trait Lexeme {}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    BlockOpen,
    BlockClose,
    Name(String),
    Fin,
}

pub enum Partial<M, D> {
    More(M),
    Done(M, D),
    Fin(M, D),
}

use std::result;
type Result<T> = result::Result<Partial<T, Lexeme>, String>;

pub trait Lexer: Sized + Clone {
    fn root(self) -> Self;
    fn next(self, &[u8]) -> (Result<Self>, usize);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Accum {
    /// Root is an Accum which hasn't yet begun to accumulate a Lexeme.
    Root,

    /// Comp is a composition.  Its value is the comp's depth.
    Comp(u32),

    /// Name is some string name.
    Name(Name),

    /// Fin is the end of a composition.  No further input is valid.
    Fin,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    acc: String,
    prev: Box<Accum>,
}

impl Lexer for Accum {
    fn root(self) -> Self {
        Accum::Root
    }

    fn next(self, from: &[u8]) -> (Result<Self>, usize) {
        match self {
            Accum::Root => begin_comp(from),
            Accum::Comp(n) => continue_comp(from, n),
            Accum::Name(n) => n.ingest(from),
            Accum::Fin => (Err("Comp finished".to_string()), 0),
        }
    }
}

fn begin_comp(from: &[u8]) -> (Result<Accum>, usize) {
    match from[0] as char {
        '{' => (
            Ok(Partial::Done(Accum::Comp(0), Lexeme::BlockOpen)),
            1,
        ),
        _ => (Err("must begin with BlockOpen".to_string()), 0),
    }
}

fn continue_comp(from: &[u8], depth: u32) -> (Result<Accum>, usize) {
    let mut n = 0;

    // There are a few possibilities when in a Comp accumulator.
    for ch in from {
        match *ch as char {
            '{' => {
                return (
                    Ok(Partial::Done(
                        Accum::Comp(depth + 1),
                        Lexeme::BlockOpen,
                    )),
                    n + 1,
                )
            }

            '}' => {
                return (
                    Ok({
                        let l = Lexeme::BlockClose;
                        match depth {
                            0 => Partial::Fin(Accum::Fin, l),
                            _ => Partial::Done(Accum::Comp(depth - 1), l),
                        }
                    }),
                    n + 1,
                )
            }

            'a'...'z' | 'A'...'Z' => {
                return (
                    Ok(Partial::More(Accum::Name(Name {
                        acc: String::new(),
                        prev: Box::new(Accum::Comp(depth)),
                    }))),
                    n,
                )
            }

            _ => (),
        }

        n += 1;
    }

    (Err("Unknown lexeme".to_string()), 0)
}

impl Name {
    fn ingest(mut self, from: &[u8]) -> (Result<Accum>, usize) {
        use std::str;
        let mut n = 0;
        let mut done = false;

        for ch in from {
            if done {
                break;
            };
            match *ch as char {
                'a'...'z' | 'A'...'Z' | '_' => n += 1,
                _ => done = true,
            }
        }

        self.acc
            .push_str(str::from_utf8(&from[0..n]).unwrap());
        (
            Ok(Partial::Done(
                *self.prev,
                Lexeme::Name(self.acc),
            )),
            n,
        )
    }
}
