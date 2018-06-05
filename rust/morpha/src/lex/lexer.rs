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
    Done(D),
}

pub trait Lexer {
    type Next;

    fn root() -> Self::Next;
    fn next(&mut self, &[u8]) -> (Partial<Self::Next, Lexeme>, usize);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Accum {
    /// Err is an Accum which has encountered a bad Lexeme.
    Err(String),

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

    fn next(&mut self, from: &[u8]) -> (Partial<Self::Next, Lexeme>, usize) {
        match match self {
            Accum::Err(_) => return (Partial::More(self.clone()), 0),
            Accum::Root => begin_comp(from),
            Accum::Comp => continue_comp(from),
        } {
            (Partial::More(Accum::Err(e)), _) => {
                (Partial::More(Accum::Err(e)), 0)
            }
            (Partial::Done(l), n) => (Partial::Done(l), n),
            (Partial::More(a), n) => (Partial::More(a), n),
        }
    }
}

fn begin_comp(from: &[u8]) -> (Partial<Accum, Lexeme>, usize) {
    match from[0] as char {
        '{' => (Partial::Done(Lexeme::BlockOpen), 0),
        _ => (
            Partial::More(Accum::Err(
                "Composition must begin with BlockOpen".to_string(),
            )),
            0,
        ),
    }
}

fn continue_comp(from: &[u8]) -> (Partial<Accum, Lexeme>, usize) {
    for ch in from {
        match *ch as char {
            '}' => return (Partial::Done(Lexeme::BlockClose), 1),
            _ => {
                return (
                    Partial::More(Accum::Err(format!("Unknown lexeme {}", ch))),
                    0,
                )
            }
        }
    }

    (
        Partial::More(Accum::Err("Unknown lexeme".to_string())),
        0,
    )
}
