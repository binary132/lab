// TODO: pub trait Lexeme {}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    Unknown,
    BlockOpen,
    BlockClose,
    Name(String),
}

pub enum Partial<M, D> {
    More(M),
    Done(M, D),
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

    /// Comp is a composition.
    Comp,

    /// Fin is the end of a composition.  No further input is valid.
    Fin,
}

impl Lexer for Accum {
    fn root(self) -> Self {
        Accum::Root
    }

    fn next(self, from: &[u8]) -> (Result<Self>, usize) {
        match self {
            Accum::Root => begin_comp(from),
            Accum::Comp => continue_comp(from),
            Accum::Fin => (Err("Comp finished".to_string()), 0),
        }
    }
}

fn begin_comp(from: &[u8]) -> (Result<Accum>, usize) {
    match from[0] as char {
        '{' => (
            Ok(Partial::Done(Accum::Comp, Lexeme::BlockOpen)),
            1,
        ),
        _ => (Err("must begin with BlockOpen".to_string()), 0),
    }
}

fn continue_comp(from: &[u8]) -> (Result<Accum>, usize) {
    let mut n = 0;

    for ch in from {
        if *ch as char == '}' {
            return (
                Ok(Partial::Done(Accum::Fin, Lexeme::BlockClose)),
                n,
            );
        }

        n += 1;
    }

    (Err("Unknown lexeme".to_string()), 0)
}
