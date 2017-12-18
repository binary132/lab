extern crate morpha;

extern crate includedir;
extern crate phf;
extern crate rlua;
extern crate rustyline;

mod lua;
pub mod loader;
pub mod repl;

/// MultiLine represents the result of an eval: either it was done (and
/// has a String representation), or it wanted more input (and has the
/// current accumulated input.)
#[derive(Debug)]
pub enum MultiLine {
    More(String),
    Done(String),
}
