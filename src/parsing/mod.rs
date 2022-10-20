mod parser;

pub use parser::Parser;

#[derive(Debug, PartialEq)]
pub enum ParseLineResult {
    Spawned(String, String, usize, usize),
    SpawnMatch(usize, usize, usize),
    Unspawn(usize),
    Success(usize, bool),
    UnhandledGameSession(String),
    Unhandled(String),
    Misc,
}
