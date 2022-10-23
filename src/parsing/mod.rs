mod parser;

pub use parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseLineResult {
    Spawned(String, String, usize, usize),
    Spawned2(String, String, usize, usize),
    SpawnMatch(usize, usize, usize),
    SpawnMatchMe(usize, usize, usize),
    Unspawn(usize),
    Success(usize, bool),
    NumPlayersAchievingObjective(usize),
    LoadedStage(String),
    UnhandledGameSession(String),
    Unhandled(String),
    Score(usize, usize),
    Start,
    Shutdown,
    Misc,
}
