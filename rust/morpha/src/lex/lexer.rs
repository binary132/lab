// TODO: pub trait Lexeme {}e

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    Unknown,
    BlockOpen,
    BlockClose,
    Name(String),
}

pub enum Partial<M, D> {
    More(M),
    Done(D),
}

use std::result;
type Result = result::Result<Partial<Accum, Lexeme>, String>;

pub trait Lexer {
    type Next;

    fn root() -> Self::Next;
    fn next(&mut self, &[u8]) -> (Self::Next, usize);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Accum {
    /// Root is an Accum which hasn't yet begun to accumulate a Lexeme.
    Root,

    /// Comp is a composition.
    Comp,
}

impl Lexer for Accum {
    type Next = Accum;

    fn root() -> Self {
        Accum::Root
    }

    fn next(&mut self, from: &[u8]) -> (Result, usize) {
        match self {
            Accum::Root => begin_comp(from),
            Accum::Comp => continue_comp(from),
        }
    }
}

fn begin_comp(from: &[u8]) -> (Result, usize) {
    match from[0] as char {
        '{' => (Ok(Partial::Done(Lexeme::BlockOpen)), 1),
        _ => (Err("must begin with BlockOpen".to_string()), 0),
    }
}

fn continue_comp(from: &[u8]) -> (Result, usize) {
    let mut n = 0;

    for ch in from {
        if *ch as char == '}' {
            return (Ok(Partial::Done(Lexeme::BlockClose)), n);
            // _ => return (Err(format!("Unknown lexeme {}", ch)), n),
        }

        n += 1;
    }

    (Err("Unknown lexeme".to_string()), 0)
}
