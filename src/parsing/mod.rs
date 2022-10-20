mod parser;

pub use parser::Parser;

#[derive(Debug, PartialEq)]
pub enum ParseLineResult {
    Spawned(String, String, usize, usize),
    SpawnMatch(usize,usize,usize),
    UnhandledGameSession(String),
    Unhandled(String),
    Misc,
}
